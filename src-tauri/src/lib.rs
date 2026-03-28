use std::{
    path::PathBuf,
    sync::Mutex,
    time::{Duration, Instant},
};

use notify::Result as NotifyResult;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};



//?use tauri::menu::{MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder};

use tauri::{AppHandle, Emitter, Runtime, State, Manager};

use tauri::menu::{MenuId, MenuItemKind, CheckMenuItem};

//use tauri_plugin_dialog::DialogExt;



fn find_menu_item_recursive<R: Runtime>(
items: Vec<MenuItemKind<R>>,
target: &MenuId,
) -> tauri::Result<Option<MenuItemKind<R>>> {
for item in items {
	if *target == item.id() {
	return Ok(Some(item));
	}

	if let MenuItemKind::Submenu(sm) = &item {
	let children = sm.items()?;
	if let Some(found) = find_menu_item_recursive(children, target)? {
		return Ok(Some(found));
	}
	}
}
Ok(None)
}






#[tauri::command]
fn set_menu_item_enabled<R: Runtime>(
  app: AppHandle<R>,
  window: String,
  id: String,
  enabled: bool,
) -> Result<(), String> {
  // Prefer app menu (macOS global menu bar)
  let menu = if let Some(menu) = app.menu() {
    menu
  } else {
    // Fallback: window menu
    let win = app
      .get_webview_window(&window)
      .ok_or_else(|| format!("No such window: {window}"))?;

    win.menu()
      .ok_or_else(|| "No menu found on window (and no app menu)".to_string())?
  };

  let target = MenuId::new(id.clone());

  let root_items = menu.items().map_err(|e| e.to_string())?;
  let item = find_menu_item_recursive(root_items, &target)
    .map_err(|e| e.to_string())?
    .ok_or_else(|| format!("Menu item not found: {id}"))?;

  match item {
    MenuItemKind::MenuItem(mi) => mi.set_enabled(enabled).map_err(|e| e.to_string())?,
    MenuItemKind::Check(ci) => ci.set_enabled(enabled).map_err(|e| e.to_string())?,
    MenuItemKind::Icon(ii) => ii.set_enabled(enabled).map_err(|e| e.to_string())?,
    MenuItemKind::Submenu(sm) => sm.set_enabled(enabled).map_err(|e| e.to_string())?,
    MenuItemKind::Predefined(_) => {
      return Err("Cannot enable/disable predefined menu items".into());
    }
  }

  Ok(())
}




// ---- existing greet command ----
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

// ---- single watcher slot state ----
// We keep exactly ONE active watcher at a time.
// Opening a new file replaces (drops) the previous watcher automatically.
struct WatcherState(Mutex<Option<RecommendedWatcher>>);

// Start watching one file path; emits "file-changed" events to JS.
#[tauri::command]
fn start_watch_file(
	app: AppHandle,
	state: State<WatcherState>,
	path: String,
) -> Result<(), String> {
	let pathbuf = PathBuf::from(&path);
	
	// Debounce bursts/atomic saves (e.g. editors writing temp files).
	let mut last_emit = Instant::now()
	.checked_sub(Duration::from_millis(1000))
	.unwrap_or_else(Instant::now);
	
	let app_clone = app.clone();
	let path_clone = path.clone();
	
	let mut watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| {
		if res.is_err() {
			return;
		}
		
		let now = Instant::now();
		if now.duration_since(last_emit) < Duration::from_millis(150) {
			return;
		}
		last_emit = now;
		
		// Emit event to the frontend with the path as payload
		let _ = app_clone.emit("file-changed", path_clone.clone());
	})
	.map_err(|e| e.to_string())?;
	
	watcher
	.watch(&pathbuf, RecursiveMode::NonRecursive)
	.map_err(|e| e.to_string())?;
	
	// Replace any existing watcher (dropping it stops watching the old file)
	let mut slot = state
	.0
	.lock()
	.map_err(|_| "Watcher lock poisoned".to_string())?;
	*slot = Some(watcher);
	
	Ok(())
}

/*--
#[tauri::command]
fn stop_watch_file(state: tauri::State<WatcherState>) {
    let mut watcher = state.0.lock().unwrap();
    *watcher = None;
}
--*/
#[tauri::command]
fn stop_watch_file(state: State<WatcherState>) -> Result<(), String> {
	let mut watcher = state
		.0
		.lock()
		.map_err(|_| "Watcher lock poisoned".to_string())?;
	
	*watcher = None;
	
	Ok(())
}







fn build_app_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<tauri::menu::Menu<R>> {
	use tauri::menu::{MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder};
	
// ---- App submenu (will only be on macOS: treats first submenu as the "App" menu) ----
	let about = MenuItem::with_id(app, "app.about", "About OutlineBoard", true, None::<&str>)?;
	let settings = MenuItem::with_id(app, "app.settings", "Settings…", true, Some("CmdOrCtrl+,"))?;
	let quit = MenuItem::with_id(app, "app.quit", "Quit OutlineBoard", true, Some("CmdOrCtrl+Q"))?;
	
	let app_submenu = SubmenuBuilder::new(app, "OutlineBoard")
		.item(&about)
		.separator()
		.item(&settings)
		.separator()
		.item(&quit)
		.build()?;



// ---- File submenu ----

	let new = MenuItem::with_id(app, "file.new", "New", true, Some("CmdOrCtrl+N"))?;
	let open = MenuItem::with_id(app, "file.open", "Open…", true, Some("CmdOrCtrl+O"))?;
	let saveas = MenuItem::with_id(app, "file.saveas", "Save as…", true, Some("CmdOrCtrl+Shift+S"))?; 
	let pincurrent = MenuItem::with_id(app, "file.pincurrent", "Pin current file", true, Some("CmdOrCtrl+D"))?;
	let settings_b = MenuItem::with_id(app, "file.settings", "Settings… [Win+Linux]", true, Some("CmdOrCtrl+,"))?;
	let exit = MenuItem::with_id(app, "file.exit", "Exit [Win+Linux]", true, Some("Alt+F4"))?;

	let file_submenu = if cfg!(target_os = "macos") {
		// macOS — no Exit in File menu
		SubmenuBuilder::new(app, "File")
			.item(&new)
			.item(&open)
			.item(&saveas)
			.item(&pincurrent)
					.separator()		// nb: on build: remove (only for testing Windows style)
					.item(&settings_b)	// nb: on build: remove (only for testing Windows style)
					.separator()		// nb: on build: remove (only for testing Windows style)
					.item(&exit)		// nb: on build: remove (only for testing Windows style)
			.build()?
	}
	else {
		// Windows/Linux — include Exit
		SubmenuBuilder::new(app, "File")
			.item(&new)
			.item(&open)
			.item(&saveas)
			.item(&pincurrent)
			.separator()
			.item(&settings_b)
			.separator()
			.item(&exit)
			.build()?
	};



// ---- Edit submenu ----

	// PredefinedMenuItem gives native behavior + symbols (esp. on macOS)
	let delete = MenuItem::with_id(app, "edit.delete", "Delete", true, None::<&str>)?;
	
	let edit_submenu = SubmenuBuilder::new(app, "Edit")
		.item(&PredefinedMenuItem::undo(app, None)?)
		.item(&PredefinedMenuItem::redo(app, None)?)
		.separator()
		.item(&PredefinedMenuItem::cut(app, None)?)
		.item(&PredefinedMenuItem::copy(app, None)?)
		.item(&PredefinedMenuItem::paste(app, None)?)
		.item(&delete)
		.separator()
		.item(&PredefinedMenuItem::select_all(app, None)?)
		.build()?;



// ---- View submenu ----

		// submenu chart view ('structure', 'tags')
		let menuitem_chartview_a = CheckMenuItem::with_id(app, "view.chart.viewa", "Structure", true, true, Some("F1"), )?;
		let menuitem_chartview_b = CheckMenuItem::with_id(app, "view.chart.viewb", "Tags", true, false, Some("F2"), )?;

		// Build the sub-submenu
		let submenu_chartview = SubmenuBuilder::new(app, "Chart view")
			.item(&menuitem_chartview_a)
			.item(&menuitem_chartview_b)
			.build()?;


		// zoom normal submenu items
		let zoom_normal_in = MenuItem::with_id(app, "view.zoom.normal.in", "Zoom in [⌘ +]", true, None::<&str>)?;
		let zoom_normal_out = MenuItem::with_id(app, "view.zoom.normal.out", "Zoom out [⌘ -]", true, None::<&str>)?;
		let zoom_normal_reset = MenuItem::with_id(app, "view.zoom.normal.reset", "Actual size [⌘ 0]", true, None::<&str>)?;

		// Build the sub-submenu
		let zoom_normal_submenu = SubmenuBuilder::new(app, "Zoom")
			.item(&zoom_normal_in)
			.item(&zoom_normal_out)
			.item(&zoom_normal_reset)
			.build()?;


		// zoom width submenu items
		/*--let zoom_width_in = MenuItem::with_id(app, "view.zoom.width.in", "Widen columns [⌥ +]", true, None::<&str>)?;
		let zoom_width_out = MenuItem::with_id(app, "view.zoom.width.out", "Narrow columns [⌥ -]", true, None::<&str>)?;
		//--let zoom_width_reset = MenuItem::with_id(app, "view.zoom.width.reset", "Reset Width [⌥ 0]", true, None::<&str>)?;

		// Build the sub-submenu
		let zoom_width_submenu = SubmenuBuilder::new(app, "Timeline width")
			.item(&zoom_width_in)
			.item(&zoom_width_out)
			//--.item(&zoom_width_reset)
			.build()?;


		// zoom lines submenu items
		let zoom_lines_in = MenuItem::with_id(app, "view.zoom.lines.in", "More lines [⌥ ⌘ +]", true, None::<&str>)?;
		let zoom_lines_out = MenuItem::with_id(app, "view.zoom.lines.out", "Fewer lines [⌥ ⌘ -]", true, None::<&str>)?;
		//--let zoom_lines_reset = MenuItem::with_id(app, "view.zoom.lines.reset", "Reset Lines [⌥ ⌘ 0]", true, None::<&str>)?;

		// Build the sub-submenu
		let zoom_lines_submenu = SubmenuBuilder::new(app, "Lines shown")
			.item(&zoom_lines_in)
			.item(&zoom_lines_out)
			//--.item(&zoom_lines_reset)
			.build()?;--*/

		// zoom UI submenu items
		let zoom_ui_in = MenuItem::with_id(app, "view.zoom.ui.in", "Zoom in [⇧ ⌘ +]", true, None::<&str>)?;
		let zoom_ui_out = MenuItem::with_id(app, "view.zoom.ui.out", "Zoom out [⇧ ⌘ -]", true, None::<&str>)?;
		let zoom_ui_reset = MenuItem::with_id(app, "view.zoom.ui.reset", "Actual size [⇧ ⌘ 0]", true, None::<&str>)?;

		// Build the sub-submenu
		let zoom_ui_submenu = SubmenuBuilder::new(app, "UI zoom")
			.item(&zoom_ui_in)
			.item(&zoom_ui_out)
			.item(&zoom_ui_reset)
			.build()?;			


	// --- Theme submenu (checkmarks) ---
	let theme_system = CheckMenuItem::with_id( app, "view.theme.system", "System", true, true, Some("CmdOrCtrl+Shift+F1"), )?;             // checked by default (pick what you want at startup) None::<&str>, 
	let theme_light = CheckMenuItem::with_id( app, "view.theme.light", "Light", true, false, Some("CmdOrCtrl+Shift+F2"), )?;
	let theme_dark = CheckMenuItem::with_id( app, "view.theme.dark", "Dark", true, false, Some("CmdOrCtrl+Shift+F3"), )?;

	let theme_submenu = SubmenuBuilder::new(app, "Theme")
		.item(&theme_system)
		.item(&theme_light)
		.item(&theme_dark)
		.build()?;


	// --- Editor Position submenu (checkmarks) ---
	let sidebar_position_left = CheckMenuItem::with_id( app, "view.sidebar.position.left", "Left", true, true, None::<&str>)?;             // checked by default (pick what you want at startup) None::<&str>, 
	let sidebar_position_right = CheckMenuItem::with_id( app, "view.sidebar.position.right", "Right", true, false, None::<&str>)?;
	let sidebar_position_top = CheckMenuItem::with_id( app, "view.sidebar.position.top", "Top", true, false, None::<&str>)?;
	let sidebar_position_bottom = CheckMenuItem::with_id( app, "view.sidebar.position.bottom", "Bottom", true, false, None::<&str>)?;

	let sidebar_position_submenu = SubmenuBuilder::new(app, "Editor position")
		.item(&sidebar_position_left)
		.item(&sidebar_position_right)
		.item(&sidebar_position_top)
		.item(&sidebar_position_bottom)
		.build()?;


	// --- Editor Position submenu (checkmarks) ---
	let graph_height_a = CheckMenuItem::with_id( app, "view.graph.height.a", "25%", true, true, None::<&str>)?;             // checked by default (pick what you want at startup) None::<&str>, 
	let graph_height_b = CheckMenuItem::with_id( app, "view.graph.height.b", "50%", true, false, None::<&str>)?;
	let graph_height_c = CheckMenuItem::with_id( app, "view.graph.height.c", "75%", true, false, None::<&str>)?;
	let graph_height_d = CheckMenuItem::with_id( app, "view.graph.height.d", "100%", true, false, None::<&str>)?;

	let graph_height_submenu = SubmenuBuilder::new(app, "Graph height")
		.item(&graph_height_a)
		.item(&graph_height_b)
		.item(&graph_height_c)
		.item(&graph_height_d)
		.build()?;



	let toggle_sidebar = CheckMenuItem::with_id(app, "view.sidebar.toggle", "Show editor", true, true, Some("CmdOrCtrl+E"))?;
	let toggle_ribbon = CheckMenuItem::with_id(app, "view.toolbar.toggle", "Show ribbon", true, true, Some("CmdOrCtrl+R"))?;
	let toggle_graph = CheckMenuItem::with_id(app, "view.graph.toggle", "Show graphs", true, true, Some("CmdOrCtrl+G"))?;

	let view_submenu = SubmenuBuilder::new(app, "View")
		.item(&toggle_sidebar)
		.item(&toggle_ribbon)
		.item(&toggle_graph)
		.separator()
		.item(&zoom_normal_submenu)
		//--.item(&zoom_width_submenu)
		//--.item(&zoom_lines_submenu)
		.separator()
		.item(&zoom_ui_submenu)
		.separator()
		.item(&submenu_chartview)
		.item(&graph_height_submenu)
		.item(&sidebar_position_submenu)
		.item(&theme_submenu)
		.separator()
		.build()?;


// ---- Help submenu ----
	let docs = MenuItem::with_id(app, "help.docs", "Documentation…", true, None::<&str>)?;
	let website = MenuItem::with_id(app, "help.website", "Website…", true, None::<&str>)?;

	let help_submenu = SubmenuBuilder::new(app, "Help")
		.item(&docs)
		.item(&website)
		.build()?;



	// Build final menu
	let menu = if cfg!(target_os = "macos") {
		MenuBuilder::new(app)
			.items(&[
				&app_submenu,
				&file_submenu,
				&edit_submenu,
				&view_submenu,
				&help_submenu,
			])
			.build()?
	} else {
		MenuBuilder::new(app)
			.items(&[
				&file_submenu,
				&edit_submenu,
				&view_submenu,
				&help_submenu,
			])
			.build()?
	};


		
	// Important on macOS; harmless elsewhere
	let _ = menu.set_as_app_menu();
		
	Ok(menu)
}




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
	.plugin(tauri_plugin_opener::init())
	.plugin(tauri_plugin_dialog::init())
	.plugin(tauri_plugin_fs::init())
	.plugin(tauri_plugin_persisted_scope::init())
	.plugin(tauri_plugin_clipboard_manager::init())
	.manage(WatcherState(Mutex::new(None)))
	.invoke_handler(tauri::generate_handler![greet, start_watch_file, stop_watch_file, set_menu_item_enabled, set_menu_checks, write_clipboard])

	.setup(|app| {
		let win = app
		.get_webview_window("main")
		.ok_or_else(|| tauri::Error::WindowNotFound)?;
		
		// Build menu once
		let menu = build_app_menu(app.handle())?;
		
		// ✅ Important for Windows/Linux accelerators:
		win.set_menu(menu.clone())?;
		
		// ✅ Keep this for macOS global menu bar (optional but nice)
		#[cfg(target_os = "macos")]
		{
			app.set_menu(menu)?;
		}
		
		// (optional) DevTools during debugging
		win.open_devtools();
		
		// Menu events -> frontend
		let handle = app.handle().clone();
		app.on_menu_event(move |app_handle, event| {
			let id = event.id().as_ref();
			
			if id == "app.quit" {
				app_handle.exit(0);
				return;
			}
			
			let _ = handle.emit("menu", id.to_string());
		});

		Ok(())
	})

	.run(tauri::generate_context!())
	.expect("error while running tauri application");
}









/*-----------

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MenuCheckUpdate {
  pub id: String,
  pub checked: bool,
}

#[tauri::command]
/*pub*/ fn set_menu_check_statesB<R: Runtime>(
  app: AppHandle<R>,
  updates: Vec<MenuCheckUpdate>,
) -> Result<(), String> {
  // Prefer app menu (macOS global menu bar); otherwise use window menu.
  let menu = if let Some(menu) = app.menu() {
    menu
  } else {
    let win = app
      .get_webview_window("main")
      .ok_or_else(|| "Main window not found".to_string())?;

    win.menu()
      .ok_or_else(|| "Window has no menu (and no app menu)".to_string())?
  };

  // Helper: set a single check item
  let set_checked = |id: &str, checked: bool| -> Result<(), String> {
    let target = MenuId::new(id);

    let items = menu.items().map_err(|e: tauri::Error| e.to_string())?;
    let item_opt = find_menu_item_recursive(items, &target).map_err(|e| e.to_string())?;
    let item = item_opt.ok_or_else(|| format!("Menu item not found: {}", id))?;

    match item {
      MenuItemKind::Check(ci) => {
        ci.set_checked(checked).map_err(|e: tauri::Error| e.to_string())?;
        Ok(())
      }
      _ => Err(format!("Menu item is not a CheckMenuItem: {}", id)),
    }
  };

  for u in updates {
    set_checked(&u.id, u.checked)?;
  }

  Ok(())
}

---*/


/*+++++
#[tauri::command]
fn set_theme_menu_checks<R: Runtime>(
  app: AppHandle<R>,
  theme: String, // "system" | "light" | "dark"
) -> Result<(), String> {
  // Prefer app menu (macOS global menu bar); otherwise use window menu.
  let menu = if let Some(menu) = app.menu() {
    menu
  } else {
    let win = app
      .get_webview_window("main")
      .ok_or_else(|| "Main window not found".to_string())?;

    win.menu()
      .ok_or_else(|| "Window has no menu (and no app menu)".to_string())?
  };

  let mut set_checked = |id: &str, checked: bool| -> Result<(), String> {
    let target = MenuId::new(id);

    let items = menu.items().map_err(|e: tauri::Error| e.to_string())?;

    // NOTE: your find_menu_item_recursive returns Result<Option<_>>
    let item_opt = find_menu_item_recursive(items, &target).map_err(|e| e.to_string())?;
    let item = item_opt.ok_or_else(|| format!("Menu item not found: {}", id))?;

    match item {
      MenuItemKind::Check(ci) => {
        ci.set_checked(checked).map_err(|e: tauri::Error| e.to_string())?;
        Ok(())
      }
      _ => Err(format!("Menu item is not a CheckMenuItem: {}", id)),
    }
  };

  let t = theme.to_lowercase();
  set_checked("view.theme.system", t == "system")?;
  set_checked("view.theme.light",  t == "light")?;
  set_checked("view.theme.dark",   t == "dark")?;

  set_checked("view.sidebar.toggle",   t == "system")?;		// TEST - WORKS FINE!

  Ok(())
}
  ++++*/

use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
fn write_clipboard(app: tauri::AppHandle, text: String) -> Result<(), String> {
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}



  #[tauri::command]
fn set_menu_checks<R: Runtime>(
  app: AppHandle<R>,
  updates: Vec<(String, bool)>,
) -> Result<(), String> {
  // Prefer app menu (macOS global menu bar); otherwise use window menu.
  let menu = if let Some(menu) = app.menu() {
    menu
  } else {
    let win = app
      .get_webview_window("main")
      .ok_or_else(|| "Main window not found".to_string())?;

    win.menu()
      .ok_or_else(|| "Window has no menu (and no app menu)".to_string())?
  };

 //-- let mut set_checked = |id: &str, checked: bool| -> Result<(), String> {
  let set_checked = |id: &str, checked: bool| -> Result<(), String> {
    let target = MenuId::new(id);

    let items = menu.items().map_err(|e: tauri::Error| e.to_string())?;
    let item_opt = find_menu_item_recursive(items, &target).map_err(|e| e.to_string())?;
    let item = item_opt.ok_or_else(|| format!("Menu item not found: {}", id))?;

    match item {
      MenuItemKind::Check(ci) => {
        ci.set_checked(checked).map_err(|e: tauri::Error| e.to_string())?;
        Ok(())
      }
      _ => Err(format!("Menu item is not a CheckMenuItem: {}", id)),
    }
  };

  for (id, checked) in updates {
    set_checked(&id, checked)?;
  }

  Ok(())
}





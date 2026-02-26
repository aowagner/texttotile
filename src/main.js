// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
const { invoke } = window.__TAURI__.core;

let currentPath = null;
let reloadTimer = null;		// debounce bursty save events (VS Code / Obsidian often save in quick bursts)

window.addEventListener("DOMContentLoaded", async () => {
	document.querySelector("#do-greet").addEventListener("click", greet); // or wrap if you prefer
	document.querySelector("#open-file").addEventListener("click", openFile);
	
	// Listen for watcher events from Rust
	window.__TAURI__.event.listen("file-changed", (event) => {
		const changedPath = event.payload;
		if (!currentPath || changedPath !== currentPath) return;
		
		// debounce and add a small delay (helps with atomic saves: replace-then-write)
		if (reloadTimer) clearTimeout(reloadTimer);
		reloadTimer = setTimeout(() => {
			readAndRender(changedPath);
		}, 100);
	});
});

async function greet(e) {
	e.preventDefault();
	document.querySelector("#greet-msg").textContent = await invoke("greet", {name: document.querySelector("#greet-input").value});
}

async function openFile() {
	const { open } = window.__TAURI__.dialog;
	
	const path = await open({
		multiple: false,
		directory: false,
		filters: [{ name: "Text", extensions: ["md", "txt", "json"] }],
	});
	
	if (!path) return;
	
	currentPath = path;
	await readAndRender(path);
	
	// start watcher (Rust) — single watcher slot in Rust will replace any previous watcher
	await window.__TAURI__.core.invoke("start_watch_file", { path });
}

async function readAndRender(path) {
	const { readTextFile } = window.__TAURI__.fs;
	
	// retry a couple of times to survive "atomic save" moments where the file is briefly unavailable (common in editors)
	let lastErr = null;
	for (let attempt = 0; attempt < 3; attempt++) {
		try {
			const text = await readTextFile(path);
			
			// If the user switched files while we were reading, don't overwrite UI
			if (path !== currentPath) return;
			
			document.querySelector("#file-path").textContent = path;
			document.querySelector("#file-contents").textContent = text;
			return;
		} catch (err) {
			lastErr = err;
			await new Promise((r) => setTimeout(r, 60));
		}
	}
	
	// optional: show a gentle message if it keeps failing
	if (path === currentPath) {
		document.querySelector("#file-contents").textContent = "Could not read file (it may be temporarily unavailable)";
	}
	console.warn("readAndRender failed:", lastErr);
}


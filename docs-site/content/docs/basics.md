---
date: '2026-02-28T11:09:51+01:00'
title: 'Basics'
weight: 30
---

## Editor

{{< theme-figure
  light="/img/texttotile-tracking1-light.png"
  dark="/img/texttotile-tracking1-dark.png"
  alt="Changes to the text is shown in real-time as a chart"
  title="Changes to the text is shown in real-time as a chart (click to zoom)"
>}}

TextToTile starts with something simple: writing structured text.

The built-in editor is intentionally minimal. You write lines, indent them using `Tab` and `Shift+Tab`, and press `Enter` to continue at the same level. If you are used to Markdown or outline-based tools, this will feel familiar right away. Using `-` at the beginning of lines is optional—it’s just standard Markdown notation and helps readability.

As you type, the chart updates immediately. There is no “run” or “refresh” step — the editor and visualization always stay in sync.

You can place the editor on the left, right, top, or bottom of the screen — whatever fits your workflow best. The editor can also be hidden entirely, allowing TextToTile to function as a dedicated visual overview alongside any other text-based writing or note-taking app.

{{< details title="Full text from screenshot above" closed="true" >}}

```text
Week 22

	Monday
		Weight 79.8 kg
		BMI 20.5
		Sleep 8.5 hours
			Apparently I needed it :-)
		Energy 70%
		Run 1.5 km
		Swim 800 meter

		Write 900 words
			Editing main plot lines

		Read 1.5 hours
			Anathem: Chapter 7
				Interesting shift in pace — more philosophical than earlier chapters
				Still getting used to the terminology, but the world is starting to make more sense

			Medium: Articles on coding
				A few solid takeaways — especially about keeping code readable			

	Tuesday
	Wednesday
	Thursday
	Friday
	Saturday
	Sunday

Week 23

	Monday
	Tuesday
	Wednesday
	Thursday
	Friday
	Saturday
	Sunday
```
{{< /details >}}


### Parse mode

Each document uses one of two parse modes. Choose the mode that matches the file in `Document → Parse mode`:

* `Outline` (the default) uses the original tab-indented outline notation. Markdown-style `- ` list markers are optional.
* `Markdown headings` uses Markdown heading levels to define the chart structure.

TextToTile remembers the selected mode for each document, so make sure it matches the way the file is written. The following examples show how the same content could be structured in the two parse modes.

```text {filename="Document notation in 'Outline' parse mode"}
- Section A
	- Chapter 1
		- Part i
			- Here is some content for this part
		- Part ii
```

```text {filename="Document notation in 'Markdown headings' parse mode"}
# Section A

## Chapter 1

### Part i
Here is some content for this part

### Part ii
```

In `Outline` mode, a top-level line appears as a chart section only when it has indented content beneath it. Unindented text on its own is not shown in the chart.

This lets you keep document titles, notes, and fenced CSS blocks before, after, or between several separate outlines in the same file. Start each outline on an unindented line, then indent its content beneath it.

Blank lines make a document easier to read, but do not define its hierarchy. Outline mode does not require Markdown; indentation alone defines the structure.

In a working outline, you may prefer conventional Markdown lists. Many editors use the `-` markers to provide list commands, such as moving a section or branch up and down. The best approach depends on your preferred editor and workflow.

{{< callout type="info" >}}
All examples in this documentation use the `Outline` parse mode with tab indentation. Some include standard Markdown `- ` list markers, while others omit them.
{{< /callout >}}




## Works with your regular editor

You do not have to use the built-in editor at all. TextToTile can also be used as a dedicated chart view alongside your preferred writing environment — such as Obsidian, VS Code, or any plain text editor working with local files.

For larger or ongoing work, a dedicated editor is often the better place to maintain the text. It offers fuller editing tools, such as undo and redo, search and replace, version control, and editor-specific features. When you use TextToTile only as a chart view, it remains read-only and never writes to your file.

The built-in editor is still perfectly suitable for quick changes and focused work. You can also combine both approaches: edit primarily in another application, then use TextToTile’s editor whenever it is the most convenient place to make a change.

When the file changes, the visualization updates automatically. Some apps save continuously, while others require a manual save (`Cmd/Ctrl+S`), but the result is the same: your chart stays synchronized with the text.

You can also click sections, columns, or parts in the chart to jump directly to the corresponding line in the built-in editor, making navigation easier in larger outlines.



## Tags and Colors

Tags are what give the structure meaning. Tags are simply words prefixed the `#` character. If you already use tags in Markdown notes or other apps, the idea is exactly the same. In TextToTile, tags help organize entries visually and control how they appear in the chart.

You decide which tags you want to create, and how many - and simply adding `#` at the start of a word makes it a tag.

```text {filename="Examples of tags - and what they could be used for"}
#metrics  (health or personal measurements)
#exercise (running, swimming, workouts)
#work     (writing, reading, research)
```

{{< theme-figure
  light="/img/texttotile-tracking2-light.png"
  dark="/img/texttotile-tracking2-dark.png"
  alt="Tags are used to group related items and control their appearance"
  title="Tags are used to group related items and control their appearance (click to zoom)"
>}}

{{< details title="Full text from screenshot above" closed="true" >}}

````text
```css
#metrics	{@fill: darkorange; }
#exercise	{@fill: seagreen; }
#work		{@fill: steelblue; }

#weekend 	{@fill: dodgerblue; }
```

Week 22

	Monday
		Weight 79.8 kg		// #metrics
		BMI 20.5			// #metrics
		Sleep 8.5 hours		// #metrics
			Apparently I needed it :-)
		Energy 70%			// #metrics
		Run 1.5 km			// #exercise
		Swim 800 meter		// #exercise

		Write 900 words		// #work
			Editing main plot lines	// #work #novel

		Read 1.5 hours		// #work
			Anathem: Chapter 7	// #novel #stephenson
				Interesting shift in pace — more philosophical than earlier chapters
				Still getting used to the terminology, but the world is starting to make more sense

			Medium: Articles on coding	// #coding
				A few solid takeaways — especially about keeping code readable

	Tuesday
		#void
		#void
		Sleep 6.5 hours		// #metrics
		Energy 80%			// #metrics
		Run 2.0 km			// #exercise
		Swim 1200 meter		// #exercise
		Write 1100 words	// #work
		Read 2.5 hours		// #work
			Anathem: Chapter 8+9	// #novel #stephenson

	Wednesday
		#void
		#void
		Sleep 6.2 hours		// #metrics
		Energy 70%			// #metrics
		#void
		#void
		Write 850 words		// #work
		Read 2.2 hours		// #work

	Thursday
		#void
		#void
		Sleep 5.6 hours		// #metrics
		Energy 60%			// #metrics
		Run 1.8 km			// #exercise
		Swim 900 meter		// #exercise
		Write 950 words		// #work
		Read 2.4 hours		// #work

	Friday
	Saturday	// #weekend
	Sunday		// #weekend

Week 23

	Monday
	Tuesday
	Wednesday
	Thursday
	Friday
	Saturday	// #weekend
	Sunday		// #weekend
````

{{< /details >}}

In the above example, each line has one of these tags: `#metrics`, `#exercise`, `#work`. Each tag defines a tag group, which determines how related parts are grouped in the chart.

These tags can then be styled using a small CSS block at the top, where you can assign colors to each tag.

Higher-level elements - Sections and Columns - can also be styled. In the example, weekends are marked with a #weekend tag and thereby given a different background color, making the timeline easier to understand at a glance.

You can use as many tags as you like — for visualization, categorization, filtering, organization, or your own notes - and any line can have several tags.

{{< callout type="info" >}}
Anything written after `//` (2 forward slash characters) is not shown as text in the chart. You can use it to add tags, values, or notes. Tags and values will still affect tag grouping and graphs, even though the actual text is not displayed in the chart.
{{< /callout >}}



## Tag Groups Layout

Sometimes it can be useful to view the chart ordered by tag groups rather than outline structure.

By selecting `Document → Chart layout → Tag groups`, the same outline can be viewed grouped by tags instead of by writing order. 

{{< theme-figure
  light="/img/texttotile-tracking3-light.png"
  dark="/img/texttotile-tracking3-dark.png"
  alt="In Tag groups layout, all parts are grouped according to their primary tag"
  title="In Tag groups layout, all parts are grouped according to their primary tag (click to zoom)"
>}}

This screenshot shows the exact same text outline as the screenshot before, just grouped by tags. Each column has exactly the same parts as shown in `Structure` layout - they are just rearranged after which tag group they belong to.

Each part in a tag group is displayed on its own line, making it possible to read longer texts extending beyond the right edge of the part. This becomes especially useful in larger outlines — such as the complete novel example shown on the [Showcase page](/docs/12-showcase/#novel-tag-groups).

When a new `Section` begins, the layout resets and starts placing parts from the top again. This is to prevent a single tag group to take up the full height of the screen - allowing many tag groups to remain visible at the same time.

Tag groups containing graphs automatically reserve additional vertical space to make the graphs easier to read.


### Visibility and Sorting

The Ribbon at the bottom of the chart shows a list of all tags used in the current outline. If a style has been created for a tag, the corresponding button will also use that color.

Clicking a tag button switches the corresponding tag group between active and passive. This does not modify the text outline itself — only how the chart is displayed.

Passive tag groups are dimmed, allowing active groups to stand out more clearly and making it easier to focus on specific parts of the outline.

In `Structure` layout, passive tag groups remain visible but dimmed. In `Tag groups` layout, passive groups are additionally collapsed to a single line.

Holding Alt while clicking a tag button will solo that tag group by setting all other groups to passive.

The same actions can also be performed using keyboard shortcuts. For example, typing Cmd/Ctrl + 1 toggles the first tag group shown in the Ribbon between active and passive, while Alt + 1 solos that tag group.

{{< theme-figure
  light="/img/texttotile-tracking4-light.png"
  dark="/img/texttotile-tracking4-dark.png"
  alt="Only tag group #2 is active, the other two are passive"
  title="Only tag group #2 is active, the other two are passive"
>}}

You can change the order of tag groups by dragging the buttons in the Ribbon. Like switching groups between active and passive, this does not change the text itself — only the order in which tag groups are displayed in Tag groups layout.

The active/passive state of tag groups, along with their sorting order, is automatically restored when the file is reopened without changing the outline text itself.



## View Settings

In `Settings → View`, you can control which UI elements are visible and adjust how the chart is displayed.

The Settings dialog can be opened by clicking the Settings button — the first button on the Ribbon — or by pressing `Ctrl/Cmd+,`.

{{< theme-figure
  light="/img/texttotile-settings-view-light.png"
  dark="/img/texttotile-settings-view-dark.png"
  alt="Editing View Settings"
  title="Editing View Settings (click to zoom)"
>}}

The View settings provide quick access to the most important layout and visualization options — including editor placement, chart appearance, zoom settings, and visibility of different interface elements.

The keyboard shortcuts for each action are also shown directly in the interface, making it easier to learn the most commonly used commands and work more efficiently.

All of these options are also available from the View menu.



## Zoom and Scale

Depending on how much detail you want to see, the chart can be zoomed in and out so each part displays between 2 and 20 lines of text. This is controlled using the `Ctrl/Cmd+J/K/L` shortcuts:

* `Ctrl/Cmd+J` zooms out
* `Ctrl/Cmd+K` resets the zoom to the default 8 lines
* `Ctrl/Cmd+L` zooms in to the maximum of 20 visible lines

Zooming only changes how many lines of text are shown inside each part. It does not affect the size of fonts, icons, or other interface elements.

In practice, chart zoom is something you will often adjust dynamically depending on the amount of detail you want to inspect.

{{< theme-figure
  light="/img/texttotile-tracking5-light.png"
  dark="/img/texttotile-tracking5-dark.png"
  alt="Zooming in on the chart for more details"
  title="Zooming in on the chart for more details (click to zoom)"
>}}

When zoomed in far enough, child lines are also shown inside the parent part. This makes it possible to view additional context directly in the chart without switching back to the editor.

In the screenshot above, 8 of the possible 20 visible lines are shown. This allows the child lines of the Write and Read parts to be displayed, while deeper nested lines remain hidden.

Another example of zoomed-in detail can be seen in the complete novel example on the [Showcase page](/docs/12-showcase/#novel-zoom).


### UI Scale

Unlike chart zoom, UI scale is typically adjusted less frequently and acts more like a general interface preference.

UI scale changes the size of fonts, icons, buttons, and interface elements across the application.

The shortcuts mirror the normal zoom controls:

* `Ctrl/Cmd+Alt+J` decreases the UI scale
* `Ctrl/Cmd+Alt+K` resets it to the default size
* `Ctrl/Cmd+Alt+L` increases the UI scale

This makes it easy to adapt the interface to different screen sizes and personal preferences.



## Files

The Files tab in the Settings dialog provides access to the main file-related functions in TextToTile.

A typical workflow is either opening an existing plain text or Markdown file or creating a new outline from scratch and saving it as a file.

Pinned files make it easy to switch quickly between frequently used outlines. You can move to the next or previous pinned file using `Ctrl+Tab` and `Ctrl+Shift+Tab`.

When switching between pinned files, TextToTile remembers:

* chart zoom level
* chart scroll position
* editor scroll position

This creates a workflow similar to switching between open tabs in a traditional editor, even though only one file is loaded at a time.

Pinned files can also be reordered to prioritize the outlines you use most often.

{{< theme-figure
  light="/img/texttotile-settings-files-light.png"
  dark="/img/texttotile-settings-files-dark.png"
  alt="Editing Files in Settings"
  title="Editing Files in Settings (click to zoom)"
>}}

Additionally, pinned files can be opened directly using `Cmd/Ctrl+Shift` together with their number in the list. Some shortcuts may conflict with operating-system shortcuts — for example `Cmd+Shift+3` on macOS, which is reserved for screenshots.

The file-related actions are also available from the File menu.



<!-- invisible image required to trigger Hextra’s medium-zoom initialization, enabling zoom on shortcode-rendered images -->
![hextra-zoom-init](/img/transparent.png "")


<script src='https://storage.ko-fi.com/cdn/scripts/overlay-widget.js'></script>
<script>
  kofiWidgetOverlay.draw('aowagner', {
    'type': 'floating-chat',
    'floating-chat.donateButton.text': 'Support me',
    'floating-chat.donateButton.background-color': '#fcbf47',
    'floating-chat.donateButton.text-color': '#323842'
  });
</script>



---
date: '2026-02-28T11:09:51+01:00'
title: 'Faq'
weight: 50
---



{{< details title="Who is the app for?" closed="true" >}}
TextToTile is for anyone who works with structured text and wants to keep the whole structure visible while working. It began as a tool for writing articles and novels, but it can also support planning, research, personal tracking, Kanban-style boards, travel ideas, and other text-based workflows.

It is especially useful when an outline is too large to hold in your head, but you still want to edit the original text directly.
{{< /details >}}

{{< details title="Why was the app created?" closed="true" >}}
TextToTile was created to solve a simple problem: how to maintain a clear visual overview while restructuring, editing, and refining structured text.

Many tools focus primarily on either writing or planning. TextToTile keeps the information in a simple text file while providing a visual layer for seeing hierarchy, relationships, progress, and patterns.
{{< /details >}}

{{< details title="What file formats are supported?" closed="true" >}}
TextToTile works with ordinary plain-text and Markdown files, including `.txt` and `.md` files. For each document, choose `Document → Parse mode`: `Outline` uses tab indentation (with optional Markdown list markers), while `Markdown headings` uses heading levels to define the chart structure.

Because the files remain ordinary text, you can also open and edit them in your preferred text editor.

See [Basics: Parse mode](/docs/basics/#parse-mode) for details on the different ways to structure a document.
{{< /details >}}

{{< details title="Can I use TextToTile with Obsidian?" closed="true" >}}
Yes. Obsidian is a note-taking application built around Markdown files. TextToTile can open the same local text or Markdown files and act as a dedicated visual overview alongside Obsidian.

When the file changes in Obsidian, TextToTile updates its chart automatically. The same approach also works with VS Code and other text editors.
{{< /details >}}

{{< details title="Does TextToTile modify my files?" closed="true" >}}
TextToTile saves changes automatically whenever you edit text in its built-in editor. If you use TextToTile only as a chart view alongside another editor, it does not modify the file.
{{< /details >}}

{{< details title="Why doesn't the built-in editor have undo and redo?" closed="true" >}}
Undo and redo are not available in the built-in editor at the moment. For substantial editing, we recommend maintaining your outline in your usual text editor, which will typically provide a fuller set of editing tools and features than TextToTile’s minimal editor.

TextToTile can then remain a dedicated, read-only chart view of the same file. If undo and redo would be important to your workflow, please share that feedback in [TextToTile Discussions](https://github.com/aowagner/texttotile/discussions). It may be added in a future version based on user needs.
{{< /details >}}

{{< details title="What are tags and metadata?" closed="true" >}}
Tags are words prefixed with `#`, such as `#work` or `#research`. They can group related parts of an outline and control how those parts appear in the chart.

Metadata is a value written with an `@` prefix, such as `@sleep 8.5h` or `@write 900`. TextToTile can turn matching values into graphs. Read more on the [Basics](/docs/basics/) and [Graphs](/docs/graphs/) pages.
{{< /details >}}

{{< details title="What is CSS?" closed="true" >}}
CSS is a simple language for describing visual appearance. In TextToTile, you can use a small CSS-style block to assign colors, icons, borders, and text styling to tags or individual parts.

You do not need to use CSS to get started. It is an optional way to make a chart fit your own workflow. See [Styling with Tags](/docs/styles/) for examples.
{{< /details >}}

{{< details title="Does the app use AI?" closed="true" >}}
No. TextToTile has no built-in AI, user accounts, backend, or network calls.

Because its information lives in ordinary text files, you can choose to work on those files with AI tools if that suits your workflow. That is separate from TextToTile itself: you remain in control of which tools can access your files.
{{< /details >}}

{{< details title="Why not just create a site with AI instead?" closed="true" >}}
You can — and for a focused, one-off visualization or a public interactive project, an AI-built site may be an excellent choice.

TextToTile serves a different purpose. It provides a stable, live visual overview of ordinary local text files without turning each document into a separate website or software project. You can keep writing and organizing your text in the editor you already prefer, while TextToTile updates the chart as the file changes.

The underlying information remains readable, portable, and under your control. There is no need to maintain a custom website, database, hosting setup, or application logic simply to keep an overview of an evolving outline.

The two approaches can also work together: AI tools can help create, reorganize, or maintain structured text, while TextToTile provides a dedicated visual layer for exploring that structure. See [AI integration](/docs/ai-integration/) for more.
{{< /details >}}

{{< details title="What about privacy and security?" closed="true" >}}
TextToTile is designed to keep your information on your own computer. It only accesses files and folders that you explicitly open, and it does not have background access to the rest of your system.

The app has no network access and does not send or receive your text—or any other data.
{{< /details >}}

{{< details title="Where are settings stored?" closed="true" >}}
Settings are stored locally in TextToTile’s application-data folder on your computer. This includes preferences and information such as pinned files and their view settings.

These settings are not stored in a cloud account or sent to a server.
{{< /details >}}

{{< details title="What platform is the app built on?" closed="true" >}}
TextToTile is built with Tauri, a framework designed for small, secure desktop applications with limited system access. The chart is rendered as SVG, while the editor, toolbar, and settings interface use standard web technologies.
{{< /details >}}

{{< details title="Are more chart layouts planned?" closed="true" >}}
TextToTile currently includes `Structure` layout and `Tag groups` layout. Additional views are possible, including calendar-like views and other ways to visualize structured text.

Future development will be guided by practical use cases—especially where a simple text-based workflow can replace a more complex tool.
{{< /details >}}

{{< details title="Why does Windows SmartScreen appear during installation?" closed="true" >}}
The current beta installer is not code-signed yet. Windows SmartScreen may therefore show a warning because it cannot verify the publisher’s reputation in the usual way; this does not mean that SmartScreen has found malware.

If you downloaded the installer from the official TextToTile release page, select **More info** and then **Run anyway**. See the [Download page](/docs/download/) for the current installation guidance.
{{< /details >}}

{{< details title="Why does macOS warn when opening the app for the first time?" closed="true" >}}
The current beta app is not code-signed or notarized yet, so macOS cannot verify it automatically. This is why macOS may say that the app cannot be verified.

If you downloaded the app from the official TextToTile release page, open **System Settings** → **Privacy & Security** and choose **Open Anyway**. See the [Download page](/docs/download/) for the full steps.
{{< /details >}}

{{< details title="How can I ask questions and give feedback?" closed="true" >}}
Questions, ideas, bug reports, workflow examples, and installation feedback are all welcome. The best place to share them is [TextToTile Discussions](https://github.com/aowagner/texttotile/discussions).
{{< /details >}}

{{< details title="How can the app be free?" closed="true" >}}
TextToTile began as a tool I made for my own writing and planning. I am sharing it freely because I believe it can be useful to others too.

Its development is currently funded by my own time. If TextToTile becomes useful to you, you can support me through [Ko-fi](https://ko-fi.com/aowagner), helping me make improvements and add new features.

<a href="https://ko-fi.com/aowagner" target="_blank">
<img src="https://ko-fi.com/img/githubbutton_sm.svg">
</a>

{{< /details >}}






<!-- invisible image required to trigger Hextra’s medium-zoom initialization, enabling zoom on shortcode-rendered images -->
![hextra-zoom-init](/img/transparent.png "")

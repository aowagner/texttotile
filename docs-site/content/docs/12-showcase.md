---
date: '2026-02-28T11:09:51+01:00'
title: 'Showcase'
weight: 12
---

<!--## Flexibility

The essence of TextToTile is to show a text-based outline in a visually 'appealing' way that lets the user get an overview of relations and connections between structures - in a way that is not so easy to get by just reading through the 'one-dimensional' text file.

The following examples are ment to show the 'range' of 'things' TextToTile can be used for, but it is important to remember that these are just examples. There are many other contexts - and types of 'information' where you can 'gain' from a visual overview of a text structure.-->

## One Outline — Many Possibilities

TextToTile is intentionally generic. It does not impose a predefined workflow or data model. Instead, it visualizes structured text in ways that make relationships, hierarchy, progress, and patterns easier to understand at a glance.

The examples below are only a small sample of possible uses. The same principles can be applied to planning, writing, research, travel, journaling, software projects, personal tracking, or any other type of structured information.



## Example 1 - Kanban Board

{{< theme-figure
  light="/img/texttotile-kanban1-light.png"
  dark="/img/texttotile-kanban1-dark.png"
  alt="Simple Kanban board with custom symbols"
  title="Simple Kanban board with custom symbols (click to zoom)"
>}}

<!--This is an example of using a text outline to show a simple Kanban board.

The added symbols - like the pen or the colored dots - are defined as simple tags and can be used for whatever you decide. They are just Emoji and have no predefined functionality in TextToTile.

Likewise with the use of <kbd>◉</kbd> and <kbd>◎</kbd> in the detail lines. These are not interactive but simple text characters, chose only for the status they convey.

In the expandable text box below, you can see how the tags are defined at the top, and how they are used in the outline.-->

This example shows a lightweight Kanban board built entirely from plain text.

Tags control colors, symbols, grouping, and styling. The symbols shown in the chart — such as colored dots, warning markers, or edit icons — are simply emoji assigned through tags. They do not carry predefined functionality and can be adapted freely to match your own workflow.

Likewise, symbols such as ◉ and ◎ in the detail lines are ordinary text characters chosen for the meaning they visually convey.

The result is a flexible planning system built from simple structured text rather than a predefined board layout imposed by a specific app or service.

In the expandable section below, you can see both the style definitions and the outline used to generate the board.


{{< details title="Full source text for Kanban board example" closed="true" >}}

Below is the full text used in the example shown in the screenshots above.

Styles can also be defined globally in Settings → Style, allowing them to be reused across multiple outlines while keeping the outline itself cleaner and simpler.


````markdown {filename="Markdown"}
### Style

```css
/* tasks */
#backlog {@fill: slategray;}
#todo {@fill: steelblue;}
#active {@fill: cadetblue;}
#testing {@fill: mediumseagreen;}
#done {@fill: seagreen;}

/* attribs */
#ok {@icon: ✅;}
#nb {@icon: ❗️;}
#warning {@icon: ⚠️; @stroke: red; @strokewidth: 5}
#pin {@icon: 📌;}
#key {@icon: 🔑;}
#edit {@icon: ✏️;}

/* dots */
#red {@icon: 🔴;}
#green {@icon: 🟢;}
#blue {@icon: 🔵;}
#purple {@icon: 🟣;}
```

### Outline

The following outline is an example of using TextToTile to view a Kanban board built as pure text.

- Kanban - Launching TextToTile - 110
	- Backlog
		- #backlog Linux Packaging // #edit #red
			- ◎ AppImage compatibility: Test on as many distros as possible.
			- Verify Ubuntu 24 build
			- ◎ Test file associations
			- ◎ Check dark mode titlebars
		- #backlog Website Animation // #pin
			- ◎ Smooth zoom transition
			- ◎ Add SVG pan effect
	- Todo
		- #todo Documentation Rewrite // #nb #blue 
			- ◉ Simplify installation guide
			- ◎ Add screenshots
		- #todo Interactive Showcase // #edit
			- ◉ Novel outline example
			- ◉ Kanban example
			- ◎ Technical architecture example
			- ◎ Mobile screenshots
	- In Progress
		- #active Dynamic Zoom // #blue #purple #red
			- ◎ Cmd + mousewheel
			- ◉ Preserve center point
			- ◎ Smooth redraw timing
			- ◉ SVG text wrapping
				- Further info here, that is relevant,
				- but not shown directly in the board
				- NB: Remember to measure font-size!
		- #active macOS Sleep Bug // #warning
			- ◉ Window redraw issue
			- ◎ Reproduce on Intel Mac
		- #active Theme Editor // #ok
			- ◉ Live CSS updates
			- ◉ Save to settings
			- ◎ Reset defaults
			- ◎ Theme presets
	- Testing
		- #testing Obsidian Workflow // #pin 
			- ◉ Markdown paste test
			- ◎ Long paragraph wrapping
		- #testing Tauri Build Pipeline // #green #ok
			- ◎ Windows build
			- ◎ macOS notarization prep
	- Done
		- #done SVG Rendering Engine // #ok
			- ◉ Overlay mode
			- ◉ Flow indexing
````

{{< callout type="info" >}}
Tip: In this example, Markdown headlines (<kbd>### Style</kbd> and <kbd>### Outline</kbd>) are added, allowing the css block to be collapsed in Obsidian when not edited.
{{< /callout >}}

{{< /details >}}


### Maintain in other editor

<!--Below is a screenshot of the Kanban board being edited in Obsidian - giving the user some more visual elements. And easy ways of moving items between sections in the outline, eg. using <kbd>Cmd/Ctrl+Shift+ArrowUp/Down</kbd> which can be a huge timesaver when working with text-based outlines,

The basic, built-in editor in TextToTile can easily be used, but without all the editing benefits and 'color info' that other editors offer - eg. the ability to open/close any element of the outline, as shown in the screenshot.-->

The outline shown above is edited in Obsidian, demonstrating how TextToTile can function alongside another writing or note-taking environment.

Many outline-based editors provide additional editing features such as collapsing sections, moving blocks, restructuring hierarchies, or applying custom styling. TextToTile focuses on visualization and overview while remaining synchronized with the underlying text.

{{< theme-figure
  light="/img/texttotile-kanban2-light.png"
  dark="/img/texttotile-kanban2-dark.png"
  alt="The text outline for the Kanban board edited in Obsidian"
  title="The text outline for the Kanban board edited in Obsidian (click to zoom)"
>}}

Because the structure remains plain text, the same outline can be edited in many different applications while still producing the same visual overview.



## Example 2 - Complete Novel

{{< theme-figure
  light="/img/texttotile-showcase1a-light.png"
  dark="/img/texttotile-showcase1a-dark.png"
  alt="Overview a full novel in Structure View"
  title="Overview a full novel in Structure View (click to zoom)"
>}}

<!--Here you see the complete outline/overview of A. O. Wagner's novel 'The Karma Ubiquity'. The screenshot above shows the novel in <kbd>Structure View</kbd>, while the screenshot below shows it in <kbd>Tag Groups View</kbd>.

Each view has its advantages, eg. in terms of monitoring the flow of events within each chapter, as well as coordinating story arcs and making sure that the intended 'story progress' is achieved.-->

<a id="novel-tag-groups"></a>

This example shows the complete outline of A. O. Wagner’s novel The Karma Ubiquity.

The screenshot above uses <kbd>Structure View</kbd>, which emphasizes hierarchy and chapter flow. The screenshot below uses <kbd>Tag Groups View</kbd>, where story arcs, themes, characters, and metadata become easier to follow across the entire novel.

{{< theme-figure
  light="/img/texttotile-showcase1b-light.png"
  dark="/img/texttotile-showcase1b-dark.png"
  alt="Same outline but in Tag Groups View"
  title="Same outline but in Tag Groups View (click to zoom)"
>}}

<!--In <kbd>Tag Groups View</kbd> (screenshot above) the individual parts and graphs - used to represent story arcs or other metadata values in the outline - are shown 'individually', making it easy to follow the evolution of individual characters and themes.

Hovering the mouse over a graph point shows its type and unit for the individual points.

In the screenshot below, the built-in Editor is 'turned on', and with a larger zoom of the parts, it is possible to go through details of each part - both by looking at it in the zoomed chart, and clicking parts and lines in the chart to jump to the specific place in the text to read and edit further.-->

In <kbd>Tag Groups View</kbd>, individual tag groups and graphs are separated visually, making it possible to track character development, emotional progression, themes, pacing, or other custom metadata throughout the story.

<a id="novel-zoom"></a>

Graph points can also display additional information such as type and unit values on hover.

With the editor enabled and the zoom level increased, the chart becomes detailed enough to navigate and inspect individual scenes, notes, and structural relationships directly inside the outline.

{{< theme-figure
  light="/img/texttotile-showcase1c-light.png"
  dark="/img/texttotile-showcase1c-dark.png"
  alt="Zooming in on the chart and viewing/editing the novel text in Editor"
  title="Zooming in on the chart and viewing/editing the novel text in Editor (click to zoom)"
>}}



### A Single Source of Truth

{{< theme-figure
  light="/img/texttotile-showcase1d-light.png"
  dark="/img/texttotile-showcase1d-dark.png"
  alt="Same novel being edited in Obsidian"
  title="Same novel being edited in Obsidian (click to zoom)"
>}}

<!--Edit your text and data and notes and ideas in one place, while maintaining a visual overview — from the first idea to the final result, or as a dynamic view of ongoing updates and everything you need to keep track of.

I have tried many apps and platforms that were excellent in themselves, but often suited only specific, isolated phases of a project — such as brainstorming, planning, or reorganizing structure. When being deep in the writing, it was cumbersome - and could easiliy break the 'flow' state - when I had to 'back' update the charts and graphs used in the brainstorm and plaaning phases.

When I started writing the actual content — for example, a novel — and made changes there, it required a lot of effort to step out of the writing process just to keep a separate planning or visualization tool up to date. So much so that I eventually stopped maintaining those external tools in order to stay focused in writing mode.

Personally, I now use Obsidian for all types of writing — whether it’s for quickly noting a phone number, drafting an email, or working on my next novel.

However, any text editor that saves content as plain text — rather than in a proprietary or unreadable format — can be used. Some apps, such as Obsidian, save your work continuously, 'meaning' the visual representation in TextToTile is always up to date without requiring manual saves. Other apps — such as Visual Studio Code or Notepad (TextEdit?) require explicit saving (Ctrl/Cmd+S) to ensure changes are reflected.

Now, I write in Obsidian, from the starting point with a blank page and an empty outline, to a finished and edited document - be it an article or a novel. And TextToTile lets me keep an overview during the full process - no matter how many structural or 'detailed' changes I make 'at any point'.-->

One of the main ideas behind TextToTile is maintaining structure, notes, planning, and writing in the same underlying text rather than distributing them across multiple disconnected tools.

For long-form writing projects, this makes it possible to move continuously between brainstorming, outlining, drafting, restructuring, tracking, and editing without having to manually recreate charts or reorganize information in separate systems.

Personally, I now use Obsidian for nearly all writing — from quick notes and article drafts to complete novels — while TextToTile provides the visual overview throughout the entire process.

Because the files remain ordinary text, the workflow stays flexible. Some editors save continuously, while others require manual saves, but the visualization in TextToTile always reflects the current structure of the file itself.


### Separation of Roles

<!--I view word processing apps - like MS Word, - as formatting and presentation tools - almost like a sort of 'DTP' (on a 'less advanced' level).

I only write in Obsidian - because here I have the quick edits, restructuring - in a small or giant outline. The simplicity. Plus Tags. Etc.

I *only* use a 'normal' word processor to create the PDF for the print of my books - so it in this way becomes a sort of DTP app. (Maybe I should look into a more 'prof' DTP/publishing app for my next novel)

Previously I used a spreadsheet app - both to build and edit the outline, *and* to write and edit the final prose, and then copying it all to MS Word (or other?) for page layout, hyphenation, pagination, headers+footers, etc. For creating the PDF used for printing when a customer buys the book).

TextToTile was created exactly to 'avoid' jumping between different apps and 'systems' for different phases of the work. I write in an 'optimal' outline 'tool' for writing, and I keep the overview and 'progress feeling' - from the first word to the last - in TextToTile.-->

I personally prefer separating writing from final document layout.

Outline-based editors such as Obsidian are excellent for drafting, restructuring, tagging, and completing large bodies of text, while traditional word processors are often better suited for the finishing steps like page layout, hyphenation, pagination, headers, and print-ready PDF generation.

TextToTile was created to bridge these phases visually — allowing the same structured text to remain useful from the first idea to the finished result.



## Example 3 - Travel ideas

{{< theme-figure
  light="/img/texttotile-travelideas-light.png"
  dark="/img/texttotile-travelideas-dark.png"
  alt="Plotting ideas for a trip"
  title="Plotting ideas for a trip (click to zoom)"
>}}

<!--This is a simple example of 'working' with ideas for a trip, as well as adding notes and reminders, as well as their status, in a single place.

Notice how the chart shows 4 'levels' of the outline (in this zoom setting). Clicking anywhere in the chart 'jumps' the Editor to the specific line in the outline, 'enabling' you to see more outline levels and information that is shown in the chart.

['mention contrast to other mainly visual organizers?'] They typically have a predefined set of fields and 'levels'. With TextToTile you define your own structure and hierarchy of information - with no limits.-->

This example shows a lightweight travel planner built entirely from structured text.

The outline combines destinations, bookings, reminders, costs, notes, and nested planning details in a single hierarchy. Because the structure is visualized directly, it becomes easier to maintain an overview while still allowing unlimited depth for additional information.

Notice how the chart only shows a subset of the outline levels at the current zoom level. Clicking entries in the chart jumps directly to the corresponding location in the text, making it easy to move between overview and detail.

Unlike many visual planning tools, the structure itself is completely user-defined. There are no fixed fields, templates, or hierarchy limits — only the structure you choose to create.


### Formal Outline vs. Freeform Indentation

<!--In this example, the normal Markdown outline is not used. You can see the missing '-' at the start of every line, as well as empty lines that may help readability in some situations.

For some, this may be more readable, and makes perfect sense - TextToTile wors just as well in this way. But you loose some of the advantages of working in a Markdown outline (eg. in Obsidian), eg. the quick and easy ways to rearrange structures - both for moving up and down, and for changing indents.

This will often come down to personal preference, and maybe the type of outline and how important 'instant' and dynamic restructuring is, vs. a nice layout.

Below is the full text used in the example shown in the screenshot above.-->

This example intentionally avoids traditional Markdown list markers (<kbd>-</kbd>) and instead uses indentation alone to define the structure.

Both approaches work equally well in TextToTile. Some people prefer the cleaner visual appearance of freeform indentation, while others prefer formal Markdown outlines because many editors provide additional restructuring shortcuts for list-based hierarchies.

The choice largely comes down to personal preference and workflow style.


{{< details title="Full text from screenshot above" closed="true" >}}

Below is the full text used in the example shown in the screenshot above.

````markdown
```css
/* groups */
#flight {@fill: skyblue; @icon: ✈️;}
#hotel {@fill: slateblue; @icon: 🏨;}
#train {@fill: darkkhaki;}
#food {@fill: peru; @icon: 🍜;}
#culture {@fill: indianred;}
#nature {@fill: seagreen; @icon: 🚶;}
#shopping {@fill: orchid;}

#idea {@fill: goldenrod; @icon: 💡;}
#budget {@fill: orange; @icon: 💵}
#packing {@fill: gold;}

/* status */
#booked {@icon: ✅;}
#ok {@icon: ✅;}
#todo {@icon: ❗️; @stroke: red; @strokewidth: 5;}

/* definitions */
#def {key: budgetleft; unit: $; min: 0; max: 120;}
```

Travel Ideas - Japan 2027

	Flight
		#flight Copenhagen → Tokyo // #booked
			Departure: March 28.0
			Overnight flight
			Window seats booked
			@cost: 78

	Tokyo
		#hotel Hotel in Shinjuku // #booked 
			5 nights
			Near metro station
			Rooftop bar
			@cost 80

		#culture Shibuya Crossing at night
			Wide-angle photography
			Explore side streets
			@cost: 22
			
		#food Tsukiji food tour
			Sushi breakfast
				Reserve early if possible — the smaller places fill up quickly
				Idea: Compare conveyor-belt sushi with a traditional counter experience
			
			Street food
				Try grilled scallops and tamagoyaki from the outer market stalls
			
			Matcha shop
				Look for a quieter café away from the busiest tourist streets
				Maybe buy tea as gifts for people back home
			
			@cost 30
				Keep some cash ready — smaller vendors may not accept cards

		#culture TeamLab Planets #booked
			Tickets prepaid
			Bring light clothing
			@cost: 55

		#nature Day trip: Nikko
			Shrines and forest trails
			Bento on train

	Kyoto
		#hotel Traditional ryokan #booked 
			Tatami room
			Private bath
			Kaiseki dinner included
			@cost 56

		#train Shinkansen from Tokyo #booked
			Morning departure
			Reserved seats
			Mount Fuji views if weather is clear

		#nature Fushimi Inari hike
			Early morning
			Thousands of torii gates
			Photography stop near summit

		#culture Gion evening walk
			Lantern streets
			Tea houses
			Quiet atmosphere

		#food Nishiki Market
			Local specialties
			Knife shops
			Pick up gifts
			@cost 36

	Osaka
		#hotel Hotel near Dotonbori // #todo
			3 nights
			Walking distance to restaurants
			@cost 50$ (expected)

		#food Street food night
			Takoyaki
			Okonomiyaki
			Neon photography

		#culture Osaka Aquarium 
			Afternoon visit
			Buy tickets online
			@cost: 22

		#shopping Retro game shopping
			Super Potato
			Camera gear stores
			Vintage manga
			
	Flight
		#flight Osaka → Copenhagen // #booked 
			Return: April 12
			One stop in Helsinki			
			@cost: 83
			
Preparation

	Ideas
		#idea Add visit to Hokkaido
		#idea Autumn colors in Kyoto // #todo
		#idea Explore smaller coastal towns
		#idea Travel with friends

	Budget Overview
		#budget Flights
		#budget Hotels
		#budget Food
		#budget Train tickets
		#budget Shopping allowance
		#budget Emergency reserve

	Packing List
		#packing Passport // #ok
		#packing Camera + extra batteries // #ok
		#packing Portable charger // #ok
		#packing Lightweight jacket // #todo
		#packing Walking shoes // #ok
		#packing Rail pass documents // #todo
````
{{< /details >}}




<!-- invisible image required to trigger Hextra’s medium-zoom initialization, enabling zoom on shortcode-rendered images -->
![hextra-zoom-init](/img/transparent.png "")


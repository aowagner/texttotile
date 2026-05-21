---
date: '2026-02-28T11:09:51+01:00'
title: 'Graphs'
weight: 34
---

<!--
## Visualizing Data

Building on the outline introduced on the Basics page, we will now extend it with structured values that can be visualized as graphs.

By adding a simple <kbd>@</kbd> prefix to values in your text, you can turn them into usable data.

When values are written using the <kbd>@key value</kbd> format, TextToTile recognizes them as numeric values and builds graphs automatically. Values sharing the same key — such as <kbd>@sleep</kbd> or <kbd>@run</kbd> — are automatically connected into timelines in the chart.

For readability, you can include the unit directly after the value—such as <kbd>@sleep 6.5h</kbd> or <kbd>@sleep 6.5 hours</kbd>—or leave the unit out altogether. This does not affect how the data is processed.

And it doesn't matter how or where the value is placed in the text - it is totally up to you. Like the following examples of a short, looging style, or a longer 'free' text.
-->

## Visualizing Data

Building on the outline introduced on the Basics page, we can now extend it with structured values that are automatically visualized as graphs.

By adding a simple <kbd>@</kbd> prefix to values in your text, you can turn them into structured data.

When values are written using the <kbd>@key value</kbd> format, TextToTile recognizes them as numeric values and automatically connects matching keys into timelines in the chart. For example, all <kbd>@sleep</kbd> values become part of the same graph.

For readability, units can be written directly after the value — such as <kbd>@sleep 6.5h</kbd> or <kbd>@sleep 6.5 hours</kbd> — or omitted entirely. The values are still recognized correctly. The values can appear anywhere in the text. Some people may prefer a short logging style, while others may mix values naturally into longer notes or journal-like writing.

```
- @sleep 8.5
- @sleep 8.5h
- Last night I managed to @sleep 8.5 hours — apparently I needed it :-)
```

You can also include multiple <kbd>@key value</kbd> entries on the same line, allowing multiple metrics to be tracked for that part.

{{< theme-figure
  light="/img/texttotile-tracking3-light.png"
  dark="/img/texttotile-tracking3-dark.png"
  alt="Structured values shown as graphs"
  title="Structured values turn your outline into visual graphs (click to zoom)"
>}}

The graphs are grouped by the tagblock defined by the first tag on each line, allowing multiple types of data to be tracked in parallel — exercise, writing, work, health metrics, or anything else you define.

Hovering graph points shows their values and highlights the corresponding part in the chart. Clicking a graph point jumps directly to the related line in the text.


{{< details title="Full text from screenshot above" closed="true" >}}

````markdown {filename="Markdown"}
```css
#metrics	{@fill: darkorange; }
#exercise	{@fill: seagreen; }
#work		{@fill: steelblue; }

#weekend 	{@fill: dodgerblue; }
```

Week 22

	Monday
		@weight 79.8 kg		// #metrics
		@bmi 20.5			// #metrics
		@sleep 8.5 hours	// #metrics
			Apparently I needed it :-)
		@energy 70%			// #metrics
		@run 1.5 km			// #exercise
		@swim 800 meter		// #exercise

		@write 900 words	// #work
			Editing main plot lines	// #work #novel

		@read 1.5 hours		// #work
			Anathem: Chapter 7	// #novel #stephenson
				Interesting shift in pace — more philosophical than earlier chapters
				Still getting used to the terminology, but the world is starting to make more sense

			Medium: Articles on coding	// #coding
				A few solid takeaways — especially about keeping code readable

	Tuesday
		#void
		#void
		@sleep 6.5 hours	// #metrics
		@energy 80%			// #metrics
		@run 2.0 km			// #exercise
		@swim 1200 meter	// #exercise
		@write 1100 words	// #work
		@read 2.5 hours		// #work
			Anathem: Chapter 8+9	// #novel #stephenson

	Wednesday
		#void
		#void
		@sleep 6.2 hours	// #metrics
		@energy 70%			// #metrics
		#void
		#void
		@write 850 words	// #work
		@read 2.2 hours		// #work

	Thursday
		#void
		#void
		@sleep 5.6 hours	// #metrics
		@energy 60%			// #metrics
		@run 1.8 km			// #exercise
		@swim 900 meter		// #exercise
		@write 950 words	// #work
		@read 2.4 hours		// #work

	Friday
		@weight 80.5 kg		// #metrics
		@bmi 21.0			// #metrics
		@sleep 6.8 hours	// #metrics
		@energy 80%			// #metrics
		@run 2.4 km			// #exercise
		#void
		@write 700 words	// #work
		@read 1.2 hours		// #work

	Saturday	// #weekend
		#void
		#void
		@sleep 7.8 hours	// #metrics
		@energy 80%			// #metrics
		@run 0.25 km		// #exercise
			It started raining right after I started
		@run 4.0 km			// #exercise
		#void
		#void

	Sunday		// #weekend
		#void
		#void
		@sleep 7.2 hours	// #metrics
		#void
		#void
		#void
		@write 1600 words	// #work
		#void
		#void

Week 23

	Monday
		@weight 79.3 kg		// #metrics
		@bmi 20.9			// #metrics
		@sleep 6.2 hours	// #metrics
		@energy 70%			// #metrics
		@run 2.0 km			// #exercise
		@swim 900 meter		// #exercise
		@write 1000 words	// #work
		@read 2.0 hours		// #work

	Tuesday
		#void
		#void
		@sleep 6.8 hours	// #metrics
		@energy 80%			// #metrics
		@run 2.5 km			// #exercise
		@swim 1100 meter	// #exercise
		@write 1200 words	// #work
		@read 1.6 hours		// #work

	Wednesday
		#void
		#void
		@sleep 6.0 hours	// #metrics
		@energy 66%			// #metrics
		#void
		#void
		#void
		@read 3.5 hours 	// #work

	Thursday
		#void
		#void
		@sleep 5.8h			// #metrics
		@energy 60%			// #metrics
		@run 2.2km			// #exercise
		@swim 1300m			// #exercise
		@write 900w			// #work
		@read 2.1h			// #work

	Friday
		@weight 78.6kg		// #metrics
		@bmi 20.8			// #metrics
		@sleep 8.0h			// #metrics
		@energy 80%			// #metrics
		#void
		#void
		@write 650w			// #work
		@read 1.0h			// #work

	Saturday	// #weekend
		#void
		#void
		@sleep 8.2h			// #metrics
		#void
		won my age group!	// @run 5.0km	#exercise
			What an amazing day — I won my age group!
			Didn’t even feel like I was pushing myself
		#void
		#void
		#void

	Sunday		// #weekend
		#void
		#void
		@sleep 7.5h	// #metrics
		@energy 80%	// #metrics
		#void
		#void
		#void
		#void

- Week 24
	Monday
	Tuesday
	Wednesday
	Thursday
	Friday
	Saturday
	Sunday
````

{{< /details >}}


## Definitions

<!--Definitions control how graphs are shown.

We want each graph to 'fill' the whole 'vertical' range, so we can see several graphs side by side. But different 'measures' may have different 'ranges', so tiny values risk not being shown as a flat line along the bottom, and high values risk going outside of the screen.

For example, @swim distance could be measured in meters between 200 and 2000 as in the example here, while in another 'context', @accuracy could be set to have values between 0 and 1, eg 0.87. This way both types of graphs will be shown as 'filling' out ['using'] the full height available. And if the values are 'outside' the specified range, the points will still be drawn in their correct, 'relative' position - over or under the graph area.

In the Global Definitions, you can make the definition for each type of data, and specify its range-->

Definitions control how values are interpreted and displayed in graphs.

Different types of data often use very different ranges. For example, how far you <kbd>@swim</kbd> on a specific day may vary between 200 and 2000 meters, while a value such as <kbd>@accuracy</kbd> might use decimal values between 0 and 1, such as 0.87.

Definitions allow each type of data to use an appropriate range, ensuring graphs remain readable and visually balanced even when very different measurements are shown side by side.

In <kbd>Settings -> Definitions</kbd>, you can specify settings such as unit, minimum value, and maximum value for each key.

{{< theme-figure
  light="/img/texttotile-settings-definitions-light.png"
  dark="/img/texttotile-settings-definitions-dark.png"
  alt="Editing definitions in Settings"
  title="Editing definitions in Settings (click to zoom)"
>}}

Below are the definitions used for the examples shown. The minimum and maximum ranges should be chosen to match the expected range of the data.

```text {filename="Settings -> Definitions"}
#def {key: weight;	unit: kg;		min: 75;	max: 85; }
#def {key: bmi;		unit: bmi;		min: 20;	max: 24; }
#def {key: swim;	unit: meter;	min: 200;	max: 2000; }
#def {key: run;		unit: km;		min: 1;		max: 5; }
#def {key: sleep;	unit: hours;	min: 4;		max: 8; }
#def {key: energy;	unit: %;		min: 0;		max: 100; }
#def {key: write;	unit: words;	min: 500;	max: 2500; }
#def {key: read;	unit: hours;	min: 0;		max: 4; }
```

The ranges do not need to be perfectly precise. In practice, approximate values are often enough to create useful visual overviews and readable graphs.

Definitions can be adjusted at any time. Changing a definition immediately affects how matching graphs are displayed across all outlines using that key.

{{< callout type="info" >}}
Definitions can also be written directly inside individual documents, but they are typically better placed in the Definitions tab in Settings so they can be reused across multiple outlines.
{{< /callout >}}



## Setting Height of Graph Area

<!--In the Structure View - where each text line is shown as parts in a chart - the Graphs area is always aligned to the bottom of the view. On the menu <kbd>View - Graph height</kbd> you can select how much of the lower 'end' of the view that the graphs should 'take up'. You can select between 25%, 50%, 75% and 100% and the choice will typically be based on the size of the screen, the number of parts in the chart, and how much detail you want to see in the chart.

In the Tag Groups View where each group of tags are shown as <kbd>tagblocks</kbd>, each of the graphs are shown 'alongside' their individual tagblock, making it easy to separate them. This is useful eg. when viewing 'progressions' for separate 'entities'. This could be story arcs for individual characters or themes in a novel, or measurement of the same 'thing' for different persons.

When showing Tag Groups View, every tagblock that has one or more graphs will be shown with a minimum height of 10 lines - in order to show the graphs properly.-->


In <kbd>Structure View</kbd>, graphs are aligned to the bottom of the chart area. Under <kbd>View → Graph Height</kbd>, you can choose how much vertical space the graphs should occupy — from 25% to 100%.

The ideal setting depends on:
- screen size
- number of visible parts
- and how much detail you want to see

In <kbd>Tag Groups View</kbd>, each tagblock displays its own graphs independently, making it easier to compare separate entities, themes, or categories side by side.



<!-- invisible image required to trigger Hextra’s medium-zoom initialization, enabling zoom on shortcode-rendered images -->
![hextra-zoom-init](/img/transparent.png "")

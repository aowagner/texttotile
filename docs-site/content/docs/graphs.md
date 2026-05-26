---
date: '2026-02-28T11:09:51+01:00'
title: 'Graphs'
weight: 34
---

## Visualizing Metadata

Building on the outline introduced on the Basics page, we can now extend it with structured values - called `metadata` - that are automatically visualized as graphs.

By adding a simple `@` prefix to values in your text, you can turn them into structured data.

When metadata values are written using the `@key value` format, TextToTile recognizes them as numeric values and automatically connects matching keys into timelines in the chart. For example, all `@sleep` values become part of the same graph.

For readability, units can be written directly after the value — such as `@sleep 6.5h` or `@sleep 6.5 hours` — or omitted entirely. The values are still recognized correctly. The metadata can appear anywhere in the text. Some people may prefer a short logging style, while others may mix values naturally into longer notes or journal-like writing.

```
- @sleep 8.5
- @sleep 8.5h
- Last night I managed to @sleep 8.5 hours — apparently I needed it :-)
```

You can also include multiple `@key value` entries on the same line, allowing multiple metrics to be tracked for that part.

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

Definitions control how values are interpreted and displayed in graphs.

Different types of data often use very different ranges. For example, how far you `@swim` on a specific day may vary between 200 and 2000 meters, while a value such as `@accuracy` might use decimal values between 0 and 1, such as 0.87.

Definitions allow each type of data to use an appropriate range, ensuring graphs remain readable and visually balanced even when very different measurements are shown side by side.

In `Settings -> Definitions`, you can specify settings such as unit, minimum value, and maximum value for each key.

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

In `Structure View`, graphs are aligned to the bottom of the chart area. Under `View → Graph Height`, you can choose how much vertical space the graphs should occupy — from 25% to 100%.

The ideal setting depends on:
- screen size
- number of visible parts
- and how much detail you want to see

In `Tag Groups View`, each tagblock displays its own graphs independently, making it easier to compare separate entities, themes, or categories side by side.



<!-- invisible image required to trigger Hextra’s medium-zoom initialization, enabling zoom on shortcode-rendered images -->
![hextra-zoom-init](/img/transparent.png "")

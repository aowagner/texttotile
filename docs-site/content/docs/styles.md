---
date: '2026-02-28T11:09:51+01:00'
title: 'Styles'
weight: 36
---

## Styling with Tags

The first tag found on a line is used to assign the part to a tag group.

Additional tags can then be added to control styling and categorization in more detail.

In the screenshot below, the entries from the [Graphs page](/docs/34-graphs/) — originally grouped only by the three primary tags `#metrics`, `#exercise`, and `#work` — have now been extended with more specific tags describing the type of information more precisely.

This makes it possible to style individual parts with distinct colors, icons, and highlighted values.

{{< theme-figure
  light="/img/texttotile-tracking4-light.png"
  dark="/img/texttotile-tracking4-dark.png"
  alt="Additional tags allow more detailed styling and categorization"
  title="Additional tags allow more detailed styling and categorization (click to zoom)"
>}}

The following tag styles have been added to the CSS block:

```text {filename="Added tag styles"}
#weight	{@fill: firebrick;		@icon: ⚖️;	@show: weight; }
#bmi	{@fill: red;			@icon: 📏;	@show: bmi; }
#sleep	{@fill: orangered;		@icon: 🛌;	@show: sleep; }
#energy	{@fill: orange;			@icon: ⚡;	@show: energy; }
#run	{@fill: mediumseagreen;	@icon: 🏃‍♂️;	@show: run; }
#swim	{@fill: teal;			@icon: 🏊;	@show: swim; }
#write	{@fill: dodgerblue;		@icon: ✍️;	@show: write; }
#read	{@fill: slateblue;		@icon: 📚;	@show: read; }
```

Like the earlier examples, these styles define a `@fill` value controlling the color of the part. In addition, they now also define:

* `@icon`, which specifies the icon shown for the part — usually any emoji symbol chosen by the user
* `@show`, which controls which value should be highlighted inside the part

In `Structure View`, icons are shown in the lower-right corner of each part. In `Tag Groups View`, the icon is placed to the left of the text, leaving more space available for longer content.

To apply a specific style to sleep-related entries, it is enough to add the `#sleep` tag to the relevant lines:

```text {filename="Line with added tag"}
@sleep 8.5 hours	// #metrics #sleep
```

Even though the line still belongs to the `#metrics` tag group — because `#metrics` remains the first tag on the line — the additional `#sleep` style overrides the shared group color and applies its own styling.

{{< details title="Full text from screenshot above" closed="true" >}}

````markdown {filename="Markdown"}
```css
#metrics	{@fill: darkorange; }
#exercise	{@fill: seagreen; }
#work		{@fill: steelblue; }

#weight	{@fill: firebrick;		@icon: ⚖️;	@show: weight; }
#bmi	{@fill: red;			@icon: 📏;	@show: bmi; }
#sleep	{@fill: orangered;		@icon: 🛌;	@show: sleep; }
#energy	{@fill: orange;			@icon: ⚡;	@show: energy; }
#run	{@fill: mediumseagreen;	@icon: 🏃‍♂️;	@show: run; }
#swim	{@fill: teal;			@icon: 🏊;	@show: swim; }
#write	{@fill: dodgerblue;		@icon: ✍️;	@show: write; }
#read	{@fill: slateblue;		@icon: 📚;	@show: read; }

#weekend 	{@fill: dodgerblue; }
```

Week 22

	Monday	           	    	
		@weight 79.8 kg		// #metrics #weight
		@bmi 20.5			// #metrics #bmi
		@sleep 8.5 hours	// #metrics #sleep
			Apparently I needed it :-)
		@energy 70%			// #metrics #energy
		@run 1.5 km			// #exercise #run
		@swim 800 meter		// #exercise #swim

		@write 900 words	// #work #write
			Editing main plot lines	// #work #novel

		@read 1.5 hours		// #work #read
			Anathem: Chapter 7	// #novel #stephenson
				Interesting shift in pace — more philosophical than earlier chapters
				Still getting used to the terminology, but the world is starting to make more sense

			Medium: Articles on coding	// #coding
				A few solid takeaways — especially about keeping code readable

	Tuesday
		#void
		#void
		@sleep 6.5 hours	// #metrics #sleep
		@energy 80%			// #metrics #energy
		@run 2.0 km			// #exercise #run
		@swim 1200 meter	// #exercise #swim
		@write 1100 words	// #work #write
		@read 2.5 hours		// #work #read
			Anathem: Chapter 8+9	// #novel #stephenson

	Wednesday
		#void
		#void
		@sleep 6.2 hours	// #metrics #sleep
		@energy 70%			// #metrics #energy
		#void
		#void
		@write 850 words	// #work #write
		@read 2.2 hours		// #work #read

	Thursday
		#void
		#void
		@sleep 5.6 hours	// #metrics #sleep
		@energy 60%			// #metrics #energy
		@run 1.8 km			// #exercise #run
		@swim 900 meter		// #exercise #swim
		@write 950 words	// #work #write
		@read 2.4 hours		// #work #read

	Friday
		@weight 80.5 kg		// #metrics #weight
		@bmi 21.0			// #metrics #bmi
		@sleep 6.8 hours	// #metrics #sleep
		@energy 80%			// #metrics #energy
		@run 2.4 km			// #exercise #run
		#void
		@write 700 words	// #work #write
		@read 1.2 hours		// #work #read

	Saturday	// #weekend
		#void
		#void
		@sleep 7.8 hours	// #metrics #sleep
		@energy 80%			// #metrics #energy
		@run 0.25 km		// #exercise #run
			It started raining right after I started
		@run 4.0 km			// #exercise #run
		#void
		#void

	Sunday		// #weekend
		#void
		#void
		@sleep 7.2 hours	// #metrics #sleep
		#void
		#void
		#void
		@write 1600 words	// #work #write
		#void
		#void

Week 23

	Monday
		@weight 79.3 kg		// #metrics #weight
		@bmi 20.9			// #metrics #bmi
		@sleep 6.2 hours	// #metrics #sleep
		@energy 70%			// #metrics #energy
		@run 2.0 km			// #exercise #run
		@swim 900 meter		// #exercise #swim
		@write 1000 words	// #work #write
		@read 2.0 hours		// #work #read

	Tuesday
		#void
		#void
		@sleep 6.8 hours	// #metrics #sleep
		@energy 80%			// #metrics #energy
		@run 2.5 km			// #exercise #run
		@swim 1100 meter	// #exercise #swim
		@write 1200 words	// #work #write
		@read 1.6 hours		// #work #read

	Wednesday
		#void
		#void
		@sleep 6.0 hours	// #metrics #sleep
		@energy 66%			// #metrics #energy
		#void
		#void
		#void
		@read 3.5 hours 	// #work #read

	Thursday
		#void
		#void
		@sleep 5.8h			// #metrics #sleep
		@energy 60%			// #metrics #energy
		@run 2.2km			// #exercise #run
		@swim 1300m			// #exercise #swim
		@write 900w			// #work #write
		@read 2.1h			// #work #read

	Friday
		@weight 78.6kg		// #metrics #weight
		@bmi 20.8			// #metrics #bmi
		@sleep 8.0h			// #metrics #sleep
		@energy 80%			// #metrics #energy
		#void
		#void
		@write 650w			// #work #write
		@read 1.0h			// #work #read

	Saturday	// #weekend
		#void
		#void
		@sleep 8.2h			// #metrics #sleep
		#void
		won my age group!	// @run 5.0km	#exercise #run @icon🥇
			What an amazing day — I won my age group!
			Didn’t even feel like I was pushing myself
		#void
		#void
		#void

	Sunday		// #weekend
		#void
		#void
		@sleep 7.5h	// #metrics #sleep
		@energy 80%	// #metrics #energy
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


{{< theme-figure
  light="/img/texttotile-tracking4b-light.png"
  dark="/img/texttotile-tracking4b-dark.png"
  alt="The same outline in Tag Groups View"
  title="The same outline in Tag Groups View (click to zoom)"
>}}

The screenshot above shows the same outline in `Tag Groups View`. The parts still remain inside their original tag groups, while individual colors and icons make different types of information easier to distinguish visually.

The added color variation also makes it easier to identify and follow values in the graphs.



## Show Highlighted Value

In the screenshots above, individual parts display selected values prominently inside the chart.

Using the `@show` field in a style definition, you can choose which value should be highlighted for parts using that tag.

```text {filename="Tag style"}
#sleep	{@fill: orangered;	@icon: 🛌;	@show: sleep; }
```

Writing `@show: sleep;` tells TextToTile to display the sleep value prominently inside the part instead of showing the full text.

The selected value is displayed together with its label and unit, while the original text remains unchanged in the outline itself.

The result is a live dashboard built entirely from plain text: simple to write, easy to adjust, and always up to date — allowing you to keep track of the information most important to you.



## Global Style

Maintaining the same styles separately in many different documents can quickly become repetitive.

In `Settings → Style`, you can define styles globally so they are automatically available across all outlines.

{{< theme-figure
  light="/img/texttotile-settings-style-light.png"
  dark="/img/texttotile-settings-style-dark.png"
  alt="Styles moved from document to Settings"
  title="Styles moved from document to Settings (click to zoom)"
>}}

The screenshot above shows how the styles previously written directly inside the document have now been moved into `Settings → Style`.

Styles can still be added locally inside individual documents whenever needed. This can be useful for quick experimentation, document-specific styling, or temporarily overriding global styles.

If the same tag is defined both globally and inside the current document, the local document style always takes precedence.



## Inline Styling

In addition to styling chart elements like Sections, Columns, and Parts using tags, you can also style individual elements directly by adding styling `metadata` to specific lines in your outline.

Inline styling uses the same metadata syntax described on the [Graphs page](/docs/34-graphs/), but with a set of reserved metadata keys used specifically for visual styling. Unlike custom metadata values, these styling keys are not shown as graphs.


{{< theme-figure
  light="/img/texttotile-tracking9-light.png"
  dark="/img/texttotile-tracking9-dark.png"
  alt="Inline styling applied to a column and an individual part"
  title="Inline styling applied to a column and an individual part"
>}}

In the example above, the selected line in the Editor has been given the metadata keys `@saturation` and `@text`.

The part already receives its main fill color from its tag, but `@saturation 90` increases the intensity of that color, while `@text white` changes the text color to white.

The column containing the part has also been styled directly using the metadata keys `@fill gold` and `@saturation 40`, making it stand out visually from the surrounding columns representing the other days of the week.

{{< callout type="info" >}}
When a styling key is specified inline, it overrides any styling inherited from tags for that specific element.
{{< /callout >}}

If you open the full text below, the highlighted lines show exactly how the column and part styling was added inline.


{{< details title="Full text from screenshot above" closed="true" >}}

Below is the full text used in the example shown in the screenshot above.

````markdown {hl_lines=[138,143], linenostart=1, filename="Outline"}
Week 22

	Monday	           	    	
		@weight 79.8 kg		// #metrics #weight
		@bmi 20.5			// #metrics #bmi
		@sleep 8.5 hours	// #metrics #sleep
			Apparently I needed it :-)
		@energy 70%			// #metrics #energy
		@run 1.5 km			// #exercise #run
		@swim 800 meter		// #exercise #swim

		@write 900 words	// #work #write
			Editing main plot lines	// #work #novel

		@read 1.5 hours		// #work #read
			Anathem: Chapter 7	// #novel #stephenson
				Interesting shift in pace — more philosophical than earlier chapters
				Still getting used to the terminology, but the world is starting to make more sense

			Medium: Articles on coding	// #coding
				A few solid takeaways — especially about keeping code readable

	Tuesday
		#void
		#void
		@sleep 6.5 hours	// #metrics #sleep
		@energy 80%			// #metrics #energy
		@run 2.0 km			// #exercise #run
		@swim 1200 meter	// #exercise #swim
		@write 1100 words	// #work #write
		@read 2.5 hours		// #work #read
			Anathem: Chapter 8+9	// #novel #stephenson

	Wednesday
		#void
		#void
		@sleep 6.2 hours	// #metrics #sleep
		@energy 70%			// #metrics #energy
		#void
		#void
		@write 850 words	// #work #write
		@read 2.2 hours		// #work #read

	Thursday
		#void
		#void
		@sleep 5.6 hours	// #metrics #sleep
		@energy 60%			// #metrics #energy
		@run 1.8 km			// #exercise #run
		@swim 900 meter		// #exercise #swim
		@write 950 words	// #work #write
		@read 2.4 hours		// #work #read

	Friday
		@weight 80.5 kg		// #metrics #weight
		@bmi 21.0			// #metrics #bmi
		@sleep 6.8 hours	// #metrics #sleep
		@energy 80%			// #metrics #energy
		@run 2.4 km			// #exercise #run
		#void
		@write 700 words	// #work #write
		@read 1.2 hours		// #work #read

	Saturday	// #weekend
		#void
		#void
		@sleep 7.8 hours	// #metrics #sleep
		@energy 80%			// #metrics #energy
		@run 0.25 km		// #exercise #run
			It started raining right after I started
		@run 4.0 km			// #exercise #run
		#void
		#void

	Sunday		// #weekend
		#void
		#void
		@sleep 7.2 hours	// #metrics #sleep
		#void
		#void
		#void
		@write 1600 words	// #work #write
		#void
		#void

Week 23

	Monday
		@weight 79.3 kg		// #metrics #weight
		@bmi 20.9			// #metrics #bmi
		@sleep 6.2 hours	// #metrics #sleep
		@energy 70%			// #metrics #energy
		@run 2.0 km			// #exercise #run
		@swim 900 meter		// #exercise #swim
		@write 1000 words	// #work #write
		@read 2.0 hours		// #work #read

	Tuesday
		#void
		#void
		@sleep 6.8 hours	// #metrics #sleep
		@energy 80%			// #metrics #energy
		@run 2.5 km			// #exercise #run
		@swim 1100 meter	// #exercise #swim
		@write 1200 words	// #work #write
		@read 1.6 hours		// #work #read

	Wednesday
		#void
		#void
		@sleep 6.0 hours	// #metrics #sleep
		@energy 66%			// #metrics #energy
		#void
		#void
		#void
		@read 3.5 hours 	// #work #read

	Thursday
		#void
		#void
		@sleep 5.8h			// #metrics #sleep
		@energy 60%			// #metrics #energy
		@run 2.2km			// #exercise #run
		@swim 1300m			// #exercise #swim
		@write 900w			// #work #write
		@read 2.1h			// #work #read

	Friday
		@weight 78.6kg		// #metrics #weight
		@bmi 20.8			// #metrics #bmi
		@sleep 8.0h			// #metrics #sleep
		@energy 80%			// #metrics #energy
		#void
		#void
		@write 650w			// #work #write
		@read 1.0h			// #work #read

	Saturday	// #weekend		@fill gold, @saturation 40
		#void
		#void
		@sleep 8.2h			// #metrics #sleep
		#void
		@run 5.0km	// #exercise #run	@icon🥇, @saturation 90, @text white
			What an amazing day — I won my age group!
			Didn’t even feel like I was pushing myself
		#void
		#void
		#void

	Sunday		// #weekend
		#void
		#void
		@sleep 7.5h	// #metrics #sleep
		@energy 80%	// #metrics #energy
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

Inline styling is a quick alternative to creating a dedicated tag when you only want to highlight a specific line or element.

If you later find yourself reusing the same styling repeatedly, you can simply turn it into a reusable tag instead. For example, a new #win tag could combine the styling shown above:

```markdown {filename="New tag in Settings -> Style"}
#win {@icon🥇; @saturation 90; @text white; }
```

The line could then simply be written like this: `@run 5.0km	// #exercise #run #win`


### Styling Keys

The same styling keys used for tags can also be applied directly as inline metadata:

| Metadata key  | Styling 'effect' |
| :---- | :-- |
| **`@fill`** | Sets the background color of an element.<br>Both named CSS colors and hex values can be used - for example `red`, `gold`, `cornflowerblue`, or hex values like `#FF0000`, `#FFD700`, `#6495ED`.<br>See the full list of named colors here: [List of named colors](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/named-color) | 
| **`@text`** | The text color of an element.<br>The same color formats used for `@fill` can also be used here. |
| **`@bold`** | Controls the font weight of the text using standard CSS values such as `normal`, `bold`, or numeric values from `100` to `900`, where 100 is very thin and 900 is very bold. |
| **`@saturation`** | Controls the saturation of the `@fill` color using a value between 0 and 100. <br>By default, colors in TextToTile use lower saturation so they work well in both light and dark themes.|
| **`@stroke`** | Sets the border color of a part. <br>The same color formats used for `@fill` can also be used here. |
| **`@strokewidth`** | Sets the border width of a part in pixels. |







<!-- invisible image required to trigger Hextra’s medium-zoom initialization, enabling zoom on shortcode-rendered images -->
![hextra-zoom-init](/img/transparent.png "")



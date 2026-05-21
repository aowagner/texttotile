---
date: '2026-02-28T11:09:51+01:00'
title: 'Styles'
weight: 36
---

## Styling with Tags

<!--The first tag found on a line of text is used to assign the part it shows to a tag group.

But you can add more tags in order to control the styling of different parts further.

In the following screenshot, all lines [for parts] that  in the examples on the 'previous' Graphs page only had one of the three tags - metrics, excercise, work - now have an additional tag added, to 'pinpoint|specify|qualify' the type of information further, allowing us to style these parts in distinct ways.-->

The first tag found on a line is used to assign the part to a tag group.

Additional tags can then be added to control styling and categorization in more detail.

In the screenshot below, the entries from the [Graphs page](/docs/34-graphs/) — originally grouped only by the three primary tags <kbd>#metrics</kbd>, <kbd>#exercise</kbd>, and <kbd>#work</kbd> — have now been extended with more specific tags describing the type of information more precisely.

This makes it possible to style individual parts with distinct colors, icons, and highlighted values.

{{< theme-figure
  light="/img/texttotile-tracking4-light.png"
  dark="/img/texttotile-tracking4-dark.png"
  alt="Additional tags allow more detailed styling and categorization"
  title="Additional tags allow more detailed styling and categorization (click to zoom)"
>}}

<!--As you can see from the screenshot above - and in the full text further down - the following tag styles have been added to the css block:-->

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

<!--Like in the previous 'example', the new tag styles have a @fill attribute [or 'field'?], but in 'addition' they have an @icon field, and a @show field.

The @icon field controls which icon ['emoji'?] will be shown for each part. In Structure View, the icon is placed in the lower right corner of each part, and in Tag Groups View, it is placed to the left of the part, allowing the full text to be shown.

The @show field is used to control which metadata will be highlighted and will be explained in the next chapter below.

So all it takes to use a specific color and icon for all the sleep related lines with a #metric tag, is to add the new style #sleep to the relevant lines:-->

Like the earlier examples, these styles define a <kbd>@fill</kbd> value controlling the color of the part. In addition, they now also define:

* <kbd>@icon</kbd>, which specifies the icon shown for the part — usually any emoji symbol chosen by the user
* <kbd>@show</kbd>, which controls which value should be highlighted inside the part

In <kbd>Structure View</kbd>, icons are shown in the lower-right corner of each part. In <kbd>Tag Groups View</kbd>, the icon is placed to the left of the text, leaving more space available for longer content.

To apply a specific style to sleep-related entries, it is enough to add the <kbd>#sleep</kbd> tag to the relevant lines:

```text {filename="Line with added tag"}
@sleep 8.5 hours	// #metrics #sleep
```

<!--NB: Even though this line - and other lines with the same tags - still belongs to the #metrics tag group (becaused this is the first tag on the line) - it will now be styled with the 'combined' styles defined for #metrics and #sleep, giving it an <kbd>orange</kbd> @fill color (because the #sleep style is placed *after* the #metrics style in the css block, therefore 'overriding' its value for the same field.-->

Even though the line still belongs to the <kbd>#metrics</kbd> tag group — because <kbd>#metrics</kbd> remains the first tag on the line — the additional <kbd>#sleep</kbd> style overrides the shared group color and applies its own styling.

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

<!--Here, the same outline is shown in Tag Groups View, and it is 'clearly seen' how each part remain in the same tag group, but still have a unique fill color and icon. Having different colors for different tags also makes it easier to identify individual values on the graphs.-->

The screenshot above shows the same outline in <kbd>Tag Groups View</kbd>. The parts still remain inside their original tag groups, while individual colors and icons make different types of information easier to distinguish visually.

The added color variation also makes it easier to identify and follow values in the graphs.



## Show Highlighted Value

In the screenshots above, individual parts display selected values prominently inside the chart.

<!--With the <kbd>@show</kbd> field in your styles, you can choose which value should be highlighted. This displays the most relevant number in a larger font directly inside each part, while still keeping the original text in your outline.-->

Using the <kbd>@show</kbd> field in a style definition, you can choose which value should be highlighted for parts using that tag.

```text {filename="Tag style"}
#sleep	{@fill: orangered;	@icon: 🛌;	@show: sleep; }
```

<!--Writing <kbd>@show: sleep;</kbd> tells TextToTile to highlight the <kbd>sleep</kbd> value inside parts using that tag - instead of showing the full text. The part will display the selected value prominently, together with its label and unit.

The result is a live dashboard built entirely from plain text: simple to write, easy to adjust, and always up to date — allowing you to keep track of the information most important to you.-->

Writing <kbd>@show: sleep;</kbd> tells TextToTile to display the sleep value prominently inside the part instead of showing the full text.

The selected value is displayed together with its label and unit, while the original text remains unchanged in the outline itself.

The result is a live dashboard built entirely from plain text: simple to write, easy to adjust, and always up to date — allowing you to keep track of the information most important to you.



## Global Style

<!--Maintaining styles in each and every document that you create can quickly become 'a lot of administration' [or something like 'cumbersome'?] - especially if the same styles are used in many different documents.

In Settings -> Style you can write all your styles - or just the ones you use 'across' different documents.-->

Maintaining the same styles separately in many different documents can quickly become repetitive.

In <kbd>Settings → Style</kbd>, you can define styles globally so they are automatically available across all outlines.

{{< theme-figure
  light="/img/texttotile-settings-style-light.png"
  dark="/img/texttotile-settings-style-dark.png"
  alt="Styles moved from document to Settings"
  title="Styles moved from document to Settings (click to zoom)"
>}}

<!--The screenshot shows how the styles in the css-block has been completely removed from the text, and instead placed in Settings -> Style.

You can still decide to add styles to specific documents. This can be useful to quickly add some 'local' styling, or if you want to 'override' one or more styles from the global Settings -> Style. The 'rule' is simple: If the same tag is defined with a different color (or other 'field') in the document, it will always have the 'final word'.-->

The screenshot above shows how the styles previously written directly inside the document have now been moved into <kbd>Settings → Style</kbd>.

Styles can still be added locally inside individual documents whenever needed. This can be useful for quick experimentation, document-specific styling, or temporarily overriding global styles.

If the same tag is defined both globally and inside the current document, the local document style always takes precedence.



<!-- invisible image required to trigger Hextra’s medium-zoom initialization, enabling zoom on shortcode-rendered images -->
![hextra-zoom-init](/img/transparent.png "")



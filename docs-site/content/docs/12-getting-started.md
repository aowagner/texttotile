---
date: '2026-02-28T11:09:51+01:00'
title: 'Getting Started'
weight: 12
---
![hextra-zoom-init](/transparent.png "")

### Editor
Simple editor, no advanced features.

Indent using Tab and Shift+Tab. Pressing Enter keeps the current indentation level.

The editor can be positioned on the left or right side, as well as at the top or bottom of the app.


![Text is edited to the left and shown as chart to the right](/koncept-a9.png "Text is edited to the left and shown as chart to the right (click to zoom)")


Below is the full text used in the example shown in the screenshot above.


{{< details title="Full text from screenshot above" closed="true" >}}

```markdown
#gitte {@fill: green; }
#susan {@fill: red;}
#familie {@fill: blue; }


#mom {@fill: #ff6b9a; @stroke: #b63b66; text: #88f;}
#dad {@fill: #4d96ff; @stroke: #245bb5; text: #ffffff;}
#lia {@fill: #9b5de5; @stroke: #5f2ca0; text: #ffffff;}
#marcus {@fill: #00c2a8; @stroke: #00796b; text: #ffffff;}

#weekend {@fill: #ccc; }

#def {key: week; unit: week; min: 1; max: 52;}
#def {key: energy; unit: score; min: 0; max: 100;}
#def {key: stress; unit: score; min: 0; max: 100;}
#def {key: mood; unit: score; min: 0; max: 10;}
#def {key: load; unit: score; min: 0; max: 100;}
#def {key: familyTime; unit: hours; min: 0; max: 8;}

#def {key: duration; unit: minutes; min: 0; max: 300;}
#def {key: effort; unit: score; min: 0; max: 10;}
#def {key: focus; unit: score; min: 0; max: 10;}
#def {key: confidence; unit: score; min: 0; max: 10;}

#def {key: distance; unit: km; min: 10; max: 100;}
#def {key: score; unit: points; min: 0; max: 50;}

#def {key: cost; unit: currency; min: 0; max: 200;}

#def {key: recovery; unit: score; min: 0; max: 10;}

.top {--bold: 600; --xtext: black; }

:root {
	--xfill: #f5faff;
	--xfill: #335553;
	
}
```
{{< /details >}}

Works well with your regular editor

The built-in text editor is fine, but a natural use case is also to use the app as a viewer for outlines and files that you work on in other text-based applications, such as note apps like Obsidian or simple text editors like Visual Studio Code.

Auto-update when a file is edited—both in the built-in editor and in external apps.

Some apps save automatically (e.g. Obsidian), while others require an explicit save (e.g. Cmd/Ctrl+S).

- // Click section, chapter, parts -> selected given linie i Editor.



### Tags
Overview using tags

Many people already use tags.

Tags = classes. The first tag on a part determines its grouping, but all tags are applied as CSS classes in the rendered visualization.



![Text is edited to the left and shown as chart to the right](/koncept-a9.png "Text is edited to the left and shown as chart to the right (click to zoom)")

Below is the full text used in the example shown in the screenshot above.

{{< details title="Full text from screenshot above" closed="true" >}}

```css
#gitte {@fill: green; }
#susan {@fill: red;}
#familie {@fill: blue; }


#mom {@fill: #ff6b9a; @stroke: #b63b66; text: #88f;}
#dad {@fill: #4d96ff; @stroke: #245bb5; text: #ffffff;}
#lia {@fill: #9b5de5; @stroke: #5f2ca0; text: #ffffff;}
#marcus {@fill: #00c2a8; @stroke: #00796b; text: #ffffff;}

#weekend {@fill: #ccc; }

#def {key: week; unit: week; min: 1; max: 52;}
#def {key: energy; unit: score; min: 0; max: 100;}
#def {key: stress; unit: score; min: 0; max: 100;}
#def {key: mood; unit: score; min: 0; max: 10;}
#def {key: load; unit: score; min: 0; max: 100;}
#def {key: familyTime; unit: hours; min: 0; max: 8;}

#def {key: duration; unit: minutes; min: 0; max: 300;}
#def {key: effort; unit: score; min: 0; max: 10;}
#def {key: focus; unit: score; min: 0; max: 10;}
#def {key: confidence; unit: score; min: 0; max: 10;}

#def {key: distance; unit: km; min: 10; max: 100;}
#def {key: score; unit: points; min: 0; max: 50;}

#def {key: cost; unit: currency; min: 0; max: 200;}

#def {key: recovery; unit: score; min: 0; max: 10;}

.top {--bold: 600; --xtext: black; }

:root {
	--xfill: #f5faff;
	--xfill: #335553;
	
}
```
{{< /details >}}



### Graphs

![Text is edited to the left and shown as chart to the right](/koncept-a9.png "Text is edited to the left and shown as chart to the right (click to zoom)")

Below is the full text used in the example shown in the screenshot above.

{{< details title="Full text from screenshot above" closed="true" >}}

```css
#gitte {@fill: green; }
#susan {@fill: red;}
#familie {@fill: blue; }


#mom {@fill: #ff6b9a; @stroke: #b63b66; text: #88f;}
#dad {@fill: #4d96ff; @stroke: #245bb5; text: #ffffff;}
#lia {@fill: #9b5de5; @stroke: #5f2ca0; text: #ffffff;}
#marcus {@fill: #00c2a8; @stroke: #00796b; text: #ffffff;}

#weekend {@fill: #ccc; }

#def {key: week; unit: week; min: 1; max: 52;}
#def {key: energy; unit: score; min: 0; max: 100;}
#def {key: stress; unit: score; min: 0; max: 100;}
#def {key: mood; unit: score; min: 0; max: 10;}
#def {key: load; unit: score; min: 0; max: 100;}
#def {key: familyTime; unit: hours; min: 0; max: 8;}

#def {key: duration; unit: minutes; min: 0; max: 300;}
#def {key: effort; unit: score; min: 0; max: 10;}
#def {key: focus; unit: score; min: 0; max: 10;}
#def {key: confidence; unit: score; min: 0; max: 10;}

#def {key: distance; unit: km; min: 10; max: 100;}
#def {key: score; unit: points; min: 0; max: 50;}

#def {key: cost; unit: currency; min: 0; max: 200;}

#def {key: recovery; unit: score; min: 0; max: 10;}

.top {--bold: 600; --xtext: black; }

:root {
	--xfill: #f5faff;
	--xfill: #335553;
	
}
```
{{< /details >}}

### Views
Structure view and Tags view. Select via the menu or with F1/F2.

The three levels can be used for anything: calendars, outlines, measurements, etc.

Zoom (and scale) available.



![Text is edited to the left and shown as chart to the right](/koncept-a9.png "Text is edited to the left and shown as chart to the right (click to zoom)")

Below is the full text used in the example shown in the screenshot above.

{{< details title="Full text from screenshot above" closed="true" >}}

```css
#gitte {@fill: green; }
#susan {@fill: red;}
#familie {@fill: blue; }


#mom {@fill: #ff6b9a; @stroke: #b63b66; text: #88f;}
#dad {@fill: #4d96ff; @stroke: #245bb5; text: #ffffff;}
#lia {@fill: #9b5de5; @stroke: #5f2ca0; text: #ffffff;}
#marcus {@fill: #00c2a8; @stroke: #00796b; text: #ffffff;}

#weekend {@fill: #ccc; }

#def {key: week; unit: week; min: 1; max: 52;}
#def {key: energy; unit: score; min: 0; max: 100;}
#def {key: stress; unit: score; min: 0; max: 100;}
#def {key: mood; unit: score; min: 0; max: 10;}
#def {key: load; unit: score; min: 0; max: 100;}
#def {key: familyTime; unit: hours; min: 0; max: 8;}

#def {key: duration; unit: minutes; min: 0; max: 300;}
#def {key: effort; unit: score; min: 0; max: 10;}
#def {key: focus; unit: score; min: 0; max: 10;}
#def {key: confidence; unit: score; min: 0; max: 10;}

#def {key: distance; unit: km; min: 10; max: 100;}
#def {key: score; unit: points; min: 0; max: 50;}

#def {key: cost; unit: currency; min: 0; max: 200;}

#def {key: recovery; unit: score; min: 0; max: 10;}

.top {--bold: 600; --xtext: black; }

:root {
	--xfill: #f5faff;
	--xfill: #335553;
	
}
```
{{< /details >}}


### Styling with tags and levels
Define general colors for tags that can be used on any line in the outline. Multiple tags can be used on a single line. The first tag determines the part’s grouping in the Tags view.

Define explicit colors for Section, Chapter, and Part (levels 1, 2, 3). For example, to highlight a specific day if the outline contains calendar data.



![Text is edited to the left and shown as chart to the right](/koncept-a9.png "Text is edited to the left and shown as chart to the right (click to zoom)")

Below is the full text used in the example shown in the screenshot above.

{{< details title="Full text from screenshot above" closed="true" >}}

Her ses den Markdown tekst der er brugt i den omtalte situation

```markdown
# Overskrift

Her er #noget tekst

## Overskrift 2

Her er *noget* mere tekst og #tags der bruges til at styre visning!


```
{{< /details >}}











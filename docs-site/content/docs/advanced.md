---
date: '2026-02-28T11:09:51+01:00'
title: 'Advanced'
weight: 52
---

## Styling the app

Tags control how the content of an outline appears. In `Settings → Style`, you can also customize the overall appearance of TextToTile itself: its fonts, background, text color, outlines, and more.

These are global styles, so they apply across all outlines. Changes take effect immediately.

{{< theme-figure
  light="/img/texttotile-advanced1-light.png"
  dark="/img/texttotile-advanced1-dark.png"
  alt="Adjusting UI elements"
  title="Adjusting UI elements (click to zoom)"
>}}


{{< callout type="info" >}}
Use the theme toggle in the documentation navigation to view this example in both light and dark mode.
{{< /callout >}}


## Global style defaults

The `:root` rule sets defaults shared by both themes. The `:root.theme-light` and `:root.theme-dark` rules then set values only for the active theme.

In the example above, `--fontfamily` changes the app’s general font to `verdana`, while `--fontfamily-texteditor` changes the editor font to `courier`. The color values for `--fill`, `--text`, and `--stroke` differ between the light and dark themes.

```css {filename="Settings → Style"}
:root {
  --fontfamily: verdana;
  --fontfamily-texteditor: courier;
  --strokewidth: 3;
}

:root.theme-light {
  --fill: #f0f8ff;
  --text: darkblue;
  --stroke: white;
}

:root.theme-dark {
  --fill: #800;
  --text: yellow;
  --stroke: black;
}
```

`--fill` sets the overall background color of the app, `--text` sets the default text color, and `--stroke` and `--strokewidth` control outlines. You can use any valid CSS color name or hex color value.




{{< callout type="info" >}}
Global styles establish the default appearance. Tag styles and inline styling remain more specific, so they continue to override the defaults where they apply.

Place your `:root` and theme rules at the top of `Settings → Style`, followed by tag-specific rules. This keeps the order and priority of your styles predictable. See [Styling with Tags](/docs/styles/) for tag and inline styling examples.
{{< /callout >}}



<!-- invisible image required to trigger Hextra’s medium-zoom initialization, enabling zoom on shortcode-rendered images -->
![hextra-zoom-init](/img/transparent.png "")

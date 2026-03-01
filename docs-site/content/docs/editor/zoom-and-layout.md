---
date: '2026-02-28T11:09:51+01:00'
draft: true
title: 'Zoom and Layout'
---


Hugo supports [Markdown](https://en.wikipedia.org/wiki/Markdown) syntax for formatting text, creating lists, and more. This page will show you some of the most common Markdown syntax examples.

<!--more-->

## Markdown Examples

### Styling Text

| Style         | Syntax                   | Example                                 | Output                                |
| :------------ | :----------------------- | :-------------------------------------- | :------------------------------------ |
| Bold          | `**bold text**`          | `**bold text**`                         | **bold text**                         |
| Italic        | `*italicized text*`      | `*italicized text*`                     | _italicized text_                     |
| Strikethrough | `~~strikethrough text~~` | `~~strikethrough text~~`                | ~~strikethrough text~~                |
| Subscript     | `<sub></sub>`            | `This is a <sub>subscript</sub> text`   | This is a <sub>subscript</sub> text   |
| Superscript   | `<sup></sup>`            | `This is a <sup>superscript</sup> text` | This is a <sup>superscript</sup> text |

### Blockquotes

Blockquote with attribution

> Don't communicate by sharing memory, share memory by communicating.<br>
> — <cite>Rob Pike[^1]</cite>

[^1]: The above quote is excerpted from Rob Pike's [talk](https://www.youtube.com/watch?v=PAAkCSZUG1c) during Gopherfest, November 18, 2015.

```markdown {filename=Markdown}
> Don't communicate by sharing memory, share memory by communicating.<br>
> — <cite>Rob Pike[^1]</cite>

[^1]: The above quote is excerpted from Rob Pike's [talk](https://www.youtube.com/watch?v=PAAkCSZUG1c) during Gopherfest, November 18, 2015.
```

#### Jakob test

Her er noget mere indrykket tekst

### Alerts

Alerts are a Markdown extension based on the blockquote syntax that you can use to emphasize critical information.
[GitHub-style alerts](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#alerts) are supported.
Please make sure you are using the latest version of Hextra and [Hugo v0.146.0](https://github.com/gohugoio/hugo/releases/tag/v0.146.0) or later.

> [!NOTE]
> Useful information that users should know, even when skimming content.

> [!TIP]
> Helpful advice for doing things better or more easily.

> [!IMPORTANT]
> Key information users need to know to achieve their goal.

> [!WARNING]
> Urgent info that needs immediate user attention to avoid problems.

> [!CAUTION]
> Advises about risks or negative outcomes of certain actions.

```markdown {filename=Markdown}
> [!NOTE]
> Useful information that users should know, even when skimming content.

> [!TIP]
> Helpful advice for doing things better or more easily.

> [!IMPORTANT]
> Key information users need to know to achieve their goal.

> [!WARNING]
> Urgent info that needs immediate user attention to avoid problems.

> [!CAUTION]
> Advises about risks or negative outcomes of certain actions.
```

### Tables

Tables aren't part of the core Markdown spec, but Hugo supports them out-of-the-box.

| Name  | Age |
| :---- | :-- |
| Bob   | 27  |
| Alice | 23  |

```markdown {filename=Markdown}
| Name  | Age |
| :---- | :-- |
| Bob   | 27  |
| Alice | 23  |
```

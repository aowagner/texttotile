---
date: '2026-02-28T11:09:51+01:00'
title: 'Download'
weight: 70
---

## Beta Release

TextToTile is currently available as a public beta release for macOS, Windows, and Linux.

The current beta version is:

> **Version 0.9.0 Beta**


This app is currently distributed as an unsigned beta application. Depending on your operating system, you may therefore need to manually confirm that you want to open or install the app.

This will become simpler in future releases once code signing and notarization have been added.



### macOS

Universal build for both Apple Silicon and Intel Macs.

[Download TextToTile for macOS](https://github.com/aowagner/texttotile/releases/download/v0.9.0-beta/TextToTile_0.9.0_universal.dmg)

When opening the app for the first time, macOS may display a warning that the application cannot be verified.

{{< figure
  src="/img/texttotile-install-mac-as1.png"
  alt="macOS warning that TextToTile cannot be verified"
  caption="On first launch, macOS blocks the unsigned app."
  class="img-install-mac-dialog"
>}}

To open the app:

1. Try opening the app normally once.
2. When the warning (image above) appears, click `Done`.
3. Open `System Settings` → `Privacy & Security`.
4. Scroll down to the security section near the bottom.
5. Click `Open Anyway` for TextToTile.

{{< figure
  src="/img/texttotile-install-mac-as2.png"
  alt="Privacy & Security settings showing the Open Anyway button for TextToTile"
  caption="The `Open Anyway` button in `System Settings` → `Privacy & Security`."
  class="img-install-mac-settings"
>}}

6. In the confirmation dialog, click `Open Anyway` again.

{{< figure
  src="/img/texttotile-install-mac-as3.png"
  alt="Final macOS confirmation dialog with the Open Anyway button"
  caption="Confirm once more by clicking `Open Anyway`."
  class="img-install-mac-dialog"
>}}

You normally need to complete these steps only once for each downloaded version of TextToTile.


{{< callout type="info" >}}
On older macOS versions, the wording and layout may differ slightly. The first warning may show `Cancel` instead of `Done`. Click `Cancel`, then continue to `System Settings` → `Privacy & Security`, where the `Open Anyway` option should appear.
{{< /callout >}}


### Windows

64-bit Windows installer.

[Download TextToTile for Windows](https://github.com/aowagner/texttotile/releases/download/v0.9.0-beta/TextToTile_0.9.0_x64_en-US.msi)

Windows SmartScreen may display a warning when starting the installer.

{{< figure
  src="/img/texttotile-install-win1.png"
  alt="Microsoft Defender SmartScreen warning with a More info link"
  caption="The initial SmartScreen warning. Click `More info` to continue."
  class="img-install-win-dialog"
>}}

To continue:

1. Click `More info`.
2. Click `Run anyway`.
3. In the TextToTile Setup window, click `Next`.
4. Click `Install`.
5. If User Account Control asks whether you want to allow the installer to make changes to your device, click `Yes`.
6. When the installation is complete, click `Finish`.

You normally see these warnings only when running the installer for a newly downloaded version.


### Linux

64-bit Linux builds are available as an AppImage or a Debian package.

#### AppImage

The AppImage is portable and can be run without installation.

[Download TextToTile AppImage](https://github.com/aowagner/texttotile/releases/download/v0.9.0-beta/TextToTile_0.9.0_amd64.AppImage)

You may need to make the file executable before starting it:

```bash
chmod +x TextToTile_0.9.0_amd64.AppImage
```

#### Debian package

The `.deb` package can be installed on Debian, Ubuntu, and compatible 64-bit Linux distributions.

[Download TextToTile Debian package](https://github.com/aowagner/texttotile/releases/download/v0.9.0-beta/TextToTile_0.9.0_amd64.deb)

Install it through your system’s graphical package installer or from a terminal:

```bash
sudo apt install ./TextToTile_0.9.0_amd64.deb
```



### All Releases

Older and experimental releases can be found here:

[View all releases](https://github.com/aowagner/texttotile/releases)



## Feedback

Questions, ideas, bug reports, and workflow discussions are very welcome.

[Open TextToTile Discussions](https://github.com/aowagner/texttotile/discussions)



### Support

If TextToTile is useful to you, you can support its continued development.

<a href="https://ko-fi.com/D7E125V6O0" target="_blank">
<img src="https://ko-fi.com/img/githubbutton_sm.svg">
</a>

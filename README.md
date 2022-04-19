# Genshin Elemental HUD
Adds a mini HUD to the game to show Elemental Skill CD for all party members  
(*This project is still work in progress, but the basic functionality works*)

* Download the latest version [here](https://github.com/bobyclaws/genshin-elemental-hud/releases)

* Write your Issues/Feature Requests [here](https://github.com/bobyclaws/genshin-elemental-hud/issues)

## Features

#### Working On
* edit party without restarting app

#### Needs Fixing
* don't show HUD icon in taskbar , show in taskbar tray instead
* HUD is not transparent for mouse input
* hide HUD when opening map, game menu etc.,
* Childe dynamic cooldown
* Party Loadouts (or) Auto Party Detection
* changing HUD location on screen

#### Not possible
* taking into account CD reductions from bennett, Chongyun etc.,
* CD extension by certain monsters

## Usage

1) edit `party.yaml` with notepad to list your party members. check `data.yaml` for exact character names (no need to type exact names in future)

```
- venti
- zhongli
- hu tao
- xingqiu
```
2) right click `estatus.exe` > 'Run as Administrator' (required for genshin, unless you want to play in windowed mode)

3) press `F11` in game to show HUD, `F12` to hide

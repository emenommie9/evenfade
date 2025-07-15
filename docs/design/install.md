# Install

The game will be distributed with an installer, via a package manager or via flatpak.

## Install directories

The following directories are used by Evenfade.
- `EVENFADE_BINARY_DIR`. The directory of the `evenfade` binary and other bundled binaries.
- `EVENFADE_SHARE_DIR`. The directory of the shared config and mods (such as core mod).
- `EVENFADE_USER_SHARE_DIR`. The directory of the user-specific config and mods (such as custom adventures).
- `EVENFADE_STATE_DIR`. The directory for logs and other user-specific written data.

### For Linux when installed on the system

`EVENFADE_BINARY_DIR` is `/usr/bin`.

`EVENFADE_SHARE_DIR` is `/usr/share/evenfade`.

`EVENFADE_USER_SHARE_DIR` is `XDG_DATA_HOME` or `/home/user/.local/share/evenfade`.

`EVENFADE_STATE_DIR` is `XDG_STATE_HOME` or `/home/user/.local/state/evenfade`.

XDG_CONFIG_HOME will not be used to keep the user data cohesive, since there is not much point having config files without the other "config" such as mods.

### For Flatpak

Identifier: org.evenfade.game

Based on org.kde.Platform/Sdk to allow for QT for the editor.

```yml
id: org.evenfade.game
runtime: org.kde.Platform
runtime-version: '6.9'
sdk: org.kde.Sdk
command: evenfade
modules:
  ...
```

`EVENFADE_BINARY_DIR` and `EVENFADE_SHARE_DIR` is the sandbox root.

`EVENFADE_USER_SHARE_DIR` and `EVENFADE_STATE_DIR` is `/home/user/.var/app/org.evenfade.game/`.

### For Windows

`EVENFADE_BINARY_DIR` and `EVENFADE_SHARE_DIR` is install directory (by default `C:\Program Files\evenfade`).

`EVENFADE_USER_SHARE_DIR` and `EVENFADE_STATE_DIR` is `C:\Users\user\Documents\evenfade`.

### For MacOS

`EVENFADE_BINARY_DIR` is `/Applications`.

`EVENFADE_SHARE_DIR` is `/Library/evenfade`.

`EVENFADE_USER_SHARE_DIR` and `EVENFADE_STATE_DIR` is `$HOME/Library/evenfade`.

### Standalone

Over time adventures will not be possible to play reliably on modern versions. Subtle differences in the code can accrue over time. There will always be a need to be able to install an older version to play an older adventure (or just to see how the older version looked).

This should be possible to accomplish with a standalone version. These versions have no desktop integration (like shortcuts or menu entries). They can essentially be put on a USB-drive.

`EVENFADE_BINARY_DIR`, `EVENFADE_SHARE_DIR`, `EVENFADE_USER_SHARE_DIR` and `EVENFADE_STATE_DIR` are all the same directory as the one the binary is in.

### Development

During development the example core mod can be used instead of the actual core mod. The example core mod is included in the main repository under `example/mods/evenfade-core/...`. Note that it is not a zip-archive in order to simplify development.

Necessary config files, saved games and characters are placed in the `example/...` directory.

## Installed files

The location of the directories are baked into the executables at compile-time. The locations can be changed via environment variables afterwards if necessary.

- `$EVENFADE_BINARY_DIR`
    - evenfade
        - The main game executable.
    - evenfade-server
        - A dedicated server for hosting an adventure for multiplayer.
    - evenfade-editor
        - An editor for creating adventures.

- `$EVENFADE_SHARE_DIR`
    - mods/evenfade-core.mod
        - The core mod of the game, containing the required assets to run.
    - packs/evenfade-default.pack
        - The default pack for adventures, containing resources for the main adventure and for custom adventures to build upon.
    - adventures/...
        - The main adventures that are shipped with the game.

- `$EVENFADE_USER_SHARE_DIR`
    - adventures
        - A directory containing adventures that the player can host and play.
    - characters
        - A directory containing characters that can be used in adventures.
    - mods
        - A directory containing user included mods (zip-files) to be loaded by the game on startup and can replace resources globally and change game behavior.
    - packs
        - A directory containing packs that adventures can use for additional resources in the game.
    - saves
        - A directory containing saved games.
    - config.json
        - The main configuration file, for video/audio/control settings.

- `$EVENFADE_STATE_DIR`
    - logs
        - A directory for logs outputted e.g. by a script in an adventure.

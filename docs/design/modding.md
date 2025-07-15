# Modding

Since the game has a heavy focus on user created content it needs to be easy to add or replace content in the game. It should for instance be easy to create a new adventure and share it with others. It should also be easy to mod other aspects of the game, such as adding new models, redesigning the UI, creating custom shaders or modifying game logic.

To accomplish this the game will have to be data-driven when it comes to the things that can be modded and provide a robust API for modifying game logic.

The three categories of modifications that will be supported are:
- mods
- adventures
- packs

Mods are loaded by the game immediately on startup. They can override/add resources globally and inject logic to be run at predefined events. They can use an API provided by the game to do certain actions in game.

Adventures are loaded by the game when that adventure is selected to be played. Adventures can also override/add resources for this specific adventure. They also provide all resources required to play the adventure, such as places, conversations, scripts, etc.

Adventures can depend on packs that need to be present to be able to play the adventure. Packs can override/add resources for the adventures that depend on them and are only loaded by the game if an adventure has it as a dependency.

`$EVENFADE_SHARE_DIR`/mods/evenfade-core.mod

`$EVENFADE_SHARE_DIR`/packs/evenfade-default.pack

`$EVENFADE_SHARE_DIR`/adventures/evenfade.adv

`$EVENFADE_USER_SHARE_DIR`/mods/mod-1.mod

`$EVENFADE_USER_SHARE_DIR`/packs/pack-1.pack

`$EVENFADE_USER_SHARE_DIR`/adventures/adventure-1.adv

All mods, adventures and packs should be zip archives with .mod, .adv or .pack file endings respectively. To simplify development, a directory can be used instead of the archive, in that case without the file ending. If both directory and archive is present with the same name, only the directory is loaded by the game.

## Mods

The first thing the game will handle is loading mods. All mods except the core mod is by default disabled. The enabled mods and their loading order is defined by the `$EVENFADE_USER_SHARE_DIR`/config.json file.

```json
"active_mods": [
    "$EVENFADE_SHARE_DIR/mods/evenfade-core.mod",
    "$EVENFADE_USER_SHARE_DIR/mods/mod-1.mod"
]
```

This array should be possible to edit via the settings in game.

The game will load the config file and find the active mods and their order. It will do an initial pass through the mods in loading order to find any changes to the loading UI and its localization and load that first (to allow for a modded loading screen). The game will then display the loading screen and start indexing the resources and what mod to fetch them from.

```
mod-1
    assets
        models
        textures
        shaders
        fonts
    scripts
        main.lua (required)
        ...
    ui
        loading-screen.html
        ...
    localization
        loading-screen.json
        ...
    icon-64.png
    icon-256.png
    manifest.json
```

The individual resources that can be overridden or added are described in more detail in separate sections.

The `main.lua` script will be called by the game on startup and is where the mod can set up event handlers to react to game events. Other lua modules can be imported via standard syntax and the scripts folder is the root.

The `manifest.json` will include the name, description and version of the mod and the icons alongside it will be presented in the game when enabling mods. The manifest can also specify resources that should override resources in adventures/packs (otherwise adventures/packs take precedence).

```json
{
    "name": "Mod 1",
    "description": "An example mod for the documentation",
    "version": "1.0.1",
    "override": [
        "assets/models/head-1.gltf"
    ]
}
```

## Adventures

Adventures are loaded by the game when it is selected to be played. They can contain areas, conversations, scripts and all other resources required to describe the adventure. They also contain a JSON file giving the adventure a name, a version, a description, starting locations, etc.

To be described in more detail once the details have been decided upon.

## Packs

Packs are loaded by adventures or transitively by other packs. Similar to mods they contain resources that can be used by an adventure but are only loaded for the adventures that require them.
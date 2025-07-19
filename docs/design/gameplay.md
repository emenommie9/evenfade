# Gameplay

To summarize the gameplay: you control one or more characters and order them to move or interact with objects or other characters, you engage in combat with enemy characters and find items to use or sell and gain experience to advance in level to fight tougher enemies.

There is of course a lot of detail that needs to be elaborated on. This design document is likely going to evolve over time as the game loop becomes more refined.

## Character Generation

After selecting an adventure to play you can create your characters. You can create one or multiple (in case you want to play with a party or multiplayer). The adventure can recommend a number of players but can not disallow multiple characters. The host can however limit the number of characters (e.g. for single character servers).

For a character you will go through the following steps:

1. Skills
2. Appearance
    1. Species
    2. Gender, pronouns
    3. Skin, hair, feature customization
    4. Voice customization
3. Biography
    1. Name
    2. Description
    3. Background
    4. Deity
4. Confirm

Skills is the only thing that will actually affect gameplay apart from conditions on other things put into the adventure by the creator.

Skills is what determines what kind of character it will be: in other words it determines your build. In essence the skills make up a tree where you will need prerequisite skills in order to pick later ones at level ups.

This system will need refining. The goal is to allow **many different archetypes to be played** and to be viable (no "best" build that everyone must play). It should also allow players to invest in their social skills which can be used for roleplaying in the adventures.

## Controls

After generating a party and starting the adventure you are placed at the starting positions. You start in with a zoomed out, top down, tactical camera mode. In this mode you can move your camera freely across the area, select your characters and move them individually through the area. You can also select a single character and change your camera mode to a chase mode for that character which allows you to get closer and use WASD to move them individually.

You can select your characters with LMB and tell them to move to locations within the area with LMB. You can use RMB to perform an action at a location or an object (such as "open chest", "cast spell", "unlock"). You can use LMB to perform a default action, e.g. attacking an enemy character.

## Areas

An adventure is divided into areas that the player can move within. Areas are connected via transitions such as doors or map edges. Areas can be both exterior and interior with cosmetic differences such as day/night cycles, weather, skybox, lighting, etc.

Areas have terrain composed of 3D tiles, cliffs and heightmaps with additional objects placed within (interactable and non-interactable). This will also need some experimentation to figure out good rules for terrain composition.

## Combat

The central piece of gameplay is the combat. The player can LMB click an enemy to auto-attack them with their weapon. They can also use RMB or saved shortcuts to use special attacks such as combat moves or spells. They can also access their inventory and use potions or magical items.

This system will also need more experimentation to find a good ruleset and will be described in more detail in another document.
# Simple CRPG

The purpose of this example is to show the louder engine interacting with rpg mechanics.
 - Equipment
 - Inventory
 - Skills
 - Experience and Levels
 - Allocating levels to skills
 - Dice rolls
 - Story state
 - Thoughts
 - Areas

The UI is a means to an end here, not supposed to be super artistic, but functional.
 - Help menu
 - Quit
 - Switch characters
 - Manage equipment
 - Manage inventory
 - Manage skills/allocate levels to skills
 - Manage experience
 - Show conversations in current area/manage movement into conversations
 - Show areas/manage movement into area
 - Inspect dice roll / dialogue card
 - Make choice
 - Visualize conversation (debug)
 - Event log (debug)
 - Visualize users
 - Visualize present users

Each character has an independent inventory, skills, experience, levels, position in the world and in dialogue. 
Characters can be configured through the Louder engine using additional data.

I'll try not to be as formal as the louder docs since that would definitely be inefficient here.

# Intended Experience
Play time: ~ 10 minutes

Player should experience:
 - Item management
 - Dialogue
 - Dice rolls
 - Leveling up
 - A cohesive small quest

I wanted money, health and morale. But I don't really have an in-story role for these. Either way, not having morale will force me to make dice checks more compelling on their own.

# Concepts

## Characters
Characters are a name, inventory, equipment, skills, and thoughts. Viewable via "inventory", "skills", "thoughts", and changeable via "switch"

## Interactables
Interactables are conversations with a screen name and a short description. Viewable via "talk".

## Areas
Areas are collections of interactables and doors mapping to other areas. Doors are visible via "move".

## Inventory
Items arranged in a grid and equipment in a vertical list attached to a character. Items can be hovered and selected, then swapped with adjacent spaces. Selecting an equipment piece swaps it immediately into the appropriate equipment slot, selecting the old item instead.

Equipment pieces can be stored in the inventory or on the person on dedicated slots.

## Skills
Skills arranged in a horizontal list with descriptions visible on hover. Selecting a skill with levels allocates a level to that skill. Level and experience are visible here also.

Different characters have different skills with their own descriptions, but they all have 6 and the skills are functionally the same.

Character also has a "total level" visible here, equal to the total number of levels spent.

## Thoughts
Thoughts arranged in a vertical list with descriptions on hover. Selecting a thought places it into an open cabinet slot, if available. 

There are limited cabinet slots. Additional slots can be unlocked with levels.\

## Dialogue
Overhaul of the conversation-interface example with "cards." Some choices have a special symbol (ill pick from https://github.com/nihitdev/prompt-symbols ?). You can hover over this with the arrows and a card will render, similar to thought or skill descriptions.

In the case of this demo, this will be used to render a dice check. When a choice is a dice check, it will have different outcomes dependent on a 2d6 + bonuses from associated skills, and save its state until the player levels an associated skill. The card shows the display probability, and the skill(s) attached.

By convention, choices which do something in the story will also be tagged with a quick description of what they'll do in parentheses. Such as "Sounds like a deal. (buy dagger)"

Each characters' skills speak to them occasionally as thoughts in their head. This is different from "unlocking a thought".

# UI
```
Help menu:

All menus can be closed with Esc.

S, switch <character> - switch character, or just `switch` to list available characters
i, inventory - open inventory and equipment menu
s, skills - open skills menu
T, thoughts - open thoughts menu
t, talk <interactable> - talk to an interactable point in your current area, or just `talk` to list interactable points
m, move <door> - walk to a new area with new interactables, or just `move` to list doors.
v - while in dialogue, visualize the conversation
log - show the event log for your current character
logall - show the global event log
```



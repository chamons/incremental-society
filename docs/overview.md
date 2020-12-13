# Incremental Society

Incremental Society is a kingdom builder/simulator incremental roguelite game. Yes, that's a lot of areas to draw from. It that might be crazy enough to work.

The core gameplay is to build a society by assigning jobs, building buildings, researching technologies, and making decisions in religion, law, and culture.

Over time your society will be challenged by both internal (unrest, civil war) and external (plague, famine, invasion) threats. At some point, your k will fail and you'll start over. Each trip through human events will build a prestige currency that will make future runs easier or more varied.

# Gameplay Concepts

## Resources

Resource management is the first primary leg of gameplay.

Society is fueled by resources, and the collection and refinement of resources is a central focus of early eras.

A small stockpile of goods provides some buffer on short falls, but the flow of goods is most important.

Resources are produced by filled jobs. Each job requires both a citizen to fill it, and an available slot. Slots are provided by districts build in your land. 

Some jobs produce raw resources directly (Hunter produces food and skins), which others refine one resource into another (Charcoal Burner consumes wood to produce fuel).

All of these resources are consumed by a number of outputs in your society:

- Citizen Needs (Food initially, clothing, fuel, and luxury goods as society advances)
- Society (Government) Overhead
- Research
- Religion
- Warfare (Equipping and maintaining an army)
- Large (Wonder) Building Programs

## Measures

Your society is measured on 5 axis (Stability, Happiness, Health, Conviction, and Might). These may rise or fall based upon resources consumed, choices made, events and challenges that occur.

The most critical of these is Stability, which is your health of your kingdom. If it reaches zero at any point, your civilization falls and you must begin again. You government is the primary driver of Stability, but Happiness, Conviction, and military outcomes also provide significant input here.

Happiness measures how content citizens are with your rule. This drops quickly when required goods are not provided (food, later fuel, clothing and goods), and slower during conflict and other challenges. Low health/conviction affects happiness.

Health measures how healthy your citizens are as measured by the age. This is affected indirectly by region planning (rural areas tend to have positive health and urban areas negative health due to overcrowding) and directly by challenges such as plague. 

Conviction measures the power of a society's belief in a shared faith. Having many religious buildings maintained by a priesthood raises this slowly over time, while reforms and revelations decrease it. 

Might measures the power of a kingdom's military. It passively grows with a positive population growth rate, and later can be actively maintained by drafting and maintaining a professional army or hiring mercenaries. 

## Decision Points

There are many "knobs" the player will turn throughout the game. 

Jobs:  There will always be more jobs available that population to work them. Deciding what to staff is the most obvious decision point.
Edicts: These are are free to toggle and provide a resource conversion over time. 
    - Example: "Feasts" edict converts Food to a small amount of research
Policies: These require a moderate amount of a resources to choose between, and make a significant change to districts, jobs or society as a whole
    - Example: The "Forced Labor" policy could be set to "None", "Slavery", or "State Workforce", with various bonuses and negatives
Decisions: These are one time decisions that can not be undone. Advancing to another age, applying upgrading, or government changes could go here.
Regions: The kingdom is made up of one or more regions, each with:

⁃ Type: plains, mountain, seaside, etc
⁃ Characteristics: River, forest
⁃ Rich in resources: fertile, clay, iron, gold
⁃ Size: Total districts (increased over time by infrastructure tech)
⁃ Valid districts: types and %

## Pops

Each population has a current job and some experience in that role. Should pops have a class/rank?

Moving jobs resets this experience, which will require time to reach their old level.


## Design Goals

Despite the incremental roots, time only moves when the game is active. Time can been speed up / slowed down.

No clicking, just choices and decisions gameplay.

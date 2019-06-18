---
layout: post
title:  "Many Pieces to Get Started"
---

It's always a good first step in a project to set an initial goal, and for incremental-society the goal was to create all of the game systems needed for the Stone age.

As the ages progress, additional game systems will be layered in (Religion, Trade, Warfare), but the base will remain constant. I wanted a rough but complete version of the stone age to determine a few things:

- Is it fun to play?
- Is there enough choice even in the ancient era to keep things interesting?
- Are the technical design decisions scalable enough for simulating an entire society.

It turns out, there were a *lot* more moving pieces than I originally expected, and I want to dig into how we got there.

The most fundamental gameplay loop is resources, buildings, people:

- Buildings produce resources (more buildings are unlocked by more people).
- People need resources to survive
- Resources are needed to make more buildings (and thus room for people)

It turns out that each of those need refinements:

- Buildings needed some employment mechanism to prevent cheesing by growing a large population, building lots of building, and then dropping the population cap to kick everyone out
- Buildings needed to be able to "convert" one good to another - drying clay to make bricks
- The original logarithmic population growth curves were much too simple, as it had many "silly" outcomes. We need to model birth/deaths/immigration/emigration.
- Resources were split into "required goods", which if unsatisfied will hard prevent population above their carry capacity, and "luxury goods" which if unmet will decrease growth rates (and then indirectly a cap)
- This resources split also gave an additional resource "sink" for a society, most of the resources are spent on the population.

Once that was all sorted out, we now have a basic "build buildings to get resources, grow population so you can build more buildings, repeat" loop. That works for a few minutes, and spamming gathering camps / huts does mirror early human expansion, but isn't very satisfying long term.

Building more buildings is fine, but we need to layer complexity on top. We need to progress through more complex buildings and goods, and to feel "progression" we need technology. The technology system was originally a simple "tree" with dependencies, which works but doesn't usually provide _that_ much choice.

It was expanded upon with "branch" points, where one of three specialization options are given. This allows us to model choices like "should your society farm, fish, or raise cattle", which all can have different buildings and resources loops. These branch points can be found again later down the tree, so in the end you'll collect all three.

Research is a primary resource dump, but the question was raised, "how do we convert resources to research". The obvious thing is to provide a building that creates research, but that did not feel thematic in the stone age. People didn't sit in libraries pondering the secrets of nature.

When I thought of history, it struck me the number of feasts, festivals, and gatherings societies of all types have held. I decided to make "edicts", a single shot conversion of one resource to another with a cooldown. In the stone age you can burn lots of food (and some firewood) to hold a feast and maybe come up with a bit of knowledge. It can be very inefficient, which will drive the player to tech towards more efficient sources of knowledge.

Once all of that was done, the last absolutely required system was a system for generating regions. Many buildings will have different uses in different terrains (you can't fish in a land of hills), so having a unique starting region is important.

It turns out that takes a bit over 4,000 lines of C# to express.

Now that all of that is sorted out, I'll need to spend a lot of time creating content (buildings, tech, edicts) and playtesting to make it fun and interesting, but it's a start.

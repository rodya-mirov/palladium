Palladium is fairly ambitious, but (at time of writing) very small and
there are a lot of gaps, even in the plan. So this document intends to
capture two things -- the overall "vision" of the game, to guide the
roadmap, as well as the roadmap divided into manageable chunks, to
guide development.

Vision
--

Palladium is a game of resets. You wake up on a spaceship (maybe?) which is
in bad shape and unfamiliar to you. You can explore around, and try to make
sense of what's happening, but it's dangerous and not very inviting.

Eventually you'll die -- due to fire, or starvation, or suffocation, or a
host of other terrible things that happen in space. Then you'll reappear
right where you started, with the skills and memories you made, but you'll
have to do it all again. Hopefully better this time.

Palladium is a "roguelike" in a very vague sense: the gameplay is a series
of "turns" (you move or take some action, then every other entity in the
world also does a tick, then the game waits for you again). It's also a
"roguelike" in that the graphics are very simple (currently ASCII), and
there are procedural elements.

However some tropes of roguelikes are not going to be used here. Most
obvious is the twist on the death mechanic -- when you die, you lose your
stuff, but you retain your skills and memories, and you come back in a
world that's identical to where you started. So you can avoid making the
same mistakes. At the same time, it's not always possible to "win" the
map you see -- you'll probably want to strategically "waste your life"
developing some skills or exploring, so in the next try you'll have some
advantage.

Also combat should be deemphasized. While there may be some, the primary
obstacle should be overcoming ... obstacles. Like starving to death, or
running out of air, or getting through a locked door, or not getting
sucked into the cold vacuum of space.

Plot should be present, but pieced together in the background. Dead cells
is a good example for how to show plot in a roguelike, like this. The
"determinism" of the map helps space out the plot beats.



Roadmap
--

* Engine
  - [x] Line of sight 
  - [x] Distinction between "unexplored / explored but not visible / visible" in rendering
  - [x] Modals with choices
  - [x] Objects?
  - [ ] NPCs? (which move around in a "turn-based" manner)
  - [ ] Save / load
* Interactivity
* Danger
* Obstacles
* Automation?
* Aliens
* Skills


Scenarios
--

This is not an attempt to map out the whole game. Instead, here are some things I would like to be part of the game, at some point, and which I think are microcosms (in some sense) of my vision for the game.

* **Plot Threads:** little notes, scattered around, which explain the world (things like the surrounding society, the nature of the Tether, other characters the player may or may not meet, etc.). These are in fixed locations (on the floor, in terminals, etc.) which the player can read, and build a feeling for the game's plot thereby.

* **Oxygen**: most living things need air to breathe. This scenario calls for certain "air problems" to exist (e.g. a busted airlock or meteor hole sucking the air out into space). Lack of air kills anything that needs air, eventually (fairly quickly). This can be a danger to the player, but can also be used strategically to solve other problems (kill bad guys, put out fires, etc.), for example by forcing certain doors open or shut, to control which parts fo the map do and do not have air. Also, as the player "levels up" through exposure to low-air environments, they should (slightly) improve their ability to hold their breath, and should be able to perceive (through a toggleable UI overlay) what the air level around them is.

* **Hacking**: in the scifi setting, hacking is a crucial tool. Many thing should be computerized, and most computerized things should be hackable (again, dependent on skill and possibly equipment). For example, an automated door could be hacked to stay open permanently, stay closed permanently, to only open for certain entities, etc.
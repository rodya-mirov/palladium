TODO for individual features (commits may handle any number of these)

### More Systems

- Windows (see through, not pass through; hackable? to open/close and allow airflow?)
- Fire (spreads to burnables, consumes oxygen, burns out when stuff burns up or it
  runs out of air)
- When an NPC dies it should leave behind a corpse (or something?
- [R] to repair broken objects (doors, lights, airlocks, windows?); require skill, require components
- Sounds (nearby things can hear them, some kind of transient entity or maybe just another callback)
  Maybe a good time to add a "transient" tag to entities and delete them at the beginning of each update loop
  Maybe not
- Inventory (objects can be possessed by an entity; must be collected somehow?)

### Refactors

- If we disable the parallel feature, can we use dispatchers? This would be a serious
  help if (e.g.) we want persistent state across runs of a System (e.g. caching)
- Destructible / mutable tiles (as setup for a big meteor hitting the station, or
  a big bomb going off, or whatever)
- Move components into their own crate
- Move worldgen into its own crate
- Move loadable into its own crate (and add a proc-macro derive crate as well)

### UI/UX

- Some kind of indicator that the player is "stuck in place" (e.g. when hacking)
- Some kind of indicator in hacks of how long it will take
- More interesting space background (not just black)

### Bugs

- You can see door state being updated offscreen (because doors are memorable);
  to fix this we need that "memory system"
- If you die while hacking, it tries to keep hacking (due to QueuedPlayerActions
  not being save/loaded). Blocked by I don't know how to ser/deser Entity objects

### Improvements

- Put RNG in resources (with save/load) and get consistent entity iteration order
  so it's impossible to game the RNG with save/load)
- CanSuffocate should be a part of Breathe
- Context-aware control indicators (e.g. "[H] Hack" only appears if there is an adjacent hackable)

### Big

- Seriously consider if the "spaceship" setting is interesting enough (maybe not)
- Start thinking about more interesting worldgen
- Start thinking about how to plug in skills

### Performance Stuff

- "Has changed" tag on certain components, manually done (FlaggedStorage is probably not good enough)
  then use this for efficiency things (e.g. only update oxygen containers if they're adjacent to something
  that changed)
- SmallVec implementation (so no heap allocation until it's too big; may help with oxygen) (probably just
  pull in a dependency; also currently not using vecs on anything important)

TODO for individual features (commits may handle any number of these)

### More Systems

- Windows (see through, not pass through; hackable? to open/close and allow airflow?)
- Fire (spreads to burnables, consumes oxygen, burns out when stuff burns up or it
  runs out of air)

### Refactors

- DialogueCallback -> Callback; we should be able to launch callbacks wherever, and
  catch them with a later system, instead of tying them to dialogue
  - while you're in there: when you die, it should launch the opening dialogue again
- If we disable the parallel feature, can we use dispatchers? This would be a serious
  help if (e.g.) we want persistent state across runs of a System (e.g. caching)
- Destructible / mutable tiles (as setup for a big meteor hitting the station, or
  a big bomb going off, or whatever)

### UI/UX

- Some kind of indicator that the player is "stuck in place" (e.g. when hacking)
- Some kind of indicator in hacks of how long it will take
- More interesting space background (not just black)

### Big

- Seriously consider if the "spaceship" setting is interesting enough (maybe not)
- Start thinking about more interesting worldgen
- Start thinking about NPCs
- Start thinking about how to plug in skills

### Performance Stuff

- "Has changed" tag on certain components, manually done (FlaggedStorage is probably not good enough)
  then use this for efficiency things (e.g. only update oxygen containers if they're adjacent to something
  that changed)
- SmallVec implementation (so no heap allocation until it's too big; may help with oxygen) (probably just
  pull in a dependency; also currently not using vecs on anything important)

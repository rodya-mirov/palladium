TODO for individual features (commits may handle any number of these)

### Easy

- Windows (hackable?)
- Some kind of indicator that the player is "stuck in place" (e.g. when hacking)
- Some kind of indicator in hacks of how long it will take

### Not Easy

- More interesting space background (not just black)

### More Features

- Fire (spreads to burnables, consumes oxygen, burns out when stuff burns up or it
  runs out of air)

### Big

- Start thinking about more interesting worldgen
- Start thinking about NPCs
- Start thinking about how to plug in skills
- Start thinking about death / rebirth

### Performance Stuff

- "Has changed" tag on certain components, manually done (FlaggedStorage is probably not good enough)
  then use this for efficiency things (e.g. only update oxygen containers if they're adjacent to something
  that changed)
- SmallVec implementation (so no heap allocation until it's too big; may help with oxygen) (probably just
  pull in a dependency; also currently not using vecs on anything important)

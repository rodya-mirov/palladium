pub mod oxygen {
    // By default, all oxygen containers have this container
    pub const DEFAULT_FULL_OXYGEN: usize = 60;

    // Constants describing the speed od the oxygen spread system
    pub const OXYGEN_SYSTEM_ITERATIONS: usize = 10;
    pub const OXYGEN_SYSTEM_SHARE_PER_ITERATION: usize = 2;

    // Constants describing breath; when and how fast
    // the breath level changes
    pub const FAST_GAIN_SPEED: usize = 2;
    pub const FAST_GAIN_THRESHOLD: usize = 50;

    pub const SLOW_GAIN_SPEED: usize = 1;
    pub const SLOW_GAIN_THRESHOLD: usize = 40;

    pub const SLOW_DROP_SPEED: usize = 1;
    pub const SLOW_DROP_THRESHOLD: usize = 20;

    pub const FAST_DROP_SPEED: usize = 2;
}

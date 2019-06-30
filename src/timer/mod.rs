#![allow(dead_code)]
#![allow(unused_macros)]

#[cfg(feature = "timing")]
use std::time::Instant;

#[cfg(feature = "timing")]
pub struct Timer {
    name: &'static str,
    start: Instant,
}

const MIN_MS: u32 = 8; // don't log time unless the timer got above this number

#[cfg(feature = "timing")]
impl Timer {
    pub fn new(name: &'static str) -> Timer {
        Timer {
            name,
            start: Instant::now(),
        }
    }

    pub fn stop(self) {
        let end = Instant::now();
        let dur = end - self.start;

        let secs = dur.as_secs();
        let ms = dur.subsec_millis();

        if secs > 0 || ms >= MIN_MS {
            println!("'{}' took {}.{:03} s", self.name, secs, ms);
        }
    }
}

#[cfg(feature = "timing")]
macro_rules! timed {
    ($name:expr, $to_run:expr) => {{
        let t = timer::Timer::new($name);
        let out = $to_run;
        t.stop();
        out
    }};
}

// No-op macro so when timing is not turned on, it has zero effect (name is ignored)
#[cfg(not(feature = "timing"))]
macro_rules! timed {
    ($name:expr, $to_run:expr) => {
        $to_run;
    };
}

// This is made purely so you can easily turn "timed" to "not_timed" when you want to turn it
// off (but still debug other things)
macro_rules! not_timed {
    ($name:expr, $to_run:expr) => {
        $to_run;
    };
}

#[cfg(feature = "timing")]
macro_rules! perf_log {
    ($lit:expr, $($arg:expr),*) => {
        println!(
            $lit
            $(
                ,$arg
            )*
        );
    }
}

#[cfg(not(feature = "timing"))]
macro_rules! perf_log {
    ($lit:expr, $($arg:expr),*) => {};
}

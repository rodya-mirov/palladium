#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct Float {
    val: f32,
}

impl Float {
    pub fn make_unchecked(val: f32) -> Float {
        Float::make_checked(val).unwrap()
    }

    pub fn make_checked(val: f32) -> Result<Float, ()> {
        if val.is_nan() {
            Err(())
        } else {
            Ok(Float { val })
        }
    }
}

impl Eq for Float {}

impl Ord for Float {
    fn cmp(&self, other: &Float) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<f32> for Float {
    fn from(val: f32) -> Float {
        Float::make_unchecked(val)
    }
}

#[allow(clippy::if_same_then_else)] // shutup clippy
pub fn force_max(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        b
    } else if b.is_nan() {
        a
    } else if a > b {
        a
    } else {
        b
    }
}

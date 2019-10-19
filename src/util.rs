/// Maps the value `val` onto the output range `min..max` from the input range `min_orig..max_orig`
/// Shamelessly stolen from arduino
pub fn map(val: f32, min_orig: f32, max_orig: f32, min: f32, max: f32) -> f32 {
    (val - min_orig) * (max - min) / (max_orig - min_orig) + min
}

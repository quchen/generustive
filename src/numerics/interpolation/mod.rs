pub fn lerp((in_lo, in_hi): (f64, f64), (out_lo, out_hi): (f64, f64), t: f64) -> f64 {
    out_lo + (out_hi - out_lo) / (in_hi - in_lo) * (t - in_lo)
}

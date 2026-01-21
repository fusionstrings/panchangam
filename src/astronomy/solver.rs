
/// Tolerance for time convergence (approx 1 second in days)
const TIME_EPSILON: f64 = 1.0 / 86400.0;

// Standard binary search to find when a geometric longitude reaches a target value.
// 
// `calculate_value`: A closure that takes a Julian Day and returns the value (e.g., Tithi angle).
// `start_jd`: Search start time.
// `end_jd`: Search end time.
// `target_value`: The value we want to cross.
// `period`: The cycle length (e.g., 360.0 degrees) to handle wrap-around.
pub fn find_crossing_time<F>(
    mut calculate_value: F,
    start_jd: f64,
    end_jd: f64,
    target_value: f64,
    period: f64,
) -> Option<f64> 
where
    F: FnMut(f64) -> f64,
{
    let mut low_time = start_jd;
    let mut high_time = end_jd;
    let mut mid_time;

    // Check bounds
    // We only compute this to ensure we are searching effectively, though binary search
    // simply narrows the gap.
    
    // Safety check: if gap is too small, return.
    if (high_time - low_time) < TIME_EPSILON {
        return Some(low_time);
    }

    for _ in 0..64 { // Max iterations to prevent infinite loop
        mid_time = (low_time + high_time) / 2.0;
        
        if (high_time - low_time) < TIME_EPSILON {
            return Some(mid_time);
        }

        let mut mid_value = calculate_value(mid_time);
        
        // Unwind wrap-around relative to target
        if period > 0.0 {
            mid_value = normalize_relative(mid_value, target_value, period);
        }

        // Binary search assumes value increases with time (or we handle signs).
        // Since we normalize relative to target (making target "0" or "center"),
        // and assuming we are approaching target from below...
        if mid_value < target_value {
            low_time = mid_time;
        } else {
            high_time = mid_time;
        }
    }

    Some((low_time + high_time) / 2.0)
}

/// Find when an angular value crosses a target (0-360 degrees context)
pub fn find_angle_crossing<F>(
    calculate_angle: F,
    start_jd: f64,
    end_jd: f64,
    target_angle: f64,
) -> Option<f64>
where
    F: FnMut(f64) -> f64,
{
    find_crossing_time(calculate_angle, start_jd, end_jd, target_angle, 360.0)
}

/// Normalize val to be within [-period/2, +period/2] of ref_val
fn normalize_relative(val: f64, ref_val: f64, period: f64) -> f64 {
    let mut diff = val - ref_val;
    if diff > period / 2.0 {
        diff -= period;
    } else if diff < -period / 2.0 {
        diff += period;
    }
    ref_val + diff
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::PI;

    #[test]
    fn test_find_crossing_linear() {
        // Find where 2*x = 10. Should be 5.0.
        // Normalized: (2*x - 10) should be 0.
        let result = find_crossing_time(
            |x| 2.0 * x,
            0.0,
            10.0,
            10.0,
            0.0 // No wrap period
        );
        assert!(result.is_some());
        let val = result.unwrap();
        assert!((val - 5.0).abs() < 1e-4);
    }

    #[test]
    fn test_find_crossing_sine() {
        // Find where sin(x) = 0.5 in [0, PI]. Should be PI/6 (0.52359...) and 5*PI/6.
        // Our solver finds *one* crossing.
        let result = find_crossing_time(
            |x| x.sin(),
            0.0,
            PI/2.0,
            0.5,
            0.0
        );
        assert!(result.is_some());
        let val = result.unwrap();
        assert!((val - PI/6.0).abs() < 1e-4);
    }
}

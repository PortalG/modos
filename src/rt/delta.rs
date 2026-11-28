use uefi::boot;
use crate::rt;

/// Returns the current CPU cycle count.
#[inline(always)]
pub fn cycle_count() -> u64 {
    no_std_time::read_time_counter()
}

/// Calculates and returns the amount of CPU cycles in a second.
pub fn calculate_cps() -> u64 {
    let init = cycle_count();
    boot::stall(rt::SECOND);
    cycle_count() - init
}

/// Calculates and returns the delta of time in seconds since the last CPU cycle
/// with the given cycles per second constant, and updates the last CPU cycle
/// referenced to the current.
pub fn calculate_delta(last: &mut u64, cps: u64) -> f64 {
    let current = cycle_count();
    let delta = (current - *last) as f64 / cps as f64;
    *last = current;
    delta
}

/// Calculates and returns the current rate per second from the given delta of
/// time.
pub fn rate_from_delta(delta: f64) -> usize {
    (1.0 / delta) as usize
}

/// Performs [`boot::stall`] at the given rate per second while tracking entropy
/// developed using the given delta of time.
pub fn stall(rate: usize, entropy: &mut f64, delta: f64) {
    let target = 1.0 / rate as f64;
    *entropy = (*entropy + target - delta).clamp(-target, target);
    boot::stall(((target + *entropy) * rt::SECOND as f64) as usize);
}
#![no_std]
use tiny_die::Die;
#[test]
fn test_non_zero_roll() {
    let dee_six = Die::new(6);
    assert_ne!(dee_six.roll(), 0);
}
#[test]
fn test_less_equal_n_sides_roll() {
    let n_sides: u8 = 20;
    let dee_twenty = Die::new(n_sides);
    assert!(dee_twenty.roll() <= n_sides);
}
#[test]
fn test_default_dee_six() {
    let dee_six = Die::default();
    assert_eq!(dee_six.sides, 6);
}
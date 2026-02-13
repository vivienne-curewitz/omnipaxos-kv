mod simulated_clock;
use simulated_clock::ClockState;

fn main() {
    let mut clock = ClockState::new(
        500,  // uncertainty: +/- 500μs
        2,    // drift_rate: 2μs per second
        1000000, // frequency: 1000000 μs (1 second) reset interval
        0     // base_offset
    );

    println!("Current Simulated Time: {} μs", clock.get_time());
    println!("Current Uncertainty:    {}", clock.get_uncertainty());
    // Re-initialize using init()
    clock.init(100, 5, 2000, 10000);
    println!("New Simulated Time:     {} μs", clock.get_time());
    println!("Current Uncertainty:    {}", clock.get_uncertainty());
}
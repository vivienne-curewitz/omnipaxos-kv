use omnipaxos_kv::simulated_clock::ClockState;
use omnipaxos_kv::one_way_delay::OWDNode;


fn main() {
    let clock = ClockState::new(
        500,  // uncertainty: +/- 500μs
        2,    // drift_rate: 2μs per second
        1000000, // frequency: 1000000 μs (1 second) reset interval
        0     // base_offset
    );

    let owd = OWDNode::new(
        clock,
        30
    );

    println!("Expected Delay: {} μs", owd.get_delay());
    println!("Current Uncertainty:    {}", owd.get_uncertainty());
}
use omnipaxos_kv::simulated_clock::ClockState;
use std::vec::Vec;
use std::collections::HashMap;


pub struct OneWayDelay {
    sim_clock: *const ClockState,
    measured_latencies: HashMap<String, Vec<i64>>,
    reported_latencies: HashMap<String, i64>,
    fixed_delay: i64,
}

impl OneWayDelay {
    pub fn new(cs: &ClockState, fd: i64) -> OneWayDelay {
        return OneWayDelay{
            sim_clock: ClockState,
            measured_latencies: HashMap::new(),
            reported_latencies: HashMap::new(),
            fixed_delay: fd,
        }
    }

    // return the one way delay
    // for bonus task, this should be expanded to take a address/node as input
    // that would be the node who's OWD we are interested in
    pub fn get_delay(&self)  -> i64 {
        self.fixed_delay
    }

    pub fn get_uncertainty(&self) -> i64 {
        self.sim_clock.get_uncertainty()
    }

    // bonus task - calculate adaptive one way delay
    // pub fn requestOWD() -- call this function to request the OWD from another OWD on the network
    // pub fn reportOWD() -- call this to determine OWD for a given requester and report the information to them

}
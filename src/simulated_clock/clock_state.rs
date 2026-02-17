use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

pub struct ClockState {
    uncertainty: i64,  // μS
    drift_rate: i64,   // μS / S
    frequency: i64,    // μS
    base_offset: i64,  
}

impl ClockState {
    // Constructor
    pub fn new(uncertainty: i64, drift_rate: i64, frequency: i64, base_offset: i64) -> Self {
        Self {
            uncertainty,
            drift_rate,
            frequency,
            base_offset,
        }
    }

    // Init function to update/reset values on an existing struct
    pub fn init(&mut self, uncertainty: i64, drift_rate: i64, frequency: i64, base_offset: i64) {
        self.uncertainty = uncertainty;
        self.drift_rate = drift_rate;
        self.frequency = frequency;
        self.base_offset = base_offset;
    }

    // Returns just the uncertainty
    pub fn get_uncertainty(&self) -> i64 {
        self.uncertainty
    }

    // Calculates the simulated time
    pub fn get_time(&self) -> i64 {
        // 1. Get current system time in microseconds
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        
        // We use i64 to allow for negative offsets in calculation
        let now_micros = since_the_epoch.as_micros() as i64;

        // Safety check to avoid divide by zero if frequency is initialized to 0
        let modulo_val: i64 = if self.frequency > 0 {
            now_micros % self.frequency
        } else {
            0
        };

        // 3. Calculate Random Jitter
        // Generates a value in range [-uncertainty, uncertainty] inclusive
        let mut rng = rand::thread_rng();
        let jitter = rng.gen_range(-self.uncertainty..=self.uncertainty);

        // 4. Final Calculation
        // Formula: now + ((now % freq) * drift) + offset + random
        now_micros 
            + (modulo_val * self.drift_rate as i64) 
            + (self.base_offset as i64) 
            + (jitter as i64)
    }
}   
use std::time::{Duration, Instant};
use std::vec::Vec;

struct Timer {
    curr_start: f64,
    samples: Vec<f64>
}

pub fn timer() -> Timer {
    Timer{samples: Vec<f64>::new()}
}

impl Timer {
    pub fn start(&mut self) {
        self.curr_start = Instant::now();
    }

    pub fn stop(&mut self) {
        self.samples.push(Instant::now() - self.curr_start);
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }

    pub fn print_stats(&self) {
        println!("FPS Stats");
        println!("\tFrames: {}", self.samples.len());
        println!("\tMean: {}s", self.samples.iter().sum() / self.samples.len());
    }
}

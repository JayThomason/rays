use std::time::{Duration, Instant};
use std::vec::Vec;

pub struct Timer {
    curr_start: Instant,
    samples: Vec<Duration>
}

pub fn timer() -> Timer {
    Timer{
        curr_start: Instant::now(),
        samples: Vec::new(),
    }
}
        

impl Timer {
    pub fn start(&mut self) {
        self.curr_start = Instant::now();
    }

    pub fn stop(&mut self) {
        self.samples.push(Instant::now() - self.curr_start);
    }

    pub fn print_stats(&self) {
        println!("FPS Stats");
        println!("\tFrames: {}", self.samples.len());
        let mean = self.samples.iter().sum::<Duration>().as_millis() / (self.samples.len() as u128);
        println!("\tMean: {} ms", mean);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

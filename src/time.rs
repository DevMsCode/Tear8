use std::time::Instant;
use chrono::prelude::*;

// Definisci una struttura per gestire il logging
#[derive(Clone, Copy)]
pub struct Time {
    program_start_time: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            program_start_time: Instant::now(),
        }
    }

    // Calcola il tempo del luogo locale
    pub fn local_time(&mut self) -> String {
        let local_time = Local::now();
        let time = format!("{}:{}:{}:{}", local_time.hour(), local_time.minute(), local_time.second(), local_time.nanosecond());
        return format!("{}; {}", local_time.date_naive(), time);
    }

    // Calcola il tempo trascorso dall'inizio del programma
    pub fn program_time(&self) -> String {
        let elapsed = self.program_start_time.elapsed();
        format!("{:02}:{:02}:{:02}", elapsed.as_secs() / 3600, (elapsed.as_secs() % 3600) / 60, elapsed.as_secs() % 60)
    }   

}
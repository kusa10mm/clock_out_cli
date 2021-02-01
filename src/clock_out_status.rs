use crate::status_enum::*;
use chrono::prelude::*;

pub struct ClockOutStatus {
    pub laundry_amount: f32,
    pub stress: Stress,
    pub time: DateTime<Local>,
    pub hunger: Hunger,
}

impl ClockOutStatus {
    pub fn new(args: &Vec<String>) -> ClockOutStatus {
        let laundry_amount_string = args.get(1);
        let laundry_amount: f32 = match laundry_amount_string {
            Some(string) => string.parse().unwrap(),
            None => panic!("Failed to load first args"),
        };

        let stress_string = args.get(2);
        let stress_string = match stress_string {
            Some(some_string) => some_string,
            None => panic!("Failed to load second args"),
        };        
        let stress = match &stress_string[..] {
            "high" => Stress::High,
            _ => Stress::Low,
        };

        let hunger = args.get(3);
        let hunger = match hunger {
            Some(some_string) => some_string,
            None => panic!("Failed to load args"),
        };
        let hunger = match &hunger[..] {
            "very" => Hunger::Very,
            _ => Hunger::ALittle,
        };

        ClockOutStatus {
            laundry_amount,
            stress,
            time: Local::now(),
            hunger,
        }
    }
}
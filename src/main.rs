use std::env;
use chrono::prelude::*;

fn main() {
    // 引数を受け取る
    let args: Vec<String> = env::args().collect();

    // 退勤時の状態の構造体を作る
    let clock_out_status = ClockOutStatus::new(&args);

    // 構造体を渡して結果を計算
    let actions = calc_actions(clock_out_status);

    // 結果を画面出力
    println!("{}", actions[0]);
}

struct ClockOutStatus {
    laundry_amount: f32,
    stress: Stress,
    time: DateTime<Local>,
    hunger: Hunger,
}

#[derive(Debug, PartialEq)]
enum Stress {
    High,
    Low,
}

#[derive(Debug)]
enum Hunger {
    Very,
    ALittle,
}

#[derive(Debug)]
enum Dinner {
    Convinience,
    NabeSet,
    Rice,
}

impl ClockOutStatus {
    fn new(args: &Vec<String>) -> ClockOutStatus {
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

fn calc_actions(status: ClockOutStatus) -> Vec<String> {
    let dinner_options = calc_dinner(status.stress, status.time);
    let do_laundry = calc_do_laundry(status.laundry_amount);
    let actions = calc_order_of_actions(dinner_options, do_laundry, status.hunger);
    actions
}

fn calc_order_of_actions(dinner_options: Dinner, do_laundry: bool, hunger: Hunger) -> Vec<String> {
    Vec::new()
}

fn calc_do_laundry(laundry_amount: f32) -> bool {
    if laundry_amount < 1.5 {
        true
    } else {
        false
    }
}

fn calc_dinner(stress: Stress, time: DateTime<Local>) -> Dinner {
    println!("{}", time.hour());
    let dinner = match stress {
        Stress::High => match time.hour() {
            0..=19 => Dinner::Rice,
            _ => Dinner::NabeSet,
        },
        Stress::Low => match time.hour() {
            0..=19 => Dinner::Rice,
            _ => Dinner::Convinience
        }
    };
    println!("{:?}", dinner);
    dinner
}


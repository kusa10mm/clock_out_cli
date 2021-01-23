use std::env;
use chrono::prelude::*;

fn main() {
    // 引数を受け取る
    let args: Vec<String> = env::args().collect();

    // 退勤時の状態の構造体を作る
    let clock_out_status = ClockOutStatus::new(&args);

    // 構造体を渡して結果を計算
    let actions = calc_actions(&clock_out_status);

    // 結果を画面出力
    println!("{}", actions[0]);
}

struct ClockOutStatus {
    laundry: String,
    stress: Stress,
    time: DateTime<Local>,
    hunger: Hunger,
}

#[derive(Debug)]
enum Stress {
    High,
    Low,
}

#[derive(Debug)]
enum Hunger {
    Very,
    ALittle,
}

impl ClockOutStatus {
    fn new(args: &Vec<String>) -> ClockOutStatus {
        let laundry = args.get(1);
        let laundry: String = match laundry {
            Some(some_string) => some_string.clone(),
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
            laundry,
            stress,
            time: Local::now(),
            hunger,
        }
    }
}

fn calc_actions(status: &ClockOutStatus) -> Vec<String> {
    let mut actions = Vec::<String>::new();

    println!("{:?}", status.laundry);
    println!("{:?}", status.stress);
    println!("{:?}", status.time);
    println!("{:?}", status.hunger);

    actions.push(String::from("wash clothes"));
    actions
}

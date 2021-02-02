mod clock_out_status;
mod status_enum;

use std::env;
use chrono::prelude::*;
use status_enum::*;
use clock_out_status::*;

fn main() {
    // 引数を受け取る
    let args: Vec<String> = env::args().collect();

    // 退勤時の状態の構造体を作る
    let clock_out_status = ClockOutStatus::new(&args);

    // 構造体を渡して結果を計算
    let actions = calc_actions(clock_out_status);

    // 結果を画面出力
    for (i, action) in actions.iter().enumerate() {
        println!("{}: {}", i + 1, action);
    };
}



fn calc_actions(status: ClockOutStatus) -> Vec<String> {
    let dinner_options = calc_dinner(status.stress, status.time);
    let do_laundry = calc_do_laundry(status.laundry_amount);
    let actions = calc_order_of_actions(dinner_options, do_laundry, status.hunger);
    actions
}

fn calc_order_of_actions(dinner_options: Dinner, do_laundry: bool, hunger: Hunger) -> Vec<String> {
    let mut actions = Vec::<String>::new();
    if do_laundry {
        actions.push(String::from("Do Laundry"));
    };
    if hunger == Hunger::ALittle {
        actions.push(String::from("Training"));
        actions.push(String::from("Dinner"));
    } else {
        actions.push(String::from("Dinner"));
        actions.push(String::from("Training"));
    }
    actions
}

fn calc_do_laundry(laundry_amount: f32) -> bool {
    if laundry_amount < 1.5 {
        false
    } else {
        true
    }
}

fn calc_dinner(stress: Stress, time: DateTime<Local>) -> Dinner {
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
    println!("Dinner Options: {:?}", dinner);
    dinner
}


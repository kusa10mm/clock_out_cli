mod clock_out_status;
mod status_enum;
mod calc_actions;

use std::env;
use clock_out_status::*;
use calc_actions::*;

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



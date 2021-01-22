use std::env;

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
}

impl ClockOutStatus {
    fn new(args: &Vec<String>) -> ClockOutStatus {
        let laundry = args.get(1);
        let laundry: String = match laundry {
            Some(some_string) => some_string.clone(),
            None => panic!("Failed to load first args"),
        };
        ClockOutStatus {
            laundry
        }
    }
}

fn calc_actions(status: &ClockOutStatus) -> Vec<String> {
    let mut actions = Vec::<String>::new();
    actions.push(String::from("wash clothes"));
    actions
}

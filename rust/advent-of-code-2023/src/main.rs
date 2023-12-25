use solutions::{one, two};

mod solutions;

fn main() {
    // exec_one();
    exec_two();
}

#[allow(dead_code)]
fn exec_one() {
    let input_values = one::read_value_from_files();

    let calibration_sum = one::calculate_calibration_sum(&input_values);
    let calibration_text_sum = one::calculate_calibration_sum_text(&input_values);

    println!("Calibration sum of all lines: {}", calibration_sum);
    println!(
        "Calibration text sum of all lines: {}",
        calibration_text_sum
    );
}

fn exec_two() {
    let games = two::read_games_from_file();

    let possible_game_sum = two::count_possible_game(&games);
    let minimum_power_sum = two::calculate_minimum_possible_game(&games);

    println!("Index sum of all possible games is {}", possible_game_sum);
    println!("Sum of power of all games {}", minimum_power_sum);
}

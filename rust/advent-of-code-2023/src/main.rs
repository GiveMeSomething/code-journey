use solutions::{four, one, seven, six, three, two};

mod solutions;

fn main() {
    // exec_one();
    // exec_two();
    // exec_three();
    // exec_four();

    // exec_six();
    exec_seven();
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

#[allow(dead_code)]
fn exec_two() {
    let games = two::read_games_from_file();

    let possible_game_sum = two::count_possible_game(&games);
    let minimum_power_sum = two::calculate_minimum_possible_game(&games);

    println!("Index sum of all possible games is {}", possible_game_sum);
    println!("Sum of power of all games {}", minimum_power_sum);
}

#[allow(dead_code)]
fn exec_three() {
    let inputs = three::read_part_from_file();

    let part_sum = three::cal_engine_parts_sum(&inputs);
    let gear_ratio_sum = three::cal_gear_ratio(&inputs);

    println!("Sum of all available parts is {}", part_sum);
    println!("Sum of all gear ratios is {}", gear_ratio_sum);
}

#[allow(dead_code)]
fn exec_four() {
    let cards = four::read_card_from_file();

    let cards_point_sum = four::calculate_cards_point(&cards);
    let card_count = four::count_duplicate_cards(&cards);

    println!("The point sum of all cards: {}", cards_point_sum);
    println!("Total card after scratching is: {}", card_count);
}

#[allow(dead_code)]
fn exec_six() {
    let races = six::read_races_from_file();

    let win_ways = six::count_win_ways(&races);
    let win_way_single = six::count_win_ways_single(&races);

    println!("Winning way count: {}", win_ways);
    println!("Winning way of a single race is {}", win_way_single);
}

fn exec_seven() {
    seven::read_hands_from_file();
}

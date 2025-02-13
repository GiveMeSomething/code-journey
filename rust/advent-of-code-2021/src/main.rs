use solutions::{
    eight,
    eleven::{read_octopus_from_file, simulate_flash, sync_flash},
    five, four, nine, one, seven, six, ten, three, two,
};

mod solutions;

fn main() {
    // exec_one();
    // exec_two();
    // exec_three();
    // exec_four();
    // exec_five();
    // exec_six();
    // exec_seven();
    // exec_eight();
    // exec_nine();
    // exec_ten();
    exec_eleven();
}

#[allow(dead_code)]
fn exec_one() {
    let sweeps = one::read_sweep_from_file();

    let increase_sweep_count = one::count_increase_sweep(&sweeps);
    println!("The number of increase sweep is {}", increase_sweep_count);

    let increase_sweep_window_count = one::count_increase_sweep_window(&sweeps);
    println!(
        "The number of increase sweep window is {}",
        increase_sweep_window_count
    );
}

#[allow(dead_code)]
fn exec_two() {
    let commands = two::read_commands_from_file();

    let (horizontal, depth) = two::simulate_commands(&commands);
    println!(
        "Multiple of horizontal position and depth is {}",
        horizontal * depth
    );

    let (final_horizontal, final_depth) = two::simulate_commands_with_aim(&commands);
    println!(
        "With aim: Multiple of horizontal position and depth is {}",
        final_horizontal * final_depth
    );
}

#[allow(dead_code)]
fn exec_three() {
    let bits = three::read_bits_from_file();

    let power_consumption = three::process_bits(&bits);
    println!("The power consumption is {}", power_consumption);

    let oxygen_rating = three::find_oxygen_rating(&bits);
    let co2_rating = three::find_co2_rating(&bits);
    println!("{}", oxygen_rating * co2_rating);
}

#[allow(dead_code)]
fn exec_four() {
    let (numbers, mut bingos) = four::read_bingo_from_file();

    let (step, point) = four::calculate_fastest_win(&numbers, &mut bingos);
    println!("Fastest win with {} step and {} points", step, point);

    let (step, point) = four::calculate_slowest_win(&numbers, &mut bingos);
    println!("Slowest win with {} step and {} points", step, point);
}

#[allow(dead_code)]
fn exec_five() {
    let vent_lines = five::read_vents_from_file();
    let intersections = five::count_intersection(&vent_lines);
    println!("Number of line intersections: {}", intersections);

    let intersections_with_diagonal = five::count_intersection_with_diagonal(&vent_lines);
    println!(
        "Number of line intersections, include diagonal line: {}",
        intersections_with_diagonal
    );
}

#[allow(dead_code)]
fn exec_six() {
    let remaining_days = 256;
    let intervals = six::read_interval_from_file();

    let lanternfish_count = six::count_lanternfish_optimize(&intervals, remaining_days);

    println!("#fish after {} days: {}", remaining_days, lanternfish_count);
}

#[allow(dead_code)]
fn exec_seven() {
    let positions = seven::read_position_from_file();

    let min_medium = seven::min_horizontal_medium(&positions);
    println!("{}", min_medium);

    let min_increment_medium = seven::min_horizontal_medium_increment(&positions);
    println!("{}", min_increment_medium);
}

#[allow(dead_code)]
fn exec_eight() {
    let signals = eight::read_signal_from_file();

    let unique_signal_count = eight::count_unique_signals(&signals);
    println!("Number of unique signal {}", unique_signal_count);

    let decode_signal_sum = eight::sum_decode_signal(&signals);
    println!("Decode sum {}", decode_signal_sum);
}

#[allow(dead_code)]
fn exec_nine() {
    let locations = nine::read_location_from_file();

    let risk_sum = nine::sum_risk_level(&locations);
    println!("Risk index {}", risk_sum);

    let mul_basin = nine::mul_top_three_basins(&locations);
    println!("Multiple of top 3 basins {}", mul_basin);
}

#[allow(dead_code)]
fn exec_ten() {
    let lines = ten::read_code_from_file();

    let illegal_point = ten::sum_illegal_point(&lines);
    println!("Illegal point: {}", illegal_point);

    let autocomplete_point = ten::sum_autocomplete_point(&lines);
    println!("Autocomplete point: {}", autocomplete_point);
}

#[allow(dead_code)]
fn exec_eleven() {
    let octopus_matrix = read_octopus_from_file();

    // Clone to avoid mutating each other input
    let flash_count = simulate_flash(&mut octopus_matrix.clone(), 100);
    println!("Flash count after 100 steps: {:?}", flash_count);

    // Clone to avoid mutating each other input
    let sync_step = sync_flash(&mut octopus_matrix.clone());
    println!("Octopus sync at {}", sync_step);
}

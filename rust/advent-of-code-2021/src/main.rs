use solutions::{four, one, three, two};

mod solutions;

fn main() {
    // exec_one();
    // exec_two();
    // exec_three();
    exec_four();
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

fn exec_four() {
    let (numbers, mut bingos) = four::read_bingo_from_file();

    let (step, point) = four::calculate_fastest_win(&numbers, &mut bingos);
    println!("Fastest win with {} step and {} points", step, point);

    let (step, point) = four::calculate_slowest_win(&numbers, &mut bingos);
    println!("Slowest win with {} step and {} points", step, point);
}

use solutions::{one, two};

mod solutions;

fn main() {
    // exec_one();
    exec_two();
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
}

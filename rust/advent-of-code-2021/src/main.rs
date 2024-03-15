use solutions::one;

mod solutions;

fn main() {
    exec_one();
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

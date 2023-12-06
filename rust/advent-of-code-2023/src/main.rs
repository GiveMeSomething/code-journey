use solutions::one;

mod solutions;

fn main() {
    exec_one();
}

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

mod example;
mod one;
mod two;

fn main() {
    // example::guess::start_guessing_game();
    // example::conversion::exec_conversion();

    // let matrix = [
    //     [1, 2, 3], // <-- the comment makes rustfmt add a newline
    //     [4, 5, 6],
    //     [7, 8, 9],
    // ];

    // println!("matrix:");
    // example::looping::pretty_print(&matrix);

    // let transposed = example::looping::transpose(matrix);
    // println!("transposed:");
    // example::looping::pretty_print(&transposed);

    exec_one();
    exec_two();
}

fn exec_one() {
    let max_calorie = one::max_calorie();
    let top_three = one::top_three_calorie();

    println!("The max calories is {}", max_calorie);
    println!(
        "The top three calories are {:?}, sum: {}",
        top_three,
        top_three.0 + top_three.1 + top_three.2
    );
    println!();
}

fn exec_two() {
    let score_part_1 = two::calculate_score_part_1();
    let score_part_2 = two::calculate_score_part_2();

    println!(
        "The score after playing with your strategy is {}",
        score_part_1
    );
    println!(
        "The score after playing with real strategy is {}",
        score_part_2
    );
    println!();
}

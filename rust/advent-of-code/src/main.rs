mod example;
mod five;
mod four;
mod one;
mod three;
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

    // exec_one();
    // exec_two();
    // exec_three();
    // exec_four();
    exec_five();
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn exec_three() {
    let single_priority_sum = three::calculate_rucksack_priority();
    let group_priority_sum = three::calculate_rucksack_group_priority();

    println!("Total single priority sum is {}", single_priority_sum);
    println!("Total group priority sum is {}", group_priority_sum);
    println!();
}

#[allow(dead_code)]
fn exec_four() {
    let overlapping_pair = four::count_overlap_pair();
    let partial_overlapping_pair = four::count_partial_overlap_pair();

    println!("The number of overlapping pair is {}", overlapping_pair);
    println!(
        "The number of partially overlapping pair is {}",
        partial_overlapping_pair
    );
    println!();
}

#[allow(dead_code)]
fn exec_five() {
    let message_part_1 = five::peek_top_crates();

    println!("Part 1 message is {}", message_part_1);
}

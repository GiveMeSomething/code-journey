use std::time::Instant;

mod eight;
mod example;
mod five;
mod four;
mod one;
mod seven;
mod six;
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
    // exec_five();
    // exec_six();
    // exec_seven();

    exec_eight();
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
    let (cargos, instructions) = five::read_cargo_from_file();

    let message_part_1 = five::peek_top_crates(&mut five::clone_vec(&cargos), &instructions);
    let message_part_2 = five::peek_top_crates_9001(&mut five::clone_vec(&cargos), &instructions);

    println!("Part 1 message is {}", message_part_1);
    println!("Part 2 message is {}", message_part_2);
}

#[allow(dead_code)]
fn exec_six() {
    let signal = six::read_signal_from_file();

    with_benchmark(&|| {
        println!("Using HashSet");
        let packet_start = six::get_start_of_packet_set(&signal);
        let message_start = six::get_start_of_message_set(&signal);

        println!("Start-of-packet at {}", packet_start);
        println!("Start-of-message at {}", message_start);
    });

    with_benchmark(&|| {
        println!("Using HashMap");
        let packet_start = six::find_unique_seq(&signal, 4);
        let message_start = six::find_unique_seq(&signal, 14);

        println!("Start-of-packet at {}", packet_start);
        println!("Start-of-message at {}", message_start);
    })
}

#[allow(dead_code)]
fn exec_seven() {
    let directory_map = seven::generate_directory_map();

    let small_files_size = seven::sum_small_files(&directory_map);
    let deletable_size = seven::deletable_size(&directory_map);

    println!(
        "Total file size of all files that at most 100_000 is {}",
        small_files_size
    );

    println!(
        "Need to delete {} more before system update",
        deletable_size
    );
}

fn exec_eight() {
    let forest = eight::read_forest_from_file();

    println!("{:?}", forest);
}

fn with_benchmark(f: &dyn Fn() -> ()) {
    let start_time = Instant::now();

    f();

    let elapsed_time = Instant::now() - start_time;

    println!("Running time {:?}", elapsed_time);
    println!();
}

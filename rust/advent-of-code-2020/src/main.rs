use solutions::{
    one, three,
    two::{self, count_valid_corp_password, count_valid_password},
};

mod solutions;

fn main() {
    // exec_one();
    // exec_two();
    exec_three();
}

#[allow(dead_code)]
fn exec_one() {
    let expenses = one::read_expense_from_file();

    let point1 = one::find_entries_by_sum(&expenses, 2020);
    println!("Point day 1 part 1: {}", point1);

    let point2 = one::find_triplet_by_sum(&expenses, 2020);
    println!("Point day 1 part 2: {}", point2);
}

#[allow(dead_code)]
fn exec_two() {
    let inputs = two::read_password_from_file();

    let valid_password_count = count_valid_password(&inputs);
    println!("# valid password: {}", valid_password_count);

    let valid_corp_password_count = count_valid_corp_password(&inputs);
    println!("# valid corp password: {}", valid_corp_password_count);
}

#[allow(dead_code)]
fn exec_three() {
    let inputs = three::read_tree_from_file();

    let tree_count = three::count_slope_tree(&inputs, 3, 1);
    println!("Tree count: {}", tree_count);

    let slope1_tree = three::count_slope_tree(&inputs, 1, 1);
    let slope2_tree = three::count_slope_tree(&inputs, 3, 1);
    let slope3_tree = three::count_slope_tree(&inputs, 5, 1);
    let slope4_tree = three::count_slope_tree(&inputs, 7, 1);
    let slope5_tree = three::count_slope_tree(&inputs, 1, 2);
    println!(
        "Tree count multiple slope: {}",
        slope1_tree * slope2_tree * slope3_tree * slope4_tree * slope5_tree
    );
}

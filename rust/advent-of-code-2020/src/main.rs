use solutions::{
    one,
    two::{self, count_valid_corp_password, count_valid_password},
};

mod solutions;

fn main() {
    // exec_one();
    exec_two();
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

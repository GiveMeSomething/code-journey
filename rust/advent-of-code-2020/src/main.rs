use solutions::one;

mod solutions;

fn main() {
    exec_one();
}

#[allow(dead_code)]
fn exec_one() {
    let expenses = one::read_expense_from_file();

    let point1 = one::find_entries_by_sum(&expenses, 2020);
    println!("Point day 1 part 1: {}", point1);

    let point2 = one::find_triplet_by_sum(&expenses, 2020);
    println!("Point day 1 part 2: {}", point2);
}

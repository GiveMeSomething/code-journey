use solutions::one::read_expense_from_file;

mod solutions;

fn main() {
    exec_one();
}

#[allow(dead_code)]
fn exec_one() {
    let expenses = read_expense_from_file();

    println!("{:?}", expenses);
}

#[allow(dead_code)]
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

#[allow(dead_code)]
pub fn exec_conversion() {
    let x: i8 = 15;
    let y: i32 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
    println!("{x} * {y} = {}", multiply(i32::from(x), y));
}

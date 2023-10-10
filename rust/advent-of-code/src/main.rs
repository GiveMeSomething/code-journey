mod example;

fn main() {
    // example::guess::start_guessing_game();
    // example::conversion::exec_conversion();

    let matrix = [
        [1, 2, 3], // <-- the comment makes rustfmt add a newline
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("matrix:");
    example::looping::pretty_print(&matrix);

    let transposed = example::looping::transpose(matrix);
    println!("transposed:");
    example::looping::pretty_print(&transposed);
}

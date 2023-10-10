pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
        }
    }

    return result;
}

pub fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{} {} {}\n", row[0], row[1], row[2]);
    }
}

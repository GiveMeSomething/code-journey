#[allow(dead_code)]
pub fn my_sqrt(x: i32) -> i32 {
    recursive_my_sqrt(x, 0, x as usize)
}

pub fn recursive_my_sqrt(x: i32, start: usize, end: usize) -> i32 {
    println!("start = {}, end = {}, value = {}", start, end, end - start);
    // Resolve 2 elements case
    if end - start == 1 {
        if end * end <= x as usize {
            return end as i32;
        }
        return start as i32;
    }

    let half = (start + end) / 2;
    let product = half * half;

    let target: usize = x as usize;
    if product == target {
        return half as i32;
    }

    if target > product {
        return recursive_my_sqrt(x, half.into(), end);
    }

    return recursive_my_sqrt(x, start, half.into());
}

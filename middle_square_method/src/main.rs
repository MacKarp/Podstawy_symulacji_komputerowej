fn main() {
    let m = 2;
    let x0 = 12;

    let result = middle_square_method(m, x0);
    println!("m = {}, x0 = {}", m, x0);
    println!("Result: {}", result);
}

fn middle_square_method(m: i32, x: i32) -> i32 {
    let range = vec![
        1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000,
    ];

    let middle_digit = 2;
    let mut sqn = x * x;
    let mut next_num = 0;

    let trim = middle_digit / 2;
    sqn /= range[trim];

    for i in 0..middle_digit {
        next_num += (sqn % (range[trim])) * (range[i]);
        sqn /= 10;
    }

    let m = m - 1;
    if m == 0 {
        return next_num;
    }

    middle_square_method(m, next_num)
}

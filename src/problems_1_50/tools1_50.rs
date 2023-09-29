pub fn fibonacci(arg: i32) -> i32 {
    if arg == 0 {
        return 0;
    }
    if arg == 1 {
        return 1;
    }
    fibonacci(arg - 1) + fibonacci(arg - 2)
}

pub fn prime(arg: u64) -> bool {
    for i in 2..arg / 2 {
        if arg % i == 0 {
            return false;
        }
    }
    true
}

pub fn is_palindrome(mut arg: i32) -> bool {
    let mut num_vec: Vec<i32> = Vec::new();

    while arg != 0 {
        num_vec.push(arg % 10);
        arg /= 10;
    }
    let vec_length = num_vec.len();
    for i in 0..vec_length {
        if num_vec[i] != num_vec[vec_length - i - 1] {
            return false;
        }
    }
    true
}
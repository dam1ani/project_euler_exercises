pub const MAX: i32 = i32::MAX;

pub mod tools1_50;

// Multiples of 3 or 5
// Find the sum of all the multiples 3 or 5 below 1000
pub fn problem_1() {
    let mut sum_of_multiples = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum_of_multiples += i;
        }
    }
    println!("{}", sum_of_multiples);
}

// Even Fibonacci Numbers
// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
// find the sum of the even-valued terms
pub fn problem_2() {
    let mut sum_of_multiples = 0;

    for i in 1..34 {
        let fib_i = tools1_50::fibonacci(i);
        if fib_i % 2 == 0 {
            sum_of_multiples += fib_i;
        }
    }
    println!("{}", sum_of_multiples);
}

// Largest Prime Factor
// What is the largest prime factor of the number 600851475143
pub fn problem_3() {
    let number: u64 = 600851475143;
    let mut largest_prime = 0;
    for i in 2..number / 2 {
        //if n is composite then n has a prime factor p*p <= n
        if i * i > number {
            break;
        }

        if number % i == 0 && tools1_50::prime(i) {
            largest_prime = i;
        }
    }
    println!("{}", largest_prime)
}

//Largest Palindrome Product
// Find the largest palindrome made from the product of two 3-digit numbers
pub fn problem_4() {
    let mut pal_vec: Vec<i32> = Vec::new();
    for i in 100..=999 {
        for j in 100..=999 {
            let mul = i * j;
            if tools1_50::is_palindrome(mul) {
                pal_vec.push(mul);
            }
        }
    }
    println!("{}", pal_vec.iter().max().unwrap());
}

//Smallest Multiple
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20
pub fn problem_5() {
    let mut divisors: Vec<i32> = Vec::new();
    for i in 2..=20 {
        divisors.push(i);
    }
    for i in 40..MAX {
        let mut bool = true;
        for j in &divisors {
            if i % j != 0 {
                bool = false;
                break;
            }
        }
        if bool {
            println!("{}", i);
            break;
        }
    }
}

//Sum Square Difference
//Find the difference between the sum of the squares of the first one hundred natural numbers and
// the square of the sum
pub fn problem_6() {
    let n = 100;
    let sum_of_squares = (n * (n + 1) * (2 * n + 1)) / 6;
    let square_of_sum = (n * (n + 1)) / 2;

    println!("{}", (square_of_sum * square_of_sum) - sum_of_squares);
}

//10001st Prime
//What is the 10001st prime number
pub fn problem_7() {
    let mut n_of_primes = 0;
    let mut i = 2;
    while n_of_primes < 10002 {
        if tools1_50::prime(i) {
            n_of_primes += 1;
        }
        i += 1;
    }
    println!("{}", i - 1);
}

//Largest Product in Series
// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product
pub fn problem_8() {
    let the_number = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let length_of_n = the_number.len();
    let mut index = 0;
    let mut mul_result = 1;
    while index + 13 != length_of_n {
        let mut temp_result: u128 = 1;

        for j in 0..13 {
            temp_result *= the_number.chars().nth(j + index).unwrap().to_digit(10).unwrap() as u128;
        }
        if mul_result < temp_result {
            mul_result = temp_result;
        }
        index += 1;
    }
    println!("{}", mul_result);
}
use std::io;

fn main() {
    println!("Enter a positive integer:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: u64 = input
        .trim()
        .parse()
        // .unwrap_or(10);
        .expect("Please enter a valid positive integer");

    let factors = prime_factors(num);

    println!(
        "Prime factors of {}: {:?}",
        num,
        format_factorized(&factors)
    );
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 1;
    }

    factors
}

fn format_factorized(factors: &[u64]) -> String {
    let mut result = String::new();
    let mut current_factor = factors[0];
    let mut count = 1;

    for &factor in &factors[1..] {
        if factor == current_factor {
            count += 1;
        } else {
            if count == 1 {
                result += &format!("{} * ", current_factor);
            } else {
                result += &format!("{}^{} * ", current_factor, count);
            }

            current_factor = factor;
            count = 1;
        }
    }

    if count == 1 {
        result += &format!("{}", current_factor);
    } else {
        result += &format!("{}^{}", current_factor, count);
    }

    result
}

/*
Problem 3:
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/
fn main() {
    let the_big_boi: i64 = 600851475143;

    let primes = find_primes_v1(the_big_boi);
    println!("{} is the largest prime factor", primes.last().unwrap());
    
    let primes2 = find_primes_v2(the_big_boi);
    println!("{} is the largest prime factor", primes2.last().unwrap());
}

fn find_primes_v1(number: i64) -> Vec<i64> {
    let mut primes = Vec::new();

    for num in 2..(number as f64).sqrt() as i64 {
        if number % num == 0 {
            if primes.is_empty() {
                primes.push(num);
                continue;
            }
            let mut is_prime = true;
            for prime in &primes {
                if num % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.push(num);
            }
        }
    }

    return primes;
}

fn find_primes_v2(number: i64) -> Vec<i64> {
    let mut primes = Vec::new();
    let mut upper = number;
    let mut curr = 2;
    let mut is_factor = false;

    while (curr < upper) && (curr < (number as f64).sqrt() as i64) {
        if upper % curr == 0 {
            upper = upper / curr;
            is_factor = true;
            continue;
        }
        if is_factor {
            primes.push( curr );
        }
        curr += 1;
    }
    primes.push( upper );

    return primes;
}

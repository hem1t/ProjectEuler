fn main() {
    let mut number: f64 = 600851475143.0;
    let mut factors: Vec<u32> = Vec::new();
    let mut i: u32 = 3;
    loop {
        if is_prime(i) {
            let temp = number / i as f64;
            if temp == {temp as u64} as f64 {
                let largest_prime_factor = i;
                factors.push(i);
                println!("{} * {} = {}", i, number, temp);
                if temp == 1.0 { println!("largest_prime_factor: {}", largest_prime_factor); break; }
                number = temp;
            }
        }
        i += 2;
    }
}

fn is_prime(number: u32) -> bool {
    if number == 0 || number == 1 {
        return false;
    }
    if number == 3 || number == 5 || number == 7 || number == 2 {
        return true;
    }
    for n in 3..10 {
        if number % n == 0 {
            return false;
        } else {
            return true;
        }
    }
    false
}

fn main() {
    let mut result = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            result += n;
        } 
    }
    println!("{}", result);
}
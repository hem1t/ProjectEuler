fn main() {
    let (mut f0,mut f1, mut result) :(u32, u32, u32) = (0, 1, 0);
    loop {
        let temp = f0;
        f0 = f1;
        f1 = temp + f1;
        if f1 % 2 == 0 {result += f1;} 
        if f1 >= 4000000 {break;}
    }
    println!("{}", result);
}
use std::io;

pub fn run() {
    let mut input1 = String::new();
    println!("Enter first num: ");
    io::stdin().read_line(&mut input1).unwrap();
    let n: i32 = input1.trim().parse().unwrap();


    let mut input2 = String::new();
    println!("Enter second num: ");
    io::stdin().read_line(&mut input2).unwrap();
    let m: i32 = input2.trim().parse().unwrap();

    println!("Here are your results: ");
    let plus = n + m;
    println!("{}", plus);
    let minus = n - m;
    println!("{}", minus);
    let mult = n * m;
    println!("{}", mult);
    if m != 0 {
        let div = n / m;
        println!("{}", div);
    } else {
        println!("You cannot divide by zero!!");
    }

}


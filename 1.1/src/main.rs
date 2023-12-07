fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a path");
        std::process::exit(1);
    }

    let path = &args[1];
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut sum = 0;
    for line in contents.lines() {
        let first = line.find(char::is_numeric).unwrap();
        let last = line.rfind(char::is_numeric).unwrap();

        let first = line.chars().nth(first).unwrap();
        let last = line.chars().nth(last).unwrap();
        let num = String::from(first) + &String::from(last);
        let num: i32 = num.parse().unwrap();
        sum += num;
    }

    println!("Sum: {}", sum);
}

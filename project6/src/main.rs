use std::io;
use std::io::Write;

fn main() {
    let mut year = String::new();
    print!("Please input a year to check if it is a leap year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year!", year);
    } else {
        println!("{} is not a leap year!", year);
    }

    let string = Some("This is a very long string");
    let message = match string {
        Some(s) if s.len() >= 10 => "Long string",
        Some(_) => "String",
        None => "Nothing",
    };
    println!("{}", message);

    let mut one = 1;
    let plus_one = move |x| {
        x + one
    };
    one += 1;
    println!("10 + 1 = {}", plus_one(10));
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}
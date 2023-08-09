use std::io::{self, Write};
use num::integer::gcd;
use std::env;

fn main() {
    let (width, height);

    let args: Vec<String> = env::args().collect();

    if args.len() == 5 && args[1].trim() == "-w" && args[3].trim() == "-h" {
        width = match args[2].trim().parse::<i32>() {
            Ok(_ok) => args[2].trim().parse::<i32>().unwrap(),
            Err(_e) => {
                println!("Invalid format. The Format is 'aspect -w [int] -h [int]'");
                return;
            }
        };

        height = match args[4].trim().parse::<i32>() {
            Ok(_ok) => args[4].trim().parse::<i32>().unwrap(),
            Err(_e) => {
                println!("Invalid arguments. The format is 'aspect -w [int] -h [int]'");
                return;
            }
        };
    }
    else if args.len() == 2 && args[1].trim() == "-help" {
        println!("Aspect, a small tool to calculate aspect ratios.\n");
        println!("Usage: aspect [-w (int)] [-h (int)]\n");
        println!("Arguments:\n  -w      width as an int\n  -h      height as an int\n  -help   displays this message\n");
        return;
    }
    else if args.len() > 1 {
        println!("Invalid arguments, The Format is 'aspect -w [int] -h [int]'");
        return;
    }
    else {
        println!("Welcome To Aspect! A Small Tool To Calculate Aspect Ratios.\n");
        (width, height) = get_values();
    }

    println!("\nThe width is {}, and the height is {}, that gives an aspect ratio of {}:{}.",
        width, height, width / gcd(width, height), height / gcd(height, width));

}

fn get_values() -> (i32, i32) {
    let mut width = String::new();
    let mut height = String::new();

    loop {
        print!("Enter Width: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut width).expect("Error Reading Width From STDIN");
        
        match width.trim().parse::<i32>() {
            Ok(_ok) => break,
            Err(_e) => println!("Invalid Input, Please Give A Number (Integer) ^-^"),
        }

        width = "".to_string();
    }

    loop {
        print!("Enter Height: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut height).expect("Error Reading Width From STDIN");
        
        match height.trim().parse::<i32>() {
            Ok(_ok) => break,
            Err(_e) => println!("Invalid Input, Please Give A Number (Integer) ^-^"),
        }

        height = "".to_string();
    }

    (width.trim().parse::<i32>().unwrap(), height.trim().parse::<i32>().unwrap())
}

use std::io;
use std::env;
use std::collections::HashMap;

//Day 05 2020 for advent of code
//
//
static mut DEBUG: bool = false;
static mut PART1: bool = true;
static mut PART2: bool = false; //assume part 1


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 { println!("No arguments passed"); }

    for s in args {
        match &s[..] {
            "--debug" => {
                println!("Debug mode");
                unsafe {DEBUG = true;}
            }
            "--part1" => {
                println!("Part1 mode explicitly set");
                unsafe{PART1 = true;}
            }
            "--part2" => {
                println!("Part2 mode explicitly set");
                unsafe{PART2 = true;}
            }
            _ => {
            }
        }
    }


    let orig_inp = read_input();  //read input buffer into strings
    let rec_inp = create_seat_nums(orig_inp);  //turn original into seat nums
    let mut max_count:i32 = 0;

    for i in rec_inp.keys() {
        if i>&max_count {
            max_count = *i;
        }
    }
    println!("The part 1 answer is {}",max_count);






}

fn read_input() -> Vec<String> {
    let mut inbuf = String::new();
    let mut outbuf = Vec::new();
    while 0<io::stdin().read_line(&mut inbuf).expect("I/O error on read"){
        outbuf.push(inbuf);
        inbuf = String::new();
    }
    outbuf
}

fn create_seat_nums(inp:Vec<String>) -> HashMap<i32,i32> {  
    let mut outbuf = HashMap::new();
    let mut seat_number_bin_str = String::new();
    let mut seat_number = 0;
    for line in inp.iter() {
        seat_number_bin_str = line.trim().replace("F","0").replace("B","1").replace("L","0").replace("R","1");
        seat_number = i32::from_str_radix(&seat_number_bin_str,2).unwrap();
        outbuf.insert(seat_number,seat_number);  // put seat number into HashMap 
    }
    outbuf
}


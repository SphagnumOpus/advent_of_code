use std::io;
use std::env;

//Day 02 2020 for advent of code
//
//

struct PwdRule {
    buffer: String,   //original input line
    low: i32,   //low count for letter
    high: i32,    //high count for letter
    letter: char,    //letter
    pwd: String,    //password string
}



fn main() {
    let args: Vec<String> = env::args().collect();

    let mut debug = false;
    if args.len() == 1 {
        println!("No arguments passed");
    }
    for s in args {
        match &s[..] {
            "-DEBUG" => {
                println!("Debug mode");
                debug = true;
            }
            _ => {
            }
        }
    }

    let mut vecbuffer = Vec::new(); //Vec to hold our data as PwdRule structs
    let mut input = String::new();
    let mut total_good = 0;
    let mut total_good_new_policy = 0;

    while 0<io::stdin().read_line(&mut input).expect("I/O error on read"){
        let splits: Vec<_> = input.split(|c: char| c.is_whitespace()|| c=='-' || c==':').collect();
        let cur_rule = PwdRule {
            buffer: input.clone(),
            low: splits[0].parse().unwrap(),
            high: splits[1].parse().unwrap(),
            letter: splits[2].parse().unwrap(),
            pwd: splits[4].to_string().clone(),
        };
        if debug {
            println!("{}={}={}={}={}",cur_rule.buffer,cur_rule.low,cur_rule.high,cur_rule.letter,cur_rule.pwd);
        }

        let mut chrcount = 0;
        for curchar in cur_rule.pwd.chars() {
            if cur_rule.letter == curchar {
                chrcount = chrcount + 1;
            }
        }

        if debug {
            println!("count equals {}",chrcount);
        }

        if chrcount >= cur_rule.low && chrcount <= cur_rule.high {
            total_good = total_good + 1;
        }
        let char_low = 
            cur_rule.pwd.chars().nth((cur_rule.low - 1) as usize).unwrap();
        let char_high = 
            cur_rule.pwd.chars().nth((cur_rule.high - 1)as usize).unwrap();
        if char_low == cur_rule.letter {
            if char_high != cur_rule.letter {
                total_good_new_policy = total_good_new_policy + 1;
            }
        }
        else {
            if char_high == cur_rule.letter {
                total_good_new_policy = total_good_new_policy + 1;
            }
        }
        vecbuffer.push(cur_rule);
        



        input = String::new();
    }


    println!("Total good responses equals {}",total_good);
    println!("Total good responses new policy equals {}",total_good_new_policy);
}

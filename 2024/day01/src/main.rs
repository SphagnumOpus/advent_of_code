use std::fs::read_to_string;
use std::env;

//Day 01 2024 for advent of code
//
//

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut debug = false;
    let mut filename = String::new();
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
                filename=s;
            }
        }
    }
    if debug {
        println!("Using input file {}",filename);
    }


    let mut lists: Vec<Vec<i32>> = Vec::new();
    lists.push(Vec::new());
    lists.push(Vec::new());  

    for input in read_to_string(filename).unwrap().lines().map(String::from){
        let splits: Vec<_> = input.split(|c: char| c.is_whitespace()|| c=='-' || c==':').collect();
        for i in 0..2 {
            lists[i].push (splits[i*3].parse().unwrap());
        }
    }   

    lists[0].sort();
    lists[1].sort();
    let mut total_distance=0;
    let mut similarity_score=0;
    for i in 0..lists[0].len() {
        total_distance += (lists[0][i]-lists[1][i]).abs();
        similarity_score += lists[0][i]* (lists[1].iter().filter(|&n| *n == lists[0][i]).count() as i32);
    }

    println!("Part 1 answer (Total distance between lists) equals {}",total_distance);
    println!("Part 2 answer (Similarity Score) equals {}",similarity_score);
}
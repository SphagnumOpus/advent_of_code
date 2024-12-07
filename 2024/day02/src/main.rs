use std::fs::read_to_string;
use std::env;

//Day 02 2024 for advent of code
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


    for input in read_to_string(filename).unwrap().lines().map(String::from){
        let mut this_list:Vec<i32> = Vec::new();
        let splits: Vec<_> = input.split(|c: char| c.is_whitespace()|| c=='-' || c==':').collect();
        for i in 0..splits.len()  {
            this_list.push(splits[i].parse().unwrap());
        }
        lists.push(this_list);
    }   

  
    let mut safe_reports=0;
    let mut safe_reports_dampened=0;
  
    for i in 0..lists.len(){
        if calc_is_it_safe(&lists[i]){
            safe_reports += 1;
            safe_reports_dampened += 1;
        }
        else {
            //look for dedampened sublists
            for j in 0..lists[i].len() {
                let mut chopped_list=lists[i].clone();
                chopped_list.remove(j);
                if calc_is_it_safe(&chopped_list){
                    safe_reports_dampened += 1;
                    break;
                }
            }
        }

        
    }

    println!("Part 1 answer (safe reports) equals {}",safe_reports);
    println!("Part 2 answer (safe reports dampened) equals {}",safe_reports_dampened);
}
fn calc_is_it_safe(inp:&Vec::<i32>) -> bool {
    let mut is_it_safe=true;   //be optimistic
    let mut safe_range: Vec<i32>=vec![1,2,3];
    if inp[0]>inp[1]{
        safe_range.iter_mut().for_each(|val| *val *= -1);  //make the vector negative
    }
    for j in 1..inp.len() {
        let curdiff=inp[j]-inp[j-1];
        is_it_safe=is_it_safe  && safe_range.contains(&curdiff);
        if !is_it_safe {
            break;
        }
    }
    is_it_safe  
}
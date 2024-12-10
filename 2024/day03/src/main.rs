use std::fs::read_to_string;
use std::env;

//Day 03 2024 for advent of code
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


    let mut multiplication_sum:i32=0;
    let mut multiplication_sum_improved:i32=0;
    let mut do_instruction = true;
    let mut do_instruction_improved = true;

    for input in read_to_string(filename).unwrap().lines().map(String::from){    
            multiplication_sum+=parse_for_mul_instruction(&input,&mut do_instruction,true);
            multiplication_sum_improved+=parse_for_mul_instruction(&input,&mut do_instruction_improved,false);
    }   
 

    println!("Part 1 answer (multiplication results) equals {}",multiplication_sum);
    println!("Part 2 answer (improved multiplication result) equals {}",multiplication_sum_improved);
}


fn parse_for_mul_instruction(inp:&String,inp_do_ind:&mut bool,ignore_do:bool) -> i32 {
    let mut multiply_total:i32 = 0;
    

    for i in 0..inp.len(){
        //look for mul( - split string by delimiter (
        let splits: Vec<_> = inp[i..].split(|c: char| c=='(' ).collect();
        if splits[0]=="don't" {
            *inp_do_ind=false;
        }
        else if splits[0]=="do" {
            *inp_do_ind=true;
        }
        else if (*inp_do_ind || ignore_do) && splits[0] == "mul" {
            //found mul(, now look take all until )
            let splits_getinparens: Vec<_> = splits[1].split(|c: char| c==')' ).collect();
            //now parse this as if its a number pair separated by commas.
            let splits_getnums: Vec<_> = splits_getinparens[0].split(|c: char| c==',' ).collect();
            //now if i have two two elements that are nums, we've found one... add results to total... otherwise move on
            if splits_getnums.len() ==2  {
                let mut product:i32=1;
                for j in 0..2{
                    product=product * match splits_getnums[j].parse::<i32>() {
                        Ok(number) => number,
                        Err(_e) =>  0 as i32,
                    };
                }
                multiply_total+=product;
            }

        }

    }
    multiply_total  
}

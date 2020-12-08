use std::io;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;

//Day 07 2020 for advent of code
//
//
static mut DEBUG: bool = false;
static mut PART1: bool = true;
static mut PART2: bool = false; //assume part 1


struct Registers {
    program_counter:isize,
    accumulator:isize,
} // future proofing this by building a register struct

#[derive(Debug, Clone, Copy)]
struct AssemCodeLine {
    code:InstrCode,
    arg1:isize,
}

#[derive(Debug, Clone, Copy)]
enum InstrCode{
    Nop,
    Acc,
    Jmp,
}


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
    let mem_map = create_mem_map(&orig_inp);  //create memory map
    let mut cur_reg = Registers{
        program_counter: 0,
        accumulator: 0,
    };

    let mut repeating_myself = false;
    let mut been_there=HashSet::new();
    while !repeating_myself{
        cur_reg=execute_code(&mem_map,cur_reg);
        repeating_myself = !been_there.insert(cur_reg.program_counter); 
    }


    println!("The part 1 answer is {}",cur_reg.accumulator);
    let past_the_end = mem_map.len() as isize;
    for i in 0..past_the_end {
        let mut new_mem_map = mem_map.clone();
        let cur_al = match new_mem_map.get(&i){
                Some(cs) =>  cs,
                _ => &AssemCodeLine{code : InstrCode::Acc, arg1 : 0},
                };
        let new_ic = match cur_al.code {
            InstrCode::Nop => InstrCode::Jmp,
            InstrCode::Jmp => InstrCode::Nop,
            InstrCode::Acc => InstrCode::Acc,
        };
        let new_acl = AssemCodeLine{code : new_ic, arg1 : cur_al.arg1};
        new_mem_map.insert(i,new_acl);
        let mut repeating_myself = false;
        let mut been_there=HashSet::new();
        cur_reg = Registers{
            program_counter: 0,
            accumulator: 0,
        };
        while !repeating_myself && cur_reg.program_counter + 1 < past_the_end{
            cur_reg=execute_code(&new_mem_map,cur_reg);
            repeating_myself = !been_there.insert(cur_reg.program_counter); 
        }
        if !repeating_myself {
            break;
        }
    }







    println!("The part 2 answer is {}",cur_reg.accumulator);






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

fn create_mem_map(inp:&Vec<String>) -> HashMap<isize,AssemCodeLine> {
    let mut out = HashMap::new();
    let mut cur_mem_loc:isize = 0;
    for i in inp {  //for each raw string containing a bag rule
        let mut itr = i.split_whitespace();  //itr to get instr and arg
        let cur_inst = match itr.next().unwrap() {
            "nop" => InstrCode::Nop,
            "acc" => InstrCode::Acc,
            "jmp" => InstrCode::Jmp,
            _ => InstrCode::Nop,     //if I don't recognize an instr, it's a nop

        };
        let cur_arg1_str = itr.next().unwrap();
        let cur_arg1 =  cur_arg1_str.parse::<isize>().unwrap();
        let cur_acl = AssemCodeLine{code : cur_inst,arg1 : cur_arg1};
        out.insert(cur_mem_loc,cur_acl);
        cur_mem_loc += 1;
    }
    out
}

fn execute_code (inp_mem_map:&HashMap<isize,AssemCodeLine>, inp_reg:Registers) -> Registers {
    let cur_al = match inp_mem_map.get(&inp_reg.program_counter){
                Some(cs) =>  cs,
                _ => &AssemCodeLine{code : InstrCode::Nop, arg1 : 0},
            };
    let new_register = match cur_al.code {
        InstrCode::Nop => Registers{
            program_counter: inp_reg.program_counter + 1, 
            accumulator: inp_reg.accumulator},
        InstrCode::Acc => Registers{
            program_counter: inp_reg.program_counter + 1, 
            accumulator: inp_reg.accumulator + cur_al.arg1},
        InstrCode::Jmp => Registers{
            program_counter: inp_reg.program_counter + cur_al.arg1, 
            accumulator: inp_reg.accumulator},
    };
    new_register
}

        

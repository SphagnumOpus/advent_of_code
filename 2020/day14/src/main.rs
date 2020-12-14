use std::io;
use std::collections::HashMap;

//Day 14 2020 for advent of code
//
//
//

//#[derive (Copy)];
enum Instruction {
    Mask(BitMask),
    Mem(MemArgs)
}

//#[derive (Copy)];
struct BitMask {
    _mask_str: String,
    mask_base: usize,
    mask_and: usize
}

//#[derive (Copy)];
struct MemArgs{
    mem_loc: usize,
    value: usize
}



fn main() {

    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let int_vec = build_int_vec(&orig_inp);
    
    let part1_answer = calc_part1(&int_vec);
    println!("The part 1 answer is {}",part1_answer);

    let part2_answer = calc_part2(&orig_inp);
    println!("The part 2 answer is {}",part2_answer);
}

fn read_input() -> Vec<String> {
    let mut inbuf = String::new();
    let mut outbuf = Vec::new();
    while 0<io::stdin().read_line(&mut inbuf).expect("I/O error on read"){
        inbuf.truncate(inbuf.trim_end().len());
        outbuf.push(inbuf);
        inbuf = String::new();
    }
    outbuf
}

fn calc_part1(inp:&Vec<Instruction>) ->usize {
    let mut cur_mask = match &inp[0]{
        Instruction::Mask(m) => m,
        _ => panic!("Error, first instruction is not a mask"),
    };
    let mut hm = HashMap::new();
    for i in inp {
        assert_eq!(true, match i {
            Instruction::Mask(m) => {
                cur_mask = &*m;
                true},
            Instruction::Mem(m) => {
                hm.insert(m.mem_loc,apply_mask(&cur_mask,&m.value));
                true},
        });
    }
    let mut out=0;
    for (_key,value) in hm {
        out += value;
    };
    out
}



fn calc_part2(_inp:&Vec<String>) -> isize {
    0
   
}

fn build_int_vec(inp:&Vec<String>) ->Vec<Instruction> {
    let mut out = Vec::new();
    for i in inp {
        let out_itm = match &i[1..2] {
            "a" => { //mask line
                let mask_string:String = i.chars().filter(|c| c.is_numeric() || *c=='X').collect();
                let mask_bm = BitMask{
                    _mask_str:mask_string.clone(),
                    mask_base: build_mask_base(&mask_string),
                    mask_and: build_mask_and(&mask_string)
                };
                Instruction::Mask(mask_bm)
            }
            "e" => {//mem line
                let splits: Vec<_> = i.split(|c: char| c.is_whitespace() || c=='[' || c==']').collect();
                let cur_mem_args = MemArgs{
                    mem_loc:splits[1].parse().expect("Parsing error on mem location"),
                    value :splits[4].parse().expect("Parsing error on value")
                };
                Instruction::Mem(cur_mem_args)
                }
            _ => panic!("Error, unrecognized input line"),
        };
        out.push(out_itm);
    }
    out
}

fn build_mask_base (inp:&String) -> usize {
    let mut out:usize=0;
    for (i_idx, i) in inp.chars().enumerate() {
        out = out | (1<<(35-&i_idx)) * match i {
            '0' => 0,
            '1' => 1,
            'X' => 0,
            _ => panic!("error, bad mask"),
        };
    }
    out
}

fn build_mask_and (inp:&String) -> usize {
    let mut out=0;
    assert_eq!(36,inp.len());
    for (i_idx, i) in inp.chars().enumerate() {
        out = out | (1<<(35-&i_idx)) * match i {
            '0' => 0,
            '1' => 0,
            'X' => 1,
            _ => panic!("error, bad mask"),
        };
    }
    out
}

fn apply_mask(inp_bm: &BitMask, inp_u: &usize) -> usize{
    let out = inp_bm.mask_base | (inp_u & inp_bm.mask_and);
    out
}



        

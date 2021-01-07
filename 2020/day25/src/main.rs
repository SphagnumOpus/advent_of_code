use std::io;

//Day 25 2020 for advent of code
//
//
//
//
type LoopSize = usize;
type PublicKey = usize;

type FinalAnswer = usize;


fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let this_public_key_list = build_this_public_key_list(&orig_inp);
    let answer = calc_answer(&this_public_key_list);
    println!("The answer is {}",&answer);
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
fn build_this_public_key_list(inp:&Vec<String>) -> Vec<PublicKey> {
    if inp.len() < 2 {
        panic!("Error, only {} lines of input, need at least two",inp.len());
    }
    else if inp.len() > 2 {
        println!("Warning... more input than needed({} lines)... taking the first two lines only",inp.len());
    }
    let out=inp.iter().map(|x| x.parse::<PublicKey>().unwrap()).collect();
    out
}

fn calc_answer(inp:&Vec<PublicKey>) ->FinalAnswer{
    let mut subject_number = 7;
    let starting_value = 1;
    let dividing_value = 20201227;
    let mut current_loop_size:LoopSize = 0;
    let mut candidate_public_key =starting_value;
    let x0 = inp[0];   // set the two array elements to specific scalar variables
    let x1 = inp[0];   // to speed up comparisons in the following while loop
    while x0 != candidate_public_key && x1 != candidate_public_key {
        candidate_public_key= (candidate_public_key*subject_number)%dividing_value;
        current_loop_size += 1;
    }
    //found a public key!  Now use that loop size on the OTHER public key
    subject_number = match x0==candidate_public_key {
        true => x1,
        false =>x0 
    };
    let mut encryption_key = 1;
    for _i in 0..current_loop_size {
        encryption_key = (encryption_key*subject_number)%dividing_value;
    }

    encryption_key
}


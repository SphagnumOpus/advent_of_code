use std::io;
use std::collections::HashSet;

//Day 09 2020 for advent of code
//
//




fn main() {

    let orig_inp = read_input();  //read input buffer into strings
    let xmas_data = create_xmas_data(&orig_inp);  //create data as vec<usize>

    //Part 1 - look for first non conforming number with preamble length 25
    let part1_answer = return_first_nc(&xmas_data, 25);
    println!("The part 1 answer is {}",part1_answer);

    //now find a contiguous set of numbers that add to part1_answer

    let mut lower_bounds=0;
    let mut upper_bounds=0;
    let mut current_sum = xmas_data[0];
    while current_sum!=part1_answer{
        if current_sum<part1_answer {
            upper_bounds += 1;
            current_sum += xmas_data[upper_bounds];
        }
        else { //this will only be less than.... equals would have left the loop
            current_sum -= xmas_data[lower_bounds];
            lower_bounds += 1;
        }
    }
    let mut min = xmas_data[lower_bounds];
    let mut max = min;
    for i in lower_bounds..upper_bounds {
        let cur=xmas_data[i];
        if cur > max {
            max = cur;
        }
        else if cur < min {
            min = cur;
        }
    }

    let part2_answer= min+max;
    println!("The part 2 answer is {}",part2_answer);






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

fn create_xmas_data(inp:&Vec<String>) -> Vec<usize> {
    let mut out = Vec::new();
    for i in inp {
        let x= i.trim().parse::<usize>().unwrap();
        out.push(x);
    }
    out
}

fn return_first_nc(inp:&Vec<usize>,preamb_len:usize) -> usize {
    for i in 0..(inp.len()-preamb_len) {
        if !valid_preamble(inp[i..i+preamb_len].to_vec(),inp[i+preamb_len]){
            return inp[i+preamb_len];
        }
    }
    0
}
fn valid_preamble(preamb:Vec<usize>, tested:usize) -> bool {
    //create a lookup table of the potential larger of the two numbers summed
    let higherthanhalf = tested/2+1;
    let highernums:HashSet<_>=preamb.iter().filter(|c| *c >= &higherthanhalf).collect();
    for i in preamb.iter().filter(|c| *c < &higherthanhalf) {
        let target_num=tested-i;
        if highernums.contains(&target_num) {
            return true;
        }
    }
    false
}

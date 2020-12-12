use std::io;
use std::cmp::min;

//Day 11 2020 for advent of code
//
//




fn main() {

    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let mut cur_seats = orig_inp.clone();  //just to separate string from input
    let mut new_seats;
    loop {
        new_seats = new_round(&cur_seats);
        if seat_maps_equal(&cur_seats, &new_seats) {
            break;
        }
        cur_seats = new_seats;
    }
    let part1_answer = new_seats.iter()
                                .fold(0,|acc,a| acc + a.matches("#").count());
    println!("The part 1 answer is {}",part1_answer);
    let part2_answer = create_part2_answer(new_seats);
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


fn create_part2_answer(_inp:Vec<String>) -> isize {
    0
}

fn seat_maps_equal(inp1:&Vec<String>, inp2:&Vec<String>) -> bool {
    if inp1.len() != inp2.len() {
        return false;
    };
    for (indx,value) in inp1.iter().enumerate() {
        if *value != inp2[indx] {
            return false;
        }
    };
    true
}

fn new_round(inp:&Vec<String>)-> Vec<String>  {
    let mut out=Vec::new();
    for (i_idx,i) in inp.iter().enumerate() {
        let mut bld_row=Vec::new();
        for (j_idx,j) in i.chars().enumerate() {
            if j == '.' {  //seats don't change
                bld_row.push('.');
                continue;
            }
            let mut occ_seats = 0;
            for x in sub1(i_idx)..=min(i_idx + 1, inp.len()-1) {
                occ_seats += inp[x][sub1(j_idx) ..= min(j_idx + 1,i.len()-1)]
                    .chars()
                    .filter(|a| *a=='#')
                    .count();
            }
            if j=='#' {
                occ_seats-=1;
            }
            bld_row.push(match occ_seats  {
                0 => '#', 
                1 | 2 | 3 => j,
                4 => 'L',
                _ => 'L',
                });

        }
        out.push(bld_row.into_iter().collect()); //initialize new string for this row
    }
    out
}

fn sub1(inp:usize)->usize{
    match inp {
        0 => 0,
        _ => inp-1,
    }
}


use std::io;

//Day 11 2020 for advent of code
//
//
#[derive(Clone,Copy)]
enum WhichPart{
    Part1,
    Part2
}




fn main() {

    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let mut cur_seats = orig_inp.clone();  //just to separate string from input
    let mut new_seats;
    loop {
        new_seats = new_round(&cur_seats,WhichPart::Part1);
        if seat_maps_equal(&cur_seats, &new_seats) {
            break;
        }
        cur_seats = new_seats;
    }
    let part1_answer = new_seats.iter()
                                .fold(0,|acc,a| acc + a.matches("#").count());
    println!("The part 1 answer is {}",part1_answer);
    let mut cur_seats = orig_inp.clone();  //reset matrix to start
    loop {
        new_seats = new_round(&cur_seats,WhichPart::Part2);
        if seat_maps_equal(&cur_seats, &new_seats) {
            break;
        }
        cur_seats = new_seats;
    }
    let part2_answer = new_seats.iter()
                                .fold(0,|acc,a| acc + a.matches("#").count());
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

fn new_round(inp:&Vec<String>,part:WhichPart)-> Vec<String>  {
    let mut out=Vec::new();
    for (i_idx,i) in inp.iter().enumerate() {
        let mut bld_row=Vec::new();
        for (j_idx,j) in i.chars().enumerate() {
            if j == '.' {  //seats don't change
                bld_row.push('.');
                continue;
            }
            let occ_seats = calc_occ_seats(inp,part,i_idx,j_idx);
            bld_row.push(match occ_seats  {
                0 => '#', 
                1 | 2 | 3 => j,
                4 => match part {
                    WhichPart::Part1 => 'L',
                    WhichPart::Part2 => j,
                }
                _ => 'L',
                });

        }
        out.push(bld_row.into_iter().collect()); //initialize new string for this row
    }
    out
}


fn calc_occ_seats(in_mat:&Vec<String>,
                  in_part:WhichPart,
                  in_row:usize,
                  in_col:usize) -> usize {
    let mut out = 0;
    let directions = vec![
        (0,-1),(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1)];
    for cd in &directions {
        let limit = match in_part {
            WhichPart::Part1 => 1,
            WhichPart::Part2 => 9999, //high value, so loops as much as needed
        };
        let mut ptr = (in_row,in_col);
        let mut counter=0;
        while inc_ptr(&in_mat,&cd,&mut ptr) && counter < limit{
            counter+=1;
            if in_mat[ptr.0][ptr.1..ptr.1+1].chars().next()==Some('L') {
                break;
            }
            if in_mat[ptr.0][ptr.1..ptr.1+1].chars().next()==Some('#') {
                out+=1;
                break;
            };
        };
    };


    out
}

fn inc_ptr(inm:&Vec<String>,
           in_cd:&(i32,i32),
           in_ptr:&mut (usize,usize)) -> bool {
    //avoid negative indices
    if (in_cd.0 == -1 && in_ptr.0 == 0) 
        || (in_cd.1 == -1 && in_ptr.1 == 0){
        return false;
    }
    //avoid out-of-bounds above
    else if (in_cd.0 == 1 && in_ptr.0+1==inm.len())
             || (in_cd.1 == 1 && in_ptr.1+1==inm[in_ptr.0].len()){
        return false;
    };
    in_ptr.0 = match in_cd.0 {
        -1 => in_ptr.0-1,
        0  => in_ptr.0,
        1 => in_ptr.0+1,
        _ => in_ptr.0,
    };
    in_ptr.1 = match in_cd.1 {
        -1 => in_ptr.1-1,
        0  => in_ptr.1,
        1 => in_ptr.1+1,
        _ => in_ptr.0,
    };
    true
}


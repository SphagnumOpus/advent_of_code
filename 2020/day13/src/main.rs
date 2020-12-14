use std::io;
use std::collections::HashMap;
use std::cmp::Reverse;

//Day 13 2020 for advent of code
//
//


fn main() {

    let orig_inp = read_input();  //read input buffer strings (trimmed)
    assert_eq!(2,orig_inp.len()); //confirm input is two lines
    let earliest_time=orig_inp[0].parse::<isize>().expect("Error, first line not a number");
    let bus_list = create_bus_list(&orig_inp[1]);
    
    let part1_answer = calc_part1(&earliest_time,&bus_list);
    println!("The part 1 answer is {}",part1_answer);

    let part2_answer = calc_part2(&orig_inp[1],&bus_list);
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

fn create_bus_list(inp:&String) -> Vec<isize> {
    let mut out = Vec::new();
    for i in inp.split(",") {
        if i != "x" {
            let x = i.trim().parse::<isize>().expect("Error, not an x and not a number");
            out.push(x);
        }
        
    }
    out
}

fn calc_part1(inpt:&isize,inpb:&Vec<isize>) ->isize {
    let mut min=*inpt;   //keeping track of minimum wait
    let mut min_i=0;
    for i in inpb {
        let divisor = inpt/i;
        let wait=(i*(divisor+1))-inpt;
        if wait < min {
            min = wait;
            min_i=*i;
        }
    };
    let out=min*min_i;
    out
}



fn calc_part2(inps:&String,inpb:&Vec<isize>) -> isize {
    let mut hm=HashMap::new();
    let mut zero_elem = 0;
    for (idx,i) in inps.split(",").enumerate() {
        if i != "x" {
            let val = i.trim().parse::<isize>().expect("Error, not an x and not a number");
            if idx==0 {
                zero_elem=val as isize;
            }
            hm.insert(val,idx as isize);
        }
    }
    let mut rev_sorted_bus_list = inpb[1..].to_vec().clone();
    rev_sorted_bus_list.sort_by_key(|k| Reverse(*k));


   let target = zero_elem;
   find_target(&rev_sorted_bus_list,&hm,&target,&target)
   
}

fn find_target(inp_bl:&Vec<isize>,inp_hm:&HashMap<isize,isize>,inp_strt_pt:&isize, inp_chunk_size:&isize)->isize{
    if inp_bl.is_empty() {
        return *inp_strt_pt;
    }
    //units of time after the primary 
    let time_units_after=inp_hm.get(&inp_bl[0]).expect("program error... hash entry missing");
    let mut cur_total = *inp_strt_pt;
    while ((cur_total+time_units_after)%inp_bl[0]) != 0 {
        cur_total += inp_chunk_size;
    }
    //we've found a match for this bus ... use this match as the incr for the next stage(s)
    let cur_nearest_1st_bus=cur_total;
    let new_chunk_size=*inp_chunk_size * inp_bl[0];
    find_target(&inp_bl[1..].to_vec(),&inp_hm,&cur_nearest_1st_bus,&new_chunk_size)
}

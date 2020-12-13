use std::io;

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

    let part2_answer = 0;
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




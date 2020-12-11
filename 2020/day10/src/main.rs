use std::io;

//Day 10 2020 for advent of code
//
//




fn main() {

    let orig_inp = read_input();  //read input buffer into strings
    let isize_data = create_isize_data(&orig_inp);  //create data as vec<isize>
    let mut isize_data_sorted=isize_data.clone();
    isize_data_sorted.sort();
    // add the "device" joltage at the end so algorithms come out right
    isize_data_sorted.push(isize_data_sorted[isize_data_sorted.len()-1] + 3);

    //Part 1 - add the diffs between values
    let strt = 0;
    let result = isize_data_sorted.iter()
        .fold((strt,strt,strt),|acc,&b| match b-acc.0{
            1 => (b, acc.1 + 1, acc.2),
            3 => (b, acc.1, acc.2 + 1),
            _ => panic!("error... invalid diff {}",b-acc.0)
            });
    let part1_answer = result.1 * (result.2);
    println!("The part 1 answer is {}",part1_answer);
    let part2_answer = create_part2_answer(isize_data_sorted);
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

fn create_isize_data(inp:&Vec<String>) -> Vec<isize> {
    let mut out = Vec::new();
    for i in inp {
        let x= i.trim().parse::<isize>().unwrap();
        out.push(x);
    }
    out
}

fn create_part2_answer(inp:Vec<isize>) -> isize {
    //create vec of "clumps" of contiguous numbers, storing size of "clump"
    let mut clumps=Vec::new();
    let mut clump_size=1; //set to 1 so first clump includes origin of 0
    let mut prv = 0;
    for i in inp{
        if i-prv == 3 {
            clumps.push(clump_size);
            clump_size=0;
        }
        clump_size+= 1;
        prv = i;
    };

    //multiply the clump combinations to get overall combinations
    let out = clumps.iter().fold(1,|acc,&b| combos(b)*acc); 


    out
}

//given an input clump size, how may legal combos are available for that clump?
//given that starting joltage and ending joltage need to be preserved?
fn combos(inp:isize) -> isize {
    match inp{
        1=>1,
        2=>1,
        3=>2,
        4=>4,
        5=>7,
        _=>panic!("haven't calculated that high!")
    }
}

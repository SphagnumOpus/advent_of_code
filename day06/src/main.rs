use std::io;
use std::env;

//Day 06 2020 for advent of code
//
//
static mut DEBUG: bool = false;
static mut PART1: bool = true;
static mut PART2: bool = false; //assume part 1


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
    let rec_inp = create_rec(orig_inp);  //turn the original input into records
    let parsed_inp = create_parsed(&rec_inp); // records into answer totals
    unsafe{if DEBUG {println!("Found {} records",parsed_inp.iter().count());}}

    let total_count:usize = parsed_inp.iter().sum();
    println!("Part 1 answer is {}",total_count);


    let parsed_inp_pt2 = create_parsed_pt2(&rec_inp);
    let total_count_pt2:usize = parsed_inp_pt2.iter().sum();
    println!("Part 2 answer is {}",total_count_pt2);

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

fn create_rec(inp:Vec<String>) -> Vec<String> {  
    let mut curbuf = String::new();
    let mut outbuf = Vec::new();
    for line in inp.iter() {  //combine all into one string until empty line
        if line.trim_end().len() == 0 {
            outbuf.push(curbuf.clone());
            unsafe{if DEBUG {println!("rec={}",curbuf);}}
            curbuf = String::new();
        }
        curbuf=curbuf + line;
    }  
    outbuf.push(curbuf.clone());  // put last input into output
    unsafe{if DEBUG {println!("rec={}",curbuf);}}
    outbuf
}

fn create_parsed(inp:&Vec<String>) -> Vec<usize> {
    let mut out = Vec::new();  //create usize to hold output
    for i in inp{
         let tot = uniq_ans(i.to_string()).len();  //count uniq chars
         out.push(tot);
    }
    out
}

fn create_parsed_pt2(inp:&Vec<String>) -> Vec<usize> {
    let mut out = Vec::new();  //create usize to hold output
    for i in inp{
        let num_ans=i.split_whitespace().count();  //get number of answerers
        let uniq=uniq_ans(i.to_string());
        let mut all_ans_tot = 0;
        for j in uniq {
            if num_ans==i.chars().filter(|c| c == &j).count() {
                all_ans_tot += 1;
            }
        }
        out.push(all_ans_tot);
    }
    out
}

fn uniq_ans(inp:String) ->Vec<char> {

         let mut a:Vec<char> = inp.chars().filter(|c| !c.is_whitespace()).collect();  //only what chars
         a.sort();
         a.dedup();
         a
}

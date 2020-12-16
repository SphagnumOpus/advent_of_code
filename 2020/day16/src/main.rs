use std::io;
use std::collections::HashMap;

//Day 16 2020 for advent of code
//
//
//


#[derive (Debug)]
struct RangeEntry {
    name: String,
    min: u32,
    max: u32,
}





fn main() {

    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let range_list = build_range_list(&orig_inp);
    println!("range_list = {:?}",range_list);
    let range_lookup = build_range_lookup(&range_list);
    println!("range_lookup = {:?}",range_lookup);
    let nearby_tickets = build_nearby_tickets(&orig_inp);
    
    let part1_answer = calc_part1(&range_lookup,&nearby_tickets);
    println!("The part 1 answer is {}",part1_answer);

    let part2_answer = calc_part2(&range_list);
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

fn calc_part1(inp_rl:&HashMap<u32,Vec<String>>,inp_nt:&Vec<Vec<u32>>) ->u32 {
    let mut out = 0;
    for i in inp_nt {
        for j in i {
            if inp_rl.get(j) == None {
                out += j;
            }
        }
    }
    out
}


fn calc_part2(inp:&Vec<RangeEntry>) ->usize {
    0
}


fn build_range_list(inp:&Vec<String>) -> Vec<RangeEntry> {
    let mut out=Vec::new();
    for i in inp {
        if i.len() == 0 {
            break;
        };
        println!("i = {}",i);
        let mut j = i.split(':');
        let tmp_name= j.next().expect("Error, badly formed input line").clone().to_string();
        let tmp_ranges= j.next().expect("Error, badly formed input line").clone().to_string();
        println!("tmp_name = {}",tmp_name);
        for k in tmp_ranges.split(' ') {
            println!("k = {}",k);
            if k.len() > 0 && k[0..1].chars().nth(0).expect("error in k").is_numeric() {
                let mut l = k.split('-');
                let tmp_min = l.next().expect("Error... no number found in range").parse().expect("Error in parse");
                let tmp_max = l.next().expect("Error... no number found in range").parse().expect("Error in parse");
                let tmp_range = RangeEntry{
                    name: tmp_name.clone(),
                    min: tmp_min,
                    max: tmp_max
                };
                out.push(tmp_range);
            };
        }
    };
    out
}

fn build_range_lookup(inp:&Vec<RangeEntry>) ->HashMap<u32,Vec<String>> {
    let mut out:HashMap<u32,Vec<String>> = HashMap::new();
    for i in inp {
        for j in i.min..=i.max {
            let mut t_vec = Vec::new();
            t_vec = match out.get_mut(&j) {
                Some(v) => v.to_vec(),
                None => t_vec
                };
            t_vec.push(i.name.clone());
            out.insert(j,t_vec);
        }
    }
    println!("out = {:?}",out);
    out
}

fn build_nearby_tickets(inp:&Vec<String>)  -> Vec<Vec<u32>> {
    let mut out = Vec::new();
    let mut into_nearby = false;
    for i in inp {
        if into_nearby {
            let mut tmpvec = Vec::new();
            for j in i.split(',') {
                let tmpval = j.parse().expect("non-numeric nearby ticket");
                tmpvec.push(tmpval);
            }
            out.push(tmpvec);
        }
        else {
            if i == "nearby tickets:" {
                into_nearby = true;
            };
        };
    };
    out
}

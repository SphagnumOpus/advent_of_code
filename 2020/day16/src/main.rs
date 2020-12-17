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
    let range_list = build_range_list(&orig_inp);           //turn the input range values into a useable vec 
    let nearby_tickets = build_nearby_tickets(&orig_inp);   //turn nearby tickets into useable vec of field vecs
    let range_lookup = build_range_lookup(&range_list);     // now build an instance table of value to valid names
    
    let part1_answer = calc_part1(&range_lookup,&nearby_tickets);
    println!("The part 1 answer is {}",part1_answer);
    let my_ticket = build_my_ticket(&orig_inp);             // now build MY ticket's fields into a vec for part 2
    let part2_answer = calc_part2(&range_lookup,&nearby_tickets,&my_ticket);
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


fn calc_part2(inp_rl:&HashMap<u32,Vec<String>>,inp_nt:&Vec<Vec<u32>>,inp_mt:&Vec<u32>) ->usize {
    let mut out:usize = 1;
    let mut field_lookup:HashMap<usize,Vec<String>> = HashMap::new();
    let mut solved_fields:HashMap<String,usize> = HashMap::new();
    //first figure out proper field list
    let mut fields_initialized = false;
    let mut num_of_choices = 1;  //can start as any number greater than the current length of field_lookup
    while field_lookup.len() < num_of_choices {  //as long as there are more choices than fields (haven't narrowed down enough)
        for i in inp_nt {
            let mut good_ticket = true;
            for j in i {
                if inp_rl.get(j) == None {
                    good_ticket = false;
                }
            }
            if good_ticket {
                if fields_initialized {
                    for (idx_j,j) in i.iter().enumerate() {
                        let prior_list = field_lookup.get(&idx_j).expect("error program insane!");
                        if prior_list.len() > 1 {
                            let this_list=inp_rl.get(j).expect("error program insane!");
                            let mut new_list=Vec::new();
                            for k in prior_list {
                                if !solved_fields.contains_key(k) {
                                    for l in this_list{
                                        if k == l{
                                            new_list.push(k.clone());
                                            break;
                                        }
                                    }
                                }
                            }
                            field_lookup.insert(idx_j,new_list);
                        }
                    }
                }
                else {
                        for (idx_j,j) in i.iter().enumerate() {
                        let tmp_vec=inp_rl.get(&j).expect("error program insane!");
                        field_lookup.insert(idx_j,tmp_vec.clone());
                        }
                    fields_initialized = true;
                };
            }
        }
        //recalc num_of_choices sum for all fields
        num_of_choices = 0;
        for (x,y) in &field_lookup {
            if y.len() == 1 {
                solved_fields.insert(y[0].clone(),*x);
            }
            num_of_choices += y.len();
        }
    }
    for (idx_i,i) in inp_mt.iter().enumerate() {
        let tmp_vec = field_lookup.get(&idx_i).unwrap();
        assert_eq!(tmp_vec.len(), 1, "my approach didn't work!");
        out = match tmp_vec[0].find("departure") {
            Some(n) => {
                if n==0 {
                    out * (*i as usize)
                }
                else {
                out
                }
            }
            None => out
        };
    };
    out
}


fn build_range_list(inp:&Vec<String>) -> Vec<RangeEntry> {
    let mut out=Vec::new();
    for i in inp {
        if i.len() == 0 {
            break;
        };
        let mut j = i.split(':');
        let tmp_name= j.next().expect("Error, badly formed input line").clone().to_string();
        let tmp_ranges= j.next().expect("Error, badly formed input line").clone().to_string();
        for k in tmp_ranges.split(' ') {
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
            let mut t_vec = match out.get_mut(&j) {
                Some(v) => v.to_vec(),
                None => Vec::new()
                };
            t_vec.push(i.name.clone());
            out.insert(j,t_vec);
        }
    }
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
fn build_my_ticket(inp:&Vec<String>) -> Vec<u32> {
    let out:Vec<u32> = Vec::new();
    let mut into_my = false;
    for i in inp {
        if into_my {
            let mut tmpvec = Vec::new();
            for j in i.split(',') {
                let tmpval = j.parse().expect("non-numeric nearby ticket");
                tmpvec.push(tmpval);
            }
            return tmpvec;
        }
        else {
            if i == "your ticket:" {
                into_my = true;
            };
        };
    };
    out
}

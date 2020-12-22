use std::io;
use std::option::Option;

//Day 19 2020 for advent of code
//
//
//
type Byte = u8;
type RuleNum = usize;

#[derive(Clone,Debug)]
struct Rule {
    test_bytes: Vec<Byte>,
    test_rules: Vec<Vec<RuleNum>>
}



fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let mut rules = build_rules(&orig_inp);
    let test_list = build_test_list(&orig_inp);
    let part1_answer = calc_both_parts(&rules,&test_list);
    println!("The part 1 answer is {}",&part1_answer);
    //before part 2, change the two rules requested...
    //first rule 8
    let mut tmp_vec1 = Vec::new();
    let mut tmp_vec2 = Vec::new();
    let mut tmp_val = 42;
    tmp_vec2.push(tmp_val);
    tmp_val = 8;
    tmp_vec2.push(tmp_val);
    tmp_vec1.push(tmp_vec2.clone());
    tmp_vec2 = Vec::new();
    tmp_val = 42;
    tmp_vec2.push(tmp_val);
    tmp_vec1.push(tmp_vec2.clone());
    rules[8].test_rules = tmp_vec1.clone();
    //now rule 11 -- restructure as recursive calls to rule 31, as this has same effect as calling
    //42 first if we check at the end that our 42 calls exceed our 31 calls.
    let mut tmp_vec1 = Vec::new();
    let mut tmp_vec2 = Vec::new();
    tmp_val = 31;
    tmp_vec2.push(tmp_val);
    tmp_val = 11;
    tmp_vec2.push(tmp_val);
    tmp_vec1.push(tmp_vec2.clone());
    tmp_vec2 = Vec::new();
    tmp_val = 31;
    tmp_vec2.push(tmp_val);
    tmp_vec1.push(tmp_vec2.clone());
    rules[11].test_rules = tmp_vec1.clone();


    let part2_answer = calc_both_parts(&rules,&test_list);
    println!("The part 2 answer is {}",&part2_answer);
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
fn build_rules(inp:&Vec<String>) -> Vec<Rule> {
    let mut out = Vec::new();
    let tmp_rule = Rule {
        test_bytes:  Vec::new(),
        test_rules: Vec::new()
    };
    for i in inp { //first build the vec of all rules
        if i.len()==0 {
            break;
        };
        out.push(tmp_rule.clone());
        out.push(tmp_rule.clone()); //push another one for testing
    };
    for i in inp { //now fill them
        if i.len()==0 {
            break;
        };
        let mut colon_iter = i.split(":");
        let rule_idx = colon_iter.next().unwrap().parse::<RuleNum>().unwrap();
        let test_str = colon_iter.next().unwrap();
        if test_str.split("\"").count() > 1 { //look for char test rules
            let mut tmp_itr=i.split("\"");
            tmp_itr.next().unwrap(); //skip before "
            let tmp_byte=tmp_itr.next().unwrap().as_bytes().first().unwrap();
            let mut tmp_vec = Vec::new();
            tmp_vec.push(*tmp_byte);
            out[rule_idx].test_bytes = tmp_vec.clone();
        }
        else { //look for rules with ands and ors 
            let mut tmp_vec1=Vec::new();
            for j in test_str.split("|") {
                let mut tmp_vec2 = Vec::new();
                for k in j.split_whitespace() {
                    if k.len()>0 {
                        let tmp_val = k.parse::<RuleNum>().unwrap();
                        tmp_vec2.push(tmp_val);
                    }
                }
                tmp_vec1.push(tmp_vec2);
            }
            out[rule_idx].test_rules = tmp_vec1;
        }
    };
    out
}
fn build_test_list(inp:&Vec<String>) -> Vec<&[Byte]> {
    let mut out = Vec::new();
    let mut reached_test_strings = false;
    for i in inp {
        if reached_test_strings {
            let b = i.as_bytes().clone();
            out.push(b);
        }
        else {
            if i.len() ==0 {
                reached_test_strings = true;
            }
        }
    }
    out
}

fn calc_both_parts(inp_r:&Vec<Rule>,inp_t:&Vec<&[Byte]>) ->u16{
    let mut total:u16 = 0;
    for i in inp_t{
        let mut hist_vec = Vec::new();
        total += match test_rule(&inp_r,&i,0,&mut hist_vec){
            Some(n)=> match n.len() == 0 {
                true=> 1,
                false=> 0,
            },
            None => 0
        }
    }
    total 
}


fn test_rule<'a>(inp_ra:&[Rule],inp_b:&'a [Byte],inp_r:RuleNum,hist:&mut Vec<RuleNum>) -> Option<&'a [Byte]> {
    let mut c_hist = Vec::new();
    c_hist.push(inp_r);
    for i in &inp_ra[inp_r].test_bytes {
        if inp_b.len() == 0 {
            return Option::None;
        };
        if *i == inp_b[0] {
            return Option::Some(&inp_b[1..]);
        }
        else {
            return Option::None;
        }
    }
    let mut out = Option::None;
    for i in &inp_ra[inp_r].test_rules {
        let mut d_hist = Vec::new();
        let mut tst_b = inp_b;
        for j in i {
            if tst_b.len() == 0 && (*j == 11 || *j == 8) {//don't recurse off the end of the string
                out = Some(tst_b.clone());
                break;
            }
            else {
                if inp_b.len() == 0 { //ending search because we ran out of chars 
                    break;
                };
            };
            out = test_rule(&inp_ra,&tst_b,*j,&mut d_hist);
            tst_b = match out {
                Some(n)=>n,
                None=> {
                    break;
                }
            }
        }
        if out.is_some() {
            c_hist.append(&mut d_hist);
            break; //we've found a match!
        };
    };
    if out.is_some() {
        //if rule 0 FILE if we don't have at least one more 42 tests complete than 31
        if inp_r == 0 {
            let c42 = c_hist.iter().filter(|&n| *n == 42).count();
            let c31 = c_hist.iter().filter(|&n| *n == 31).count();
            if c31 == 0 || c42 <= c31 {//we need both rule 42 and rule 31, but at least one more 42, otherwise fail
                return None;
            }
        }
        hist.append(&mut c_hist);
    };

    out
}

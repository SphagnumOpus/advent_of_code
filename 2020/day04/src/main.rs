use std::io;
use std::env;
use std::collections::HashMap;

//Day 04 2020 for advent of code
//
//
static mut DEBUG: bool = false;
static mut PART1: bool = true;
static mut PART2: bool = false; //assume part 1

#[derive(Debug,PartialEq,Eq,Hash)]
enum FieldCode {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
    Unknown,
}

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
    let parsed_inp = create_parsed(rec_inp); // records into field HashMaps
    unsafe{if DEBUG {println!("Found {} records",parsed_inp.iter().count());}}
    let mut legal_count = 0;

    for i in parsed_inp {
        if is_legal(i) {
            legal_count += 1;
        }
    }
    println!("Found {} legal records",legal_count);






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

fn create_parsed(inp:Vec<String>) -> Vec<HashMap<FieldCode,String>> {
    let mut cur_flds= HashMap::new();   //create HashMap to hold cur fields list
    let mut out = Vec::new();  //create buffer to hold output fields

    for x in inp {
        for y in x.split_ascii_whitespace(){
            let mut parsedfield=y.split(':');
            let fldtype = assign_field(parsedfield.next().unwrap());
            let fldstring = parsedfield.next().unwrap().to_string();
            cur_flds.insert(fldtype,fldstring);
        }
        out.push(cur_flds);
        cur_flds = HashMap::new();
    }
    out
}

fn assign_field(inp:&str) -> FieldCode {
    let mut output =  FieldCode::Unknown;
    match inp {
        "byr" => output = FieldCode::Byr,
        "iyr" => output = FieldCode::Iyr,
        "eyr" => output = FieldCode::Eyr,
        "hgt" => output = FieldCode::Hgt,
        "hcl" => output = FieldCode::Hcl,
        "ecl" => output = FieldCode::Ecl,
        "pid" => output = FieldCode::Pid,
        "cid" => output = FieldCode::Cid,
        _ => (),
    }
    output
}

fn is_legal(inp:HashMap<FieldCode,String>) -> bool {
    let mut part2 = true;
    if part2 {unsafe {part2 = PART2;}}
    let mut return_bool = true;   //be optimistic

    let num_elems = inp.keys().count();
    let mut req_elems = 8;
    if !inp.contains_key(&FieldCode::Cid) {
        req_elems -= 1;
    }
    if num_elems < req_elems { // required elements not found
        return_bool = false;
    }
    else { 
        if part2 {
            //birth year check first
            let mut curstr = inp[&FieldCode::Byr].clone();
            let byrnum:i32 = curstr.parse().unwrap();
            if !curstr.chars().all(char::is_numeric) ||
                curstr.len() != 4 ||
                byrnum < 1920 ||
                byrnum > 2002 {
                        return_bool = false;
            }
            //issue year next
            if return_bool{
                curstr = inp[&FieldCode::Iyr].clone();
                let iyrnum:i32 = curstr.parse().unwrap();
                if !curstr.chars().all(char::is_numeric) ||
                    curstr.len() != 4 ||
                    iyrnum < 2010 ||
                    iyrnum > 2020 {
                            return_bool = false;
                }
            }
            if return_bool {
                //expiration year
                curstr = inp[&FieldCode::Eyr].clone();
                let eyrnum:i32 = curstr.parse().unwrap();
                if !curstr.chars().all(char::is_numeric) ||
                    curstr.len() != 4 ||
                    eyrnum < 2020 ||
                    eyrnum > 2030 {
                        return_bool = false;
                }
            }
            if return_bool {
                //height
                curstr = inp[&FieldCode::Hgt].clone();
                if curstr.contains("cm") {
                    let hgtval:i32 =curstr.replace("cm","  ").trim().parse().unwrap();
                    if hgtval < 150 || hgtval > 193 {
                        return_bool = false;
                    }
                }
                else if curstr.contains("in") {
                    let hgtval:i32 =curstr.replace("in","  ").trim().parse().unwrap();
                    if hgtval < 59 || hgtval > 76 {
                        return_bool = false;
                    }
                }
                else {
                    return_bool = false;
                }

            }

            if return_bool {
                //hair color
                curstr = inp[&FieldCode::Hcl].clone();
                let mut it = curstr.chars();
                if curstr.len() != 7 || it.next().unwrap() != '#' {
                    return_bool = false;
                }
                else {
                    for x in curstr[1..].chars() {
                       if !"0123456789abcdef".contains(x) {
                           return_bool = false;
                       }
                    }
                }
            }
            
            //eye color
            if return_bool {
                curstr = inp[&FieldCode::Ecl].clone();
                let mut ecl_bool = false;
                for x in vec!["amb".to_string(),"blu".to_string(),"brn".to_string(),"gry".to_string(),"grn".to_string(),"hzl".to_string(),"oth".to_string()]{
                    if x == curstr {
                        ecl_bool = true;
                        break;
                    }
                }
                return_bool = ecl_bool;
            }
            
            
            //passport id
            if return_bool {
                curstr = inp[&FieldCode::Pid].clone();
                if curstr.len() != 9 || !curstr.chars().all(char::is_numeric){
                    return_bool = false;
                }
            }
        }
    }

    return_bool
}

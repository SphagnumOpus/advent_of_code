use std::fs::read_to_string;
use std::env;

//Day 04 2024 for advent of code
//
//

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut debug = false;
    let mut filename = String::new();
    if args.len() == 1 {
        println!("No arguments passed");
    }
    for s in args {
        match &s[..] {
            "-DEBUG" => {
                println!("Debug mode");
                debug = true;
            }
            _ => {
                filename=s;
            }
        }
    }
    if debug {
        println!("Using input file {}",filename);
    }


    let mut xmas_sum:i32=0;
//    let mut multiplication_sum_improved:i32=0;
//    let mut do_instruction = true;
//    let mut do_instruction_improved = true;

    let mut chr_input:Vec<Vec<char>> = vec![];
    let mut multi_view:Vec<String> = Vec::new();

    for input in read_to_string(filename).unwrap().lines().map(String::from){    
       chr_input.push(input.chars().collect());   //collect the input into a vec of character array
       multi_view.push(input.clone());  //horizontal
       multi_view.push(input.chars().rev().collect::<String>());  //reverse horizontal
    }
    let vert_input=make_vert(&chr_input);
    for i in vert_input {
        multi_view.push(i.clone());  //verticle
        multi_view.push(i.chars().rev().collect::<String>());  //reverse verticle
    }
    let diag_slope_input=make_diag(&mut chr_input,false);
    for i in diag_slope_input {
        multi_view.push(i.clone());  //diagonal slope up
        multi_view.push(i.chars().rev().collect::<String>());  //reverse diagonals
    }
    let diag_slope_input=make_diag(&mut chr_input.clone().into_iter().rev().collect(),false);   //reverse the rows before doing diags... opposite diagonal
    for i in diag_slope_input {
        multi_view.push(i.clone());  //diagonal slope down
        multi_view.push(i.chars().rev().collect::<String>());  //reverse down diagonals
    }

    for i in multi_view {
        xmas_sum+=i.match_indices("XMAS").count() as i32;
    }

    println!("Part 1 answer (XMAS entries found) equals {}",xmas_sum);
    multi_view=Vec::new();
    //let diag_slope_input=make_diag(&mut chr_input,true);
    make_diag(&mut chr_input,true);
    let diag_slope_input=make_diag(&mut chr_input.into_iter().rev().collect(),false);   //reverse the rows to look for opposite diagonal 
    for i in diag_slope_input {
        multi_view.push(i.clone());  //diagonal slope down
        multi_view.push(i.chars().rev().collect::<String>());  //reverse down diagonals
    }
    xmas_sum=0;
    for i in multi_view {
        xmas_sum+=i.match_indices("M.S").count() as i32;
    }

    println!("Part 2 answer (number of X-MAS entries found) equals {}",xmas_sum);
}


fn make_vert(inp:&Vec<Vec<char>>) -> Vec<String> {
    let mut output:Vec<String> = Vec::new();
    for i in 0..inp[0].len() {
        let mut tmp_chr_vec:Vec<char>=Vec::new();
        for j in 0..inp.len() {
            tmp_chr_vec.push(inp[j][i]);
        }
        output.push(tmp_chr_vec.into_iter().collect());
    }
    output
}

fn make_diag(inp:&mut Vec<Vec<char>>,ispart2:bool) -> Vec<String> {
    let mut output:Vec<String> = Vec::new();
    for i in 0..inp.len()*2 {
        let mut startj = 0;
        if i>=inp.len() {
            startj=i-inp.len()+1;
        }
        let mut tmp_chr_vec:Vec<char>=Vec::new();
        for j in startj..inp[0].len(){
            if i>=j {
                tmp_chr_vec.push(inp[i-j][j]);            
                if ispart2 {
                    if tmp_chr_vec.len() >=3{
                        let look4it:String=tmp_chr_vec.clone().into_iter().rev().take(3).collect();
                        if look4it=="SAM" || look4it=="MAS"{
                            inp[i-(j-1)][j-1]='.';
                        }
                    }
                }
            }
        }
        output.push(tmp_chr_vec.into_iter().collect());
        
    }
    output
}

use std::io;
use std::env;

//Day 03 2020 for advent of code
//
//

//enum Spot {
//    Snow,
//    Tree,
//    ImOnSnow,
//    ImOnTree,
//}



struct Treeline {
    buffer: String,   //original input line
}

struct Slope {
    right:i32,
    down:i32,
}



fn main() {
    let args: Vec<String> = env::args().collect();

    let mut debug = false;
    let mut part1 = false;
    let mut part2 = true; //assume part 2

    if args.len() == 1 {
        println!("No arguments passed");
    }
    for s in args {
        match &s[..] {
            "-DEBUG" => {
                println!("Debug mode");
                debug = true;
            }
            "-PART1" => {
                println!("Part1 mode explicitly set");
                part1 = true;
            }
            "-PART2" => {
                println!("Part2 mode explicitly set");
                part2 = true;
            }
            _ => {
            }
        }
    }

    let mut vecbuffer = Vec::new(); //Vec to hold our data as Treeline structs
    let mut input = String::new();
    let mut width = 0;  //width of the input matrix... shouldn't vary 
    let mut curwidth = 0;   //width of current line
    let mut first_input_row = true;
    let mut trees_hit = 0;


    while 0<io::stdin().read_line(&mut input).expect("I/O error on read"){
        curwidth= input.chars().count() - 1;//size of this line 
        if first_input_row {
            width = curwidth;
            first_input_row = false;
        }
        assert_eq!(width,curwidth, "width changed during input");

        let cur_row = Treeline {
            buffer: input.clone(),
        };
        vecbuffer.push(cur_row);
        input = String::new();
    }


    let mut slope_vec = Vec::new(); 
    if part1 {
        slope_vec.push(Slope{right:3, down:1});
    }
    if part2 {
        slope_vec.push(Slope{right:1, down:1});
        slope_vec.push(Slope{right:3, down:1});
        slope_vec.push(Slope{right:5, down:1});
        slope_vec.push(Slope{right:7, down:1});
        slope_vec.push(Slope{right:1, down:2});
    }

    let mut product:i64 = 1; 
    for cslope in slope_vec {
        let mut bufrow = 0; // row in the buffer
        let mut crow = 0;  //row we're on.  we'll increment as we travel "down"
        let mut ccol = 0; // col we're on.

        for x in vecbuffer.iter() {
            if bufrow == crow {
                    let col_in_matrix = (ccol as usize) % width;
                if debug {
                    println!("{} {} {}",x.buffer[..curwidth].to_string(), 
                             ccol,col_in_matrix);
                }
                let cur_sym:char = x.buffer.chars().nth(col_in_matrix).unwrap();
                match cur_sym {
                    '#' => trees_hit = trees_hit + 1,
                    _ => if debug {println!("no tree at row {} {}",crow, cur_sym.to_string())},
                }


            crow = crow + cslope.down;
            ccol = ccol + cslope.right;
            }
            bufrow = bufrow + 1;
        }
        product = product * trees_hit;
        println!("trees_hit = {} product = {}", trees_hit, product);
        trees_hit = 0;
    }


}


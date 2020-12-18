use std::io;
use std::convert::TryInto;

//Day 17 2020 for advent of code
//
//
//

static PD_SIZE:isize = 25;

type Byte = u8;
type PocketDim = Vec<Vec<Vec<Byte>>>;
type PocketDim4d = Vec<Vec<Vec<Vec<Byte>>>>;


fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let init_pocket_dim = build_pocket_dim(&orig_inp);
    let part1_answer = calc_part1(&init_pocket_dim);
    println!("The part 1 answer is {}",part1_answer);
    let init_pocket_dim_4d = build_pocket_dim_4d(&orig_inp);
    let part2_answer = calc_part2(&init_pocket_dim_4d);
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

fn calc_part1(inp:&PocketDim) ->usize {
    let mut out = 0;
    let mut tmp_pd = do_cycle(&inp);
    for _i in 0..5 {
        tmp_pd=do_cycle(&tmp_pd)
    }
    //print_pocket_dim(&inp);
    //print_pocket_dim(&tmp_pd);
    for i in tmp_pd {
        for j in i {
            out += j.iter().filter(|&n| *n==b'#').count();
        }
    }

    out
}

fn calc_part2(inp:&PocketDim4d) ->usize {
    let mut out = 0;
    let mut tmp_pd = do_cycle_4d(&inp);
    for _i in 0..5 {
        tmp_pd=do_cycle_4d(&tmp_pd)
    }
    for i in tmp_pd {
        for j in i {
            for l in j {
                out += l.iter().filter(|&n| *n==b'#').count();
            }
        }
    }

    out
}


fn build_pocket_dim(inp_s:&Vec<String>) -> PocketDim {
        
    //create an array that represents one dim row of dots.
//    let x_row:Vec<Byte> = vec![b'.';(PD_SIZE.try_into().unwrap())];  
//    let y_surface:Vec<Vec<Byte>> = vec![x_row;PD_SIZE as usize];
//    let mut out:PocketDim = vec![y_surface;PD_SIZE as usize];
    let mut out = build_new_pocket_dim();
    // we're counting DOWN in these dimensions... 0,0,0 represents closest upper left corner of
    // cube.  So let's overlay # in proper spots.  Also, rust vecs funcs work best with z,y,x
    // layout.
    //
    //Start input so it's in the middle of the dimensional cube
    let start_at_z = (PD_SIZE as usize) / 2;
    let start_at_xy= start_at_z-(inp_s[0].len()/2);
    for (idx_i, i) in inp_s.iter().enumerate() {
        for (idx_j,j) in i.bytes().enumerate() {
            out[start_at_z][start_at_xy+idx_i][start_at_xy+idx_j] = j;
        }
    }
    //println!("out = {:?}",out);
    out
}

fn build_new_pocket_dim() -> PocketDim {
        
    //create an array that represents one dim row of dots.
    let x_row:Vec<Byte> = vec![b'.';PD_SIZE.try_into().unwrap()];  
    let y_surface:Vec<Vec<Byte>> = vec![x_row;PD_SIZE as usize];
    let out:PocketDim = vec![y_surface;PD_SIZE as usize];
    // we're counting DOWN in these dimensions... 0,0,0 represents closest upper left corner of
    // cube.  So let's overlay # in proper spots.  Also, rust vecs funcs work best with z,y,x
    // layout.
    //
    out
}

fn build_pocket_dim_4d(inp_s:&Vec<String>) -> PocketDim4d {
        
    //create an array that represents one dim row of dots.
    let mut out = build_new_pocket_dim_4d();
    // we're counting DOWN in these dimensions... 0,0,0 represents closest upper left corner of
    // cube.  So let's overlay # in proper spots.  Also, rust vecs funcs work best with z,y,x
    // layout.
    //
    //Start input so it's in the middle of the dimensional cube
    let start_at_wz = (PD_SIZE as usize) / 2;
    let start_at_xy= start_at_wz-(inp_s[0].len()/2);
    for (idx_i, i) in inp_s.iter().enumerate() {
        for (idx_j,j) in i.bytes().enumerate() {
            out[start_at_wz][start_at_wz][start_at_xy+idx_i][start_at_xy+idx_j] = j;
        }
    }
    //println!("out = {:?}",out);
    out
}

fn build_new_pocket_dim_4d() -> PocketDim4d {
        
    //create an array that represents one dim row of dots.
    let x_row:Vec<Byte> = vec![b'.';PD_SIZE.try_into().unwrap()];  
    let y_surface:Vec<Vec<Byte>> = vec![x_row;PD_SIZE as usize];
    let z_space:Vec<Vec<Vec<Byte>>> = vec![y_surface;PD_SIZE as usize];
    let out:PocketDim4d = vec![z_space;PD_SIZE as usize];
    // we're counting DOWN in these dimensions... 0,0,0 represents closest upper left corner of
    // cube.  So let's overlay # in proper spots.  Also, rust vecs funcs work best with z,y,x
    // layout.
    //
    out
}

// This print function was only written for testing, and only works for 3D version (part 1)
//fn print_pocket_dim(inp_pd:&PocketDim) {
//    for i in 0..(PD_SIZE as usize){
//        if true {//theres stuff in this z {
//            let print_z:isize=(PD_SIZE/2) - (i as isize);
//            println!("z = {}",print_z);
//            for j in 0..(PD_SIZE as usize) {
//                let ptr=String::from_utf8_lossy(&inp_pd[i][j]);
//                println!("{}",ptr);
//            }
//        }
//    }
//}

fn do_cycle(inp:&PocketDim) -> PocketDim{
    let mut out = build_new_pocket_dim();
    for i_idx in 0..PD_SIZE as usize {
        for j_idx in 0..PD_SIZE as usize {
            for k_idx in 0..PD_SIZE as usize {
                out[i_idx][j_idx][k_idx]=eval_cell(&inp,i_idx,j_idx,k_idx);
            }
        }
    }
    out
}
fn eval_cell(inp_pd:&PocketDim,inp_z:usize,inp_y:usize,inp_x:usize) -> u8{
    let mut out = b'.';
    if (1..(PD_SIZE-1) as usize).contains(&inp_z) && (1..(PD_SIZE-1) as usize).contains(&inp_y) && (1..(PD_SIZE-1) as usize).contains(&inp_x) {
        //count the neighbors 
        let mut neighbors = 0;
        for z_idx in inp_z-1..=inp_z+1 {
            for y_idx in inp_y-1..=inp_y+1 {
                for x_idx in inp_x-1..=inp_x+1 {
                    if inp_pd[z_idx][y_idx][x_idx]==b'#' {
                        neighbors+=1;
                    }
                }
            }
        }
        let current = inp_pd[inp_z][inp_y][inp_x];
        if current==b'#' {
            neighbors-=1;  //don't count the cube being evaluated!
        }
        out= match current {
            b'#' => {
                if (2..=3).contains(&neighbors) {
                    b'#'
                }
                else {
                    b'.'
                }
            },
            b'.' => {
                if neighbors == 3 {
                    b'#'
                }
                else {
                    current
                }
            },
            _ => panic!("error... we should never find something other than . or #")
        }
    }
    out
}

fn do_cycle_4d(inp:&PocketDim4d) -> PocketDim4d{
    let mut out = build_new_pocket_dim_4d();
    for i_idx in 0..PD_SIZE as usize {
        for j_idx in 0..PD_SIZE as usize {
            for k_idx in 0..PD_SIZE as usize {
                for l_idx in 0..PD_SIZE as usize {
                    out[i_idx][j_idx][k_idx][l_idx]=eval_cell_4d(&inp,i_idx,j_idx,k_idx,l_idx);
                }
            }
        }
    }
    out
}
fn eval_cell_4d(inp_pd:&PocketDim4d,inp_w:usize,inp_z:usize,inp_y:usize,inp_x:usize) -> u8{
    let mut out = b'.';
    if (1..(PD_SIZE-1) as usize).contains(&inp_w)
        && (1..(PD_SIZE-1) as usize).contains(&inp_z) 
            && (1..(PD_SIZE-1) as usize).contains(&inp_y) 
                && (1..(PD_SIZE-1) as usize).contains(&inp_x) {
        //count the neighbors 
        let mut neighbors = 0;
        for w_idx in inp_w-1..=inp_w+1 {
            for z_idx in inp_z-1..=inp_z+1 {
                for y_idx in inp_y-1..=inp_y+1 {
                    for x_idx in inp_x-1..=inp_x+1 {
                        if inp_pd[w_idx][z_idx][y_idx][x_idx]==b'#' {
                            neighbors+=1;
                        }
                    }
                }
            }
        }
        let current = inp_pd[inp_w][inp_z][inp_y][inp_x];
        if current==b'#' {
            neighbors-=1;  //don't count the cube being evaluated!
        }
        out= match current {
            b'#' => {
                if (2..=3).contains(&neighbors) {
                    b'#'
                }
                else {
                    b'.'
                }
            },
            b'.' => {
                if neighbors == 3 {
                    b'#'
                }
                else {
                    current
                }
            },
            _ => panic!("error... we should never find something other than . or #")
        }
    }
    out
}

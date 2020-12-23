use std::io;
use std::collections::HashMap;

//Day 20 2020 for advent of code
//
//
//
type Edge = u32;
type Tile = usize;

//#[derive(Clone,Debug)]
//struct Rule {
//    test_bytes: Vec<Byte>,
//    test_rules: Vec<Vec<RuleNum>>
//}



fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let part1_answer = calc_part_1(&orig_inp);
    println!("The part 1 answer is {}",&part1_answer);

    let part2_answer = calc_part_2(&orig_inp);
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
fn calc_part_1(inp:&Vec<String>) -> Tile{
    let mut normal:HashMap<Edge,Tile> = HashMap::new();
    let mut reverse:HashMap<Edge,Tile> = HashMap::new();
    let mut line = 0; //counter to current line in input
    while line < inp.len() {
        //read the tile num
        let tile:Tile = inp[line][5..9].parse().unwrap();
        line += 1;
        //create array of edges
        let mut edge_list:Vec<Edge> = vec![0;4];
        //parse the top line first
        for (idx_i,i) in inp[line].chars().enumerate() {
            edge_list[0] = edge_list[0] | char_to_bit(i) << 9-idx_i;
        };
        //now read through the ten rows to get left and right edges (we're counting clockwise so 1
        //is right, 3 is left
        for i in 0..10 {
            edge_list[1] = (edge_list[1] << 1) | char_to_bit(inp[line].chars().last().unwrap()); 
            edge_list[3] = edge_list[3]  | (char_to_bit(inp[line].chars().next().unwrap())<<i); 
            line += 1; //skip to the next line
        };
        //now read the bottom edge into edge_list[2]
        for (idx_i,i) in inp[line-1].chars().enumerate() {
            edge_list[2] = edge_list[2] | char_to_bit(i) << idx_i;
        };

        //now insert these edges into the normal and reverse hashmaps (only if other edge not
        //already there)
        //
        //if other edge is there, delete it
        for i in 0..4 {
            normal.insert (edge_list[i],tile);
            let r = bit_reverse(&edge_list[i]);
            // look for the reverse of this edge and remove it if it's there - we've matched
            // otherwise add this entry to reverse awaiting match with later processed edge
            match reverse.remove_entry(&r) {
                Some(_n) => {
                    reverse.remove_entry(&edge_list[i]);  //remove the other reversed entry for this pair
                    0
                }
                None => {
                    reverse.insert(r,tile);
                    reverse.insert(edge_list[i],tile);
                    0
                }
            };
        };
        line += 1;   //skip the blank line
    };
    //Okay... we've processed all tiles and have a list of unmatched edged in reverse - look for
    //tile numbers that show up four times (two edges, each with their reverse)... those are the corners
    let mut pic_edges_tile_numbers:Vec<Tile> = Vec::new();
    for (_idx_i,i) in reverse.iter() {
        pic_edges_tile_numbers.push(*i);
    };
    pic_edges_tile_numbers.sort();
    let mut prv = 0;
    let mut count = 0;
    let mut corner_list:Vec<Tile> = Vec::new();
    for (idx_i,i) in pic_edges_tile_numbers.iter().enumerate() {
        if pic_edges_tile_numbers[idx_i] == prv {
            count += 1;
        }
        else {
            count = 0;
        };
        if count == 3 {
            corner_list.push(*i);
        }
        prv = *i;
    }
    let out = corner_list.iter().fold(1,|n,&v| n * v);

    out
}

fn calc_part_2(_inp:&Vec<String>) -> Tile {
    0
}

fn bit_reverse(inp:&Edge) -> Edge {
    let mut out = 0;
    for i in 0..10 {
        out = out | ((inp & 1<<i)>>i)<<(9-i);
    }
    out
}

fn char_to_bit (inp: char) -> Edge {
    match inp {
        '#' => 1,
        '.' => 0,
        _   => panic!("invalid pic element {}",inp)
    }
}


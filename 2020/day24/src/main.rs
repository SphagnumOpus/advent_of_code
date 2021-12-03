use std::io;
use std::collections::HashMap;

//Day 24 2020 for advent of code
//
// Added a one line change to test my VS Code GIT setup∏
//
//
//
type TileCoordinate = (i32,i32);
#[derive (Clone,Debug)]
enum Direction {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest
}
enum DirectionPrefix {
    North,
    South,
    None
}

type FinalAnswerPart1 = u32;
type FinalAnswerPart2 = u32;
static NEIGHBOR_ARRAY:[TileCoordinate;6] = [(1,0),(0,1),(-1,1),(-1,0),(0,-1),(1,-1)];


fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let this_directions_list = build_this_directions_list(&orig_inp);
    let this_tile_map = build_this_tile_map(&this_directions_list);
    let part1_answer = calc_part_1(&this_tile_map);
    println!("The part 1 answer is {}",&part1_answer);
    let part2_answer = calc_part_2(&this_tile_map);
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
fn build_this_directions_list(inp:&Vec<String>) -> Vec<Vec<Direction>> {
    let mut out = Vec::new();
    for i in inp { //we only expect one line
        let mut movements=Vec::new();
        let mut previous = DirectionPrefix::None;
        for j in i.split("") {
            movements.push(match previous {
                DirectionPrefix::North => match j {
                    "e"=> Direction::NorthEast,
                    "w"=> Direction::NorthWest,
                    _=> panic!("unrecognized input... North what?")
                },
                DirectionPrefix::South => match j {
                    "e"=> Direction::SouthEast,
                    "w"=> Direction::SouthWest,
                    _=> panic!("unrecognized input... South what?"),
                },
                DirectionPrefix::None => match j {
                    "e"=>Direction::East,
                    "w"=>Direction::West,
                    "n"=>  {previous = DirectionPrefix::North;
                        continue;
                    },
                    "s"=>  {previous = DirectionPrefix::South;
                        continue;
                    },
                    "" => {
                        continue;
                    },
                    _=> panic!("unrecognized input... {}  huh? what?",j),
                }
            });
            previous = DirectionPrefix::None;
        }
        out.push(movements);
    }
    out
}
fn build_this_tile_map(inp:&Vec<Vec<Direction>>) -> HashMap<TileCoordinate,bool> {
    let mut out = HashMap::new();
    for i in inp {
        let mut coordinate:TileCoordinate = (0,0);  //start at the origin
        for j in i {
            coordinate = add_coordinate (coordinate, match j {
                Direction::NorthEast => (1 as i32,-1 as i32),
                Direction::East      => (1 as i32,0 as i32),
                Direction::SouthEast => (0 as i32,1 as i32),
                Direction::SouthWest => (-1 as i32,1 as i32),
                Direction::West      => (-1 as i32,0 as i32),
                Direction::NorthWest => (0 as i32,-1 as i32)
            });
        }
        let _result = match insert_tile_info(&mut out,coordinate,true) {  //try to insert the tile coordinate
            //this means the tile coordinate was already inserted .. it it was already true, set
            //back to false
            Some(x) => {if x {
                    out.insert(coordinate,!x)   //don't need to call insert_tile_info... it's just been called for this coordinate
                }
                else {
                    None
                }
            },
            None => None,
            };
    }
    out
}



fn calc_part_1(inp:&HashMap<TileCoordinate,bool>) ->FinalAnswerPart1{
    let out = inp.iter().filter(|(_x,y)| **y).count() as FinalAnswerPart1;
   
    out
}

fn calc_part_2(inp:&HashMap<TileCoordinate,bool>) ->FinalAnswerPart2{
    let mut copy = inp.clone();
    for _i in 0..100 {
        copy = apply_rules(&copy);
        //println!("Day {}: {}",_i+1, copy.iter().filter(|(_x,y)| **y).count() as FinalAnswerPart2);
    }
    let out = copy.iter().filter(|(_x,y)| **y).count() as FinalAnswerPart2;
    out 
}

fn add_coordinate(inpx:TileCoordinate,inpy:TileCoordinate) -> TileCoordinate {
    (inpx.0+inpy.0,inpx.1+inpy.1)
}

fn apply_rules(inp:&HashMap<TileCoordinate,bool>) -> HashMap<TileCoordinate,bool> {
    let mut out = inp.clone();
    let mut update_list=Vec::new();
    for (key_i,i) in out.iter() {
        let mut count = 0;
        for j in &NEIGHBOR_ARRAY {
            count += match out.get(&add_coordinate(*key_i,*j)) {
                Some(n) => match n {
                    true =>1,
                    false => 0
                },
                None  => 0
            }
        }
        let should_flip = match *i {
            true => match count {
                1 | 2 => false,
                0 | _ => true
            },
            false => match count {
                2 => true,
                _ => false
            }
        };
        if should_flip {
            update_list.push((*key_i,!*i));
        }
    }
    for i in update_list {
        insert_tile_info(&mut out,i.0,i.1);
    }
    out
}

fn insert_tile_info(inp_hm:&mut HashMap<TileCoordinate,bool>,inp_c:TileCoordinate,inp_b: bool) -> Option<bool> {
    if inp_b { 
        for j in &NEIGHBOR_ARRAY  {  //make sure every neighbor of a black tile exists in the tile map
                if !inp_hm.contains_key(&add_coordinate(inp_c,*j)) {
                    inp_hm.insert(add_coordinate(inp_c,*j),false);
                }
        }
    };
    inp_hm.insert(inp_c,inp_b)
}

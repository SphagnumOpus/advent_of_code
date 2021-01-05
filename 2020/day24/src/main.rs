use std::io;
use std::collections::HashMap;

//Day 24 2020 for advent of code
//
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
        let _result = match out.insert(coordinate,true) {  //try to insert the tile coordinate
            //this means the tile coordinate was already inserted .. it it was already true, set
            //back to false
            Some(x) => {if x {
                    out.insert(coordinate,!x)
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
    let out = inp.iter().filter(|(x,y)| **y).count() as FinalAnswerPart1;
   
    out
}

fn calc_part_2(inp:&HashMap<TileCoordinate,bool>) ->FinalAnswerPart2{
   let out = 0;
   out 
}

fn add_coordinate(inpx:TileCoordinate,inpy:TileCoordinate) -> TileCoordinate {
    (inpx.0+inpy.0,inpx.1+inpy.1)
}



            


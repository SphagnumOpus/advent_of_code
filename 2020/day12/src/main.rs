use std::io;

//Day 12 2020 for advent of code
//
//
#[derive(Clone,Copy)]
enum WhichPart{
    Part1,
    Part2
}

enum Movement {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize)
}

#[derive(Clone,Copy)]
struct Coordinate {
    x:isize,
    y:isize
}

#[derive(Clone,Copy)]
struct FerryStatus {
    heading:isize,  //direction expressed in degrees clockwise, north =0
    position:Coordinate
}





fn main() {

    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let mvmts_vec = build_mvmts_vec(&orig_inp);  //translate to vec of movements
    let initial_status = FerryStatus{
        heading : 90,
        position : Coordinate {
            x:0,
            y:0}
    };
    let final_status = mvmts_vec.iter()
                                .fold(initial_status,|acc,a| apply_mvmt(&acc,&*a));
    let part1_answer = manhattan_offset(&initial_status,&final_status);

    println!("The part 1 answer is {}",part1_answer);

    let part2_answer =  0;
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

fn build_mvmts_vec(inp:&Vec<String>) ->Vec<Movement>{
    let mut out = Vec::new();
    for i in inp  {
        let value = i[1..].parse::<isize>().expect("Error, bad input string");
        let cur_mvmt = match i.chars().next().expect("Error - empty input string"){
        'N' => Movement::North(value),
        'S' => Movement::South(value),
        'E' => Movement::East(value),
        'W' => Movement::West(value),
        'L' => Movement::Left(value),
        'R' => Movement::Right(value),
        'F' => Movement::Forward(value),
        _ => panic!("Error, unrecognized movement"),
        };
        out.push(cur_mvmt);
    }
    out
}

//Take a FerryStatus, apply a movement and return a new FerryStatus        
fn apply_mvmt(inps:&FerryStatus,inpm:&Movement) -> FerryStatus{
//    let out=*inps; //create the output FerryStatus struct
    let out =match inpm {
        Movement::North(n)   => FerryStatus{
                                    heading:inps.heading,
                                    position:Coordinate{
                                        x:inps.position.x+n,
                                        y:inps.position.y
                                        }
                                },
        Movement::South(n)   => FerryStatus{
                                    heading:inps.heading,
                                    position:Coordinate{
                                        x:inps.position.x-n,
                                        y:inps.position.y
                                        }
                                },
        Movement::East(n)    => FerryStatus{
                                    heading:inps.heading,
                                    position:Coordinate{
                                        x:inps.position.x,
                                        y:inps.position.y+n
                                        }
                                },
        Movement::West(n)    => FerryStatus{
                                    heading:inps.heading,
                                    position:Coordinate{
                                        x:inps.position.x,
                                        y:inps.position.y-n
                                        }
                                },
        Movement::Left(n)    => FerryStatus{
                                    heading:(inps.heading+360-n)%360,
                                    position:inps.position,
                                },
        Movement::Right(n)   => FerryStatus{
                                    heading:(inps.heading+n)%360,
                                    position:inps.position,
                                },
        Movement::Forward(n) => apply_mvmt(&inps,&match inps.heading { 0  =>Movement::North(*n),
                                                                       90 =>Movement::East(*n),
                                                                       180=>Movement::South(*n),
                                                                       270=>Movement::West(*n),
                                                                       _  =>panic!("Insanity!{}",inps.heading)}
                                                                           )
    };

    out
}

fn manhattan_offset(inp1:&FerryStatus,inp2:&FerryStatus) -> isize {

    return (inp2.position.x-inp1.position.x).abs() + (inp2.position.y-inp1.position.y).abs();
}

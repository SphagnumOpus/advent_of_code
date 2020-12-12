use std::io;

//Day 12 2020 for advent of code
//
//
enum Movement {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize)
}

#[derive(Clone,Copy,Debug)]
struct Coordinate {
    x:isize,
    y:isize
}

#[derive(Clone,Copy)]
struct FerryStatus {
    heading:isize,  //direction expressed in degrees clockwise, north =0
    position:Coordinate,
    wp_position:Coordinate,
}





fn main() {

    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let mvmts_vec = build_mvmts_vec(&orig_inp);  //translate to vec of movements
    let initial_status = FerryStatus{
        heading : 90,
        position : Coordinate {
            x:0,
            y:0},
        wp_position : Coordinate {
            x:1,
            y:10}
    };
    let final_status = mvmts_vec.iter()
                                .fold(initial_status,|acc,a| apply_mvmt_pt1(&acc,&*a));
    let part1_answer = manhattan_offset(&initial_status,&final_status);
    println!("The part 1 answer is {}",part1_answer);

    let final_status = mvmts_vec.iter()
                                .fold(initial_status,|acc,a| apply_mvmt_pt2(&acc,&*a));
    let part2_answer = manhattan_offset(&initial_status,&final_status);
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
fn apply_mvmt_pt1(inps:&FerryStatus,inpm:&Movement) -> FerryStatus{
//    let out=*inps; //create the output FerryStatus struct
    let out =match inpm {
        Movement::North(n)   => FerryStatus{
                                    heading:inps.heading,
                                    position:Coordinate{
                                        x:inps.position.x+n,
                                        y:inps.position.y
                                        },
                                    wp_position:inps.wp_position,
                                },
        Movement::South(n)   => FerryStatus{
                                    heading:inps.heading,
                                    position:Coordinate{
                                        x:inps.position.x-n,
                                        y:inps.position.y
                                        },
                                    wp_position:inps.wp_position,
                                },
        Movement::East(n)    => FerryStatus{
                                    heading:inps.heading,
                                    position:Coordinate{
                                        x:inps.position.x,
                                        y:inps.position.y+n
                                        },
                                    wp_position:inps.wp_position,
                                },
        Movement::West(n)    => FerryStatus{
                                    heading:inps.heading,
                                    position:Coordinate{
                                        x:inps.position.x,
                                        y:inps.position.y-n
                                        },
                                    wp_position:inps.wp_position,
                                },
        Movement::Left(n)    => FerryStatus{
                                    heading:(inps.heading+360-n)%360,
                                    position:inps.position,
                                    wp_position:inps.wp_position,
                                },
        Movement::Right(n)   => FerryStatus{
                                    heading:(inps.heading+n)%360,
                                    position:inps.position,
                                    wp_position:inps.wp_position,
                                },
        Movement::Forward(n) => apply_mvmt_pt1(&inps,&match inps.heading { 0  =>Movement::North(*n),
                                                                       90 =>Movement::East(*n),
                                                                       180=>Movement::South(*n),
                                                                       270=>Movement::West(*n),
                                                                       _  =>panic!("Insanity!{}",inps.heading)}
                                                                           )
    };

    out
}

fn apply_mvmt_pt2(inps:&FerryStatus,inpm:&Movement) -> FerryStatus{
//    let out=*inps; //create the output FerryStatus struct
    let out =match inpm {
        Movement::North(n)   => FerryStatus{
                                    heading:inps.heading,
                                    position:inps.position,
                                    wp_position:Coordinate{
                                        x:inps.wp_position.x+n,
                                        y:inps.wp_position.y
                                        },
                                },
        Movement::South(n)   => FerryStatus{
                                    heading:inps.heading,
                                    position:inps.position,
                                    wp_position:Coordinate{
                                        x:inps.wp_position.x-n,
                                        y:inps.wp_position.y
                                        },
                                },
        Movement::East(n)    => FerryStatus{
                                    heading:inps.heading,
                                    position:inps.position,
                                    wp_position:Coordinate{
                                        x:inps.wp_position.x,
                                        y:inps.wp_position.y+n
                                        },
                                },
        Movement::West(n)    => FerryStatus{
                                    heading:inps.heading,
                                    position:inps.position,
                                    wp_position:Coordinate{
                                        x:inps.wp_position.x,
                                        y:inps.wp_position.y-n
                                        },
                                },
        Movement::Left(n)    => FerryStatus{
                                    heading:inps.heading,
                                    position:inps.position,
                                    wp_position:rot_right(&inps.position,&inps.wp_position,360-n),
                                },
        Movement::Right(n)   => FerryStatus{
                                    heading:inps.heading,
                                    position:inps.position,
                                    wp_position:rot_right(&inps.position,&inps.wp_position,*n),
                                },
        Movement::Forward(n) => {
                                    let xoff=inps.position.x-inps.wp_position.x;
                                    let yoff=inps.position.y-inps.wp_position.y;
                                    FerryStatus{
                                        heading:inps.heading,
                                        position:Coordinate{
                                            x:inps.position.x+xoff*n,
                                            y:inps.position.y+yoff*n,
                                            },
                                        wp_position:Coordinate{
                                            x:inps.wp_position.x+xoff*n,
                                            y:inps.wp_position.y+yoff*n,
                                            },
                                    }
                                },
    };

    out
}

fn manhattan_offset(inp1:&FerryStatus,inp2:&FerryStatus) -> isize {

    return (inp2.position.x-inp1.position.x).abs() + (inp2.position.y-inp1.position.y).abs();
}
fn rot_right(ipos:&Coordinate,iwp_pos:&Coordinate,inpr:isize) -> Coordinate {

    let mut tmp_c = Coordinate{
                        x: iwp_pos.x-ipos.x,
                        y: iwp_pos.y-ipos.y,
                    };
    let mut degree_counter = inpr;
    assert_eq!(degree_counter > 0,true);
    while degree_counter > 0 {  //to rotate right, we transpose x,y and set x to negative 
        let holder=tmp_c.x;
        tmp_c.x = -1 * tmp_c.y;
        tmp_c.y = holder;
        degree_counter -= 90;
    }
    let tmp_c = Coordinate{
                    x: ipos.x+tmp_c.x,
                    y: ipos.y+tmp_c.y,
                };
    tmp_c
}


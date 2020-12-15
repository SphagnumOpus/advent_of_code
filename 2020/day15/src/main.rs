use std::io;
use std::collections::HashMap;

//Day 15 2020 for advent of code
//
//
//


#[derive (Debug)]
struct Game {
    move_tally: Vec<usize>,
    when_last_spoken: HashMap<usize,usize>,
}




fn main() {

    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let int_vec = build_int_vec(&orig_inp);
    
    let part1_answer = calc_part1(&int_vec);
    println!("The part 1 answer is {}",part1_answer);

    let part2_answer = calc_part2(&int_vec);
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

fn calc_part1(inp:&Vec<usize>) ->usize {
    let nums_spoken = 2020;
    let mut this_game=build_game(&inp);
    for _i in 0..nums_spoken-inp.len() {
        play_game(&mut this_game);
    }
    this_game.move_tally[nums_spoken-1]
}


fn calc_part2(inp:&Vec<usize>) ->usize {
    let nums_spoken = 30000000;
    let mut this_game=build_game(&inp);
    for _i in 0..nums_spoken-inp.len() {
        play_game(&mut this_game);
    }
    this_game.move_tally[nums_spoken-1]
}


fn build_int_vec(inp:&Vec<String>) ->Vec<usize> {
    let mut out:Vec<usize> = Vec::new();
    for x in inp{
        for y in x.split(','){
            let thisnum:usize=y.parse().expect("error, not a number in input");
            out.push(thisnum);
        }
    }
    out
}

fn build_game(inp:&Vec<usize>) -> Game {
    let mut outgame = Game{
        move_tally: Vec::new(),
        when_last_spoken: HashMap::new(), 
    };
    let inplen = inp.len();
    for (i_idx, i) in inp.iter().enumerate() {
        outgame.move_tally.push(*i); //add this num as the next spoken
        if i_idx < (inplen - 1) { //add to hashmap except last one, which will be processed when game is played
            outgame.when_last_spoken.insert(*i, i_idx+1);
        }
    }
    outgame
}

fn play_game(inp:&mut Game) {   // pass in the Game struct - last move listed in tally is expect to be unprocessed
    let moves_so_far = inp.move_tally.len();
    let latest_num_spoken= inp.move_tally[moves_so_far-1];
    let next_num = match inp.when_last_spoken.insert(latest_num_spoken,moves_so_far) {
        Some(n) => moves_so_far - n,
        None  => 0,
        };
    inp.move_tally.push(next_num);  
}

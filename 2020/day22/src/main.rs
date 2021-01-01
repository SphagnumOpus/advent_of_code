use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::VecDeque;

//Day 22 2020 for advent of code
//
//
//
//
#[derive (Clone,Debug)]
struct CardGame {
    player1: VecDeque<Card>,
    player2: VecDeque<Card>,
}
type Card = u32;
type Score = u32;
type FinalAnswer = u32;
type NumberOfRounds = u32;



fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let this_card_game = build_this_card_game(&orig_inp);
    let part1_answer = calc_part_1(&this_card_game);
    println!("The part 1 answer is {}",&part1_answer);
    let part2_answer = calc_part_2(&this_card_game);
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
fn build_this_card_game(inp:&Vec<String>) -> CardGame {
    let mut out = CardGame {
        player1: VecDeque::new(),
        player2: VecDeque::new()
    };
    let mut which_player=&mut out.player1;

    for i in inp {
        if i == "Player 1:" {
            which_player=&mut out.player1;
            continue;
        }
        else if i == "Player 2:" {
            which_player=&mut out.player2;
            continue;
        }
        else if i == "" {
            continue;
        }
        let value:Card=i.parse().unwrap();
        (*which_player).push_back(value);
    }

    out
}

fn calc_part_1(inp:&CardGame) ->FinalAnswer{
    let mut clone = (*inp).clone();
    let (out,_number_of_rounds) = play_game(&mut clone);
    out
}
fn calc_part_2(inp:&CardGame) ->FinalAnswer{
0
}

fn play_game(inp:&mut CardGame) -> (Score,NumberOfRounds) {
    let mut counter=0;
    while (inp.player1.len()>0) && (inp.player2.len()>0) {
        let (mut winner,loser) = match inp.player1.front().unwrap() > inp.player2.front().unwrap() {
            true =>  (&mut inp.player1,&mut inp.player2),
            false => (&mut inp.player2,&mut inp.player1)
        };
        (*winner).rotate_left(1);
        (*winner).push_back((*loser).pop_front().unwrap());
        counter+=1;
    }
    let final_winner = match inp.player1.len() == 0 {
        true => &inp.player2,
        false => &inp.player1
    };
    let mut score = 0;
    let wlen = (*final_winner).len();
    for (idx_i,i) in (*final_winner).iter().enumerate() {
        score += (wlen-idx_i)* *i as usize;
    }
    (score as Score,counter)
}




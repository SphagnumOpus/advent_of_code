use std::io;
use std::collections::VecDeque;
use std::option::Option;

//Day 22 2020 for advent of code
//
//
//
//
#[derive (Clone,Debug)]
struct CardGame {
    player1: VecDeque<Card>,
    player2: VecDeque<Card>,
    history: Vec<CardGame>
}
type Card = u32;
type Score = u32;
type FinalAnswer = u32;


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
        player2: VecDeque::new(),
        history: Vec::new()
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
    let (out,_did_1_win) = play_game(&mut clone,false);
    out
}
fn calc_part_2(inp:&CardGame) ->FinalAnswer{
    let mut clone = (*inp).clone();
    let (out,_did_1_win) = play_game(&mut clone,true);
    out
}

fn play_game(mut inp:&mut CardGame,inp_is_recursive:bool) -> (Score,bool) {
    while (inp.player1.len()>0) && (inp.player2.len()>0) {
        let (winner,loser) = match does_player1_win(&mut inp,inp_is_recursive) {
            Some(n) => match n {
                       true =>  (&mut inp.player1,&mut inp.player2),
                       false => (&mut inp.player2,&mut inp.player1)
            },
            None => return (0,true) //means player 1 won because we'd played a deck layout twice
        };
        (*winner).rotate_left(1);
        (*winner).push_back((*loser).pop_front().unwrap());
    }
    let (final_winner,did_player_1_win) = match inp.player1.len() == 0 {
        true => (&inp.player2,false),
        false => (&inp.player1,true),
    };
    let mut score = 0;
    let wlen = (*final_winner).len();
    for (idx_i,i) in (*final_winner).iter().enumerate() {
        score += (wlen-idx_i)* *i as usize;
    }
    (score as Score,did_player_1_win)
}

fn does_player1_win(inp: &mut CardGame,inp_is_recursive:bool) -> Option<bool> {
    //returns true or false, wrapped in an option - returns None if Player1 wins the game by
    //repetitive deck
    if !inp_is_recursive {
        return Some(inp.player1.front().unwrap() > inp.player2.front().unwrap());
    }
    //playing by recursive rules... first look if we've played this deck
    if we_have_played_this_deck(&inp) {  //player 1 wins if this is a repeat position
        return None;
    }
    else { //add this deck layout to the list since it's new
        let new_deck = CardGame {
            player1: inp.player1.clone(),
            player2: inp.player2.clone(),
            history: Vec::new()
        };
        inp.history.push(new_deck);
    }
    let out;
    // get values of the top cards
    let size1 = *inp.player1.front().unwrap();
    let size2 = *inp.player2.front().unwrap();
    if (inp.player1.len() as Card) > size1 && (inp.player2.len() as Card) > size2 {
        //we're gonna play a subgame
        //create the deck
        //for each deck... take the first card OFF the deck, then deal as many cards as that top
        //card equalled
        let mut new_deck = CardGame {
            player1: inp.player1.iter().skip(1).take(size1 as usize).copied().collect::<VecDeque<Card>>(),
            player2: inp.player2.iter().skip(1).take(size2 as usize).copied().collect::<VecDeque<Card>>(),
            history: Vec::new()
        };
        //look for subgames that don't need to be played
        //if player 1 holds a card greater than the sum of the two decks minus 1 (s)he will will
        let max1 = *new_deck.player1.iter().max().unwrap(); 
        if max1 > (size1 + size2 -1) && max1 > *new_deck.player2.iter().max().unwrap() {
            return Some(true);
        }
        
        
        //play the subgame
        let tmp = play_game(&mut new_deck,inp_is_recursive);
        out=tmp.1;
        //we only care who won
    }
    else {
        out = inp.player1.front().unwrap() > inp.player2.front().unwrap();
    }
    Some(out)
}



fn we_have_played_this_deck(inp: &CardGame) -> bool {
    //look for player1's deck and player2's deck showing up in the deck history for this game

    'outer: for i in &inp.history {
        if inp.player1.len() == i.player1.len() && 
            inp.player2.len() == i.player2.len(){
                for (j,k) in inp.player1.iter().zip(i.player1.iter()) {
                    if j != k {
                        continue 'outer;
                    }
                }
                for (j,k) in inp.player2.iter().zip(i.player2.iter()) {
                    if j != k {
                        continue 'outer;
                    }
                }
                return true;
        }
    }
    false
}
            


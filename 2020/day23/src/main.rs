use std::io;
use std::collections::VecDeque;

//Day 23 2020 for advent of code
//
//
//
//
type Cup = usize;
#[derive (Clone,Debug)]
struct FastCup {
    previous: Cup,
    next: Cup
}

type FinalAnswerPart1 = String;
type FinalAnswerPart2 = Cup;


fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let this_cup_circle = build_this_cup_circle(&orig_inp);
    let this_fast_cup_circle = build_this_fast_cup_circle(&this_cup_circle,0);
    let part1_answer = calc_part_1(&this_cup_circle);
    println!("The part 1 answer is {}",&part1_answer);
    let fast_part1_answer = calc_fast_part_1(&this_fast_cup_circle,*(this_cup_circle.front().unwrap()));
    println!("The fast part 1 answer is {}",&fast_part1_answer);
    let this_big_fast_cup_circle = build_this_fast_cup_circle(&this_cup_circle,1000000);
    let part2_answer = calc_part_2(&this_big_fast_cup_circle,*(this_cup_circle.front().unwrap()));
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
fn build_this_cup_circle(inp:&Vec<String>) -> VecDeque<Cup> {
    for i in inp { //we only expect one line
        let out: VecDeque<_> = i.split("").filter(|x| *x != "").map(|x| x.parse::<Cup>().unwrap()).collect();
        return out;
    }
    VecDeque::new()
}
fn build_this_fast_cup_circle(inp:&VecDeque<Cup>,inp_c:Cup) -> Vec<FastCup>{
    //building a double linked list where the index into the list is the value
    //for simplicity of expression we're using 1 as the first element, ignoring the 0 element
    let mut copy=inp.clone();
    let mut out = vec!(FastCup {previous:0,next:0};*(inp.iter().max().unwrap()).max(&inp_c)+1); //build a vec either as large as input or larger if arg says so
    let mut previous = *(copy.front().unwrap());
    copy.rotate_left(1);
    let mut current = *(copy.front().unwrap());
    while out[current].previous == 0 {
        out[previous].next = current;
        out[current].previous = previous;
        previous = current;
        copy.rotate_left(1);
        current = *(copy.front().unwrap());
    }
    //previous ends the loop holding the first item in the VecDeque
    //now add any extra elements called for
    if inp_c > inp.len(){
        for i in inp.len()+1 .. inp_c + 1 {
            out[i].next=i+1;
            out[i].previous= i-1;
        }
        //now link the extra elements into the array at the end creating a ring
        let prior_end=out[previous].previous;
        out[prior_end].next = inp.len() + 1;
        out[inp_c].next=previous;
        out[inp.len()+1].previous = out[previous].previous;
        out[previous].previous = inp_c;
    }
    out
}



fn calc_part_1(inp:&VecDeque<Cup>) ->FinalAnswerPart1{
    let mut copy=inp.clone();// don't mess up the original cup list... we may need it
    make_moves(&mut copy,100);
    while 1 != *(copy.front().unwrap()) {
        copy.rotate_right(1);
    }
    let out:String = copy.iter().skip(1).map(|x| x.to_string()).collect();
   
   out 
}

fn calc_fast_part_1(inp:&Vec<FastCup>,inp_f:Cup) ->FinalAnswerPart1{
    let mut copy=inp.clone();// don't mess up the original cup list... we may need it
    make_fast_moves(&mut copy,100,inp_f);
    let mut collector = Vec::new();
    let mut current = copy[1].next;
    while 1 != current {
        collector.push(current);
        current=copy[current].next;
    }
    let out:String = collector.iter().map(|x| x.to_string()).collect();
   
   out 
}
fn calc_part_2(inp:&Vec<FastCup>,inp_f:Cup) ->FinalAnswerPart2{
    let mut copy=inp.clone();// don't mess up the original cup list... we may need it
    make_fast_moves(&mut copy,10000000,inp_f);
    let mut collector = Vec::new();
    let mut current = copy[1].next;
    for _i in 0..2 {
        collector.push(current);
        current=copy[current].next;
    }
    let out = collector[0] * collector [1];
   
   out 
}

fn make_moves(inp:&mut VecDeque<Cup>,inp_c: Cup) {
    for _i in 0..inp_c {
        let current_cup = *(inp.front().unwrap());
        inp.rotate_left(1);
        let mut holding_tank = Vec::new();
        assert!(inp.len() > 3);  //kill the program if there aren't three elements to remove!
        for _j in 0..3 {
            holding_tank.push(inp.pop_front().unwrap());
        }
        //now determine what the "destination cup" is
        let mut destination_cup = (current_cup+9)%10;
        while !inp.contains(&destination_cup) {
            destination_cup = (destination_cup+9)%10;
        }
        while destination_cup != *(inp.front().unwrap()) {
            inp.rotate_right(1);
        }
        //found the cup!  now insert the three removed cups to the right of this one.
        inp.rotate_left(1);
        for _j in 0..3 {
            inp.push_front(holding_tank.pop().unwrap());
        }
        while current_cup != *(inp.front().unwrap()) {
           inp.rotate_right(1); //put the current cup back at the front.
        }
        inp.rotate_left(1);//go to the next cup
    }
}
fn make_fast_moves(inp:&mut Vec<FastCup>,inp_c: Cup, inp_f: Cup) {
    //inp_c is the iterations to do, inp_f is the first cup
    let size = inp.len();
    let mut current_cup = inp_f;
    for _i in 0..inp_c {
        let mut holding_tank = Vec::new();
        let mut ptr = inp[current_cup].next;
        assert!(inp.len() > 3);  //kill the program if there aren't three elements to remove!
        for _j in 0..3 {  //copy the three cups we're removing
            holding_tank.push(ptr);
            ptr = inp[ptr].next;
        }
        inp[current_cup].next = ptr; //link the current cup to cup after the three we've "removed"

        //now determine what the "destination cup" is
        let mut destination_cup = (current_cup+size-1)%size;
        while destination_cup == 0 || holding_tank.contains(&destination_cup) {
            destination_cup = (destination_cup+size-1)%size;
        }
        //found the cup!  now insert the three cups after the destination cup
        let prv_destination_cup_next = inp[destination_cup].next;
        inp[destination_cup].next = holding_tank[0];
        inp[holding_tank[0]].previous = destination_cup;
        inp[holding_tank[2]].next = prv_destination_cup_next;
        inp[prv_destination_cup_next].previous = holding_tank[2];
        current_cup = inp[current_cup].next;  //go to the next cup;
    }
}



            


use std::io;
use std::env;
use std::collections::HashMap;

//Day 07 2020 for advent of code
//
//
static mut DEBUG: bool = false;
static mut PART1: bool = true;
static mut PART2: bool = false; //assume part 1

struct BagRuleItem { 
    bag_name:String,
    bag_qty :usize,
}  //any individual Bag list in Bags



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 { println!("No arguments passed"); }

    for s in args {
        match &s[..] {
            "--debug" => {
                println!("Debug mode");
                unsafe {DEBUG = true;}
            }
            "--part1" => {
                println!("Part1 mode explicitly set");
                unsafe{PART1 = true;}
            }
            "--part2" => {
                println!("Part2 mode explicitly set");
                unsafe{PART2 = true;}
            }
            _ => {
            }
        }
    }


    let orig_inp = read_input();  //read input buffer into strings
    let bag_rules = create_bag_rules(&orig_inp);  //create set of bag rules
    let bags_inside = create_bags_inside(&bag_rules); //
    
    let mut match_count:usize = 0;
    for i in bags_inside.values() {
        if i.contains_key("shiny gold") {
            match_count+= 1;
        }
    }


    println!("The part 1 answer is {}",match_count);
    //find the hashmap for "shiny gold" bags 
    let total_bags = match bags_inside.get("shiny gold"){
            Some(bags_hm) =>  { let mut x:usize = 0;
                for i in bags_hm.values(){
                    x += i;
                }
                x
            }
            _ => 0,

        };




    println!("The part 2 answer is {}",total_bags);






}

fn read_input() -> Vec<String> {
    let mut inbuf = String::new();
    let mut outbuf = Vec::new();
    while 0<io::stdin().read_line(&mut inbuf).expect("I/O error on read"){
        outbuf.push(inbuf);
        inbuf = String::new();
    }
    outbuf
}

fn create_bag_rules(inp:&Vec<String>) -> HashMap<String,Vec<BagRuleItem>> {
    let mut out = HashMap::new();
    for i in inp {  //for each raw string containing a bag rule
        let mut itr = i.split(" bags contain ");  //itr to capture the bag name
        let bag_str = itr.next().unwrap().to_string();  //capture the bag name
        let rules_str=itr.next().unwrap().to_string(); //capture the bag rules
        let mut bag_vec = Vec::new();
        if rules_str != "no other bags.\n" {
            for j in rules_str.split(","){
                let mut itr = j.split_whitespace();
                let br_qty = itr.next().unwrap().parse::<usize>().unwrap(); 
                let br_str = format!("{} {}",itr.next().unwrap(),itr.next().unwrap()); 
                let bri = BagRuleItem{
                    bag_name : br_str,
                    bag_qty : br_qty};
                bag_vec.push(bri);
            }
        }

        out.insert(bag_str,bag_vec);
    }
    out
}

fn create_bags_inside(input: &HashMap<String,Vec<BagRuleItem>>) -> HashMap<String,HashMap<String,usize>>{
    let mut out = HashMap::new();
    for (bagname,_bagrules) in input{
        let mut str_hs = HashMap::new();   //keys of used bag names for this key
        bld_bag_list(&input, bagname.clone(),1, &mut str_hs); 
        out.insert(bagname.clone(),str_hs);
    }
    out
}

fn bld_bag_list(in_rules: &HashMap<String,Vec<BagRuleItem>>, inp_bag:String,num_bags:usize, mut hs:&mut HashMap<String,usize>){
    let cur_bag_items=in_rules.get(&inp_bag).unwrap();
    if cur_bag_items.len() > 0 {
        for i in cur_bag_items {
            let new_bags = i.bag_qty;   //bags to be added to total needed
            let prior_bags = match hs.get(&i.bag_name){
                Some(prv_bags) =>  prv_bags,
                _ => &(0 as usize),
            };
            let new_tot_bags = (new_bags*num_bags)+prior_bags;  //get updated total number of bags
            //add new bags here
            hs.insert(i.bag_name.clone(),new_tot_bags);
            bld_bag_list(in_rules,i.bag_name.clone(),new_bags*num_bags, &mut hs)
        }
    }

}

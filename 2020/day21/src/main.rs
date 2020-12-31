use std::io;
use std::collections::HashSet;
use std::collections::HashMap;

//Day 21 2020 for advent of code
//
//
//
//
struct Food {
    ingredients: HashSet<Ingredient>,
    allergens: Vec<Allergen>
}
type Ingredient = String;
type Allergen = String;
type FinalAnswer = u32;



fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let food_list = build_food_list(&orig_inp);
    let part1_answer = calc_part_1(&food_list);
    println!("The part 1 answer is {}",&part1_answer);
    let part2_answer = calc_part_2(&food_list);
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
fn build_food_list(inp:&Vec<String>) -> Vec<Food> {
    let mut out = Vec::new();

    for i in inp {
        let mut t_ingredients = HashSet::new();
        let mut t_allergens = Vec::new();
        let mut into_allergens = false;
        for j in i.split_whitespace() {
            if j == "(contains" {
                into_allergens = true;
                continue;
            }
            if into_allergens {
                t_allergens.push(j[0..j.len()-1].to_string());
            }
            else { 
                t_ingredients.insert(j.to_string().clone());
            }
        };
        //println!("ingredients = {:?}",t_ingredients);
        //println!("allergens = {:?}",t_allergens);
        let tmpfood = Food{
            ingredients: t_ingredients,
            allergens: t_allergens};
        out.push(tmpfood);
    }

    out
}

fn calc_part_1(inp:&Vec<Food>) ->FinalAnswer{
    let mut out = 0;
    let mut allergen_list:HashMap<Allergen,HashSet<Ingredient>> = HashMap::new();
    for i in inp {   //narrow down the allergens by looking for ingredients in every entry that has a given allergen
        for j in &i.allergens {
            let old_list = match allergen_list.get(j) {
                Some(n) => n,
                None => &i.ingredients
                };
            let new_list:HashSet<Ingredient> = old_list.intersection(&i.ingredients).cloned().collect();
            allergen_list.insert(j.to_string(),new_list);
        }
    }
    //println!("allergen_list = {:?}",allergen_list);
    //flatten the allergen list 
    let mut final_list:HashSet<Ingredient> = HashSet::new();
    for (idx_i, i) in &allergen_list {
        for j in i {
            let tmp = j.clone();
            final_list.insert(tmp);
        }
    }
    //println!("narrowed allergen_list = {:?}",final_list);
    //now count up all the entries that aren't in that hashset across all the input
    for i in inp {
        for j in &i.ingredients {
            if !final_list.contains(j) {
                out += 1;
            }
        }
    }
    out
}
fn calc_part_2(inp:&Vec<Food>) ->FinalAnswer{
    0
}

//fn parse_allergen_list(inp:&HashMap<Allergen,HashSet<Ingredient>>)) ->Vec<Ingredient>{
    // read the list of foods, create a vec of the ingredients that contain the allergens  (we
    // don't need to know which ingredient contains which allergen
 //   let mut out = Vec::new();
  //  let num_of_allergens = inp.len();



use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeMap;

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
    let (part1_answer,mut allergen_list) = calc_part_1(&food_list);
    println!("The part 1 answer is {}",&part1_answer);
    let part2_answer = calc_part_2(&mut allergen_list);
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
        let tmpfood = Food{
            ingredients: t_ingredients,
            allergens: t_allergens};
        out.push(tmpfood);
    }

    out
}

fn calc_part_1(inp:&Vec<Food>) ->(FinalAnswer,HashMap<Allergen,HashSet<Ingredient>>){
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
    let mut final_list:HashSet<Ingredient> = HashSet::new();
    for (_idx_i, i) in &allergen_list {
        for j in i {
            let tmp = j.clone();
            final_list.insert(tmp);
        }
    }
    //now count up all the entries that aren't in that hashset across all the input
    for i in inp {
        for j in &i.ingredients {
            if !final_list.contains(j) {
                out += 1;
            }
        }
    }
    (out,allergen_list)
}
fn calc_part_2(inp:&mut HashMap<Allergen,HashSet<Ingredient>>) ->String{
    let mut sorted_list:BTreeMap<Allergen,Ingredient> = BTreeMap::new();
    let mut found_ingredients:HashSet<Ingredient> = HashSet::new();
    loop{
        let mut nothing_processed = true;
        for (idx_i,mut i) in &mut *inp {
            let tmp:&mut HashSet<String> = &mut i;
            if tmp.len() == 1 {
                let tmp_str=i.iter().next().unwrap().to_string();
                let tmp_str_copy = tmp_str.clone();
                sorted_list.insert((&idx_i).to_string(),tmp_str);
                found_ingredients.insert(tmp_str_copy);

            }
            else {
                nothing_processed = false;
                let mut tmp_list = Vec::new();
                for j in tmp.iter() {
                    if found_ingredients.contains(j) {
                        //let tmp_str=.iter().next().unwrap().to_string();
                        tmp_list.push(j.clone());
                    }
                }
                for j in tmp_list {
                    tmp.remove(&j);
                }
            }

        }
        if nothing_processed {
            break;
        }
    }
    let mut out = String::new();
    for (_idx_i,i) in sorted_list {
        out.push_str(&i);
        out.push_str(",");
    }

    out.truncate(out.len()-1);
    out
}

//fn parse_allergen_list(inp:&HashMap<Allergen,HashSet<Ingredient>>)) ->Vec<Ingredient>{
    // read the list of foods, create a vec of the ingredients that contain the allergens  (we
    // don't need to know which ingredient contains which allergen
 //   let mut out = Vec::new();
  //  let num_of_allergens = inp.len();



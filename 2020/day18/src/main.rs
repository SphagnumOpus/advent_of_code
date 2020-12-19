use std::io;

//Day 18 2020 for advent of code
//
//
//
enum Expression {
Unparsed(String),
Scalar(isize),
Operator(Operation),
Expression(Vec<Expression>,Option<(isize,isize)>), //returns part1 and part2 isizes
}

enum Operation {
Addition,
Subtraction,
Multiplication,
Division
}


fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let expressions = build_expressions(&orig_inp);
    let part1_answer = calc_part1(&expressions);
    println!("The part 1 answer is {}",part1_answer);
    let part2_answer = calc_part2(&expressions);
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

fn calc_part1(inp:&Vec<Expression>) ->isize {
    let mut out = 0;
    for i in inp {
        out += match i {
            Expression::Scalar(n) => *n,
            Expression::Expression(_v,n) => {
                let tmp = n.unwrap();
                tmp.0
            },
            _ => panic!("help!  whazzat?"),
        }
    }
    out
}

fn calc_part2(inp:&Vec<Expression>) ->isize {
    let mut out = 0;
    for i in inp {
        out += match i {
            Expression::Scalar(n) => *n,
            Expression::Expression(_v,n) => {
                let tmp = n.unwrap();
                tmp.1
            },
            _ => panic!("help!  whazzat?"),
        }
    }
    out
}

fn build_expressions(inp: &Vec<String>) -> Vec<Expression> {
    let mut out = Vec::new();
    for i in inp {
        let mut tmp_ex = Expression::Unparsed(i.clone());
        tmp_ex = parse_unparsed(&tmp_ex);
        out.push(tmp_ex);
    }
    out
}

fn parse_unparsed(inp: &Expression) -> Expression {
    // This routine takes an unparsed expression and does one level of parsing of that expression, breaking
    // it down to sub-expressions or full tokens
    // it returns a replacement expression, and if possible a valuation in the .value variable

    let mut cur_str = match inp {
        Expression::Unparsed(n) => n.clone(),
        _ => panic!("should not find anything but an unparsed string here!"),
    };
    trim_leading_whitespace(&mut cur_str);
    //at start of expression... next element either an expression or a scalar
    let mut this_expr = match harvest_expression(&mut cur_str) { //get an expression off the beginning of the string
        Some(n) => n,
        None => panic!("Error... invalid expression syntax"),
    };
    if cur_str.len() == 0 {
        return this_expr;
    }
    //what we're parsing is an expression, so we have a binary operation with at least one
    //operator/expression set, so let's continue building an "Expression" expression type
    
    let mut out_vec=Vec::new();
    out_vec.push(this_expr);

    trim_leading_whitespace(&mut cur_str);

    //now into expression... look for more elements
    while cur_str.len() > 0 {  //as long as there are tokens, look for <operator> <expression> pairs
        //next element must be  operator or we have a syntax error
        let this_operator = match char_to_operation(&mut cur_str) {
            Some(n) => n,
            None => panic!("Error ... invalid operator - string is \"{}\" len is {}",cur_str, cur_str.len()),
        };
        this_expr = Expression::Operator(this_operator);
        out_vec.push(this_expr);
        trim_leading_whitespace(&mut cur_str);
        this_expr = match harvest_expression(&mut cur_str) { //get the expression after the operator
            Some(n) => n,
            None => panic!("Error... invalid expression syntax"),
        };
        out_vec.push(this_expr);
        trim_leading_whitespace(&mut cur_str);
    }
    let value = calculate_value(&out_vec);
    Expression::Expression(out_vec,value)
}

fn harvest_expression(inp:&mut String) -> Option<Expression> {
    // takes one complete expression from the front of inp string (deleting it from that string)
    // and returns Expression representation of it
    let out = match inp.chars().next().unwrap() {
        '(' => {
            //we have another expression in ()!  find the trailing parenthesis and cut this off the front
            //of the input string into the element we're returning as unparsed
            assert_eq!(true,inp.len() > 1);
            let mut tmp_iter = inp.chars();
            tmp_iter.next(); //skip the opening parenthesis
            let mut paren_count= 1;
            let mut ptr = 1; //set to one - we've skipped the opening parenthesis - opening paren will get skipped
            while paren_count > 0 {
                paren_count += match tmp_iter.next().unwrap() {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };
                ptr += 1; 
            }
            //take the processed expression off the front of the string, stripping the parenthesis
            let out_str = inp[1..ptr-1].to_string();
            if inp.len() > ptr+1 {
                *inp = inp[ptr+1..].to_string(); 
            }
            else {
                *inp = String::new();
            }
            Some(parse_unparsed(&Expression::Unparsed(out_str)))
        },
        '0'..='9' => {//harvest a scalar element 
            let exp_len = inp.len();
            let mut ptr = 0;

            let mut tmp_iter = inp.chars();
            while ptr < exp_len && "0123456789".contains(tmp_iter.next().unwrap()){
                ptr+=1;
            }
            let out_int = inp[..ptr].parse().unwrap();
            if ptr == exp_len {
                *inp = String::new();
            }
            else {
                *inp = inp[ptr..].to_string();
            };
            Some(Expression::Scalar(out_int)) 
        },
        
        _ => {
            println!("Unexpected character :{}", inp);
            None
        }
    };
    out
}

fn char_to_operation(inp:&mut String) ->Option<Operation>{
    let out = match inp.chars().next().unwrap() {
        '*' => Some(Operation::Multiplication),
        '/' => Some(Operation::Division),
        '+' => Some(Operation::Addition),
        '-' => Some(Operation::Subtraction),
        _  => None,
    };
    *inp = inp[1..].to_string().chars().rev().collect::<String>().trim().chars().rev().collect::<String>();
    out
}

fn trim_leading_whitespace(inp:&mut String) {
    *inp = inp.chars().rev().collect::<String>().trim().chars().rev().collect::<String>(); //trim leading whitespace
}

fn calculate_value(inp:&Vec<Expression>) -> Option<(isize,isize)>{
    let mut tmp_iter = inp.iter();
    let mut lvalue = match tmp_iter.next().unwrap(){
        Expression::Expression(_v,n) => n.unwrap(),
        Expression::Scalar(n) => (*n,*n),
        _ => panic!("invalid left element of expression")
    };
    let mut ending_value = lvalue.1;
    let mut pt2_vec = Vec::new(); // keeping a vec of the multiplicants
    let mut next_one = tmp_iter.next();
    while next_one.is_some() {  //apply series of operator/value sub-expressions to the lvalue
        let operator = match next_one.unwrap() {
                Expression::Operator(n) => n,
                _ => panic!("error expected operator")
            };
        let rvalue = match tmp_iter.next() {
            Some(x) => match x {
                Expression::Scalar(n) => (*n,*n),
                Expression::Expression(_v,n) => n.unwrap(),
                _ => panic!("error expected operator")
            },
            None => {panic!("error... insane condition")
                //Operation::Multiplication
            },
        };
        lvalue = match operator {
            Operation::Addition => (lvalue.0 + rvalue.0,lvalue.1 + rvalue.1),
            Operation::Subtraction => (lvalue.0 - rvalue.0,0),
            Operation::Multiplication => {
                                             pt2_vec.push(lvalue.1);
                                             (lvalue.0 * rvalue.0,rvalue.1)
                                         },
            Operation::Division => (lvalue.0 / rvalue.0,0),
        };
        ending_value = lvalue.1;  //in case this is the last operation, keep track of the last item
        next_one = tmp_iter.next();
    };
    pt2_vec.push(ending_value); //make sure the last product gets onto the multiplier list;

    //now do the multiplications (later, as per precedents)
    let mut pt2_ctr = 1;   //iterate through the vec doing multiplications
    for i in pt2_vec {
        pt2_ctr *= i;
    }
    
    lvalue=(lvalue.0,pt2_ctr);

    Option::Some(lvalue)
}


        


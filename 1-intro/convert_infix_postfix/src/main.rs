use std::char;

/*
    1. priorities of operators 
        ->  +,- 
        ->  *,/ 
        ->  ^
    2. if scanned oeprator is <= then the top of the stack in priority 
    then pop opertors until we have low priority. Add the the poped elements 
    to the postfix

    3. if "(" pust it to the stack 

    4. if ")" pop elements until "(" and add poped elements to postfix

    5. if operand then just add to the postfix
*/ 

fn get_next_token(exp: &Vec<char>, idx: usize) -> (String,usize) {
    match exp[idx] {
        '(' | ')' | '+' | '-' | '*' | '/' => (exp[idx].to_string(),idx+1),
        _ => {
            let mut nidx=idx;
            let mut ans= exp[idx].to_string();
            nidx+=1;
            while nidx<exp.len() && exp[nidx].is_digit(10) {
                ans.push(exp[nidx]);
                nidx+=1;
            }
            (ans,nidx)
        }
    }
}

fn get_precedence(op: &str) -> i32 {
    match op {
        "+"|"-" => 1,
        "*"|"/" => 2,
        _ => 0,
    }
}

fn conv_infix_postfix(exp: &String) -> Vec<String> {
    let mut ans=Vec::new();
    let mut my_stack:Vec<String>=Vec::new();
    let mut idx=0;
    let exp_vec_chars: Vec<char>=exp.chars().collect();
    let mut token:String;
    while idx<exp.len() {
        (token,idx)=get_next_token(&exp_vec_chars, idx);
        let tchars:Vec<char> = token.chars().collect();
        match tchars[0] {
            '(' => my_stack.push(token.clone()),
            ')' => {
                while let Some(t) = my_stack.pop() {
                    if t == "(" {
                        break;
                    }
                    ans.push(t);
                }
            },
            '+'|'-'|'*'|'/' => {
                while let Some(last) = my_stack.last() {
                    if get_precedence(last) >= get_precedence(&token) {
                        ans.push(my_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                my_stack.push(token.clone());
            }
            _ => ans.push(token.clone()),
        }
        println!("token: {}, stack: {:?}, ans: {:?}",token, my_stack, ans);
    }
    while let Some(t) = my_stack.pop() {
        ans.push(t);
    }
    return ans;
}

fn main() {
    let s1=String::from("(33+45/3*(2+9)-50)");
    println!("{:?}",conv_infix_postfix(&s1));
}

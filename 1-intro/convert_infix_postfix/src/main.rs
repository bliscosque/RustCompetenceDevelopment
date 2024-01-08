use std::char;

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
            '(' => ans.push(token.clone()),
            ')' => {
                let mut t=my_stack.pop().unwrap();
                while t!="(" {
                    ans.push(t);
                    t=my_stack.pop().unwrap();
                }
            }
            _ =>{}
        }
        println!("token: {}, ans: {:?}",token, ans);
    }
    return ans;
}

fn main() {
    let s1=String::from("(33+45/3*(2+9)-50)");
    println!("{:?}",conv_infix_postfix(&s1));
}

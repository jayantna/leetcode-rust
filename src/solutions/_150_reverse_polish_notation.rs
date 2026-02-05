pub struct Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<String> = vec![];
        for i in tokens {
            if let Ok(i) = i.parse::<i32>() { 
                stack.push(i.to_string());
            } else {
                println!("found {:?}", i);
                if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                    let popped_result = match i.as_str() {
                        "+" => b.parse::<i32>().unwrap() + a.parse::<i32>().unwrap(),
                        "*" => b.parse::<i32>().unwrap() * a.parse::<i32>().unwrap(),
                        "/" => b.parse::<i32>().unwrap() / a.parse::<i32>().unwrap(),
                        "-" => b.parse::<i32>().unwrap() - a.parse::<i32>().unwrap(),
                        _ => 0
                    };
                    stack.push(popped_result.to_string());}
            }
            println!("{:?}", stack);
        }
        stack.pop().and_then(|x| x.parse::<i32>().ok()).unwrap()
    }
    pub fn run() {
        let input: Vec<String> = vec![
            "4","3","-"
        ]
        .into_iter()
        .map(|x| x.into())
        .collect();
        let result = Solution::eval_rpn(input);
        println!("{:?}", result);
    }
}

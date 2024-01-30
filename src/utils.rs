pub fn eval_rpn(tokens: Vec<String>) -> i32 {
	let mut stack = Vec::new();

	for token in tokens {
		match token.as_str() {
			"+" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a+b);
			},
			"-" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a-b);
			},
			"*" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a*b);
			},
			"/" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a/b);
			},
			t => {
				let num = t.parse::<i32>().unwrap();
				stack.push(num)
			}
		}
	}

	stack[0]
		
}


fn main(){}
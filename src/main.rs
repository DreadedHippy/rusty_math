use crate::structs::RPNInfo;

mod structs;

pub fn eval_rpn(tokens: Vec<String>) -> f64 {
	let mut stack: Vec<f64> = Vec::new();

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
			"^" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a.powi(b as i32));
			},
			t => {
				let num = t.parse::<f64>().unwrap();
				stack.push(num);
			}
		}
	}

	stack[0]
		
}


// Bisection method of finding roots of an equation, f(x), using RPN
fn main(){
	// Define your f(x) e.g f(x) = x^2 - 5x + 6

	// Define a, b, where f(a) and f(b), have opposing signs (this implies a root of f(x) exists in the interval (a, b))
	let (mut a, mut b) = ((2.5).to_string(), (10).to_string()); // Here a root, 3, exists between 2.5 and 10

	// Define the bisection formula, (a+b)/2 in form of RPN
	let mut bisection_formula = vec![a.clone(), b.clone(), "+".to_string(), 2.to_string(), "/".to_string()];

	// Get x₀, the initial value of the bisection of a and b
	let mut x = eval_rpn(bisection_formula.clone());

	// Define f(x) in form of RPN. Here, f(x) = x^2 - 5x + 6;
	let mut f_x: Vec<String> = vec!["x" ,"2", "^", "5", "x", "*", "-", "6", "+"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();

	
	// Use our utility Struct to store all of f(x)'s information
	let mut rpn_info = RPNInfo::new(f_x);
	
	// ! Get the value of f(a₀), this enables us tell wether to replace a with x, or b with x based on their signs
	let mut f_a_val = rpn_info.immediate_eval(a.clone()).unwrap();

	// Initialize 'x' in our helper struct
	rpn_info.update_x(x);

	// Set maximum number of bisections, in this case,20 to prevent infinite loop
	let (mut bisections, max_bisections) = (0 ,20);

	// As long as "x" is not equal to "a" or b to some degree, or bisections  limit not reached repeat the following steps:

	while 
		(
			(x * 10000_f64) as i32 != (a.parse::<f64>().unwrap() * 10000_f64) as i32
				||
			(x * 10000_f64) as i32 != (b.parse::<f64>().unwrap() * 10000_f64) as i32 
		)
			&&
		bisections < max_bisections
		{

		let value= rpn_info.evaluate().unwrap();

		//	if 'value' and 'f(a₀)' have same signs, a becomes x, otherwise, b becomes x
		match  (value < 0.0 && f_a_val < 0.0) || (value > 0.0 && f_a_val > 0.0) {
			true => {
				a = x.to_string();
				bisection_formula[0] = a.clone();
			},
			false => {
				b = x.to_string();
				bisection_formula[1] = b.clone();
			}
		}

		// Get the new "x" value by further bisection
		x = eval_rpn(bisection_formula.clone());

		// Input the new "x" values in f(x) in case of re-evaluation
		rpn_info.update_x(x);

		bisections+=1;

	}

	println!("Root of f(x) is {:.2}", x);
}
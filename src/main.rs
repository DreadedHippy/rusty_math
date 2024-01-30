use std::ops::{Add, Sub, Mul, Div};

use structs::CachedRPN;
mod structs;

pub fn eval_rpn(tokens: Vec<String>) -> (f64, CachedRPN) {
	let mut stack: Vec<f64> = Vec::new();
	let mut cached_rpn: CachedRPN = CachedRPN {
		numbers: Vec::new(),
		operations: Vec::new()
	};

	for token in tokens {
		match token.as_str() {
			"+" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a+b);
				cached_rpn.operations.push(f64::add)

			},
			"-" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a-b);
				cached_rpn.operations.push(f64::sub)
			},
			"*" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a*b);
				cached_rpn.operations.push(f64::mul)
			},
			"/" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a/b);
				cached_rpn.operations.push(f64::div)
			},
			"^" => {
				let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(a.powi(b as i32));
				cached_rpn.operations.push(pow)
			},
			t => {
				let num = t.parse::<f64>().unwrap();
				stack.push(num);
				cached_rpn.numbers.push(num);
			}
		}
	}

	(stack[0], cached_rpn)
		
}

fn pow(b: f64, e: f64)  -> f64 {
	b.powi(e as i32) as f64
}


// Bisection method of finding roots of an equation, f(x), using RPN
fn main(){

	// Define your f(x) e.g f(x) = x^2 - 6x + 8;

	// Define a, b, where f(a) and f(b), have opposing signs (this implies a root of f(x) exists in the interval (a, b))

	// A root of f(x) above is 4: let a = 3, b = 7 (4 lies within [3, 7])

	let (mut a, mut b) = (3.to_string(), 7.to_string());

	// !  Note: f(a) = f(3) = negative value, f(b) = f(7) = positive value;

	// Define the bisection formula, (a+b)/2 in form of RPN

	let bisection_formula = vec![a.clone(), b.clone(), "+".to_string(), 2.to_string(), "/".to_string()];

	// Get x₀, the initial value of the bisection of a and b

	let (mut x, mut cached_bisection_formula) = eval_rpn(bisection_formula);

	// // Cast x₀ to a string for further re-evaluation;

	// // let mut x_as_str = x.to_string();

	// Define f(x) in terms of RPN. Here, f(x) = x^2 - 6x + 8;

	let f_x: Vec<String> = vec![&x.to_string() ,"2", "^", "6", &x.to_string(), "*", "-", "8", "+"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
	let (_, mut cached_fx) = eval_rpn(f_x);

	// As long as "x" is not equal to "a" or b with a precision of 4 D.P, repeat the following steps:

	while 
		(x * 10000_f64) as i32 != (a.parse::<f64>().unwrap() * 10000_f64) as i32
		||
		(x * 10000_f64) as i32 != (b.parse::<f64>().unwrap() * 10000_f64) as i32 
		{
			
	// 	// Get first_value f₀ , i.e f(x₀)
		// let (mut value, cached_fx) = eval_rpn(
		// 	f_x
		// );

		let (value, _) = cached_fx.clone().evaluate();

		//	if value is negative, a becomes x, otherwise, b becomes x

		match  value < 0.0 {
			true => {
				a = x.to_string();
				cached_bisection_formula.numbers[0] = x.clone();
			},
			_ => {
				b = x.to_string();
				cached_bisection_formula.numbers[1] = x.clone();
			}
		}

		// Get the new "x" value
		(x, _) = cached_bisection_formula.clone().evaluate();

		// Input them in the RPN as needed
		cached_fx.numbers[0] = x;
		cached_fx.numbers[3] = x;



	}

	println!("Root of f(x): {:.2}", x);
	


	


	// // Get first_value f₀ , i.e f(x₀)
	// let (mut value, cached_fx) = eval_rpn(
	// 	f_x
	// );

	// //	if value is negative, a becomes x, otherwise, b becomes x

	// if value < 0.0 { a = x.to_string()} else {b = x.to_string()}

	//

	// println!("{}, {:?}", initial_result, cached_rpn)	;

	// println!("{:?}", cached_rpn.evaluate())
}
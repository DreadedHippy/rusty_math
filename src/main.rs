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
	// Define your f(x) e.g f(x) = x^2 - 6x + 8;

	// Define a, b, where f(a) and f(b), have opposing signs (this implies a root of f(x) exists in the interval (a, b))

	// A root of f(x) above is 4: let a = 3, b = 7 (4 lies within [3, 7])

	let (mut a, mut b) = (3.to_string(), 7.to_string());

	// !  Note: f(a) = f(3) = negative value, f(b) = f(7) = positive value;

	// Define the bisection formula, (a+b)/2 in form of RPN

	let mut bisection_formula = vec![a.clone(), b.clone(), "+".to_string(), 2.to_string(), "/".to_string()];

	// Get x₀, the initial value of the bisection of a and b

	let mut x = eval_rpn(bisection_formula.clone());

	// Define f(x) in form of RPN. Here, f(x) = x^2 - 6x + 8;

	let mut f_x: Vec<String> = vec![&x.to_string() ,"2", "^", "6", &x.to_string(), "*", "-", "8", "+"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();

	// As long as "x" is not equal to "a" or b with a precision of 4 D.P, repeat the following steps:

	while  (x * 10000_f64) as i32 != (a.parse::<f64>().unwrap() * 10000_f64) as i32 || (x * 10000_f64) as i32 != (b.parse::<f64>().unwrap() * 10000_f64) as i32 {

		let value= eval_rpn(f_x.clone());
		println!("{}", x);

		//	if value is negative, a becomes x, otherwise, b becomes x

		println!("{}", value);
		match  value < 0.0 {
			true => {
				a = x.to_string();
				bisection_formula[0] = a.clone();
			},
			_ => {
				b = x.to_string();
				bisection_formula[1] = b.clone();
		}
		}

		// Get the new "x" value
		x = eval_rpn(bisection_formula.clone());

		// Input the new "x" values in f(x) in case of re-evaluation
		f_x[0] = x.to_string();
		f_x[4] = x.to_string();

	}

	println!("Root of f(x) = (x^2 - 6x + 8) is {:.2}", x);
}
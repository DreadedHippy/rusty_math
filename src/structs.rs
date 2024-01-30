// ! COMPLETELY DISREGARD

// #[derive(Debug, Clone)]
// pub struct CachedRPN {

// 	pub numbers: Vec<f64>,
// 	pub operations: Vec<fn(f64, f64) -> f64>
// }

// impl CachedRPN {
// 	pub fn evaluate(self) -> (f64, Self) {
// 		let mut a = self.numbers[0];
// 		for i in 1..self.numbers.len() {
// 			let b = self.numbers[i];
// 			a = self.operations[i-1](a, b);
// 		}

// 		(a, self)
// 	}
// }
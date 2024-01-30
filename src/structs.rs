use crate::eval_rpn;


pub struct RPNInfo {
	pub rpn: Vec<String>,
	pub x_indexes: Vec<usize>,
	pub x_initialized: bool
}

impl RPNInfo {
    pub fn new(rpn: Vec<String>) -> Self {
			let mut x_indexes: Vec<usize> = Vec::new();

			for (i, string) in rpn.iter().enumerate() {
				match string.as_str() {
					"x" => {
						x_indexes.push(i)
					},
					_ => {}
				}
			}

			Self {
				rpn,
				x_indexes,
				x_initialized: false
			}
		}

		pub fn update_x(&mut self, x: String) {
			for index in self.x_indexes.clone() {
				self.rpn[index] = x.clone()
			}

			self.x_initialized = true
		}

		pub fn evaluate(&self) -> Result<f64, String>{
			if !self.x_initialized {
				Err("'x' is not initialized, please initialized 'x' with the update_x method".to_string())
			} else {
				Ok(eval_rpn(self.rpn.clone()))
			}
		}

		// Takes an f64 as string
		pub fn immediate_eval(&self, arg: String) -> Result<f64, String> {
			if self.x_initialized {
				Err("'x' is already initialized, use an uninitialized RPN for immediate evaluation".to_string())
			} else {
				let new_rpn = self.rpn.iter().map(|s| {
					match s.as_str() {
						"x" => { arg.to_string() },
						s => {s.to_string()}
					}
				}).collect::<Vec<String>>();

				Ok(eval_rpn(new_rpn))
			}
		}
}	
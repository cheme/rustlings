// traits3.rs
//
// Fix without changing anything in tests tests.

// I AM NOT DONE

/* Solution
trait AppendCst: std::fmt::Debug {
  fn append_cst(&mut self);
}

impl AppendCst for String {
  fn append_cst(&mut self) {
		self.push_str("Bar");
	}
}

impl AppendCst for usize {
  fn append_cst(&mut self) {
		*self += self.to_append();
	}
}


impl AppendCst for Vec<&mut dyn AppendCst> {
 fn append_cst(&mut self) {
		self.iter_mut().for_each(|d| d.append_cst())
	}
}
*/

trait AppendCst: std::fmt::Debug {
	const ToAppend: Self;

  fn append_cst(&mut self);
}

impl AppendCst for String {
	const ToAppend: Self = "Bar".into();
  fn append_cst(&mut self) {
		*self += Self::ToAppend
	}
}

impl AppendCst for usize {
	const ToAppend: Self = 42;
  fn append_cst(&mut self) {
		*self += Self::ToAppend
	}
}


impl<T: AppendCst> AppendCst for Vec<&mut T> {
	const ToAppend: Self = Vec::new();
	fn append_cst(&mut self) {
		self.iter_mut().for_each(|d| d.append_cst())
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_multiple_type() {
			let mut first = String::from("Foo");
			let mut second = 5;
			let mut input: Vec<&mut dyn AppendCst> = vec![&mut first, &mut second];

			input.append_cst();
      assert_eq!(format!("{:?}", &input[..]), "[\"FooBar\", 47]");
    }
}

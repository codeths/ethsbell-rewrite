pub trait MaxElement {
	type Iter;

	fn max_element(self) -> Self::Iter;
}

impl<I> MaxElement for I
where
	I: Iterator + Clone,
	I::Item: PartialOrd,
{
	type Iter = Self;

	fn max_element(mut self) -> Self::Iter {
		let mut max_iter = self.clone();
		let mut max_val = None;

		while let Some(val) = self.next() {
			if max_val.as_ref().map_or(true, |m| &val > m) {
				max_iter = self.clone();
				max_val = Some(val);
			}
		}

		max_iter
	}
}

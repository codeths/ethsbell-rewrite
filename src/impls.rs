//! Contains simple trait implementations.

/// This is only used in the legacy section, and will soon be deprecated.
pub trait MaxElement {
	/// The type of the element to be sorted.
	type Iter;
	/// Consume the iterator and return its largest element.
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

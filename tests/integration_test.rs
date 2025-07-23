use core::fmt::Debug;
use dynemit::eq::DynEq;

trait MyTrait: DynEq + Debug {}
dynemit::eq_trait_object!(MyTrait);

#[derive(Debug, PartialEq, Eq)]
struct A {
	value: u32,
}
impl MyTrait for A {}

#[derive(Debug, PartialEq, Eq)]
struct B {
	value: u32,
}
impl MyTrait for B {}

mod with_box {
	use super::*;

	#[test]
	fn a_and_a_same_value_equal() {
		let a1: Box<dyn MyTrait> = Box::new(A { value: 5 });
		let a2: Box<dyn MyTrait> = Box::new(A { value: 5 });

		assert!(a1 == a2);
	}

	#[test]
	fn a_and_a_different_value_not_equal() {
		let a1: Box<dyn MyTrait> = Box::new(A { value: 5 });
		let a2: Box<dyn MyTrait> = Box::new(A { value: 6 });

		assert!(a1 != a2);
	}

	#[test]
	fn b_and_b_same_value_equal() {
		let b1: Box<dyn MyTrait> = Box::new(B { value: 5 });
		let b2: Box<dyn MyTrait> = Box::new(B { value: 5 });

		assert!(b1 == b2);
	}

	#[test]
	fn b_and_b_different_value_not_equal() {
		let b1: Box<dyn MyTrait> = Box::new(B { value: 5 });
		let b2: Box<dyn MyTrait> = Box::new(B { value: 6 });

		assert!(b1 != b2);
	}

	#[test]
	fn a_and_b_same_value_not_equal() {
		let a: Box<dyn MyTrait> = Box::new(A { value: 5 });
		let b: Box<dyn MyTrait> = Box::new(B { value: 5 });

		assert!(a != b);
	}

	#[test]
	fn a_and_b_different_value_not_equal() {
		let a: Box<dyn MyTrait> = Box::new(A { value: 5 });
		let b: Box<dyn MyTrait> = Box::new(B { value: 6 });

		assert!(a != b);
	}

	#[test]
	fn comparaison_dont_move_box() {
		let a: Box<dyn MyTrait> = Box::new(A { value: 5 });
		let b: Box<dyn MyTrait> = Box::new(B { value: 6 });

		assert!(!(a == b));
		println!("{:?}", b);
	}
}

mod with_ref {
	use super::*;

	#[test]
	fn a_and_a_same_value_equal() {
		let a1: &dyn MyTrait = &A { value: 5 };
		let a2: &dyn MyTrait = &A { value: 5 };

		assert!(a1 == a2);
	}

	#[test]
	fn a_and_a_different_value_not_equal() {
		let a1: &dyn MyTrait = &A { value: 5 };
		let a2: &dyn MyTrait = &A { value: 6 };

		assert!(a1 != a2);
	}

	#[test]
	fn b_and_b_same_value_equal() {
		let b1: &dyn MyTrait = &B { value: 5 };
		let b2: &dyn MyTrait = &B { value: 5 };

		assert!(b1 == b2);
	}

	#[test]
	fn b_and_b_different_value_not_equal() {
		let b1: &dyn MyTrait = &B { value: 5 };
		let b2: &dyn MyTrait = &B { value: 6 };

		assert!(b1 != b2);
	}

	#[test]
	fn a_and_b_same_value_not_equal() {
		let a: &dyn MyTrait = &A { value: 5 };
		let b: &dyn MyTrait = &B { value: 5 };

		assert!(a != b);
	}

	#[test]
	fn a_and_b_different_value_not_equal() {
		let a: &dyn MyTrait = &A { value: 5 };
		let b: &dyn MyTrait = &B { value: 6 };

		assert!(a != b);
	}
}

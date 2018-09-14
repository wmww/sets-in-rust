// this was one of my first Rust projects, written on MUNI on Jun 19th, 2017
enum Set {
	Is(f64),
	Or(Vec<Set>),
	And(Vec<Set>),
	Comp(Box<Set>, CompOp),
}

struct CompOp {
	greater: bool, // if greater than, less than otherwise
	or_eq: bool, // if or equal to
}

fn does_val_compare_to_set(val: f64, set: &Set, op: &CompOp) -> bool {
	match *set {
		Set::Is(super_val) => {
			return if op.greater {
				if op.or_eq {val >= super_val} else {val > super_val}
			} else {
				if op.or_eq {val <= super_val} else {val < super_val}
			}
		},
		Set::Or(ref super_children) => {
			for super_child in super_children {
				if !does_val_compare_to_set(val, super_child, &op) {
					return false;
				}
			}
			return true;
		},
		Set::And(ref super_children) => {
			for super_child in super_children {
				if does_val_compare_to_set(val, super_child, &op) {
					return true;
				}
			}
			return false;
		},
		Set::Comp(ref comp_set, ref comp_op) => {
			if sub_op.greater == op.greater { // if they are both greater than or both less than
				return false;
			} else {
				let new_op = CompOp{greater: op.greater, or_eq: op.or_eq || !comp_op.or_eq};
				return does_val_compare_to_set(val, comp_set, &new_op);
			}
		},
	}
}

fn is_val_in_set(val: f64, set: &Set) -> bool {
	match *set {
		Set::Is(super_val) => {
			return val == super_val;
		},
		Set::Or(ref super_children) => {
			for super_child in super_children {
				if is_val_in_set(val, super_child) {
					return true;
				}
			}
			return false;
		},
		Set::And(ref super_children) => {
			for super_child in super_children {
				if !is_val_in_set(val, super_child) {
					return false;
				}
			}
			return true;
		},
		Set::Comp(ref comp_set, ref comp_op) => {
			return does_val_compare_to_set(val, comp_set, comp_op);
		}
	}
}

fn is_subset(sub_set: &Set, super_set: &Set) -> bool {

	match *sub_set {
		Set::Is(sub_val) => {
			return is_val_in_set(sub_val, super_set);
		},
		Set::Or(ref sub_children) => {
			for sub_child in sub_children {
				if is_subset(sub_child, super_set) {
					return true;
				}
			}
			return false;
		},
		Set::And(ref sub_children) => {
			for sub_child in sub_children {
				if !is_subset(sub_child, super_set) {
					return false;
				}
			}
			return true;
		},
		Set::Comp(ref comp_set, ref comp_op) => {
			println!("not implemented!");
		}
	}

	return false;
}

fn main() {
	let set_a = Set::Is(4.0);
	let set_b = Set::Or(vec![Set::Is(2.0), Set::Is(3.0), Set::Is(4.0)]);
	println!("set is subset? {}", is_subset(&set_a, &set_b));
}

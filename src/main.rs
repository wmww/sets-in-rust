
enum Set {
	Is(f64),
	Or(Vec<Set>),
	And(Vec<Set>),
}

fn is_val_in_set(val: f64, set: &Set) -> bool {
	match set {
		&Set::Is(super_val) => {
			return val == super_val;
		},
		&Set::Or(ref super_children) => {
			for super_child in super_children {
				if is_val_in_set(val, super_child) {
					return true;
				}
			}
			return false;
		},
		&Set::And(ref super_children) => {
			for super_child in super_children {
				if !is_val_in_set(val, super_child) {
					return false;
				}
			}
			return true;
		},
	}
}

fn is_subset(sub_set: &Set, super_set: &Set) -> bool {
	
	match sub_set {
		&Set::Is(sub_val) => {
			return is_val_in_set(sub_val, super_set);
		},
		&Set::Or(ref sub_children) => {
			for sub_child in sub_children {
				if is_subset(sub_child, super_set) {
					return true;
				}
			}
			return false;
		},
		&Set::And(ref sub_children) => {
			for sub_child in sub_children {
				if !is_subset(sub_child, super_set) {
					return false;
				}
			}
			return true;
		},
	}
	
	return false;
}

fn main() {
	let set_a = Set::Is(4.0);
	let set_b = Set::Or(vec![Set::Is(2.0), Set::Is(3.0), Set::Is(4.0)]);
	println!("set is subset? {}", is_subset(&set_a, &set_b));
}

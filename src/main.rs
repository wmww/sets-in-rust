
enum Set {
	Is(f64),
	Or(Vec<Set>),
	And(Vec<Set>),
}

fn is_subset(sub_set: &Set, super_set: &Set) -> bool {
	
	match sub_set {
		&Set::Is(ref sub_val) => {
			match super_set {
				&Set::Is(ref super_val) => {
					return sub_val == super_val;
				},
				&Set::Or(ref super_children) => {
					for super_child in super_children {
						if is_subset(sub_set, super_child) {
							return true;
						}
					}
					return false;
				},
				&Set::And(ref super_children) => {
					for super_child in super_children {
						if !is_subset(sub_set, super_child) {
							return false;
						}
					}
					return true;
				},
			}
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

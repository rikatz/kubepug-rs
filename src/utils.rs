pub fn should_parse(group: &str, ignoregroup: &Vec<&str>, includegroup: &Vec<&str>) -> bool {
    if group == "" || !group.contains(".") {
		return true
    }

	if ignoregroup.contains(&group) {
		return false
	}

	for i in  includegroup {
		if group.ends_with(i) {
			return true
		}
	}

	includegroup.len() == 0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_accept_core() {
		let empty: Vec<&str> = Vec::new();
		let result = should_parse("", &empty, &empty);
		assert!(result)
    }

	#[test]
	fn ignore_some_apis() {
		let ignored_groups: Vec<&str> = vec!["something.k8s.io"];
		let included_groups: Vec<&str> = vec!["k8s.io"];
		let result = should_parse("something.k8s.io", &ignored_groups, &included_groups);
		assert!(!result)
	}

	#[test]
    fn should_accept_with_no_groups() {
		let empty: Vec<&str> = Vec::new();
		let result = should_parse("bla.k8s.io", &empty, &empty);
		assert!(result)
    }

	#[test]
	fn do_not_ignore_included_groups() {
		let ignored_groups: Vec<&str> = vec![];
		let included_groups: Vec<&str> = vec!["random.api"];
		let result = should_parse("bla.random.api", &ignored_groups, &included_groups);
		assert!(result)
	}

	#[test]
	fn ignore_api_outside_included_group() {
		let ignored_groups: Vec<&str> = vec![];
		let included_groups: Vec<&str> = vec!["random.api"];
		let result = should_parse("bla.other.api", &ignored_groups, &included_groups);
		assert!(!result)
	}

	#[test]
	fn ignore_if_inside_ignored_vec() {
		let ignored_groups: Vec<&str> = vec!["random.api", "gateway.api"];
		let included_groups: Vec<&str> = vec!["other.api"];
		let result = should_parse("bla.gateway.api", &ignored_groups, &included_groups);
		assert!(!result)
	}

			#[test]
	fn never_ignore_v1_api() {
		let ignored_groups: Vec<&str> = vec!["random.api", "gateway.api"];
		let included_groups: Vec<&str> = vec!["k8s.io"];
		let result = should_parse("apps", &ignored_groups, &included_groups);
		assert!(result)
	}


}
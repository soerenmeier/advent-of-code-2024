use std::{
	cmp::Ordering,
	collections::{HashMap, HashSet},
	fs,
};

fn main() {
	let content = fs::read_to_string("./inputs/day5.txt").unwrap();

	let (rules_raw, updates_raw) = content.split_once("\n\n").unwrap();

	// now parse the rules
	let mut rules: HashMap<usize, Rule> = HashMap::new();

	for rule in rules_raw.lines() {
		let mut iter = rule.split('|').map(|n| n.parse::<usize>().unwrap());
		let a = iter.next().unwrap();
		let b = iter.next().unwrap();

		let rule_a = rules.entry(a).or_default();
		rule_a.after.insert(b);

		let rule_b = rules.entry(b).or_default();
		rule_b.before.insert(a);
	}

	let mut total = 0;

	for update in updates_raw.lines() {
		let pages = update
			.split(',')
			.map(|n| n.parse::<usize>().unwrap())
			.collect::<Vec<_>>();

		let is_sorted = pages.is_sorted_by(|a, b| cmp(*a, *b, &rules).is_le());

		// the order is not valid
		if !is_sorted {
			continue;
		}

		total += pages[pages.len() / 2];
	}

	println!("Total: {total}");

	let mut total = 0;

	for update in updates_raw.lines() {
		let mut pages = update
			.split(',')
			.map(|n| n.parse::<usize>().unwrap())
			.collect::<Vec<_>>();

		let is_sorted = pages.is_sorted_by(|a, b| cmp(*a, *b, &rules).is_le());

		// the order is valid
		if is_sorted {
			continue;
		}

		pages.sort_by(|a, b| cmp(*a, *b, &rules));

		total += pages[pages.len() / 2];
	}

	println!("Newly ordered total: {total}");
}

#[derive(Debug, Default)]
struct Rule {
	before: HashSet<usize>,
	after: HashSet<usize>,
}

fn cmp(a: usize, b: usize, rules: &HashMap<usize, Rule>) -> Ordering {
	rules
		.get(&a)
		.map(|rule| {
			// does b need to be before a or after
			if rule.before.contains(&b) {
				Ordering::Greater
			} else if rule.after.contains(&b) {
				Ordering::Less
			} else {
				Ordering::Equal
			}
		})
		.unwrap_or(Ordering::Equal)
}

extern crate iterslide ;

use iterslide::SlideIterator;

pub fn build_proverb(list: &[&str]) -> String {
	
	let mut s: Vec<String> = Vec::with_capacity(list.len());
	
	let mut l = list.iter().peekable();
	
	match l.peek() {
		Some(x) => s.push(format!("And all for the want of a {}.", x)),
		None => return String::from(""),
	};
	
	let l = l.collect::<Vec<&&str>>();
	
	for i in l.into_iter().rev().slide(2) {
		s.push(format!("For want of a {} the {} was lost.", i[1], i[0]))
	}
	
	s.reverse();
	s.join("\n")
}

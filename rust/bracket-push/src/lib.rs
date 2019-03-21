pub fn brackets_are_balanced(string: &str) -> bool {
    let mut b: Vec<char> = Vec::with_capacity(string.len());
	let mut ok: bool = true;
	
	for c in string.chars() {
		match c {
			'[' =>  b.push('['),
			'(' =>  b.push('('),
			'{' =>  b.push('{'),
			']' =>  match b.pop() {
						Some('[') => (),
						Some(_) =>  {ok = false;
										break;},
						None =>  {ok = false;
										break;},
						},
			')' =>  match b.pop() {
						Some('(') => (),
						Some(_) =>  {ok = false;
										break;},
						None =>  {ok = false;
										break;},
						},
			'}' =>  match b.pop() {
						Some('{') => (),
						Some(_) =>  {ok = false;
										break;},
						None =>  {ok = false;
										break;},
						},
			_ => (),
		
		}
	}
	if b.len() != 0 {
		ok = false
	}
	ok
}

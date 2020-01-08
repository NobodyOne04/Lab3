pub fn handler(args:&Vec<String>, arg:&'static str, default:&'static str)->String {
	let index = args.iter().position(|unit| unit.to_string() == arg);
    let result :i64 = if index.is_some() {index.unwrap() as i64} else {-1};
    if result == -1 {return default.to_string()}
    if args.len()-1 <= result as usize {
    	panic!("specify arg like: {message} value", message = arg);
    }
	args[result as usize + 1].to_string()
}

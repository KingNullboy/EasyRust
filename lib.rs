use std::io;

pub fn inputs(que: &str) -> Result<String, Box<dyn std::error::Error>> {
	print!("{que}");
	std::io::Write::flush(&mut std::io::stdout())?;

	let mut output = String::new();
	io::stdin().read_line(&mut output).expect("Failed to read input");

	Ok(output.trim().to_string())
}

pub fn inputi(que: &str) -> Result<i64, Box<dyn std::error::Error>> {
	print!("{que}");
	std::io::Write::flush(&mut std::io::stdout())?;

	let mut output = String::new();
	io::stdin().read_line(&mut output).expect("Failed to read input");

	Ok(output.trim().parse()?)
}
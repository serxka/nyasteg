fn main() {
	// Encode if `-e` passed, otherwise decode
	let do_encode = {
		let arg = std::env::args().nth(1);
		matches!(arg, Some(f) if f == "-e")
	};

	let input = std::io::BufReader::new(std::io::stdin());
	let output = std::io::BufWriter::new(std::io::stdout());

	match do_encode {
		true => nyasteg::encode(input, output).unwrap(),
		false => nyasteg::decode(input, output).unwrap(),
	}
}

use std::collections::HashMap;
use std::io::{BufRead, Error, Read, Write};

pub const WORD_LIST: &[&str; 32] = &[
	"nya",
	"mrrrpm",
	"meeow",
	"meow",
	"yowl",
	"meoowww",
	"miau",
	"purrrr",
	"mreeoowww",
	"ppurr",
	"mreeoww",
	"nnyaa",
	"nyaaa",
	"mnyaa",
	"mrp",
	"purr",
	"myaa",
	"myaaa",
	"kre",
	"purrrrr",
	"mrrp",
	"mreeoww",
	"kree",
	"mrrrp",
	"chirrup",
	"mya",
	"mew",
	"mreow",
	"nyaa",
	"chirup",
	"meoow",
	"yoowll",
];

pub fn decode<R: BufRead, W: Write>(mut input: R, mut output: W) -> Result<(), Error> {
	let table = decode_table();

	let mut buf = [0u8; 8];
	let mut i = 0;
	let mut word = Vec::new();
	loop {
		word.clear();
		match input.read_until(b' ', &mut word) {
			Ok(0) => break,
			Ok(_) => (),
			Err(e) => return Err(e),
		}

		match table.get(&word[..word.len() - 1]) {
			Some(index) => {
				buf[i] = *index as _;
				i += 1;
				if i == 8 {
					decode_chunk(&buf, &mut output)?;
					buf = [0u8; 8];
					i = 0;
				}
			}
			None => continue,
		};
	}

	if i != 0 {
		decode_chunk(&buf, &mut output)?;
	}

	output.flush()
}

fn decode_chunk<W: Write>(chunk: &[u8; 8], output: &mut W) -> Result<(), Error> {
	output.write(&[(chunk[0] << 3) | (chunk[1] >> 2)])?;
	output.write(&[(chunk[1] << 6) | (chunk[2] << 1) | (chunk[3] >> 4)])?;
	output.write(&[(chunk[3] << 4) | (chunk[4] >> 1)])?;
	output.write(&[(chunk[4] << 7) | (chunk[5] << 2) | (chunk[6] >> 3)])?;
	output.write(&[(chunk[6] << 5) | chunk[7]])?;

	Ok(())
}

fn decode_table() -> HashMap<&'static [u8], usize> {
	let mut table = HashMap::new();
	for (i, &word) in WORD_LIST.iter().enumerate() {
		table.insert(word.as_bytes(), i);
	}
	table
}

pub fn encode<R: Read, W: Write>(input: R, mut output: W) -> Result<(), Error> {
	let mut buf = [0u8; 5];
	let mut i = 0;
	for byte in input.bytes() {
		match byte {
			Ok(b) => {
				buf[i] = b;
				i += 1;
				if i == 5 {
					encode_chunk(&buf, &mut output)?;
					buf = [0u8; 5];
					i = 0;
				}
			}
			Err(e) => return Err(e),
		}
	}

	// Write whatever is left over
	if i != 0 {
		encode_chunk(&buf, &mut output)?;
	}

	output.flush()
}

fn encode_chunk<W: Write>(chunk: &[u8; 5], output: &mut W) -> Result<(), Error> {
	let indices = [
		((chunk[0] & 0xF8) >> 3) as usize,
		(((chunk[0] & 0x07) << 2) | ((chunk[1] & 0xC0) >> 6)) as usize,
		((chunk[1] & 0x3E) >> 1) as usize,
		(((chunk[1] & 0x01) << 4) | ((chunk[2] & 0xF0) >> 4)) as usize,
		(((chunk[2] & 0x0F) << 1) | (chunk[3] >> 7)) as usize,
		((chunk[3] & 0x7C) >> 2) as usize,
		(((chunk[3] & 0x03) << 3) | ((chunk[4] & 0xE0) >> 5)) as usize,
		(chunk[4] & 0x1F) as usize,
	];

	for index in indices {
		output.write_fmt(format_args!("{} ", WORD_LIST[index]))?;
	}

	Ok(())
}

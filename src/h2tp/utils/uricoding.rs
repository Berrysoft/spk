use crate::h2tp::utils::uricoding_excepts::ENCODE_URI_EXCEPTS;

const UPPERHEX: &[u8] = "0123456789ABCDEF".as_bytes();

pub fn encode_uri(dist: &mut String, src: &str) {
	let bytes = src.as_bytes();
	for i in 0..bytes.len() {
		let b = bytes[i];
		if b < 128 && ENCODE_URI_EXCEPTS[b as usize] {
			dist.push(b as char);
			continue;
		}
		dist.push('%');
		dist.push((UPPERHEX[(b as usize) >> 4]) as char);
		dist.push((UPPERHEX[(b as usize) & 15]) as char)
	}
}

pub fn encode_uri_component(_dist: &mut str, _src: &str) {}

pub fn decode_uri(_dist: &mut str, _src: &str) {}

#[cfg(test)]
mod tests {
	use crate::h2tp::utils::uricoding::encode_uri;

	#[test]
	fn test_encode_uri() {
		let mut dist = String::with_capacity(100);
		encode_uri(&mut dist, "ABC abc 123😄");
		println!("{}", dist);
	}
}
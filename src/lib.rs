pub fn decode(url: String) -> String {
    let mut decoded = String::from("");
    let mut skip = 0;
    for i in 0..url.len() {
        if skip != 0 {
            skip -= 1;
            continue;
        }
        let c: char = url.chars().nth(i).unwrap();
        if c == '%' {
            let left = url.chars().nth(i + 1).unwrap();
            let right = url.chars().nth(i + 2).unwrap();
            let byte = u8::from_str_radix(&format!("{}{}", left, right), 16).unwrap();
            decoded += &(byte as char).to_string();
            skip = 2;
        } else {
            decoded += &c.to_string();
        }
    }
    decoded
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("https://github.com", crate::decode(String::from("https%3A%2F%2Fgithub.com")));
    }
}

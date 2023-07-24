pub struct RunLengthEncoding {}

impl RunLengthEncoding {
    pub fn encode(s: &str) -> String {
        let mut s = s.split("");
        let mut encoding_str = String::new();
        let mut count = 1;
        let mut current: &str = s.nth(1).unwrap();

        for c in s {
            if c != current || count == 9 {
                encoding_str.push_str(&i32::to_string(&count));
                encoding_str.push_str(current);
                current = c;
                count = 1;
            } else {
                count += 1;
            }
        }

        encoding_str
    }

    pub fn decode(s: &str) -> String {
        let mut digit = String::new();
        let mut encoding_str = String::new();

        s.chars().for_each(|v| {
            if v.is_digit(10) {
                digit.push(v)
            } else {
                for _ in 0..digit.parse::<u32>().unwrap() {
                    encoding_str.push(v);
                }
                digit = String::new();
            }
        });

        encoding_str
    }
}

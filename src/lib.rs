mod tests;

pub fn encode(source: &str) -> String {
    (source.to_owned() + "1")
        .chars()
        .fold(
            (String::new(), '1', 0),
            |(encoding, last_char, count), char| {
                if char != last_char && last_char != '1' {
                    (
                        encoding
                            + if count > 1 {
                                format!("{}{}", count, last_char)
                            } else {
                                last_char.to_string()
                            }
                            .as_str(),
                        char,
                        1,
                    )
                } else {
                    (encoding, char, count + 1)
                }
            },
        )
        .0
}

pub fn decode(source: &str) -> String {
    (source.to_string() + "1")
        .chars()
        .fold((String::new(), String::new()), |(result, count), char| {
            if char.is_digit(10) {
                (result, (count + char.to_string().as_str()))
            } else {
                (
                    result
                        + char
                            .to_string()
                            .repeat(usize::from_str_radix(count.as_str(), 10).unwrap_or(1))
                            .as_str(),
                    String::new(),
                )
            }
        })
        .0
}

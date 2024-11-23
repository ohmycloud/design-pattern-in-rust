use std::io::prelude::*;

#[derive(Debug)]
pub enum QuotesSates {
    LeftQuote,
    RightQuote,
    Ignore,
}

#[derive(Debug)]
pub struct CollectQuotes {
    pub buffer: String,
    pub saved: Vec<String>,
    pub state: QuotesSates,
}

impl CollectQuotes {
    pub fn is_left_quote(&self, item: &str) -> bool {
        let left_quotes: Vec<String> = vec!["\"".into(), "“".into()];
        let mut result = false;

        for quote in left_quotes {
            if quote.eq(item) {
                result = true;
            }
        }
        result
    }

    pub fn is_right_quote(&self, item: &str) -> bool {
        let right_quotes: Vec<String> = vec!["\"".into(), "”".into()];
        let mut result = false;

        for quote in right_quotes {
            if quote.eq(item) {
                result = true;
            }
        }
        result
    }

    pub fn new() -> Self {
        CollectQuotes {
            buffer: String::new(),
            saved: vec![],
            state: QuotesSates::Ignore,
        }
    }

    pub fn process(&mut self, item: &str) {
        // println!("debug: item: {} -- {:?}", item, &self);

        match self.state {
            QuotesSates::LeftQuote => match self.is_right_quote(item) {
                true => {
                    self.state = QuotesSates::RightQuote;
                    self.buffer.push_str(item);
                    self.saved.push(self.buffer.clone());

                    // clear buffer
                    self.buffer = String::new();
                }
                false => self.buffer.push_str(item),
            },
            QuotesSates::RightQuote => match self.is_left_quote(item) {
                true => {
                    self.state = QuotesSates::LeftQuote;
                    self.buffer.push_str(item);
                }
                false => self.state = QuotesSates::Ignore,
            },
            QuotesSates::Ignore => match self.is_left_quote(item) {
                true => {
                    self.state = QuotesSates::LeftQuote;
                    self.buffer.push_str(item);
                }
                false => {}
            },
        }
    }
}

fn process_string(input: String) -> CollectQuotes {
    // Will process the string as bytes to handle utf8
    let string_as_bytes = input.as_bytes();

    // Initialize my state machine
    let mut dfa = CollectQuotes::new();

    // Temp storage for bytes that not valid utf8 strings yet
    let mut temp = Vec::new();

    // Processing one byte at a time
    for byte in string_as_bytes.bytes() {
        let item = byte.unwrap();
        temp.push(item);

        match String::from_utf8(temp.clone()) {
            Ok(character) => {
                // Process that one character
                dfa.process(&character);

                // CLear the temp buffer
                temp.clear();
            }
            Err(_) => println!("Bytes list is not a valid itf8 string yet: {:?}", temp),
        }
    }
    dfa
}
fn main() {
    let input = "Marcus said, \"Yoo! Have you eaten? \". I replied, \"Not yet. I am currently looking for food now, How about you?\"".to_string();
    let dfa = process_string(input);
    println!("DFA: {:?}", dfa);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_quote() {
        let dfa = CollectQuotes::new();
        assert_eq!(dfa.is_right_quote("\""), true);
        assert_eq!(dfa.is_right_quote("”"), true);
    }

    #[test]
    fn test_left_quote() {
        let dfa = CollectQuotes::new();
        assert_eq!(dfa.is_left_quote("\""), true);
        assert_eq!(dfa.is_left_quote("“"), true);
    }

    #[test]
    fn test_process_with_english() {
        let input = "Marcus said, \"Yoo! Have you eaten? \". I replied, \"Not yet. I am currently looking for food now, How about you?\"".to_string();
        let dfa = process_string(input);
        assert_eq!("\"Yoo! Have you eaten? \"".to_string(), dfa.saved[0]);
        assert_eq!(
            "\"Not yet. I am currently looking for food now, How about you?\"".to_string(),
            dfa.saved[1]
        );
    }

    #[test]
    fn test_process_with_chinese() {
        let input =
            "李白问王昌龄, “桃花潭的水深吗？”。 王昌龄回答, “水不在深，有龙则灵！”".to_string();
        let dfa = process_string(input);
        assert_eq!("“桃花潭的水深吗？”".to_string(), dfa.saved[0]);
        assert_eq!("“水不在深，有龙则灵！”".to_string(), dfa.saved[1]);
    }
}

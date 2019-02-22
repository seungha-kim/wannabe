pub struct Tokenizer {
    input: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Mate(String),
    Date(String),
    In(String),
    Out(String),
    LineSep,
    Eof,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer { input }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut result = vec![];
        for line in self.input.split("\n") {
            let line = line.trim();
            if line.len() == 0 {
                // DO NOTHING
            } else if line.starts_with("+") {
                result.push(Token::In(String::from(&line[1..])))
            } else if line.starts_with("-") {
                result.push(Token::Out(String::from(&line[1..])))
            } else if line.starts_with("~") {
                result.push(Token::Date(String::from(&line[1..])))
            } else if line.starts_with("*") {
                for word in line[1..].split(",") {
                    let word = word.trim();
                    result.push(Token::Mate(String::from(word)));
                }
            }
            result.push(Token::LineSep);
        }
        result.push(Token::Eof);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Token::*;
    use super::*;
    #[test]
    fn works() {
        let sample = "+hudson
+jimmy
+lance
+dan

~20190225

*dan,lance,jimmy
-hudson
+rio";
        let tokenizer = Tokenizer::new(String::from(sample));
        let result = tokenizer.tokenize();
        assert_eq!(
            &result[..],
            &[
                In(String::from("hudson")),
                LineSep,
                In(String::from("jimmy")),
                LineSep,
                In(String::from("lance")),
                LineSep,
                In(String::from("dan")),
                LineSep,
                LineSep,
                Date(String::from("20190225")),
                LineSep,
                LineSep,
                Mate(String::from("dan")),
                Mate(String::from("lance")),
                Mate(String::from("jimmy")),
                LineSep,
                Out(String::from("hudson")),
                LineSep,
                In(String::from("rio")),
                LineSep,
                Eof,
            ]
        );
    }
}

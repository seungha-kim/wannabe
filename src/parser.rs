use super::tokenizer::Token;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub struct Connection<'a>(&'a str, &'a str);

pub struct Parser<'a> {
    input: &'a Vec<Token>,
}

pub struct ParserResult<'a> {
    available_members: HashSet<String>,
    connections: Vec<Connection<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &Vec<Token>) -> Parser {
        Parser { input }
    }

    pub fn parse(&self) -> ParserResult {
        let mut result = ParserResult {
            available_members: HashSet::new(),
            connections: Vec::new(),
        };
        let mut it = self.input.iter();
        let mut mate_buffer: Vec<&str> = Vec::with_capacity(10);
        'outer: loop {
            // println!("loop");
            for token in
                it.peeking_take_while(|v| if let Token::LineSep = v { true } else { false })
            {
                println!("token {:?}", token);
            }
            for token in
                it.peeking_take_while(|v| if let Token::Date(_) = v { true } else { false })
            {
                println!("token {:?}", token);
            }
            for token in it.peeking_take_while(|v| if let Token::Eof = v { true } else { false }) {
                println!("token {:?}", token);
                break 'outer;
            }
            for token in it.peeking_take_while(|v| {
                // println!("take_while {:?}", v);
                if let Token::In(ref name) = v {
                    true
                } else {
                    false
                }
            }) {
                println!("token {:?}", token);
                // FIXME
                if let Token::In(ref name) = token {
                    result.available_members.insert(String::from(&name[..]));
                }
            }
            for token in it.peeking_take_while(|v| {
                // println!("take_while {:?}", v);

                if let Token::Out(ref name) = v {
                    true
                } else {
                    false
                }
            }) {
                println!("token {:?}", token);
                // FIXME
                if let Token::Out(ref name) = token {
                    result.available_members.remove(name);
                }
            }
            for token in it.peeking_take_while(|v| {
                // println!("take_while {:?}", v);

                if let Token::Mate(ref name) = v {
                    true
                } else {
                    false
                }
            }) {
                println!("token {:?}", token);
                // FIXME
                if let Token::Mate(ref name) = token {
                    mate_buffer.push(name);
                }
            }
            if mate_buffer.len() == 1 {
                panic!("There is no mate for: {}", mate_buffer[0]);
            }
            for (mate1, mate2) in mate_buffer.iter().tuple_combinations() {
                result.connections.push(Connection(mate1, mate2));
            }
            mate_buffer.clear();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::tokenizer::Tokenizer;
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
        let tokenize_result = tokenizer.tokenize();
        let parser = Parser::new(&tokenize_result);
        let parse_result = parser.parse();
        assert_eq!(parse_result.available_members.len(), 4);
        assert!(parse_result.available_members.contains("dan"));
        assert!(parse_result.available_members.contains("lance"));
        assert!(parse_result.available_members.contains("jimmy"));
        assert!(parse_result.available_members.contains("rio"));
        assert!(!parse_result.available_members.contains("hudson"));

        assert_eq!(parse_result.connections.len(), 3);
        assert_eq!(&parse_result.connections[0], &Connection("dan", "lance"));
        assert_eq!(&parse_result.connections[1], &Connection("dan", "jimmy"));
        assert_eq!(&parse_result.connections[2], &Connection("lance", "jimmy"));
    }
}

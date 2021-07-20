use core::fmt;
use std::{fmt::write, iter::Peekable, str::Chars};

pub fn parse_htmlisp(input: &str) -> String {
    format!(
        "{}",
        Parser::new(input)
            .parse()
            .expect("Failed to parse HTMLisp :(")
    )
}

#[derive(Debug)]
enum Node {
    Text(String),
    Tag {
        name: String,
        attributes: Vec<(String, String)>,
        inner: Vec<Node>,
    },
}

struct Parser<'input> {
    input: Peekable<Chars<'input>>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<Node> {
        match self.input.peek()? {
            '(' => self.parse_tag(),
            '"' => self.parse_string().map(|x| Node::Text(x)),
            _ => unreachable!(),
        }
    }

    fn parse_tag(&mut self) -> Option<Node> {
        self.input.next();
        let name: String = self
            .input
            .by_ref()
            .take_while(char::is_ascii_alphanumeric)
            .collect();

        self.skip_whitespace()?;

        let mut attributes: Vec<(String, String)> = vec![];
        while self.input.peek()? == &':' {
            self.input.next();

            let attr: String = self
                .input
                .by_ref()
                .take_while(char::is_ascii_alphanumeric)
                .collect();

            self.skip_whitespace();

            let value = if self.input.peek()? == &':' {
                "".to_string()
            } else {
                if self.input.peek()? != &'"' {
                    return None;
                } else {
                    self.parse_string()?
                }
            };
            attributes.push((attr, value));

            self.skip_whitespace()?;
        }

        let mut inner: Vec<Node> = vec![];
        while {
            let peek = self.input.peek()?;
            peek == &'(' || peek == &'"'
        } {
            inner.push(self.parse()?);
            self.skip_whitespace();
        }

        self.skip_whitespace();
        self.input.next();

        Some(Node::Tag {
            name,
            attributes,
            inner,
        })
    }

    fn parse_string(&mut self) -> Option<String> {
        self.input.next();

        let mut string = String::new();
        while self.input.peek()? != &'"' {
            string.push(self.input.next()?);
        }
        self.input.next();
        Some(string)
    }

    fn skip_whitespace(&mut self) -> Option<()> {
        while self.input.peek()?.is_whitespace() {
            self.input.next();
        }
        Some(())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Text(s) => s.to_string(),
                Self::Tag {
                    name,
                    attributes,
                    inner,
                } => {
                    format!(
                        "<{}{}>{}</{}>",
                        name,
                        attributes
                            .iter()
                            .map(|(attr, val)| format!(" {}=\"{}\"", attr, val))
                            .collect::<Vec<_>>()
                            .join(" "),
                        inner
                            .iter()
                            .map(|n| format!("{}", n))
                            .collect::<Vec<_>>()
                            .join(""),
                        name
                    )
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let test = r#"(img :src "https://via.placeholder.com/150")"#;
        let parsed = Parser::new(test).parse().unwrap();
        println!("Input: {}\nOutput:\n{}", test, parsed);
    }

    #[test]
    fn nested() {
        let test = r#"(div :style "background: white" (h1 "hello") (p "world"))"#;
        let parsed = Parser::new(test).parse().unwrap();
        println!("Input: {}\nOutput:\n{}", test, parsed);
    }

    #[test]
    fn multi_attr() {
        let test = r#"(meta :name "viewport" :content "width=device-width, initial-scale=1")"#;
        let parsed = Parser::new(test).parse().unwrap();
        println!("Input: {}\nOutput:\n{}", test, parsed);
    }
}

use core::fmt;
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Node {
    Text(String),
    Tag {
        name: String,
        attributes: Vec<(String, String)>,
        inner: Vec<Node>,
    },
}

pub struct Parser<'input> {
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
        let name = self.parse_ident()?;

        self.skip_whitespace()?;

        let mut attributes: Vec<(String, String)> = vec![];
        while self.input.peek()? == &':' {
            self.input.next();

            let attr = self.parse_ident()?;

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

    /// DAMN YOU `Iterator::take_while` WHY CAN'T YOU BE NORMAL AND PEEK BEFORE CONSUMING
    fn parse_ident(&mut self) -> Option<String> {
        if !self.input.peek()?.is_ascii_alphanumeric() {
            return None;
        }
        let mut attr = String::new();
        while self.input.peek()?.is_ascii_alphanumeric() {
            attr.push(self.input.next()?);
        }

        Some(attr)
    }

    fn parse_string(&mut self) -> Option<String> {
        let mut prev = self.input.next()?;
        let mut string = String::new();
        while !(self.input.peek()? == &'"') || (prev == '\\') {
            let next = self.input.next()?;
            string.push(next);
            prev = next;
        }

        self.input.next();
        string = string.replace("\\\"", "\"");
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
                        fmt_attrs(attributes),
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

impl Node {
    pub fn pretty_print(&self, depth: usize) -> String {
        match self {
            Self::Text(s) => format!("{}{}", "\t".repeat(depth), s),
            Self::Tag {
                name,
                attributes,
                inner,
            } => {
                format!(
                    "{}<{}{}>{}{}{}{}</{}>",
                    "\t".repeat(depth),
                    name,
                    fmt_attrs(attributes),
                    if inner.len() > 0 { "\n" } else { "" },
                    inner
                        .iter()
                        .map(|node| node.pretty_print(depth + 1))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    if inner.len() > 0 { "\n" } else { "" },
                    "\t".repeat(depth),
                    name
                )
            }
        }
    }
}

fn fmt_attrs(attrs: &[(String, String)]) -> String {
    attrs
        .iter()
        .map(|(attr, val)| format!(" {}=\"{}\"", attr, val))
        .collect::<Vec<_>>()
        .join(" ")
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

    #[test]
    fn escape_string() {
        let test = r#"(p "jioj\"jiojio\"")"#;
        let parsed = Parser::new(test).parse().unwrap();
        println!("Input: {}\nOutput:\n{}", test, parsed);
    }

    #[test]
    fn empty() {
        let test = r#"(html)"#;
        let parsed = Parser::new(test).parse().unwrap();
        println!("Input: {}\nOutput:\n{}", test, parsed);
    }
}

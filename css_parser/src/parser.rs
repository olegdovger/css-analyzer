use std::iter::{Iterator, Peekable};
use std::str::Chars;

use crate::color::Color;
use crate::selector::{Selector, SimpleSelector};
use crate::structs::{Declaration, Rule, Stylesheet, Unit, Value};

pub struct CssParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> CssParser<'a> {
    pub fn new(full_css: &str) -> CssParser {
        CssParser {
            chars: full_css.chars().peekable(),
        }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet::default();

        while self.chars.peek().is_some() {
            self.consume_while(char::is_whitespace);
            self.consume_comments();

            let selectors = self.parse_selectors();
            let styles = self.parse_declarations();
            let rule = Rule::new(selectors, styles);

            stylesheet.rules.push(rule);
        }

        stylesheet
    }

    fn consume_comments(&mut self) {
        let start = {
            if self.chars.peek() == Some(&'/') {
                self.chars.next();
            }

            self.chars.peek() == Some(&'*')
        };

        if start {
            let mut stop = false;

            while !stop {
                self.chars.next();

                stop = {
                    if self.chars.peek() == Some(&'*') {
                        self.chars.next();
                    }

                    self.chars.next() == Some('/')
                };

                if stop {
                    self.consume_while(char::is_whitespace);

                    if self.chars.peek() == Some(&'/') {
                        self.consume_comments()
                    }
                }
            }
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        while self.chars.peek().map_or(false, |c| *c != '{') {
            let selector = self.parse_selector();

            if selector != Selector::default() {
                selectors.push(selector);
            }

            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c == ',') {
                self.chars.next();
            }
        }

        self.chars.next();
        selectors
    }

    fn parse_selector(&mut self) -> Selector {
        let mut s_selector = SimpleSelector::default();
        let mut selector = Selector::default();

        self.consume_while(char::is_whitespace);

        s_selector.tag_name = match self.chars.peek() {
            Some(&c) if is_valid_start_ident(c) => Some(self.parse_identifier()),
            _ => None,
        };

        let mut multiple_ids = false;
        while self
            .chars
            .peek()
            .map_or(false, |c| *c != ',' && *c != '{' && !(*c).is_whitespace())
        {
            match self.chars.peek() {
                Some(&c) if c == '#' => {
                    self.chars.next();
                    if s_selector.id.is_some() || multiple_ids {
                        s_selector.id = None;
                        multiple_ids = true;
                        self.parse_id();
                    } else {
                        s_selector.id = self.parse_id();
                    }
                }
                Some(&c) if c == '.' => {
                    self.chars.next();
                    let class_name = self.parse_identifier();

                    if class_name != String::from("") {
                        s_selector.classes.push(class_name);
                    }
                }
                _ => {
                    self.consume_while(|c| c != ',' && c != '{');
                }
            }
        }

        if s_selector != SimpleSelector::default() {
            selector.simple.push(s_selector);
        }

        selector
    }

    fn parse_identifier(&mut self) -> String {
        let mut ident = String::new();

        match self.chars.peek() {
            Some(&c) => {
                if is_valid_start_ident(c) {
                    ident.push_str(&self.consume_while(is_valid_ident))
                }
            }
            None => {}
        }
        ident.to_lowercase()
    }

    fn parse_id(&mut self) -> Option<String> {
        match &self.parse_identifier()[..] {
            s if s.len() > 0 => Some(s.to_string()),
            _ => None,
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::<Declaration>::new();

        while self.chars.peek().map_or(false, |c| *c != '}') {
            self.consume_while(char::is_whitespace);
            self.consume_comments();

            let property = self.consume_while(|x| x != ':').to_lowercase();

            self.chars.next();
            self.consume_while(char::is_whitespace);

            let value = self
                .consume_while(|x| x != ';' && x != '\n' && x != '}')
                .to_lowercase();

            let value_enum = match property.as_ref() {
                "background-color" | "border-color" | "color" => {
                    Value::Color(Color::new(value.as_ref()))
                }
                "margin-right"
                | "margin-bottom"
                | "margin-left"
                | "margin-top"
                | "padding-right"
                | "padding-bottom"
                | "padding-left"
                | "padding-top"
                | "border-right-width"
                | "border-bottom-width"
                | "border-left-width"
                | "border-top-width"
                | "height"
                | "width" => translate_length(&value),
                _ => Value::Other(value),
            };

            let declaration = Declaration::new(property, value_enum);

            if self.chars.peek().map_or(false, |c| *c == ';') {
                declarations.push(declaration);
                self.chars.next();
            } else {
                self.consume_while(char::is_whitespace);

                if self.chars.peek().map_or(false, |c| *c == '}') {
                    declarations.push(declaration);
                }
            }
            self.consume_while(char::is_whitespace);
            self.consume_comments();
        }

        self.chars.next();
        declarations
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }

        result
    }
}

fn translate_length(value: &str) -> Value {
    let mut num_str = String::new();
    let mut unit = String::new();
    let mut parsing_num = true;

    for c in value.chars() {
        if (c.is_numeric() || c == '.') && parsing_num {
            num_str.push(c);
        } else {
            unit.push(c);
            parsing_num = false;
        }
    }

    let number = num_str.parse().unwrap_or(0.0);

    match unit.as_ref() {
        "em" => Value::Length(number, Unit::Em),
        "ex" => Value::Length(number, Unit::Ex),
        "ch" => Value::Length(number, Unit::Ch),
        "rem" => Value::Length(number, Unit::Rem),
        "vh" => Value::Length(number, Unit::Vh),
        "vw" => Value::Length(number, Unit::Vw),
        "vmin" => Value::Length(number, Unit::Vmin),
        "vmax" => Value::Length(number, Unit::Vmax),
        "px" | "" => Value::Length(number, Unit::Px),
        "mm" => Value::Length(number, Unit::Mm),
        "q" => Value::Length(number, Unit::Q),
        "cm" => Value::Length(number, Unit::Cm),
        "in" => Value::Length(number, Unit::In),
        "pt" => Value::Length(number, Unit::Pt),
        "pc" => Value::Length(number, Unit::Pc),
        "%" => Value::Length(number, Unit::Pct),
        _ => Value::Length(number, Unit::Px),
    }
}

fn is_valid_ident(c: char) -> bool {
    is_valid_start_ident(c) || c.is_digit(10) || c == '-'
}

fn is_valid_start_ident(c: char) -> bool {
    is_letter(c) || is_non_ascii(c) || c == '_'
}

fn is_letter(c: char) -> bool {
    is_upper_letter(c) || is_lower_letter(c)
}

fn is_upper_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_lower_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn is_non_ascii(c: char) -> bool {
    c >= '\u{0080}'
}

pub fn parse(content: &str) -> Stylesheet {
    let mut parser = CssParser::new(content.trim());

    parser.parse_stylesheet()
}

pub fn stringify(styles: Stylesheet) -> String {
    format!("{}", styles)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::color::ColorData;
    use std::fs::read_to_string;

    #[test]
    fn parse_string_empty() {
        assert_eq!(
            parse(""),
            Stylesheet { rules: vec![] },
            "parse empty string"
        );
    }

    #[test]
    fn parse_string_plain() {
        assert_eq!(
            parse("body {color: red;}"),
            Stylesheet {
                rules: vec![Rule {
                    selectors: vec![Selector {
                        simple: vec![SimpleSelector {
                            tag_name: Some("body".to_string()),
                            id: None,
                            classes: vec![]
                        }],
                        combinators: vec![],
                    }],
                    declarations: vec![Declaration {
                        property: "color".to_string(),
                        value: Value::Color(Color {
                            original: "red".to_string(),
                            pattern: "#XXXXXX".to_string(),
                            data: ColorData::RGBA {
                                r: 255.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0
                            }
                        })
                    }],
                }]
            },
            "parse plain css"
        );
    }

    #[test]
    fn stringify_plain_styles() {
        assert_eq!("", stringify(Stylesheet { rules: vec![] }), "empty result");

        let file_path = "../samples/plain.css";

        let contents = read_to_string(&file_path).expect("Something went wrong reading the file");

        assert_eq!(
            contents.trim(),
            stringify(Stylesheet {
                rules: vec![Rule {
                    selectors: vec![Selector {
                        simple: vec![SimpleSelector {
                            tag_name: Some("body".to_string()),
                            id: None,
                            classes: vec![]
                        }],
                        combinators: vec![],
                    }],
                    declarations: vec![Declaration {
                        property: "color".to_string(),
                        value: Value::Color(Color {
                            original: "red".to_string(),
                            pattern: "#XXXXXX".to_string(),
                            data: ColorData::RGBA {
                                r: 255.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0
                            }
                        })
                    }],
                }]
            })
            .trim(),
            "plain result"
        );
    }

    #[test]
    fn parse_stringify_empty() {
        assert_eq!("", stringify(parse("")), "empty result");
    }

    #[test]
    fn parse_stringify_plain_styles() {
        let file_path = "../samples/plain.css";
        let contents = read_to_string(&file_path).expect("Something went wrong reading the file");

        assert_eq!(contents.trim(), stringify(parse(&contents)));
    }

    #[test]
    fn parse_stringify_ethalon_css_file() {
        let file_path = "../samples/ethalon.css";
        let contents = read_to_string(&file_path).expect("Something went wrong reading the file");

        assert_eq!(contents.trim(), stringify(parse(&contents)));
    }

    #[test]
    fn parse_single_comment() {
        let mut parser = CssParser::new("/**/ body {color: red;}");

        parser.consume_comments();

        assert_eq!("body {color: red;}", parser.chars.collect::<String>());
    }

    #[test]
    fn parse_only_single_comment() {
        let mut parser = CssParser::new("/*  */ ");

        parser.consume_comments();

        assert_eq!(None, parser.chars.peek());
    }

    #[test]
    fn parse_comments() {
        let content_with_comments = read_to_string("../samples/comments/with.css")
            .expect("Something went wrong reading the file");
        let content_without_comments = read_to_string("../samples/comments/without.css")
            .expect("Something went wrong reading the file");

        assert_eq!(
            content_without_comments.trim(),
            stringify(parse(&content_with_comments))
        );
    }

    #[test]
    fn parse_selector() {
        let mut parser = CssParser::new("#we4");

        assert_eq!(
            parser.parse_selector(),
            Selector {
                simple: vec![SimpleSelector {
                    tag_name: None,
                    id: Some("we4".to_string()),
                    classes: vec![]
                }],
                combinators: vec![]
            },
            "parse id"
        );

        let mut parser = CssParser::new("#first, #second");

        assert_eq!(
            parser.parse_selector(),
            Selector {
                simple: vec![SimpleSelector {
                    tag_name: None,
                    id: Some("first".to_string()),
                    classes: vec![]
                }],
                combinators: vec![]
            },
            "parse ids"
        );
    }
}

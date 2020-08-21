use super::color::Color;
use super::selector::{Selector, SimpleSelector};

use std::default::Default;
use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}
#[derive(PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(PartialEq)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
}
#[derive(PartialEq)]
pub enum Value {
    Color(Color),
    Length(f32, Unit),
    Other(String),
}
#[derive(PartialEq)]
pub enum Unit {
    Em,
    Ex,
    Ch,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
    Px,
    Mm,
    Q,
    Cm,
    In,
    Pt,
    Pc,
    Pct,
}

#[allow(unreachable_patterns)]
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Em => write!(f, "{}", "em"),
            Unit::Ex => write!(f, "{}", "ex"),
            Unit::Ch => write!(f, "{}", "ch"),
            Unit::Rem => write!(f, "{}", "rem"),
            Unit::Vh => write!(f, "{}", "vh"),
            Unit::Vw => write!(f, "{}", "vw"),
            Unit::Vmin => write!(f, "{}", "vmin"),
            Unit::Vmax => write!(f, "{}", "vmax"),
            Unit::Px => write!(f, "{}", "px"),
            Unit::Mm => write!(f, "{}", "mm"),
            Unit::Q => write!(f, "{}", "q"),
            Unit::Cm => write!(f, "{}", "cm"),
            Unit::In => write!(f, "{}", "in"),
            Unit::Pt => write!(f, "{}", "pt"),
            Unit::Pc => write!(f, "{}", "pc"),
            Unit::Pct => write!(f, "{}", "%"),
            _ => write!(f, "{}", "px"),
        }
    }
}

impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Stylesheet {
        Stylesheet { rules }
    }
}
impl Default for Stylesheet {
    fn default() -> Self {
        Stylesheet { rules: Vec::new() }
    }
}
impl fmt::Debug for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_result = String::new();
        for rule in &self.rules {
            if rule_result.len() > 0 {
                rule_result.push_str("\n\n");
            }
            rule_result.push_str(&format!("{:?}", rule));
        }
        write!(f, "{}", rule_result)
    }
}

impl fmt::Display for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_result = String::new();
        for rule in &self.rules {
            if rule_result.len() > 0 {
                rule_result.push_str("\r\n\r\n");
            }
            rule_result.push_str(&format!("{}", rule));
        }
        write!(f, "{}", rule_result)
    }
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
        Rule {
            selectors,
            declarations,
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            selectors: Vec::new(),
            declarations: Vec::new(),
        }
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sel_result = String::new();
        let mut decl_result = String::new();
        let tab = "    ";

        for selector in &self.selectors {
            if sel_result.len() > 0 {
                sel_result.push_str(", ");
            }
            sel_result.push_str(&format!("{:?}", selector));
        }

        for declaration in &self.declarations {
            decl_result.push_str(tab);
            decl_result.push_str(&format!("{:?}", declaration));
            decl_result.push('\n');
        }

        write!(f, "{} {}", sel_result, decl_result)
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sel_result = String::new();
        let mut decl_result = String::new();
        let tab = "    ";

        for selector in &self.selectors {
            if sel_result.len() > 0 {
                sel_result.push_str(", ");
            }
            sel_result.push_str(&format!("{}", selector));
        }

        for declaration in &self.declarations {
            decl_result.push_str(tab);
            decl_result.push_str(&format!("{}", declaration));
            decl_result.push('\r');
            decl_result.push('\n');
        }

        write!(f, "{} {{\r\n{}}}", sel_result, decl_result)
    }
}

impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinators: Vec<char>) -> Selector {
        Selector {
            simple,
            combinators,
        }
    }
}
impl Default for Selector {
    fn default() -> Self {
        Selector {
            simple: Vec::new(),
            combinators: Vec::new(),
        }
    }
}
impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for sel in &self.simple {
            if result.len() > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{:?}", sel));
        }

        write!(f, "{}", result)
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for sel in &self.simple {
            if result.len() > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{}", sel));
        }

        write!(f, "{}", result)
    }
}

impl SimpleSelector {
    pub fn new(
        tag_name: Option<String>,
        id: Option<String>,
        classes: Vec<String>,
    ) -> SimpleSelector {
        SimpleSelector {
            tag_name,
            id,
            classes,
        }
    }
}

impl Default for SimpleSelector {
    fn default() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
        }
    }
}

impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        match self.tag_name {
            Some(ref t) => result.push_str(t),
            None => {}
        }

        match self.id {
            Some(ref s) => {
                result.push('#');
                result.push_str(s);
            }
            None => {}
        }

        for class in &self.classes {
            result.push('.');
            result.push_str(class);
        }

        write!(f, "{}", result)
    }
}

impl fmt::Display for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        match self.tag_name {
            Some(ref t) => result.push_str(t),
            None => {}
        }

        match self.id {
            Some(ref s) => {
                result.push('#');
                result.push_str(s);
            }
            None => {}
        }

        for class in &self.classes {
            result.push('.');
            result.push_str(class);
        }

        write!(f, "{}", result)
    }
}

impl Declaration {
    pub fn new(property: String, value: Value) -> Declaration {
        Declaration { property, value }
    }
}

impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            property: String::from(""),
            value: Value::Other(String::from("")),
        }
    }
}

impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.property, self.value)
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {};", self.property, self.value)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Color(ref c) => write!(f, "{:?}", c),
            Value::Length(l, _) => write!(f, "{:?}", l),
            Value::Other(ref s) => write!(f, "{:?}", s),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Color(ref c) => write!(f, "{}", c),
            Value::Length(l, u) => write!(f, "{}{}", l, u),
            Value::Other(ref s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit() {
        assert_eq!("em", format!("{}", Unit::Em));
        assert_eq!("ex", format!("{}", Unit::Ex));
        assert_eq!("ch", format!("{}", Unit::Ch));
        assert_eq!("rem", format!("{}", Unit::Rem));
        assert_eq!("vh", format!("{}", Unit::Vh));
        assert_eq!("vw", format!("{}", Unit::Vw));
        assert_eq!("vmin", format!("{}", Unit::Vmin));
        assert_eq!("vmax", format!("{}", Unit::Vmax));
        assert_eq!("px", format!("{}", Unit::Px));
        assert_eq!("mm", format!("{}", Unit::Mm));
        assert_eq!("q", format!("{}", Unit::Q));
        assert_eq!("cm", format!("{}", Unit::Cm));
        assert_eq!("in", format!("{}", Unit::In));
        assert_eq!("pt", format!("{}", Unit::Pt));
        assert_eq!("pc", format!("{}", Unit::Pc));
        assert_eq!("%", format!("{}", Unit::Pct));
    }

    #[test]
    fn length() {
        assert_eq!("1em", format!("{}", Value::Length(1.0, Unit::Em)));
        assert_eq!("1.5em", format!("{}", Value::Length(1.5, Unit::Em)));
        assert_eq!("20%", format!("{}", Value::Length(20.0, Unit::Pct)));
        // assert_eq!("ex", format!("{}", Unit::Ex));
        // assert_eq!("ch", format!("{}", Unit::Ch));
        // assert_eq!("rem", format!("{}", Unit::Rem));
        // assert_eq!("vh", format!("{}", Unit::Vh));
        // assert_eq!("vw", format!("{}", Unit::Vw));
        // assert_eq!("vmin", format!("{}", Unit::Vmin));
        // assert_eq!("vmax", format!("{}", Unit::Vmax));
        // assert_eq!("px", format!("{}", Unit::Px));
        // assert_eq!("mm", format!("{}", Unit::Mm));
        // assert_eq!("q", format!("{}", Unit::Q));
        // assert_eq!("cm", format!("{}", Unit::Cm));
        // assert_eq!("in", format!("{}", Unit::In));
        // assert_eq!("pt", format!("{}", Unit::Pt));
        // assert_eq!("pc", format!("{}", Unit::Pc));
        // assert_eq!("%", format!("{}", Unit::Pct));
    }
}

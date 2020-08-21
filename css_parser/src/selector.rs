#[derive(PartialEq, Eq)]
pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>,
}
#[derive(PartialEq, Eq)]
pub struct SimpleSelector {
    /// # Example: *
    // pub any: Option<bool>,
    /// Example: div
    pub tag_name: Option<String>,
    /// # Example: [name="value"]
    // pub attribute: Option<String>,
    /// # Example: :visited
    // pub pseudo_class: Option<String>,

    /// Example: #page
    pub id: Option<String>,
    /// Example: .wrapper
    pub classes: Vec<String>,
}

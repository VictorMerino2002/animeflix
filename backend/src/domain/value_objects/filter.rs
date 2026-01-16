#[derive(Debug)]
pub enum Filter {
    EqualsString(String, String),
    NotEqualsString(String, String),
    LessInt(String, i32),
    GreaterInt(String, i32),
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Token {
    //pub enum Token<'input> {
    LParen,
    RParen,
    Symbol(String),
    Str(String),
    Number(String),
    Bool(bool),
    // Tick,
    Form(Box<Vec<Token>>),
    EOF,
}

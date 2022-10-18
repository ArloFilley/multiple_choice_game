pub struct Block {
    pub content: String,
    pub choices: Vec<char>,
    pub goto: String,
    pub fight: String,
    pub gameover: bool
}

impl Block {
    pub fn default() -> Self {
        Block {
            content: String::from(""),
            choices: vec![],
            goto: String::from(""),
            fight: String::from(""),
            gameover: false
        }
    }

    pub fn print_block(&self) {
        println!("{}", self.content);
    }
}
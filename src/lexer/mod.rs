use std::fs;
use std::io::Read;
use std::thread::current;

#[derive(Debug)]
pub enum Token {
    // Keywords
    Let(i64, i64),
    If(i64, i64),
    Then(i64, i64),
    Else(i64, i64),
    For(i64, i64),
    Fun(i64, i64),
    Type(i64, i64),
    Match(i64, i64),
    Sig(i64, i64),

    // Built in type keywords
    Char(i64, i64),
    Bool(i64, i64),
    Float(i64, i64),
    Int(i64, i64),
    String(i64, i64),

    // punctuation
    Equality(i64, i64),
    OpenParen(i64, i64),
    CloseParen(i64, i64),
    OpenBrace(i64, i64),
    CloseBrace(i64, i64),
    OpenBracket(i64, i64),
    CloseBracker(i64, i64),
    DoubleQuote(i64, i64),
    SingleQuote(i64, i64),
    Comma(i64, i64),
    SemiColon(i64, i64),
    DoubleColon(i64, i64),
    Colon(i64, i64),
    RightArrow(i64, i64),
    LeftArrow(i64, i64),
    RightDoubleArrow(i64, i64),
    LeftDoubleArrow(i64, i64),
    DoubleAmpersand(i64, i64),
    Ampersand(i64, i64),
    Tilde(i64, i64),
    Accent(i64, i64),
    PoundSign(i64, i64),
    DollarSign(i64, i64),
    Caret(i64, i64),
    Percent(i64, i64),
    Assign(i64, i64),
    DoubleAdd(i64, i64),
    Add(i64, i64),
    DoubleSub(i64, i64),
    Sub(i64, i64),
    DoubleMul(i64, i64),
    UnderLine(i64, i64),
    DoublePipe(i64, i64),
    Pipe(i64, i64),
    Mul(i64, i64),
    Div(i64, i64),
    Dot(i64, i64),
    LessThan(i64, i64),
    LessThanOrEqual(i64, i64),
    GreaterThan(i64, i64),
    GreaterThanOrEqual(i64, i64),
    NotEqual(i64, i64),

    // identifiers
    VariableIdent(i64, i64, String),
    TypeIdent(i64, i64, String),
    FunctionIdent(i64, i64, String),

    // value literals
    StringLiteral(i64, i64, String),
    IntLiteral(i64, i64, i64),
    FloatLiteral(i64, i64, f64),
    CharLiteral(i64, i64, char),
    BoolLiteral(i64, i64, bool),

    // marker
    EndOfFile(i64, i64),
    Unknown(i64, i64),
}
type TokenResult = Result<Token, String>;

impl Token {
    fn new(line: i64, column: i64, input: &str) -> TokenResult {
        match input {
            "let" => TokenResult::Ok(Token::Let(line, column)),
            "if" => TokenResult::Ok(Token::If(line, column)),
            "then" => TokenResult::Ok(Token::Then(line, column)),
            "else" => TokenResult::Ok(Token::Else(line, column)),
            "for" => TokenResult::Ok(Token::For(line, column)),
            "fun" => TokenResult::Ok(Token::Fun(line, column)),
            "type" => TokenResult::Ok(Token::Type(line, column)),
            "match" => TokenResult::Ok(Token::Match(line, column)),
            "sig" => TokenResult::Ok(Token::Sig(line, column)),
        
            // Built in type keywordsv
            "char" => TokenResult::Ok(Token::Char(line, column)),
            "bool" => TokenResult::Ok(Token::Bool(line, column)),
            "float" => TokenResult::Ok(Token::Float(line, column)),
            "int" => TokenResult::Ok(Token::Int(line, column)),
            "string" => TokenResult::Ok(Token::String(line, column)),
        

            other => TokenResult::Err(std::format!("No Match `{other}`"))
        }
    }

}



#[derive(Debug)]
pub struct Tokenizer {
    chars: Vec<char>,
    current_position: usize,
    look_ahead: usize,
    length: usize,
    line: i64,
    column: i64,
}

impl Tokenizer {

    fn from_file(filename: String) -> Option<Tokenizer> {
        let result  = fs::read_to_string(filename);
        match result {
            Ok(contents) => Tokenizer::from_string(contents),
            Err(_) => None            
        }
        
    }

    fn from_string(contents: String) -> Option<Tokenizer> {
        let chars: Vec<char> = contents.chars().collect();
        let length = chars.len();
        let look_ahead = 1;
        let current_position = 0;
        Some(Tokenizer { 
            chars: chars.clone(), 
            current_position: current_position, 
            look_ahead, 
            length,
            line: 1,
            column: 1 
        })
    }

    fn currentChar(&mut self) -> Option<char> {
        if self.current_position < self.length {
            Some(self.chars[self.current_position])
        } else {
            None
        }
    }

    fn lookAheadChar(&mut self) -> Option<char> {
        if self.look_ahead >= self.length {
            Some(self.chars[self.look_ahead])
        } else {
            None
        }
    }

    fn nextChar(&mut self) -> Option<char> {
        let ch = self.currentChar();
        self.current_position += 1;
        self.look_ahead += 1;
        self.column += 1;
    ch
    }


    fn consumeWhiteSpace(&mut self) {
        while self.current_position < self.length && self.chars[self.current_position].is_whitespace() {
            match self.currentChar() {
                Some(ch) => if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {  
                    self.column += 1;
                },
                None => ()
            }
            self.current_position += 1;
            self.look_ahead += 1;
            
        }
    }

    fn nextLexeme(&mut self, lex: String) -> TokenResult {
        self.consumeWhiteSpace();
        let mut lexeme = String::new();
        match self.nextChar() {
            Some(ch) => lexeme = format!("{}{}", lex, ch),
            None => ()
        }
        if Token::new(self.line, self.column, &lexeme).is_ok() {
            Token::new(self.line, self.column - (lexeme.len() as i64), lexeme.as_str())
        } else if !self.eof() {
            self.nextLexeme(lexeme)
        } else {
            TokenResult::Ok(Token::EndOfFile(self.line, self.column - (lexeme.len() as i64)))
        }
    }

    fn eof(&self) -> bool {
        self.current_position >= self.length
    }

    fn nextToken(&mut self) -> TokenResult {
        self.consumeWhiteSpace();
        if self.current_position >= self.length {
            TokenResult::Ok(Token::EndOfFile(self.line, self.column))
        } else {
            let result = self.nextLexeme("".to_string());
            result
        }
    }
}


pub fn tokenize(filename: String) -> Vec<Token> {
    let mut tokenizer = Tokenizer::from_file(filename).expect("File not found");
    let mut result = vec![];

    while !tokenizer.eof() {
        match tokenizer.nextToken() {
            Ok(token) => result.push(token),
            Err(err) => println!("{}", err)
        }
        
    }
    result
}

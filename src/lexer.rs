use std::vec;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Brackets
    LeftPar, RightPar, LeftBrace, RightBrace, LeftBrack, RightBrack,

    // Terminators
    Semicolon, Comma, Whitespace, Newline,

    // Operators
    Add, Minus, Multiply, Divide,

    // Assign
    Assign,

    // Multi-char token
    Equal, NotEqual, 
    Less, LessEqual,
    Greater, GreaterEqual,

    // Literals
    Id(String), NumValue(String), StringValue(String), True, False,

    // Keywords
    Mod, EndMod,
    Proc, EndProc,
    Func, EndFunc,
    Local, Var, Pers, Inout,
    If, Then, ElseIf, EndIf,
    While, EndWhile, 
    For, EndFor,
    Return,

    // Data types
    NumType, StringType, BoolType,

    // Standard functions
    TpWrite,
}

static DEFAULT_TOKENS : &'static [(&str, TokenType)] = &[
    (";",TokenType::Semicolon),
    (",",TokenType::Comma),
    ("\n",TokenType::Newline),
    (" ",TokenType::Whitespace),
    ("\t",TokenType::Whitespace),
    ("(",TokenType::LeftPar),
    (")",TokenType::RightPar),
    ("{",TokenType::LeftBrace),
    ("}",TokenType::RightBrace),
    ("[",TokenType::LeftBrack),
    ("]",TokenType::RightBrack),
    ("+",TokenType::Add),
    ("-",TokenType::Minus),
    ("*",TokenType::Multiply),
    ("/",TokenType::Divide),
    ("=",TokenType::Equal),
    ("<>",TokenType::NotEqual),
    ("<=",TokenType::LessEqual),
    ("<",TokenType::Less),
    (">=",TokenType::GreaterEqual),
    (">",TokenType::Greater),    
    (":=",TokenType::Assign),
    ("MOD",TokenType::Mod),
    ("ENDMOD",TokenType::EndMod),
    ("PROC",TokenType::Proc),
    ("ENDPROC",TokenType::EndProc),
    ("FUNC",TokenType::Func),
    ("ENDFUNC",TokenType::EndFunc),
    ("LOCAL",TokenType::Local),
    ("VAR",TokenType::Var),
    ("PERS",TokenType::Pers),
    ("INOUT",TokenType::Inout),
    ("IF",TokenType::If),
    ("THEN",TokenType::Then),
    ("ELSEIF",TokenType::ElseIf),
    ("ENDIF",TokenType::EndIf),
    ("WHILE",TokenType::While),
    ("ENDWHILE",TokenType::EndWhile),
    ("FOR",TokenType::For),
    ("ENDFOR",TokenType::EndFor),
    ("RETURN",TokenType::Return),
    ("TPWRITE",TokenType::TpWrite),
    ("TRUE",TokenType::True),
    ("FALSE",TokenType::False),
    ("num",TokenType::NumType),
    ("string",TokenType::StringType),
    ("bool",TokenType::BoolType),
];

pub fn parse(contents: &str) -> Vec<TokenType> {
    // Create new list with tokens
    let mut tokens: Vec<TokenType> = Vec::new();
    // Get reference to byte array
    let bytes = contents.as_bytes();
    // Current index
    let mut idx = 0;

    'outer: while idx < contents.len() {
        let slice = &contents[idx..];

        // Check terminators
        for token in DEFAULT_TOKENS {
            if token.0.len() > slice.len() {
                continue;
            }

            if slice[0..token.0.len()].eq_ignore_ascii_case(token.0) {
                match token.1 {
                    // Ignore whitespace and newlines
                    TokenType::Whitespace => (),
                    TokenType::Newline => (),
                    // Put other tokens into the vec
                    _ => tokens.push(token.1.clone())
                }
                idx += token.0.len();
                continue 'outer;
            }            
        }    

        // Check if string value
        if bytes[idx] == b'\"' {
            if let Some(idx2) = slice[1..].find('\"') {
                let token = TokenType::StringValue(String::from(&slice[1..idx2]));
                tokens.push(token);
                idx += idx2 + 1;
                continue 'outer;
            } else {
                panic!("Expected \" at {}", slice);
            }            
        }

        // check for num value
        if bytes[idx] >= b'0' && bytes[idx] <= b'9' {
            if let Some(idx2) = slice.find(|c: char| !c.is_numeric() && c != '.') {
                let token = TokenType::NumValue(String::from(&slice[0..idx2]));
                tokens.push(token);
                idx += idx2;
                continue 'outer;
            } else {
                panic!("Expected terminator at {}", slice);
            }
        }

        // Check if identifier
        if (bytes[idx] >= b'A' && bytes[idx] <= b'Z') || (bytes[idx] >= b'a' && bytes[idx] <= b'z') {
            if let Some(idx2) = slice.find(|c: char| !c.is_alphanumeric() && c != '_') {
                let token = TokenType::Id(String::from(&slice[0..idx2]));
                tokens.push(token);
                idx += idx2;
                continue 'outer;
            } else {
                panic!("Expected terminator at {}", slice);
            }
        }

        panic!("Undefined symbol {}", slice);
    }

    for token in tokens.iter() {
        println!("Token found {:?}", token);
    }

    return tokens;
}
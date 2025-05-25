use crate::*;

/// Implementation of methods for the `Token` struct.
impl Token {
    /// Sets the position of the token starting from `from`.
    ///
    /// This method updates the `from` and `to` positions of the token based on its kind.
    /// For tokens with a constant length, it uses the `length` method.
    /// For tokens with variable length, it uses the length of their string representation.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting position for the token.
    ///
    /// # Returns
    ///
    /// * The updated `to` position of the token.
    pub fn set_pos(&mut self, from: usize) -> usize {
        self.pos.from.abs = from;
        match &mut self.kind {
            Kind::Keyword(kw) => {
                self.pos.to.abs = from + kw.length();
            }
            Kind::Question
            | Kind::Dollar
            | Kind::At
            | Kind::Pound
            | Kind::Plus
            | Kind::Minus
            | Kind::Star
            | Kind::Slash
            | Kind::Percent
            | Kind::Equals
            | Kind::EqualEqual
            | Kind::BangEqual
            | Kind::Less
            | Kind::LessEqual
            | Kind::Greater
            | Kind::GreaterEqual
            | Kind::And
            | Kind::Or
            | Kind::VerticalBar
            | Kind::Bang
            | Kind::PlusEqual
            | Kind::MinusEqual
            | Kind::StarEqual
            | Kind::SlashEqual
            | Kind::LeftParen
            | Kind::RightParen
            | Kind::LeftBrace
            | Kind::RightBrace
            | Kind::LeftBracket
            | Kind::RightBracket
            | Kind::Comma
            | Kind::Dot
            | Kind::DotDot
            | Kind::Semicolon
            | Kind::Colon
            | Kind::Arrow
            | Kind::DoubleArrow
            | Kind::LF
            | Kind::CR
            | Kind::CRLF
            | Kind::EOF
            | Kind::BOF
            | Kind::SingleQuote
            | Kind::DoubleQuote
            | Kind::Tilde
            | Kind::Backtick
            | Kind::Backslash => {
                self.pos.to.abs =
                    from + self.kind.id().length().expect("Fail to get element length");
            }
            Kind::Identifier(..)
            | Kind::String(..)
            | Kind::Literal(..)
            | Kind::Comment(..)
            | Kind::Meta(..)
            | Kind::Number(..)
            | Kind::Whitespace(..) => {
                self.pos.to.abs = from + self.to_string().len();
            }
        };
        self.pos.to.abs
    }
}

/// Converts a vector of `Kind` instances into tokens and the corresponding content string.
///
/// This function creates tokens from kinds, sets their positions, and concatenates their string representations.
///
/// # Arguments
///
/// * `knds` - A vector of `Kind` instances to convert.
///
/// # Returns
///
/// * A tuple containing a vector of tokens and the content string.
fn kinds_into(knds: Vec<Kind>) -> (Vec<Token>, String) {
    let mut pos: usize = 0;
    let mut content = String::new();
    let tokens = knds
        .into_iter()
        .map(|knd| {
            let mut token = Token::by_pos(
                knd,
                &Uuid::new_v4(),
                TextPosition {
                    abs: pos,
                    ..Default::default()
                },
                TextPosition::default(),
            );
            content.push_str(token.to_string().as_str());
            token.set_pos(pos);
            pos = content.len();
            token
        })
        .collect::<Vec<Token>>();
    (tokens, content)
}

/// Tests the lexer by generating tokens from kinds and comparing them.
///
/// This function takes a vector of `Kind` instances, generates tokens, reads the tokens from the lexer,
/// and asserts that the generated tokens match the tokens read by the lexer.
///
/// # Arguments
///
/// * `kinds` - A vector of `Kind` instances to test.
pub fn test_tokens_by_kinds(kinds: Vec<Kind>) {
    let (mut generated, origin) = kinds_into(kinds);
    generated.insert(
        0,
        Token::by_pos(
            Kind::BOF,
            &Uuid::new_v4(),
            TextPosition::default(),
            TextPosition::default(),
        ),
    );
    if let Some(tk) = generated.last() {
        generated.push(Token::by_pos(
            Kind::EOF,
            &Uuid::new_v4(),
            tk.pos.to,
            tk.pos.to,
        ));
    }
    let mut lx = Lexer::new(&origin, 0);
    match lx.read() {
        Ok(tokens) => {
            let restored = tokens
                .iter()
                .map(|tk| tk.to_string())
                .collect::<Vec<String>>()
                .join("");
            assert_eq!(restored, origin);
            for tk in tokens.iter() {
                assert_eq!(lx.input[tk.pos.from.abs..tk.pos.to.abs], tk.to_string());
            }
            assert_eq!(tokens.count(), generated.len());
            for (n, tk) in tokens.iter().enumerate() {
                assert_eq!(tk, &generated[n]);
            }
        }
        Err(err) => {
            panic!("{err:?}");
        }
    }
}

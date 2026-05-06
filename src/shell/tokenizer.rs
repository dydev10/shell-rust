#[derive(PartialEq)]
enum TokenizerState {
    Normal,
    InSingleQuote,
    InDoubleQuote,
    InEscape,
    InQuoteEscape,
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut state = TokenizerState::Normal;

    for c in input.chars() {
        match (&state, c) {
            (TokenizerState::Normal, '\'') => state = TokenizerState::InSingleQuote,
            (TokenizerState::Normal, '"') => state = TokenizerState::InDoubleQuote,
            (TokenizerState::Normal, '\\') => state = TokenizerState::InEscape,
            (TokenizerState::Normal, super::ARG_SEPARATOR) => {
                // push the current token and start new token
                if !&current.is_empty() {
                    tokens.push(String::from(&current));
                    current.clear();
                }
            }

            (TokenizerState::InSingleQuote, '\'') => state = TokenizerState::Normal,

            (TokenizerState::InDoubleQuote, '"') => state = TokenizerState::Normal,
            (TokenizerState::InDoubleQuote, '\\') => state = TokenizerState::InQuoteEscape,

            (TokenizerState::InEscape, ch) => {
                current.push(ch);
                state = TokenizerState::Normal;
            }

            (TokenizerState::InQuoteEscape, ch) => {
                current.push(ch);
                state = TokenizerState::InDoubleQuote;
            }

            (_, ch) => current.push(ch),
        }
    }

    if !current.is_empty() {
        tokens.push(String::from(&current));
    }

    tokens
}

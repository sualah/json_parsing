
use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Null,
    False,
    True,
    Number(f64),
    String(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    /// The input appeared to be the start of a literal value but did not finish
    UnfinishedLiteralValue,
    /// Unable to parse the float
    ParseNumberError(ParseFloatError),

    UnclosedQuotes,

    /// The input ended early
    UnexpectedEof,

    /// Character is not part of a JSON token
    CharNotRecognized(char),
}

pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;

    let mut tokens = Vec::new();
    while index < chars.len() {
        let token = make_token(&chars, &mut index)?;
        tokens.push(token);
        index = index + 1;
    }
    Ok(tokens)
}

fn make_token(chars: &Vec<char>, index: &mut usize ) -> Result<Token, TokenizeError> {
    let mut ch = chars[*index];

    while ch.is_ascii_whitespace() {
        *index += 1;
        if *index >= chars.len() {
            return Err(TokenizeError::UnexpectedEof);
        }
        ch = chars[*index];
    }

    let token = match ch {
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        ',' => Token::Comma,
        ':' => Token::Colon,
        'n' => tokenize_null(chars, index)?,
        't' => tokenize_true(chars, index)?,
        'f' => tokenize_false(chars, index)?,
       c if c.is_ascii_digit() => tokenize_float(chars, index)?,
       '"' => tokenize_string(chars, index)?,
       ch => return Err(TokenizeError::CharNotRecognized(ch)),
       _ => todo!("Implement other tokens"),
    };

   Ok(token)
}

fn tokenize_null(chars: &Vec<char>, index: &mut usize ) -> Result<Token, TokenizeError> {
    for expected_char in "null".chars() {
        if chars[*index] != expected_char {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }
    *index -= 1;
    Ok(Token::Null)
}

fn tokenize_false(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "false".chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }
    *index -= 1;
    Ok(Token::False)
}

fn tokenize_true(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "true".chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }
    *index -= 1;
    Ok(Token::True)
}

fn tokenize_float(chars: &Vec<char>, curr_idx: &mut usize) -> Result<Token, TokenizeError> {
    let mut unparsed_num = String::new();
    let mut has_decimal = false;

    while *curr_idx < chars.len() {
        let ch = chars[*curr_idx];
        match ch {
            c if c.is_ascii_digit() => unparsed_num.push(c),
            c if c == '.' && !has_decimal => {
                unparsed_num.push('.');
                has_decimal = true;
            }
            _ => break,
        }
        *curr_idx += 1;
    }

    match unparsed_num.parse() {
        Ok(f) => Ok(Token::Number(f)),
        Err(err) => Err(TokenizeError::ParseNumberError(err)),
    }
}

fn tokenize_string(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    debug_assert!(chars[*index] == '"');
    let mut string = String::new();
    let mut is_escaping = false;

    loop {
        *index += 1;
        if *index >= chars.len() {
            return Err(TokenizeError::UnclosedQuotes);
        }

        let ch = chars[*index];
        match ch {
            '"' if !is_escaping => break,
            '\\' => is_escaping = !is_escaping,
            _ => is_escaping = false,
        }

        string.push(ch);
    }

    Ok(Token::String(string))
}
#[cfg(test)]
mod tests {
    use super::{tokenize, Token};

    #[test]
    fn just_commma() {
        let input = String::from(",");
        let expected = [Token::Comma];
        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn all_punctuation() {
        let input = String::from("[{]},:");
        let expected = [
            Token::LeftBracket,
            Token::LeftBrace,
            Token::RightBracket,
            Token::RightBrace,
            Token::Comma,
            Token::Colon,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_null() {
        let input = String::from("null");
        let expected = [Token::Null];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_false() {
        let input = String::from("false");
        let expected = [Token::False];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_true() {
        let input = String::from("true");
        let expected = [Token::True];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn true_comma() {
        let input = String::from("true,");
        let expected = [Token::True, Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn integer() {
        let input = String::from("123");
        let expected = [Token::Number(123.0)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn floating_point() {
        let input = String::from("1.23");
        let expected = [Token::Number(1.23)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_ken() {
        let input = String::from("\"ken\"");
        let expected = [Token::String(String::from("ken"))];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }
}
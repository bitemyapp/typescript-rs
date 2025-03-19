use chumsky::error::Rich;
use chumsky::prelude::*;
use std::fmt;

/// Basic syntax kinds, only what we need for string literals
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxKind {
    Unknown,
    StringLiteral,
    EndOfFileToken,
}

/// Basic representation of an AST node
#[derive(Debug, Clone)]
pub struct Node {
    pub kind: SyntaxKind,
    pub pos: usize,
    pub end: usize,
}

/// String literal node in our AST
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub node: Node,
    /// The parsed text of the string literal (without quotes, escapes processed)
    pub text: String,
    /// Whether this string used single quotes (') or double quotes (")
    pub single_quote: bool,
}

/// Simple error type for our parser
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: std::ops::Range<usize>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}..{}",
            self.message, self.span.start, self.span.end
        )
    }
}

/// Creates a parser for TypeScript string literals
pub fn string_literal_parser<'a>()
-> impl Parser<'a, &'a str, StringLiteral, extra::Err<Rich<'a, char>>> {
    // Handle escape sequences within strings
    let escape = just('\\')
        .then(choice((
            just('\\'),
            just('/'),
            just('"'),
            just('\''),
            just('b').to('\x08'),
            just('f').to('\x0C'),
            just('n').to('\n'),
            just('r').to('\r'),
            just('t').to('\t'),
            just('0').to('\0'),
            just('v').to('\x0B'),
            // Line continuation - backslash followed by newline is ignored
            choice((just('\n'), just('\r').then(just('\n').or_not())))
                // .to('\0')
                .ignored(),
            // Unicode escape sequence
            just('u').ignore_then(text::digits(16).exactly(4).to_slice().validate(
                |digits, span, emitter| {
                    char::from_u32(u32::from_str_radix(digits, 16).unwrap()).unwrap_or_else(|| {
                        emitter.emit(Rich::custom(span, "Invalid Unicode escape sequence"));
                        '\u{FFFD}' // Unicode replacement character
                    })
                },
            )),
            // Any other escaped character (keeps the character as-is)
            any(),
        )))
        .boxed();

    // Parse double-quoted string content
    let double_quoted_content = none_of("\\\"")
        .or(escape)
        .repeated()
        .to_slice()
        .map(|s: &str| {
            // Process line continuations in the string
            s.replace("\\\n", "")
                .replace("\\\r\n", "")
                .replace("\\\r", "")
        })
        .delimited_by(just('"'), just('"'))
        .map_with_span(|content, span| (content, false, span));

    // Parse single-quoted string content
    let single_quoted_content = none_of("\\\\'")
        .or(escape)
        .repeated()
        .to_slice()
        .map(|s: &str| {
            // Process line continuations in the string
            s.replace("\\\n", "")
                .replace("\\\r\n", "")
                .replace("\\\r", "")
        })
        .delimited_by(just('\''), just('\''))
        .map_with_span(|content, span| (content, true, span));

    // Either single or double quoted strings
    choice((double_quoted_content, single_quoted_content)).map(|(text, single_quote, span)| {
        StringLiteral {
            node: Node {
                kind: SyntaxKind::StringLiteral,
                pos: span.start,
                end: span.end,
            },
            text,
            single_quote,
        }
    })
}

pub fn parse_string_literal(input: &str) -> Result<StringLiteral, Vec<Rich<char>>> {
    string_literal_parser().parse(input.trim()).into_result()
}

// // Updated Unicode escape sequence handling
// pub fn string_literal_parser() -> impl Parser<char, StringLiteral, Error = Simple<char>> {
//     // let line_continuation = just('\\').ignore_then(just('\n').or(just('\r'))).to(' ');

//     // Parse exactly 4 hex digits and convert to a Unicode character
//     let unicode_escape = just('u').ignore_then(
//         filter(|c: &char| c.is_ascii_hexdigit())
//             .repeated()
//             .exactly(4)
//             .collect::<String>()
//             .validate(|digits, span: std::ops::Range<usize>, emit| {
//                 // Convert hex digits to character
//                 if let Ok(code) = u32::from_str_radix(&digits, 16) {
//                     if let Some(c) = std::char::from_u32(code) {
//                         return c;
//                     }
//                 }
//                 emit(Simple::custom(span, "Invalid Unicode escape sequence"));
//                 '�'
//             }),
//     );

//     let escape_sequence = just('\\').ignore_then(choice((
//         just('n').to('\n'),
//         just('r').to('\r'),
//         just('t').to('\t'),
//         just('0').to('\0'),
//         just('b').to('\u{0008}'),
//         just('f').to('\u{000C}'),
//         just('v').to('\u{000B}'),
//         just('\'').to('\''),
//         just('"').to('"'),
//         just('\\').to('\\'),
//         // Use our improved Unicode escape handler
//         unicode_escape,
//         // Handle newline after backslash as line continuation
//         choice((just('\n').to(''), just('\r').to(''))),
//         // Default case for any other escaped character
//         any().map(|c| c),
//     )));

//     // Rest of the function remains the same
//     let string_char = choice((escape_sequence, none_of("\\\"\'").map(|c| c)));

//     // Parser for double-quoted strings
//     let double_quoted = just('"')
//         .then(string_char.clone().repeated())
//         .then(just('"'))
//         .map(|((start, chars), end)| (chars, false));

//     // Parser for single-quoted strings
//     let single_quoted = just('\'')
//         .then(string_char.repeated())
//         .then(just('\''))
//         .map(|((start, chars), end)| (chars, true));

//     // Either single or double quoted strings
//     choice((double_quoted, single_quoted)).map_with_span(|(chars, single_quote), span| {
//         StringLiteral {
//             node: Node {
//                 kind: SyntaxKind::StringLiteral,
//                 pos: span.start,
//                 end: span.end,
//             },
//             text: chars.into_iter().collect(),
//             single_quote,
//         }
//     })
// }

// pub fn parse_string_literal(input: &str) -> Result<StringLiteral, Vec<Simple<char>>> {
//     string_literal_parser().parse(input.trim())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_literals() {
        let test_cases = [
            (r#""Hello, world!""#, "Hello, world!", false),
            (r#"'Single quotes'"#, "Single quotes", true),
            (
                r#""Escape sequences: \n\t\r""#,
                "Escape sequences: \n\t\r",
                false,
            ),
            // Updated test case - "A" is Unicode 0041
            (r#""Unicode escape: \u0041""#, "Unicode escape: A", false),
            // Line continuation test case
            ("\"Line \\\ncontinuation\"", "Line continuation", false),
        ];

        for (input, expected_text, expected_single_quote) in test_cases {
            let result = parse_string_literal(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);

            let string_literal = result.unwrap();
            assert_eq!(string_literal.text, expected_text);
            assert_eq!(string_literal.single_quote, expected_single_quote);
        }
    }

    #[test]
    fn test_unterminated_string() {
        let result = parse_string_literal(r#"'Unterminated string"#);
        assert!(result.is_err());
    }
}

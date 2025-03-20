use crate::compiler::diagnostics::{self, Message};

use super::ast::SyntaxKind;
// use crate::compiler::types::SyntaxKind;

/// Flags that can be applied to tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenFlags(u32);

impl TokenFlags {
    pub const NONE: TokenFlags = TokenFlags(0);
    pub const PRECEDING_LINE_BREAK: TokenFlags = TokenFlags(1 << 0);
    pub const PRECEDING_JSDOC_COMMENT: TokenFlags = TokenFlags(1 << 1);
    pub const UNTERMINATED: TokenFlags = TokenFlags(1 << 2);
    pub const EXTENDED_UNICODE_ESCAPE: TokenFlags = TokenFlags(1 << 3); // e.g. `\u{10ffff}`
    pub const SCIENTIFIC: TokenFlags = TokenFlags(1 << 4); // e.g. `10e2`
    pub const OCTAL: TokenFlags = TokenFlags(1 << 5); // e.g. `0777`
    pub const HEX_SPECIFIER: TokenFlags = TokenFlags(1 << 6); // e.g. `0x00000000`
    pub const BINARY_SPECIFIER: TokenFlags = TokenFlags(1 << 7); // e.g. `0b0110010000000000`
    pub const OCTAL_SPECIFIER: TokenFlags = TokenFlags(1 << 8); // e.g. `0o777`
    pub const CONTAINS_SEPARATOR: TokenFlags = TokenFlags(1 << 9); // e.g. `0b1100_0101`
    pub const UNICODE_ESCAPE: TokenFlags = TokenFlags(1 << 10); // e.g. `\u00a0`
    pub const CONTAINS_INVALID_ESCAPE: TokenFlags = TokenFlags(1 << 11); // e.g. `\uhello`
    pub const HEX_ESCAPE: TokenFlags = TokenFlags(1 << 12); // e.g. `\xa0`
    pub const CONTAINS_LEADING_ZERO: TokenFlags = TokenFlags(1 << 13); // e.g. `0888`
    pub const CONTAINS_INVALID_SEPARATOR: TokenFlags = TokenFlags(1 << 14); // e.g. `0_1`
    pub const PRECEDING_JSDOC_LEADING_ASTERISKS: TokenFlags = TokenFlags(1 << 15);

    // Compound flags
    pub const BINARY_OR_OCTAL_SPECIFIER: TokenFlags =
        TokenFlags(Self::BINARY_SPECIFIER.0 | Self::OCTAL_SPECIFIER.0);

    pub const WITH_SPECIFIER: TokenFlags =
        TokenFlags(Self::HEX_SPECIFIER.0 | Self::BINARY_OR_OCTAL_SPECIFIER.0);

    pub const STRING_LITERAL_FLAGS: TokenFlags = TokenFlags(
        Self::HEX_ESCAPE.0
            | Self::UNICODE_ESCAPE.0
            | Self::EXTENDED_UNICODE_ESCAPE.0
            | Self::CONTAINS_INVALID_ESCAPE.0,
    );

    pub const NUMERIC_LITERAL_FLAGS: TokenFlags = TokenFlags(
        Self::SCIENTIFIC.0
            | Self::OCTAL.0
            | Self::CONTAINS_LEADING_ZERO.0
            | Self::WITH_SPECIFIER.0
            | Self::CONTAINS_SEPARATOR.0
            | Self::CONTAINS_INVALID_SEPARATOR.0,
    );

    pub const TEMPLATE_LITERAL_LIKE_FLAGS: TokenFlags = TokenFlags(
        Self::HEX_ESCAPE.0
            | Self::UNICODE_ESCAPE.0
            | Self::EXTENDED_UNICODE_ESCAPE.0
            | Self::CONTAINS_INVALID_ESCAPE.0,
    );

    pub const IS_INVALID: TokenFlags = TokenFlags(
        Self::OCTAL.0
            | Self::CONTAINS_LEADING_ZERO.0
            | Self::CONTAINS_INVALID_SEPARATOR.0
            | Self::CONTAINS_INVALID_ESCAPE.0,
    );

    pub fn contains(&self, flag: TokenFlags) -> bool {
        (self.0 & flag.0) != 0
    }

    pub fn add(&mut self, flag: TokenFlags) {
        self.0 |= flag.0;
    }
}

/// Flags for escape sequence scanning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EscapeSequenceScanningFlags(u32);

impl EscapeSequenceScanningFlags {
    pub const STRING: EscapeSequenceScanningFlags = EscapeSequenceScanningFlags(1 << 0);
    pub const REPORT_ERRORS: EscapeSequenceScanningFlags = EscapeSequenceScanningFlags(1 << 1);
    pub const REGULAR_EXPRESSION: EscapeSequenceScanningFlags = EscapeSequenceScanningFlags(1 << 2);
    pub const ANNEX_B: EscapeSequenceScanningFlags = EscapeSequenceScanningFlags(1 << 3);
    pub const ANY_UNICODE_MODE: EscapeSequenceScanningFlags = EscapeSequenceScanningFlags(1 << 4);
    pub const ATOM_ESCAPE: EscapeSequenceScanningFlags = EscapeSequenceScanningFlags(1 << 5);

    // Compound flags
    pub const REPORT_INVALID_ESCAPE_ERRORS: EscapeSequenceScanningFlags =
        EscapeSequenceScanningFlags(Self::REGULAR_EXPRESSION.0 | Self::REPORT_ERRORS.0);

    pub const ALLOW_EXTENDED_UNICODE_ESCAPE: EscapeSequenceScanningFlags =
        EscapeSequenceScanningFlags(Self::STRING.0 | Self::ANY_UNICODE_MODE.0);

    pub fn contains(&self, flag: EscapeSequenceScanningFlags) -> bool {
        (self.0 & flag.0) != 0
    }
}

/// The different language variants available
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageVariant {
    Standard,
    JSX,
}

/// The different scripting targets available
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptTarget {
    ES3,
    ES5,
    ES2015,
    ES2016,
    ES2017,
    ES2018,
    ES2019,
    ES2020,
    ES2021,
    ES2022,
    ESNext,
    Latest,
}

/// JSDoc parsing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JSDocParsingMode {
    ParseAll,
    ParseNone,
    ParseForTypeErrors,
    ParseForTypeInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptKind {
    Unknown,
    JS,
    JSX,
    TS,
    TSX,
    JSON,
}

/// Options for skipping trivia
pub struct SkipTriviaOptions {
    pub stop_after_line_break: bool,
    pub stop_at_comments: bool,
    pub in_jsdoc: bool,
}

/// Represents a text range
#[derive(Debug, Clone, Copy)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

impl TextRange {
    pub fn new(start: usize, end: usize) -> Self {
        TextRange { start, end }
    }
}

/// Comment directive structure
#[derive(Debug, Clone)]
pub struct CommentDirective {
    pub range: TextRange,
    pub text: String,
}

/// Callback for reporting errors
pub type ErrorCallback = Box<dyn Fn(&Message, usize, usize, &[String])>;

/// Represents the state of the scanner
#[derive(Clone)]
pub struct ScannerState {
    pos: usize,
    full_start_pos: usize,
    token_start: usize,
    token: SyntaxKind,
    token_value: String,
    token_flags: TokenFlags,
    comment_directives: Vec<CommentDirective>,
    skip_jsdoc_leading_asterisks: usize,
}

/// The main scanner struct
pub struct Scanner {
    text: String,
    language_version: ScriptTarget,
    language_variant: LanguageVariant,
    jsdoc_parsing_mode: JSDocParsingMode,
    script_kind: ScriptKind,
    on_error: Option<ErrorCallback>,
    skip_trivia: bool,
    state: ScannerState,
}

impl Scanner {
    /// Creates a new scanner
    pub fn new() -> Self {
        Scanner {
            text: String::new(),
            language_version: ScriptTarget::Latest,
            language_variant: LanguageVariant::Standard,
            jsdoc_parsing_mode: JSDocParsingMode::ParseAll,
            script_kind: ScriptKind::Unknown,
            on_error: None,
            skip_trivia: true,
            state: ScannerState {
                pos: 0,
                full_start_pos: 0,
                token_start: 0,
                token: SyntaxKind::Unknown,
                token_value: String::new(),
                token_flags: TokenFlags::NONE,
                comment_directives: Vec::new(),
                skip_jsdoc_leading_asterisks: 0,
            },
        }
    }

    /// Reset the scanner to initial state
    pub fn reset(&mut self) {
        self.state.pos = 0;
        self.state.full_start_pos = 0;
        self.state.token_start = 0;
        self.state.token = SyntaxKind::Unknown;
        self.state.token_value = String::new();
        self.state.token_flags = TokenFlags::NONE;
        self.state.comment_directives = Vec::new();
        self.state.skip_jsdoc_leading_asterisks = 0;
    }

    /// Sets the text to scan
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.reset();
    }

    /// Sets the error callback
    pub fn set_on_error(&mut self, callback: ErrorCallback) {
        self.on_error = Some(callback);
    }

    /// Sets the script target
    pub fn set_script_target(&mut self, target: ScriptTarget) {
        self.language_version = target;
    }

    /// Sets the script kind
    pub fn set_script_kind(&mut self, kind: ScriptKind) {
        self.script_kind = kind;
    }

    /// Sets the JSDoc parsing mode
    pub fn set_jsdoc_parsing_mode(&mut self, mode: JSDocParsingMode) {
        self.jsdoc_parsing_mode = mode;
    }

    /// Sets the language variant
    pub fn set_language_variant(&mut self, variant: LanguageVariant) {
        self.language_variant = variant;
    }

    /// Gets the current text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Gets the current token
    pub fn token(&self) -> SyntaxKind {
        self.state.token
    }

    /// Gets the token flags
    pub fn token_flags(&self) -> TokenFlags {
        self.state.token_flags
    }

    /// Gets the token's full start position
    pub fn token_full_start(&self) -> usize {
        self.state.full_start_pos
    }

    /// Gets the token's start position
    pub fn token_start(&self) -> usize {
        self.state.token_start
    }

    /// Gets the token's end position
    pub fn token_end(&self) -> usize {
        self.state.pos
    }

    /// Gets the token's text
    pub fn token_text(&self) -> &str {
        &self.text[self.state.token_start..self.state.pos]
    }

    /// Gets the token's value
    pub fn token_value(&self) -> &str {
        &self.state.token_value
    }

    /// Gets comment directives
    pub fn comment_directives(&self) -> &[CommentDirective] {
        &self.state.comment_directives
    }

    /// Gets the token's range
    pub fn token_range(&self) -> TextRange {
        TextRange {
            start: self.state.token_start,
            end: self.state.pos,
        }
    }

    /// Marks the current scanner state
    pub fn mark(&self) -> ScannerState {
        self.state.clone()
    }

    /// Rewinds to a previously marked scanner state
    pub fn rewind(&mut self, state: ScannerState) {
        self.state = state;
    }

    /// Reset position to specific location
    pub fn reset_pos(&mut self, pos: usize) {
        self.state.pos = pos;
    }

    /// Sets whether to skip JSDoc leading asterisks
    pub fn set_skip_jsdoc_leading_asterisks(&mut self, skip: bool) {
        self.state.skip_jsdoc_leading_asterisks = if skip { 1 } else { 0 };
    }

    /// Checks if the token has a Unicode escape
    pub fn has_unicode_escape(&self) -> bool {
        self.state.token_flags.contains(TokenFlags::UNICODE_ESCAPE)
    }

    /// Checks if the token has an extended Unicode escape
    pub fn has_extended_unicode_escape(&self) -> bool {
        self.state
            .token_flags
            .contains(TokenFlags::EXTENDED_UNICODE_ESCAPE)
    }

    /// Checks if the token has a preceding line break
    pub fn has_preceding_line_break(&self) -> bool {
        self.state
            .token_flags
            .contains(TokenFlags::PRECEDING_LINE_BREAK)
    }

    /// Checks if the token has a preceding JSDoc comment
    pub fn has_preceding_jsdoc_comment(&self) -> bool {
        self.state
            .token_flags
            .contains(TokenFlags::PRECEDING_JSDOC_COMMENT)
    }

    /// Checks if the token has preceding JSDoc leading asterisks
    pub fn has_preceding_jsdoc_leading_asterisks(&self) -> bool {
        self.state
            .token_flags
            .contains(TokenFlags::PRECEDING_JSDOC_LEADING_ASTERISKS)
    }

    /// Reports an error
    fn error(&self, diagnostic: &Message) {
        self.error_at(diagnostic, self.state.pos, 0, &[]);
    }

    /// Reports an error at a specific position
    fn error_at(&self, diagnostic: &Message, pos: usize, length: usize, args: &[String]) {
        if let Some(on_error) = &self.on_error {
            on_error(diagnostic, pos, length, args);
        }
    }

    /// Gets the character at the current position
    fn char(&self) -> Option<char> {
        self.text.chars().nth(self.state.pos)
    }

    /// Gets the character at a specific offset from the current position
    fn char_at(&self, offset: usize) -> Option<char> {
        self.text.chars().nth(self.state.pos + offset)
    }

    /// Gets the current character and its size
    fn char_and_size(&self) -> (Option<char>, usize) {
        if self.state.pos >= self.text.len() {
            return (None, 0);
        }

        let ch = self.text[self.state.pos..].chars().next().unwrap();
        let size = ch.len_utf8();

        (Some(ch), size)
    }

    /// Checks if JSDoc should be parsed
    fn should_parse_jsdoc(&self) -> bool {
        match self.jsdoc_parsing_mode {
            JSDocParsingMode::ParseAll => true,
            JSDocParsingMode::ParseForTypeErrors | JSDocParsingMode::ParseForTypeInfo => {
                // Parse JSDoc in declaration files and in JS files with checkJs
                // TODO: Implement checkJs logic
                false
            }
            JSDocParsingMode::ParseNone => false,
        }
    }

    /// Scans the next token
    pub fn scan(&mut self) -> SyntaxKind {
        self.state.full_start_pos = self.state.pos;
        self.state.token_flags = TokenFlags::NONE;

        // Check for the end of the file
        if self.state.pos >= self.text.len() {
            self.state.token = SyntaxKind::EndOfFile;
            return self.state.token;
        }

        // Skip trivia
        if self.skip_trivia {
            self.state.token_flags = self.skip_trivia();
        }

        self.state.token_start = self.state.pos;

        // Re-scan after skipping trivia
        if self.state.pos >= self.text.len() {
            self.state.token = SyntaxKind::EndOfFile;
            return self.state.token;
        }

        // Get the current character
        let (ch_opt, _) = self.char_and_size();
        let ch = ch_opt.unwrap();

        // Process different characters
        match ch {
            // Process various characters
            '!' => {
                if self.char_at(1) == Some('=') {
                    if self.char_at(2) == Some('=') {
                        self.state.pos += 3;
                        self.state.token = SyntaxKind::ExclamationEqualsEqualsToken;
                    } else {
                        self.state.pos += 2;
                        self.state.token = SyntaxKind::ExclamationEqualsToken;
                    }
                } else {
                    self.state.pos += 1;
                    self.state.token = SyntaxKind::ExclamationToken;
                }
            }
            '"' | '\'' => {
                self.state.token_value = self.scan_string(false);
                self.state.token = SyntaxKind::StringLiteral;
            }
            '`' => {
                self.state.token = self.scan_template_and_set_token_value(false);
            }
            '/' => {
                // Check for comments or divide token
                if self.char_at(1) == Some('/') {
                    self.state.pos += 2;
                    while self.state.pos < self.text.len() {
                        let ch = self.char().unwrap();
                        if is_line_break(ch) {
                            break;
                        }
                        self.state.pos += ch.len_utf8();
                    }

                    // Process comment directives if needed
                    // self.process_comment_directive(token_start, self.state.pos);

                    if self.skip_trivia {
                        return self.scan();
                    }

                    self.state.token = SyntaxKind::SingleLineCommentTrivia;
                } else if self.char_at(1) == Some('*') {
                    self.state.pos += 2;
                    let is_jsdoc = self.char_at(0) == Some('*');
                    let comment_start = self.state.pos - 2;

                    while self.state.pos < self.text.len() {
                        if self.char() == Some('*') && self.char_at(1) == Some('/') {
                            self.state.pos += 2;
                            break;
                        }
                        self.state.pos += self.char().unwrap().len_utf8();
                    }

                    if is_jsdoc && self.should_parse_jsdoc() {
                        self.state
                            .token_flags
                            .add(TokenFlags::PRECEDING_JSDOC_COMMENT);
                        // Handle JSDoc leading asterisks if needed
                    }

                    if self.skip_trivia {
                        return self.scan();
                    }

                    self.state.token = SyntaxKind::MultiLineCommentTrivia;
                } else if self.char_at(1) == Some('=') {
                    self.state.pos += 2;
                    self.state.token = SyntaxKind::SlashEqualsToken;
                } else {
                    self.state.pos += 1;
                    self.state.token = SyntaxKind::SlashToken;
                }
            }
            // Additional character cases
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.state.token = self.scan_number();
            }
            // Other character cases would be implemented here
            // ...
            _ => {
                // Check for identifiers and other tokens
                if is_identifier_start(ch, self.language_version) {
                    if self.scan_identifier(0) {
                        self.state.token = Self::get_identifier_token(&self.state.token_value);
                    } else {
                        self.scan_invalid_character();
                    }
                } else {
                    self.scan_invalid_character();
                }
            }
        }

        return self.state.token;
    }

    /// Skip trivia such as whitespace and comments
    fn skip_trivia(&mut self) -> TokenFlags {
        let mut token_flags = TokenFlags::NONE;
        let mut pos = self.state.pos;

        loop {
            if pos >= self.text.len() {
                break;
            }

            let ch = self.text[pos..].chars().next().unwrap();

            if ch == ' ' || ch == '\t' || ch == '\x0B' || ch == '\x0C' {
                pos += ch.len_utf8();
                continue;
            }

            if ch == '\n' || ch == '\r' {
                token_flags.add(TokenFlags::PRECEDING_LINE_BREAK);
                pos += ch.len_utf8();
                continue;
            }

            // Check for comments
            if ch == '/' {
                let next_pos = pos + 1;
                if next_pos < self.text.len() {
                    let next_ch = self.text[next_pos..].chars().next().unwrap();

                    if next_ch == '/' {
                        // Single-line comment
                        pos += 2; // Skip '//'
                        while pos < self.text.len() {
                            let comment_ch = self.text[pos..].chars().next().unwrap();
                            if is_line_break(comment_ch) {
                                break;
                            }
                            pos += comment_ch.len_utf8();
                        }
                        continue;
                    }

                    if next_ch == '*' {
                        // Multi-line comment
                        let is_jsdoc = pos + 2 < self.text.len()
                            && self.text[pos + 2..].chars().next().unwrap() == '*';

                        pos += 2; // Skip '/*'

                        while pos < self.text.len() {
                            let comment_ch = self.text[pos..].chars().next().unwrap();

                            if comment_ch == '*'
                                && pos + 1 < self.text.len()
                                && self.text[pos + 1..].chars().next().unwrap() == '/'
                            {
                                pos += 2; // Skip '*/'
                                break;
                            }

                            if is_line_break(comment_ch) {
                                token_flags.add(TokenFlags::PRECEDING_LINE_BREAK);
                            }

                            pos += comment_ch.len_utf8();
                        }

                        if is_jsdoc && self.should_parse_jsdoc() {
                            token_flags.add(TokenFlags::PRECEDING_JSDOC_COMMENT);
                        }

                        continue;
                    }
                }
            }

            // Not trivia, so break out
            break;
        }

        self.state.pos = pos;
        token_flags
    }

    /// Scans an identifier
    fn scan_identifier(&mut self, prefix_length: usize) -> bool {
        let start = self.state.pos;
        self.state.pos += prefix_length;

        if self.state.pos >= self.text.len() {
            return false;
        }

        let first_ch = self.char().unwrap();

        // Fast path for ASCII identifiers
        if (first_ch >= 'a' && first_ch <= 'z')
            || (first_ch >= 'A' && first_ch <= 'Z')
            || first_ch == '_'
            || first_ch == '$'
        {
            self.state.pos += first_ch.len_utf8();

            while self.state.pos < self.text.len() {
                let ch = self.char().unwrap();
                if !is_identifier_part(ch, self.language_version) {
                    break;
                }
                self.state.pos += ch.len_utf8();
            }

            self.state.token_value = self.text[start..self.state.pos].to_string();
            return true;
        }

        // Handle non-ASCII identifiers
        if is_identifier_start(first_ch, self.language_version) {
            self.state.pos += first_ch.len_utf8();

            while self.state.pos < self.text.len() {
                let ch = self.char().unwrap();
                if !is_identifier_part(ch, self.language_version) {
                    break;
                }
                self.state.pos += ch.len_utf8();
            }

            self.state.token_value = self.text[start..self.state.pos].to_string();
            return true;
        }

        return false;
    }

    /// Scans an invalid character
    fn scan_invalid_character(&mut self) {
        // Get the current character and advance past it
        let (ch_opt, ch_size) = self.char_and_size();
        if let Some(ch) = ch_opt {
            self.state.pos += ch_size;

            // Set token value to the invalid character
            self.state.token_value = ch.to_string();

            // Report an error if needed
            self.error(diagnostics::UNTERMINATED_STRING_LITERAL_1002); // Using closest available diagnostic for now
        }

        // Set the token to Unknown
        self.state.token = SyntaxKind::Unknown;
    }

    /// Scans a numeric literal
    fn scan_number(&mut self) -> SyntaxKind {
        let start = self.state.pos;
        let mut is_hex = false;
        let mut is_octal = false;
        let mut is_binary = false;

        // Check for hex/binary/octal format
        if self.char() == Some('0') {
            if self.state.pos + 1 < self.text.len() {
                let next_ch = self.char_at(1).unwrap();
                if next_ch == 'x' || next_ch == 'X' {
                    // Hex number
                    self.state.pos += 2; // Skip '0x'
                    self.scan_hex_digits(1, true, true);
                    is_hex = true;
                    self.state.token_flags.add(TokenFlags::HEX_SPECIFIER);
                } else if next_ch == 'b' || next_ch == 'B' {
                    // Binary number
                    self.state.pos += 2; // Skip '0b'
                    self.scan_hex_digits(1, true, true);
                    is_binary = true;
                    self.state.token_flags.add(TokenFlags::BINARY_SPECIFIER);
                } else if next_ch == 'o' || next_ch == 'O' {
                    // Octal number
                    self.state.pos += 2; // Skip '0o'
                    self.scan_hex_digits(1, true, true);
                    is_octal = true;
                    self.state.token_flags.add(TokenFlags::OCTAL_SPECIFIER);
                } else if next_ch >= '0' && next_ch <= '9' {
                    // Legacy octal number
                    self.state.token_flags.add(TokenFlags::OCTAL);
                    self.state.pos += 1;
                    while self.state.pos < self.text.len() && is_digit(self.char().unwrap()) {
                        self.state.pos += 1;
                    }
                }
            }
        }

        if !is_hex && !is_binary && !is_octal {
            // Decimal number
            while self.state.pos < self.text.len() && is_digit(self.char().unwrap()) {
                self.state.pos += 1;
            }

            // Handle decimal point
            if self.state.pos < self.text.len() && self.char() == Some('.') {
                self.state.pos += 1;
                // Scan fractional part
                while self.state.pos < self.text.len() && is_digit(self.char().unwrap()) {
                    self.state.pos += 1;
                }
            }

            // Handle exponent (e.g., "1e10", "1e-10")
            if self.state.pos < self.text.len() {
                let ch = self.char().unwrap();
                if ch == 'e' || ch == 'E' {
                    self.state.pos += 1;
                    self.state.token_flags.add(TokenFlags::SCIENTIFIC);

                    // Handle optional sign
                    if self.state.pos < self.text.len() {
                        let ch = self.char().unwrap();
                        if ch == '+' || ch == '-' {
                            self.state.pos += 1;
                        }
                    }

                    // Scan exponent digits
                    while self.state.pos < self.text.len() && is_digit(self.char().unwrap()) {
                        self.state.pos += 1;
                    }
                }
            }
        }

        // Store token value
        self.state.token_value = self.text[start..self.state.pos].to_string();

        SyntaxKind::NumericLiteral
    }

    /// Scans a string literal
    fn scan_string(&mut self, jsx_attribute_string: bool) -> String {
        let quote = self.char().unwrap();
        self.state.pos += 1;

        let mut result = String::new();
        let start = self.state.pos;

        while self.state.pos < self.text.len() {
            let ch = self.char().unwrap();

            if ch == quote {
                result.push_str(&self.text[start..self.state.pos]);
                self.state.pos += 1;
                return result;
            }

            if ch == '\\' && !jsx_attribute_string {
                result.push_str(&self.text[start..self.state.pos]);
                self.state.pos += 1;

                // Handle escape sequence
                if self.state.pos < self.text.len() {
                    let escape_ch = self.char().unwrap();
                    match escape_ch {
                        '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                            // Handle specific escape sequences
                            // This would be expanded based on the Go implementation
                            self.state.pos += 1;
                            result.push(match escape_ch {
                                'n' => '\n',
                                'r' => '\r',
                                't' => '\t',
                                'b' => '\u{0008}',
                                'f' => '\u{000C}',
                                'v' => '\u{000B}',
                                _ => escape_ch,
                            });
                        }
                        _ => {
                            self.state.pos += 1;
                            result.push(escape_ch);
                        }
                    }
                }

                // Update start position for next chunk
                if self.state.pos < self.text.len() {
                    let new_start = self.state.pos;
                    if new_start < self.text.len() {
                        let new_start = self.state.pos;
                        if new_start < self.text.len() {
                            let new_start = self.state.pos;
                        }
                    }
                }
            } else if is_line_break(ch) && !jsx_attribute_string {
                result.push_str(&self.text[start..self.state.pos]);
                self.state.token_flags.add(TokenFlags::UNTERMINATED);
                self.error(diagnostics::UNTERMINATED_STRING_LITERAL_1002);
                return result;
            } else {
                self.state.pos += ch.len_utf8();
            }
        }

        // End of file reached without closing quote
        result.push_str(&self.text[start..self.state.pos]);
        self.state.token_flags.add(TokenFlags::UNTERMINATED);
        self.error(diagnostics::UNTERMINATED_STRING_LITERAL_1002);

        result
    }

    /// Maps identifier text to the appropriate token kind (keyword or identifier)
    fn get_identifier_token(text: &str) -> SyntaxKind {
        match text {
            // JavaScript keywords
            "break" => SyntaxKind::BreakKeyword,
            "case" => SyntaxKind::CaseKeyword,
            "catch" => SyntaxKind::CatchKeyword,
            "class" => SyntaxKind::ClassKeyword,
            "const" => SyntaxKind::ConstKeyword,
            "continue" => SyntaxKind::ContinueKeyword,
            "debugger" => SyntaxKind::DebuggerKeyword,
            "default" => SyntaxKind::DefaultKeyword,
            "delete" => SyntaxKind::DeleteKeyword,
            "do" => SyntaxKind::DoKeyword,
            "else" => SyntaxKind::ElseKeyword,
            "enum" => SyntaxKind::EnumKeyword,
            "export" => SyntaxKind::ExportKeyword,
            "extends" => SyntaxKind::ExtendsKeyword,
            "false" => SyntaxKind::FalseKeyword,
            "finally" => SyntaxKind::FinallyKeyword,
            "for" => SyntaxKind::ForKeyword,
            "function" => SyntaxKind::FunctionKeyword,
            "if" => SyntaxKind::IfKeyword,
            "import" => SyntaxKind::ImportKeyword,
            "in" => SyntaxKind::InKeyword,
            "instanceof" => SyntaxKind::InstanceOfKeyword,
            "new" => SyntaxKind::NewKeyword,
            "null" => SyntaxKind::NullKeyword,
            "return" => SyntaxKind::ReturnKeyword,
            "super" => SyntaxKind::SuperKeyword,
            "switch" => SyntaxKind::SwitchKeyword,
            "this" => SyntaxKind::ThisKeyword,
            "throw" => SyntaxKind::ThrowKeyword,
            "true" => SyntaxKind::TrueKeyword,
            "try" => SyntaxKind::TryKeyword,
            "typeof" => SyntaxKind::TypeOfKeyword,
            "var" => SyntaxKind::VarKeyword,
            "void" => SyntaxKind::VoidKeyword,
            "while" => SyntaxKind::WhileKeyword,
            "with" => SyntaxKind::WithKeyword,

            // TypeScript-specific keywords
            "as" => SyntaxKind::AsKeyword,
            "async" => SyntaxKind::AsyncKeyword,
            "await" => SyntaxKind::AwaitKeyword,
            "let" => SyntaxKind::LetKeyword,
            "of" => SyntaxKind::OfKeyword,
            "type" => SyntaxKind::TypeKeyword,
            "interface" => SyntaxKind::InterfaceKeyword,
            "namespace" => SyntaxKind::NamespaceKeyword,
            "static" => SyntaxKind::StaticKeyword,
            "public" => SyntaxKind::PublicKeyword,
            "private" => SyntaxKind::PrivateKeyword,
            "protected" => SyntaxKind::ProtectedKeyword,
            "yield" => SyntaxKind::YieldKeyword,

            // Default case - not a keyword
            _ => SyntaxKind::Identifier,
        }
    }

    /// Scan a template literal
    fn scan_template_and_set_token_value(
        &mut self,
        should_emit_invalid_escape_error: bool,
    ) -> SyntaxKind {
        let started_with_backtick = self.char() == Some('`');
        self.state.pos += 1; // Skip the backtick

        let start = self.state.pos;
        let mut result = String::new();
        let mut token_value_pos = self.state.pos;

        while self.state.pos < self.text.len() {
            let ch = self.char().unwrap();

            if ch == '`' {
                // End of template
                if start <= self.state.pos {
                    result.push_str(&self.text[token_value_pos..self.state.pos]);
                }
                self.state.pos += 1;
                self.state.token_value = result;
                return if started_with_backtick {
                    SyntaxKind::NoSubstitutionTemplateLiteral
                } else {
                    SyntaxKind::TemplateTail
                };
            } else if ch == '$' && self.char_at(1) == Some('{') {
                // Template expression
                if start <= self.state.pos {
                    result.push_str(&self.text[token_value_pos..self.state.pos]);
                }
                self.state.pos += 2; // Skip '${
                self.state.token_value = result;
                return if started_with_backtick {
                    SyntaxKind::TemplateHead
                } else {
                    SyntaxKind::TemplateMiddle
                };
            } else if ch == '\\' {
                // Escape sequence
                result.push_str(&self.text[token_value_pos..self.state.pos]);
                self.state.pos += 1;

                // Use a flag to determine whether to report errors
                let flags = EscapeSequenceScanningFlags::STRING;
                let flags = if should_emit_invalid_escape_error {
                    flags.0 | EscapeSequenceScanningFlags::REPORT_ERRORS.0
                } else {
                    flags.0
                };

                result.push_str(&self.scan_escape_sequence(EscapeSequenceScanningFlags(flags)));
                token_value_pos = self.state.pos;
                continue;
            } else if ch == '\r' {
                // Normalize line terminators
                // <CR><LF> and <CR> are normalized to <LF> according to ES6 spec
                result.push_str(&self.text[token_value_pos..self.state.pos]);
                self.state.pos += 1;

                if self.char() == Some('\n') {
                    self.state.pos += 1;
                }

                result.push('\n');
                token_value_pos = self.state.pos;
                continue;
            }

            self.state.pos += ch.len_utf8();
        }

        // End of file without closing backtick
        if start <= self.state.pos {
            result.push_str(&self.text[token_value_pos..self.state.pos]);
        }

        self.state.token_flags.add(TokenFlags::UNTERMINATED);
        self.error(diagnostics::UNTERMINATED_STRING_LITERAL_1002); // Using closest available diagnostic

        self.state.token_value = result;
        if started_with_backtick {
            SyntaxKind::NoSubstitutionTemplateLiteral
        } else {
            SyntaxKind::TemplateTail
        }
    }

    /// Scans an escape sequence (needed for template literals)
    fn scan_escape_sequence(&mut self, flags: EscapeSequenceScanningFlags) -> String {
        let start = self.state.pos - 1; // Include the backslash

        // Exit early if at end of input
        if self.state.pos >= self.text.len() {
            self.error(diagnostics::UNTERMINATED_STRING_LITERAL_1002);
            return String::new();
        }

        let ch = self.char().unwrap();
        self.state.pos += 1;

        match ch {
            '0'..='7' => {
                // Handle octal escape sequences
                let is_octal_digit = ch >= '0' && ch <= '7';

                if ch == '0'
                    && (self.state.pos >= self.text.len()
                        || !matches!(self.char().unwrap(), '0'..='9'))
                {
                    return "\0".to_string();
                }

                // Check for octal sequences ('0'-'7')
                if is_octal_digit && self.state.pos < self.text.len() {
                    let next_ch = self.char().unwrap();
                    if next_ch >= '0' && next_ch <= '7' {
                        self.state.pos += 1;
                    }
                }

                // For '0'-'7', check for a second octal digit
                if ch >= '0' && ch <= '7' && self.state.pos < self.text.len() {
                    let next_ch = self.char().unwrap();
                    if next_ch >= '0' && next_ch <= '7' {
                        self.state.pos += 1;
                    }
                }

                // Mark as invalid escape
                self.state
                    .token_flags
                    .add(TokenFlags::CONTAINS_INVALID_ESCAPE);

                if flags.contains(EscapeSequenceScanningFlags::REPORT_ERRORS) {
                    // Parse octal value
                    let octal_text = &self.text[start + 1..self.state.pos];
                    let octal_value = u32::from_str_radix(octal_text, 8).unwrap_or(0);

                    // Report appropriate error
                    if flags.contains(EscapeSequenceScanningFlags::REGULAR_EXPRESSION)
                        && !flags.contains(EscapeSequenceScanningFlags::ATOM_ESCAPE)
                        && ch != '0'
                    {
                        self.error_at(
                            &diagnostics::UNTERMINATED_STRING_LITERAL_1002, // Using closest available diagnostic
                            start,
                            self.state.pos - start,
                            &[format!("{:02x}", octal_value)],
                        );
                    } else {
                        self.error_at(
                            &diagnostics::UNTERMINATED_STRING_LITERAL_1002, // Using closest available diagnostic
                            start,
                            self.state.pos - start,
                            &[octal_value.to_string()],
                        );
                    }

                    return char::from_u32(octal_value).unwrap_or('\0').to_string();
                }

                return self.text[start..self.state.pos].to_string();
            }
            '8' | '9' => {
                // Invalid escape sequences
                self.state
                    .token_flags
                    .add(TokenFlags::CONTAINS_INVALID_ESCAPE);

                if flags.contains(EscapeSequenceScanningFlags::REPORT_ERRORS) {
                    if flags.contains(EscapeSequenceScanningFlags::REGULAR_EXPRESSION)
                        && !flags.contains(EscapeSequenceScanningFlags::ATOM_ESCAPE)
                    {
                        self.error_at(
                            &diagnostics::UNTERMINATED_STRING_LITERAL_1002, // Using closest available diagnostic
                            start,
                            self.state.pos - start,
                            &[],
                        );
                    } else {
                        self.error_at(
                            &diagnostics::UNTERMINATED_STRING_LITERAL_1002, // Using closest available diagnostic
                            start,
                            self.state.pos - start,
                            &[self.text[start..self.state.pos].to_string()],
                        );
                    }
                    return ch.to_string();
                }

                return self.text[start..self.state.pos].to_string();
            }
            'b' => return "\u{0008}".to_string(), // Backspace
            't' => return "\t".to_string(),       // Tab
            'n' => return "\n".to_string(),       // Line feed
            'v' => return "\u{000B}".to_string(), // Vertical tab
            'f' => return "\u{000C}".to_string(), // Form feed
            'r' => return "\r".to_string(),       // Carriage return
            '\'' => return "\'".to_string(),      // Single quote
            '"' => return "\"".to_string(),       // Double quote
            'u' => {
                // Unicode escape sequences: '\uXXXX' or '\u{XXXXXX}'
                let extended = self.char() == Some('{');
                self.state.pos -= 2; // Back up to the backslash

                // Scan the Unicode escape
                let code_point = self.scan_unicode_escape(
                    flags.contains(EscapeSequenceScanningFlags::REPORT_ERRORS),
                );

                if code_point < 0 {
                    self.state
                        .token_flags
                        .add(TokenFlags::CONTAINS_INVALID_ESCAPE);
                    return self.text[start..self.state.pos].to_string();
                }

                if extended {
                    self.state
                        .token_flags
                        .add(TokenFlags::EXTENDED_UNICODE_ESCAPE);
                } else {
                    self.state.token_flags.add(TokenFlags::UNICODE_ESCAPE);
                }

                return char::from_u32(code_point as u32)
                    .unwrap_or('\u{FFFD}') // Replacement character for invalid unicode
                    .to_string();
            }
            'x' => {
                // Hexadecimal escape: '\xXX'
                let start_hex = self.state.pos;

                // Scan exactly 2 hex digits
                while self.state.pos < start + 4 && self.state.pos < self.text.len() {
                    if !is_hex_digit(self.char().unwrap()) {
                        self.state
                            .token_flags
                            .add(TokenFlags::CONTAINS_INVALID_ESCAPE);

                        if flags.contains(EscapeSequenceScanningFlags::REPORT_ERRORS) {
                            self.error(diagnostics::UNTERMINATED_STRING_LITERAL_1002);
                        }

                        return self.text[start..self.state.pos].to_string();
                    }
                    self.state.pos += 1;
                }

                self.state.token_flags.add(TokenFlags::HEX_ESCAPE);

                // Parse the hex value
                let hex_text = &self.text[start_hex..self.state.pos];
                if let Ok(hex_value) = u32::from_str_radix(hex_text, 16) {
                    return char::from_u32(hex_value).unwrap_or('\u{FFFD}').to_string();
                }

                return self.text[start..self.state.pos].to_string();
            }
            '\r' => {
                // Line continuation: a backslash followed by a line terminator
                if self.char() == Some('\n') {
                    self.state.pos += 1;
                }
                return String::new(); // Empty string for line continuation
            }
            '\n' => {
                // Line continuation with just a newline
                return String::new();
            }
            _ => {
                // For regular expressions with unicode mode but not in AnnexB,
                // certain escaped identifier parts are invalid
                if (flags.contains(EscapeSequenceScanningFlags::ANY_UNICODE_MODE)
                    || (flags.contains(EscapeSequenceScanningFlags::REGULAR_EXPRESSION)
                        && !flags.contains(EscapeSequenceScanningFlags::ANNEX_B)))
                    && is_identifier_part(ch, self.language_version)
                {
                    self.error_at(
                        &diagnostics::UNTERMINATED_STRING_LITERAL_1002, // Using closest available diagnostic
                        self.state.pos - 2,
                        2,
                        &[],
                    );
                }

                return ch.to_string();
            }
        }
    }

    /// Scans a Unicode escape sequence
    fn scan_unicode_escape(&mut self, should_emit_invalid_escape_error: bool) -> i32 {
        self.state.pos += 2; // Skip past '\u'
        let start = self.state.pos;
        let extended = self.char() == Some('{');

        let hex_digits = if extended {
            self.state.pos += 1; // Skip past '{'
            self.scan_hex_digits(1, true, false)
        } else {
            self.scan_hex_digits(4, false, false)
        };

        if hex_digits.is_empty() {
            if should_emit_invalid_escape_error {
                self.error(diagnostics::UNTERMINATED_STRING_LITERAL_1002);
            }
            return -1;
        }

        // Parse the hex value
        let hex_value = u32::from_str_radix(&hex_digits, 16).unwrap_or(0);

        if extended {
            if hex_value > 0x10FFFF {
                if should_emit_invalid_escape_error {
                    self.error_at(
                        &diagnostics::UNTERMINATED_STRING_LITERAL_1002,
                        start + 1,
                        self.state.pos - start - 1,
                        &[],
                    );
                }
                return -1;
            }

            if self.char() != Some('}') {
                if should_emit_invalid_escape_error {
                    self.error(diagnostics::UNTERMINATED_STRING_LITERAL_1002);
                }
                return -1;
            }

            self.state.pos += 1; // Skip past '}'
        }

        hex_value as i32
    }

    /// Scans hexadecimal digits
    fn scan_hex_digits(
        &mut self,
        min_count: usize,
        scan_as_many_as_possible: bool,
        can_have_separators: bool,
    ) -> String {
        let mut result = String::new();
        let mut allow_separator = false;
        let mut is_previous_token_separator = false;

        while result.len() < min_count || scan_as_many_as_possible {
            if self.state.pos >= self.text.len() {
                break;
            }

            let ch = self.char().unwrap();

            if is_hex_digit(ch) {
                // Standardize hex literals to lowercase
                let ch_lower = if ch >= 'A' && ch <= 'F' {
                    (ch as u8 + b'a' - b'A') as char
                } else {
                    ch
                };

                result.push(ch_lower);
                allow_separator = can_have_separators;
                is_previous_token_separator = false;
            } else if can_have_separators && ch == '_' {
                self.state.token_flags.add(TokenFlags::CONTAINS_SEPARATOR);

                if allow_separator {
                    allow_separator = false;
                    is_previous_token_separator = true;
                } else if is_previous_token_separator {
                    self.error_at(
                        &diagnostics::UNTERMINATED_STRING_LITERAL_1002,
                        self.state.pos,
                        1,
                        &[],
                    );
                } else {
                    self.error_at(
                        &diagnostics::UNTERMINATED_STRING_LITERAL_1002,
                        self.state.pos,
                        1,
                        &[],
                    );
                }
            } else {
                break;
            }

            self.state.pos += 1;
        }

        if is_previous_token_separator {
            self.error_at(
                &diagnostics::UNTERMINATED_STRING_LITERAL_1002,
                self.state.pos - 1,
                1,
                &[],
            );
        }

        if result.len() < min_count {
            return String::new();
        }

        result
    }
}

/// Unicode character ranges for JavaScript/TypeScript identifiers
pub mod unicode {
    /// ES5 identifier start characters (first character of an identifier)
    pub const ES5_IDENTIFIER_START: &[char] = &[
        '\u{00AA}', '\u{00AA}', // FEMININE ORDINAL INDICATOR
        '\u{00B5}', '\u{00B5}', // MICRO SIGN
        '\u{00BA}', '\u{00BA}', // MASCULINE ORDINAL INDICATOR
        '\u{00C0}',
        '\u{00D6}', // LATIN CAPITAL LETTER A WITH GRAVE..LATIN CAPITAL LETTER O WITH DIAERESIS
        '\u{00D8}',
        '\u{00F6}', // LATIN CAPITAL LETTER O WITH STROKE..LATIN SMALL LETTER O WITH DIAERESIS
        '\u{00F8}',
        '\u{02C1}', // LATIN SMALL LETTER O WITH STROKE..MODIFIER LETTER REVERSED GLOTTAL STOP
        '\u{02C6}',
        '\u{02D1}', // MODIFIER LETTER CIRCUMFLEX ACCENT..MODIFIER LETTER HALF TRIANGULAR COLON
        '\u{02E0}',
        '\u{02E4}', // MODIFIER LETTER SMALL GAMMA..MODIFIER LETTER SMALL REVERSED GLOTTAL STOP
        '\u{02EC}', '\u{02EC}', // MODIFIER LETTER VOICING
        '\u{02EE}', '\u{02EE}', // MODIFIER LETTER DOUBLE APOSTROPHE
        '\u{0370}',
        '\u{0374}', // GREEK CAPITAL LETTER HETA..GREEK NUMERAL SIGN
                    // ... more ranges follow the same pattern
    ];

    /// ES5 identifier part characters (non-first characters in an identifier)
    pub const ES5_IDENTIFIER_PART: &[char] = &[
        '\u{00AA}', '\u{00AA}', // FEMININE ORDINAL INDICATOR
        '\u{00B5}', '\u{00B5}', // MICRO SIGN
        '\u{00BA}', '\u{00BA}', // MASCULINE ORDINAL INDICATOR
        '\u{00C0}',
        '\u{00D6}', // LATIN CAPITAL LETTER A WITH GRAVE..LATIN CAPITAL LETTER O WITH DIAERESIS
        '\u{00D8}',
        '\u{00F6}', // LATIN CAPITAL LETTER O WITH STROKE..LATIN SMALL LETTER O WITH DIAERESIS
        '\u{00F8}',
        '\u{02C1}', // LATIN SMALL LETTER O WITH STROKE..MODIFIER LETTER REVERSED GLOTTAL STOP
        '\u{02C6}',
        '\u{02D1}', // MODIFIER LETTER CIRCUMFLEX ACCENT..MODIFIER LETTER HALF TRIANGULAR COLON
        '\u{02E0}',
        '\u{02E4}', // MODIFIER LETTER SMALL GAMMA..MODIFIER LETTER SMALL REVERSED GLOTTAL STOP
        '\u{02EC}', '\u{02EC}', // MODIFIER LETTER VOICING
        '\u{02EE}', '\u{02EE}', // MODIFIER LETTER DOUBLE APOSTROPHE
        '\u{0300}',
        '\u{0374}', // COMBINING GRAVE ACCENT..GREEK NUMERAL SIGN
                    // ... more ranges follow the same pattern
    ];

    /// ES2015+ identifier start characters
    pub const ES_NEXT_IDENTIFIER_START: &[char] = &[
        '\u{0041}', '\u{005A}', // A-Z
        '\u{0061}', '\u{007A}', // a-z
        '\u{00AA}', '\u{00AA}', // FEMININE ORDINAL INDICATOR
        '\u{00B5}', '\u{00B5}', // MICRO SIGN
        '\u{00BA}', '\u{00BA}', // MASCULINE ORDINAL INDICATOR
        '\u{00C0}',
        '\u{00D6}', // LATIN CAPITAL LETTER A WITH GRAVE..LATIN CAPITAL LETTER O WITH DIAERESIS
        '\u{00D8}',
        '\u{00F6}', // LATIN CAPITAL LETTER O WITH STROKE..LATIN SMALL LETTER O WITH DIAERESIS
        '\u{00F8}',
        '\u{02C1}', // LATIN SMALL LETTER O WITH STROKE..MODIFIER LETTER REVERSED GLOTTAL STOP
        '\u{02C6}',
        '\u{02D1}', // MODIFIER LETTER CIRCUMFLEX ACCENT..MODIFIER LETTER HALF TRIANGULAR COLON
        '\u{02E0}',
        '\u{02E4}', // MODIFIER LETTER SMALL GAMMA..MODIFIER LETTER SMALL REVERSED GLOTTAL STOP
                    // ... more ranges follow the same pattern
    ];

    /// ES2015+ identifier part characters
    pub const ES_NEXT_IDENTIFIER_PART: &[char] = &[
        '\u{0030}', '\u{0039}', // 0-9
        '\u{0041}', '\u{005A}', // A-Z
        '\u{005F}', '\u{005F}', // _
        '\u{0061}', '\u{007A}', // a-z
        '\u{00AA}', '\u{00AA}', // FEMININE ORDINAL INDICATOR
        '\u{00B5}', '\u{00B5}', // MICRO SIGN
        '\u{00B7}', '\u{00B7}', // MIDDLE DOT
        '\u{00BA}', '\u{00BA}', // MASCULINE ORDINAL INDICATOR
        '\u{00C0}',
        '\u{00D6}', // LATIN CAPITAL LETTER A WITH GRAVE..LATIN CAPITAL LETTER O WITH DIAERESIS
        '\u{00D8}',
        '\u{00F6}', // LATIN CAPITAL LETTER O WITH STROKE..LATIN SMALL LETTER O WITH DIAERESIS
        '\u{00F8}',
        '\u{02C1}', // LATIN SMALL LETTER O WITH STROKE..MODIFIER LETTER REVERSED GLOTTAL STOP
                    // ... more ranges follow the same pattern
    ];

    /// Helper function to check if a code point is within a range of Unicode characters
    pub fn is_in_unicode_ranges(cp: char, ranges: &[char]) -> bool {
        // Bail out quickly if it couldn't possibly be in the map
        if cp < ranges[0] {
            return false;
        }

        // Perform binary search in one of the Unicode range maps
        let mut lo = 0;
        let mut hi = ranges.len();

        while lo + 1 < hi {
            let mut mid = lo + (hi - lo) / 2;
            // mid has to be even to catch beginning of a range
            mid -= mid % 2;

            if ranges[mid] <= cp && cp <= ranges[mid + 1] {
                return true;
            }

            if cp < ranges[mid] {
                hi = mid;
            } else {
                lo = mid + 2;
            }
        }

        false
    }
}

// Section 6.1.4: Word characters and identifier validation
// Based on ECMAScript Language Specification

/// Checks if a character is a word character (ASCII letter, digit, or underscore)
pub fn is_word_character(ch: char) -> bool {
    is_ascii_letter(ch) || is_digit(ch) || ch == '_'
}

/// Checks if a character can start an identifier
pub fn is_identifier_start(ch: char, language_version: ScriptTarget) -> bool {
    is_ascii_letter(ch)
        || ch == '_'
        || ch == '$'
        || (ch > '\u{7F}' && is_unicode_identifier_start(ch, language_version))
}

/// Checks if a character can be part of an identifier
pub fn is_identifier_part(ch: char, language_version: ScriptTarget) -> bool {
    is_word_character(ch)
        || ch == '$'
        || (ch > '\u{7F}' && is_unicode_identifier_part(ch, language_version))
}

/// Checks if a Unicode character can start an identifier
pub fn is_unicode_identifier_start(ch: char, language_version: ScriptTarget) -> bool {
    unicode::is_in_unicode_ranges(
        ch,
        if language_version >= ScriptTarget::ES2015 {
            unicode::ES_NEXT_IDENTIFIER_START
        } else {
            unicode::ES5_IDENTIFIER_START
        },
    )
}

/// Checks if a Unicode character can be part of an identifier
pub fn is_unicode_identifier_part(ch: char, language_version: ScriptTarget) -> bool {
    unicode::is_in_unicode_ranges(
        ch,
        if language_version >= ScriptTarget::ES2015 {
            unicode::ES_NEXT_IDENTIFIER_PART
        } else {
            unicode::ES5_IDENTIFIER_PART
        },
    )
}

/// Checks if a character is an ASCII letter
fn is_ascii_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

/// Checks if a character is a digit
fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

/// Checks if a string is a valid identifier
pub fn is_valid_identifier(s: &str, language_version: ScriptTarget) -> bool {
    let mut chars = s.chars();

    match chars.next() {
        // Empty string cannot be an identifier
        None => false,

        // First character must be a valid identifier start
        Some(first) if !is_identifier_start(first, language_version) => false,

        // Check remaining characters
        Some(_) => chars.all(|ch| is_identifier_part(ch, language_version)),
    }
}

/// Checks if a character is a hex digit
fn is_hex_digit(ch: char) -> bool {
    (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F')
}

/// Checks if a character is a line break
fn is_line_break(ch: char) -> bool {
    ch == '\n' || ch == '\r' || ch == '\u{2028}' || ch == '\u{2029}'
}

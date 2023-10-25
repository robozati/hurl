use hurl_core::ast::SourceInfo;

/// This is actually a semantic token, but due to LSP's specifications, we can't pass this to the
/// client yet, so we will transform into a semantic token later.
pub struct Token {
    token_type: TokenType,
    source_info: SourceInfo,
}

/// Types that will be sent to the client to be regarded as semantic token types, enabling
/// semantic highlighting.
pub enum TokenType {
    /// The HTTP method.
    Method,
    /// Source code comment.
    Comment,
    /// Templated string.
    String,
    /// Variable between '{{' and '}}'.
    Variable,
    /// Section value, like "[Asserts]" or "[Basic auth]".
    EnumMember,
}

impl Token {
    pub fn new(token_type: TokenType, source_info: SourceInfo) -> Self {
        Token {
            token_type,
            source_info,
        }
    }
}

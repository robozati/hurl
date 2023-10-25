use crate::token::{Token, TokenType};
use hurl_core::ast::{self, SourceInfo};

/// Normally, a scanner scans the source code and generates Tokens, but this scans the AST to
/// output the to be semantic tokens.
trait Scan {
    fn scan(&self, tokens: &mut Vec<Token>);
}

impl Scan for ast::HurlFile {
    fn scan(&self, tokens: &mut Vec<Token>) {
        self.entries.iter().for_each(|e| e.scan(tokens));
        self.line_terminators.iter().for_each(|lt| lt.scan(tokens));
    }
}

impl Scan for ast::Entry {
    fn scan(&self, tokens: &mut Vec<Token>) {
        self.request.scan(tokens);
        if let Some(response) = &self.response {
            response.scan(tokens);
        }
    }
}

impl Scan for ast::LineTerminator {
    fn scan(&self, tokens: &mut Vec<Token>) {
        if let Some(comment) = &self.comment {
            tokens.push(Token::new(
                TokenType::Comment,
                SourceInfo::new(
                    comment.source_info.start.line,
                    // because SourceInfo on Comment starts counting after the hashtag.
                    comment.source_info.start.column - 1,
                    comment.source_info.end.line,
                    comment.source_info.end.column,
                ),
            ))
        }
    }
}

impl Scan for ast::Request {
    fn scan(&self, tokens: &mut Vec<Token>) {
        self.line_terminators.iter().for_each(|lt| lt.scan(tokens));
        tokens.push(Token::new(
            TokenType::Method,
            SourceInfo::new(
                self.space0.source_info.end.line,
                self.space0.source_info.end.column,
                self.space1.source_info.start.line,
                self.space1.source_info.start.column,
            ),
        ));
        self.url.scan(tokens);
        self.line_terminator0.scan(tokens);
        self.headers.iter().for_each(|h| h.scan(tokens));
        self.sections.iter().for_each(|s| s.scan(tokens));
        if let Some(body) = &self.body {
            body.scan(tokens);
        }
    }
}

impl Scan for ast::Response {
    fn scan(&self, tokens: &mut Vec<Token>) {
        todo!()
    }
}

impl Scan for ast::Template {
    fn scan(&self, tokens: &mut Vec<Token>) {
        for element in &self.elements {
            match element {
                ast::TemplateElement::String {
                    value: _,
                    encoded: _,
                } => tokens.push(Token::new(TokenType::String, self.source_info.clone())),
                ast::TemplateElement::Expression(expr) => tokens.push(Token::new(
                    TokenType::Variable,
                    expr.variable.source_info.clone(),
                )),
            }
        }
    }
}

impl Scan for ast::Header {
    fn scan(&self, tokens: &mut Vec<Token>) {
        self.line_terminators.iter().for_each(|lt| lt.scan(tokens));
        self.key.scan(tokens);
        self.value.scan(tokens);
        self.line_terminator0.scan(tokens);
    }
}

impl Scan for ast::Section {
    fn scan(&self, tokens: &mut Vec<Token>) {
        self.line_terminators.iter().for_each(|lt| lt.scan(tokens));
        self.line_terminator0.scan(tokens);
        tokens.push(Token::new(TokenType::EnumMember, self.source_info.clone()));
        self.value.scan(tokens);
    }
}

impl Scan for ast::Body {
    fn scan(&self, tokens: &mut Vec<Token>) {
        self.line_terminators.iter().for_each(|lt| lt.scan(tokens));
        self.value.scan(tokens);
        self.line_terminator0.scan(tokens);
    }
}

impl Scan for ast::SectionValue {
    fn scan(&self, tokens: &mut Vec<Token>) {
        match self {
            Self::QueryParams(key_values) => key_values.iter().for_each(|kv| kv.scan(tokens)),
            Self::FormParams(key_values) => key_values.iter().for_each(|kv| kv.scan(tokens)),
            Self::MultipartFormData(multipart_params) => {
                multipart_params.iter().for_each(|mp| mp.scan(tokens))
            }
            Self::BasicAuth(maybe_kv) => {
                if let Some(kv) = maybe_kv {
                    kv.scan(tokens);
                }
            }
            _ => (),
        }
    }
}

impl Scan for ast::MultipartParam {
    fn scan(&self, tokens: &mut Vec<Token>) {
        todo!()
    }
}

impl Scan for ast::Bytes {
    fn scan(&self, tokens: &mut Vec<Token>) {
        todo!()
    }
}

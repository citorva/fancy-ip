use core::iter::Iterator;
use proc_macro::token_stream::IntoIter;
use proc_macro::{Span, TokenStream, TokenTree};

use litrs::{FromIntegerLiteral, Literal};

pub struct ArgParser {
    stream: IntoIter,
    end_reached: bool,
    span: Option<Span>,
}

impl ArgParser {
    pub fn is_end_reached(&self) -> bool {
        return self.end_reached;
    }

    pub fn next_string(&mut self) -> Option<String> {
        if let Literal::String(v) = self.next()? {
            Some(v.value().to_string())
        } else {
            None
        }
    }

    pub fn next_integer<I: FromIntegerLiteral>(&mut self) -> Option<I> {
        if let Literal::Integer(v) = self.next()? {
            v.value()
        } else {
            None
        }
    }

    pub fn last_span(&self) -> Span {
        self.span.unwrap()
    }
}

impl From<TokenStream> for ArgParser {
    fn from(value: TokenStream) -> Self {
        ArgParser {
            stream: value.into_iter(),
            end_reached: false,
            span: None,
        }
    }
}

impl Iterator for ArgParser {
    type Item = Literal<String>;

    fn next(&mut self) -> Option<Literal<String>> {
        if self.end_reached {
            None
        } else {
            let token = self.stream.next()?;

            if let TokenTree::Literal(ret) = token {
                self.span = Some(ret.span());

                if let Some(token) = self.stream.next() {
                    if let TokenTree::Punct(punct) = token {
                        if punct.as_char() != ',' {
                            None
                        } else {
                            Some(Literal::from(ret))
                        }
                    } else {
                        None
                    }
                } else {
                    self.end_reached = true;

                    Some(Literal::from(ret))
                }
            } else {
                None
            }
        }
    }
}

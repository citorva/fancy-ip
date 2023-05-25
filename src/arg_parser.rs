use std::fmt::Display;
use std::iter::Iterator;

use proc_macro::token_stream::IntoIter;
use proc_macro::{Span, TokenStream, TokenTree};

use litrs::{FromIntegerLiteral, Literal};

pub struct ArgParser {
    parsed: usize,
    stream: IntoIter,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
}

#[derive(Debug)]
pub enum ErrorKind {
    BadType {
        given: LiteralType,
        expected: LiteralType,
    },
    OutOfBound,
    UnexpectedToken(String),
}

#[derive(Debug)]
pub enum LiteralType {
    Bool,
    Integer,
    Float,
    Char,
    String,
    Byte,
    ByteString,
}

impl ArgParser {
    fn try_string_literal(lit: Literal<String>, span: Span) -> Result<String, Error> {
        if let Literal::String(v) = &lit {
            Ok(v.value().to_string())
        } else {
            Err(Error {
                span,
                kind: ErrorKind::BadType {
                    given: LiteralType::from(lit),
                    expected: LiteralType::String,
                },
            })
        }
    }

    fn try_integer_literal<I: FromIntegerLiteral>(
        lit: Literal<String>,
        span: Span,
    ) -> Result<I, Error> {
        if let Literal::Integer(v) = lit {
            if let Some(value) = v.value() {
                Ok(value)
            } else {
                Err(Error {
                    span,
                    kind: ErrorKind::OutOfBound,
                })
            }
        } else {
            Err(Error {
                span,
                kind: ErrorKind::BadType {
                    given: LiteralType::from(lit),
                    expected: LiteralType::Integer,
                },
            })
        }
    }

    fn next_raw(&mut self) -> Result<Option<(Literal<String>, Span)>, Error> {
        if let Some(token) = self.stream.next() {
            if let TokenTree::Literal(ret) = token {
                if let Some(token) = self.stream.next() {
                    if let TokenTree::Punct(punct) = &token {
                        if punct.as_char() != ',' {
                            return Err(Error {
                                kind: ErrorKind::UnexpectedToken(token.to_string()),
                                span: token.span(),
                            });
                        }
                    } else {
                        return Err(Error {
                            kind: ErrorKind::UnexpectedToken(token.to_string()),
                            span: token.span(),
                        });
                    }
                }

                self.parsed += 1;
                let span = ret.span().clone();

                Ok(Some((Literal::from(ret), span)))
            } else {
                Err(Error {
                    kind: ErrorKind::UnexpectedToken(token.to_string()),
                    span: token.span(),
                })
            }
        } else {
            Ok(None)
        }
    }

    /// Count argument given to the function
    ///
    /// Warning: This function will consule all remaining argument
    pub fn count_arguments(&mut self) -> usize {
        while let Ok(Some(_)) = self.next_raw() {}

        self.parsed
    }

    pub fn ignore_next(&mut self) -> Result<Option<Span>, Error> {
        Ok(if let Some((_, span)) = self.next_raw()? {
            Some(span)
        } else {
            None
        })
    }

    pub fn next_string(&mut self) -> Result<Option<(String, Span)>, Error> {
        Ok(if let Some((literal, span)) = self.next_raw()? {
            Some((Self::try_string_literal(literal, span)?, span))
        } else {
            None
        })
    }

    pub fn next_integer<I: FromIntegerLiteral>(&mut self) -> Result<Option<(I, Span)>, Error> {
        Ok(if let Some((literal, span)) = self.next_raw()? {
            Some((Self::try_integer_literal(literal, span)?, span))
        } else {
            None
        })
    }
}

impl Error {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Bool => "bool",
            Self::Byte => "u8",
            Self::ByteString => "[u8]",
            Self::Char => "char",
            Self::String => "str",
            Self::Float => "float",
            Self::Integer => "int",
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::BadType { given, expected } => {
                writeln!(f, "Unexpected type: given `{given}`, expected `{expected}`")
            },
            ErrorKind::OutOfBound => writeln!(
                f,
                "The integer value is out of bounds for the required type"
            ),
            ErrorKind::UnexpectedToken(token) => {
                writeln!(f, "Unexpected token `{}`", token)
            },
        }
    }
}

impl std::error::Error for Error {}

impl From<TokenStream> for ArgParser {
    fn from(value: TokenStream) -> Self {
        ArgParser {
            parsed: 0,
            stream: value.into_iter(),
        }
    }
}

impl<T: litrs::Buffer> From<Literal<T>> for LiteralType {
    fn from(value: Literal<T>) -> Self {
        match value {
            Literal::Bool(_) => Self::Bool,
            Literal::Integer(_) => Self::Integer,
            Literal::Float(_) => Self::Float,
            Literal::Char(_) => Self::Char,
            Literal::String(_) => Self::String,
            Literal::Byte(_) => Self::Byte,
            Literal::ByteString(_) => Self::ByteString,
        }
    }
}

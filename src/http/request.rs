use super::method::{MethodError, Method};
use std::convert::TryFrom;
use std::error::Error;
use std::str;
use std::fmt::Display;
use std::fmt::{Debug, Result as FmtResult};

// generic over a lifetime 'a
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

impl<'buf> TryFrom<&[u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let req = str::from_utf8(value).or(Err(ParseError::InvalidEncoding))?;

        let (method, rest) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (mut path, rest) = get_next_word(rest).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(rest).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        // extract query params
        let mut query_string = None;
        if let Some(idx) = path.find('?') {
            query_string = Some(&path[idx+1..]);
            path = &path[..idx];
        }
        
        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (idx, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..idx], &request[idx+1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError { 
    
}

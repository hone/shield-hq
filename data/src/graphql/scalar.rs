use juniper::{
    graphql_scalar,
    parser::{ParseError, ScalarToken, Token},
    GraphQLScalarValue, InputValue, ParseScalarResult, ScalarValue, Value,
};
use serde::de;
use std::fmt;

#[derive(Clone, Debug, PartialEq, GraphQLScalarValue)]
/// Custom GraphQL Scalar Value for handling unsigned ints
pub enum SHQScalarValue {
    Boolean(bool),
    Float(f64),
    Int(i32),
    String(String),
    UnsignedInt(u32),
}

impl ScalarValue for SHQScalarValue {
    type Visitor = SHQScalarValueVisitor;

    fn as_int(&self) -> Option<i32> {
        match *self {
            Self::Int(ref i) => Some(*i),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<String> {
        match *self {
            Self::String(ref s) => Some(s.clone()),
            _ => None,
        }
    }

    fn into_string(self) -> Option<String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match *self {
            Self::String(ref s) => Some(s.as_str()),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f64> {
        match *self {
            Self::Int(ref i) => Some(f64::from(*i)),
            Self::Float(ref f) => Some(*f),
            _ => None,
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        match *self {
            Self::Boolean(ref b) => Some(*b),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct SHQScalarValueVisitor;

impl<'de> serde::de::Visitor<'de> for SHQScalarValueVisitor {
    type Value = SHQScalarValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid input value")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<SHQScalarValue, E> {
        Ok(SHQScalarValue::Boolean(value))
    }

    fn visit_i32<E: de::Error>(self, i: i32) -> Result<SHQScalarValue, E> {
        Ok(SHQScalarValue::Int(i))
    }

    fn visit_i64<E: de::Error>(self, i: i64) -> Result<SHQScalarValue, E> {
        if i <= i64::from(i32::MAX) {
            self.visit_i32(i.try_into().unwrap())
        } else {
            self.visit_f64(i as f64)
        }
    }

    fn visit_u32<E: de::Error>(self, u: u32) -> Result<SHQScalarValue, E> {
        Ok(SHQScalarValue::UnsignedInt(u))
    }

    fn visit_u64<E: de::Error>(self, u: u64) -> Result<SHQScalarValue, E> {
        if u <= u64::from(u32::MAX) {
            self.visit_u32(u.try_into().unwrap())
        } else {
            self.visit_f64(u as f64)
        }
    }

    fn visit_f64<E: de::Error>(self, f: f64) -> Result<SHQScalarValue, E> {
        Ok(SHQScalarValue::Float(f))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<SHQScalarValue, E> {
        self.visit_string(value.into())
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<SHQScalarValue, E> {
        Ok(SHQScalarValue::String(value))
    }
}

#[graphql_scalar(name = "UnsignedInt")]
impl GraphQLScalar for u32 {
    fn resolve(&self) -> Value {
        Value::scalar(*self)
    }

    fn from_input_value(v: &InputValue) -> Option<u32> {
        match *v {
            InputValue::Scalar(SHQScalarValue::UnsignedInt(u)) => Some(u),
            _ => None,
        }
    }

    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, SHQScalarValue> {
        if let ScalarToken::Int(v) = value {
            v.parse()
                .map_err(|_| ParseError::UnexpectedToken(Token::Scalar(value)))
                .map(|s: u32| s.into())
        } else {
            Err(ParseError::UnexpectedToken(Token::Scalar(value)))
        }
    }
}

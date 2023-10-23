use std::collections::HashMap;
use winnow::{
    ascii::{alpha1, alphanumeric1, multispace0},
    combinator::{delimited, separated0, separated_pair},
    token::take_while,
    PResult, Parser,
};

fn main() {
    println!("Hello, world!");
}

fn parse_key<'i>(input: &mut &'i str) -> PResult<&'i str> {
    let key = alphanumeric1.parse_next(input)?;
    Ok(key)
}

fn parse_val<'a>(input: &mut &'a str) -> PResult<&'a str> {
    let inner = take_while(1.., |c: char| {
        c.is_alphanumeric() || c == '.' || c == '/' || c == ':'
    });
    let val = delimited('"', inner, '"').parse_next(input)?;
    Ok(val)
}

fn parse_attribute<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str)> {
    separated_pair(
        parse_key,
        delimited(multispace0, "=", multispace0),
        parse_val,
    )
    .parse_next(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Attributes<'i> {
    kvs: HashMap<&'i str, &'i str>,
}

impl<'i> Attributes<'i> {
    fn parse(input: &mut &'i str) -> PResult<Self> {
        let kvs = separated0(parse_attribute, (",", multispace0)).parse_next(input)?;
        Ok(Self { kvs })
    }
}
// parser combinators --> calls parsers many times

#[derive(Debug, PartialEq, Eq)]
pub struct Tag<'i> {
    tag_type: &'i str,
    attributes: Attributes<'i>,
}

impl<'i> Tag<'i> {
    fn parse(input: &mut &'i str) -> PResult<Self> {
        let parse_parts = (alpha1, ' ', Attributes::parse);
        let parse_tag = parse_parts.map(|(tag_type, _space, attributes)| Self {
            tag_type,
            attributes,
        });
        let tag = delimited("<", parse_tag, ">").parse_next(input)?;
        Ok(tag)
    }
}

// <div width="40" height="100">

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_key() {
        let input = "width";
        let actual = parse_key.parse(input).unwrap();
        let expected = "width";
        assert_eq!(actual, expected)
    }
    #[test]
    fn test_val() {
        let input = r#""40""#;
        let actual = parse_val.parse(input).unwrap();
        let expected = "40";
        assert_eq!(actual, expected)
    }
    #[test]
    fn test_attribute_with_spaces() {
        let input = r#"width = "40""#;
        let actual = parse_attribute.parse(input).unwrap();
        let expected = ("width", "40");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_attributes() {
        let input = r#"width="40", height="30""#;
        let actual = Attributes::parse.parse(input).unwrap();
        let expected = Attributes {
            kvs: HashMap::from([("width", "40"), ("height", "30")]),
        };
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_tag() {
        let input = r#"<div width="40",height="30">"#;
        let expected = Tag {
            tag_type: "div",
            attributes: Attributes {
                kvs: HashMap::from([("width", "40"), ("height", "30")]),
            },
        };
        let actual = Tag::parse.parse(input).unwrap();
        assert_eq!(actual, expected);
    }
}

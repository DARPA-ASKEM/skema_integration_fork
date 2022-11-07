use crate::ast::{
    Math, MathExpression,
    MathExpression::{
        Mfrac, Mi, Mn, Mo, MoLine, Mover, Mrow, Mspace, Msqrt, Mstyle, Msub, Msubsup, Msup, Mtext,
        Munder,
    },
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, multispace0},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError<'a> {
    span: Span<'a>,
    message: String,
}

/// We implement the ParseError trait here to support the Span type.
impl<'a> ParseError<'a> {
    pub fn new(message: String, span: Span<'a>) -> Self {
        Self { message, span }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn line(&self) -> u32 {
        self.span().location_line()
    }

    pub fn offset(&self) -> usize {
        self.span().location_offset()
    }
}

/// Further trait implementation for Span
impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::new(format!("Parse error {:?}", kind), input)
    }

    fn append(_input: Span<'a>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self::new(format!("Unexpected character '{}'", c), input)
    }
}

/// Implementing ContextError to support Span
impl<'a> nom::error::ContextError<Span<'a>> for ParseError<'a> {
    fn add_context(input: Span<'a>, ctx: &'static str, other: Self) -> Self {
        let message = format!("{}: {}", ctx, other.message);
        ParseError::new(message, input)
    }
}

/// Redefine IResult, filling in the first generic type parameter with Span, for increased brevity.
type IResult<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading
/// and trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: FnMut(Span<'a>) -> IResult<O>,
{
    delimited(multispace0, inner, multispace0)
}

///Quoted string
fn quoted_string(input: Span) -> IResult<Span> {
    delimited(tag("\""), take_until("\""), tag("\""))(input)
}

fn attribute(input: Span) -> IResult<(&str, &str)> {
    let (s, (key, value)) = ws(separated_pair(alphanumeric1, ws(tag("=")), quoted_string))(input)?;
    Ok((s, (&key, &value)))
}

macro_rules! stag {
    ($tag:expr) => {{
        tuple((tag("<"), tag($tag), many0(attribute), tag(">")))
    }};
}

macro_rules! etag {
    ($tag:expr) => {{
        delimited(tag("</"), tag($tag), tag(">"))
    }};
}

/// A macro to help build tag parsers
macro_rules! tag_parser {
    ($tag:expr, $parser:expr) => {{
        ws(delimited(stag!($tag), $parser, etag!($tag)))
    }};
}

/// A macro to help build parsers for simple MathML elements (i.e., without further nesting).
macro_rules! elem0 {
    ($tag:expr) => {{
        let tag_end = concat!("</", $tag, ">");
        tag_parser!($tag, take_until(tag_end))
    }};
}

/// A macro to help build parsers for MathML elements with 1 argument.
macro_rules! elem1 {
    ($tag:expr, $t:ident) => {{
        map(tag_parser!($tag, math_expression), |x| $t(Box::new(x)))
    }};
}

/// A macro to help build parsers for MathML elements with 2 arguments.
macro_rules! elem2 {
    ($tag:expr, $t:ident) => {{
        map(
            tag_parser!($tag, pair(math_expression, math_expression)),
            |(x, y)| $t(Box::new(x), Box::new(y)),
        )
    }};
}

/// A macro to help build parsers for MathML elements with zero or more arguments.
macro_rules! elem_many0 {
    ($tag:expr) => {{
        tag_parser!($tag, many0(math_expression))
    }};
}

/// Identifiers
fn mi(input: Span) -> IResult<MathExpression> {
    let (s, element) = elem0!("mi")(input)?;
    Ok((s, Mi(&element)))
}

/// Numbers
fn mn(input: Span) -> IResult<MathExpression> {
    let (s, element) = elem0!("mn")(input)?;
    Ok((s, Mn(&element)))
}

/// Operators
fn mo(input: Span) -> IResult<MathExpression> {
    let (s, element) = elem0!("mo")(input)?;
    Ok((s, Mo(&element)))
}

/// Rows
fn mrow(input: Span) -> IResult<MathExpression> {
    let (s, elements) = elem_many0!("mrow")(input)?;
    Ok((s, Mrow(elements)))
}

/// Fractions
fn mfrac(input: Span) -> IResult<MathExpression> {
    let (s, frac) = elem2!("mfrac", Mfrac)(input)?;
    Ok((s, frac))
}

/// Superscripts
fn msup(input: Span) -> IResult<MathExpression> {
    let (s, expression) = elem2!("msup", Msup)(input)?;
    Ok((s, expression))
}

/// Subscripts
fn msub(input: Span) -> IResult<MathExpression> {
    let (s, expression) = elem2!("msub", Msub)(input)?;
    Ok((s, expression))
}

/// Square roots
fn msqrt(input: Span) -> IResult<MathExpression> {
    let (s, expression) = elem1!("msqrt", Msqrt)(input)?;
    Ok((s, expression))
}

// Underscripts
fn munder(input: Span) -> IResult<MathExpression> {
    let (s, elements) = elem_many0!("munder")(input)?;
    Ok((s, Munder(elements)))
}

// Overscipts
fn mover(input: Span) -> IResult<MathExpression> {
    let (s, elements) = elem_many0!("mover")(input)?;
    Ok((s, Mover(elements)))
}

// Subscript-superscript Pair
fn msubsup(input: Span) -> IResult<MathExpression> {
    let (s, elements) = elem_many0!("msubsup")(input)?;
    Ok((s, Msubsup(elements)))
}

//Text
fn mtext(input: Span) -> IResult<MathExpression> {
    let (s, element) = elem0!("mtext")(input)?;
    Ok((s, Mtext(&element)))
}

//mstyle
fn mstyle(input: Span) -> IResult<MathExpression> {
    let (s, elements) = elem_many0!("mstyle")(input)?;
    Ok((s, Mstyle(elements)))
}

// function for xml
fn xml_declaration(input: Span) -> IResult<()> {
    let (s, _contents) = ws(delimited(tag("<?"), take_until("?>"), tag("?>")))(input)?;
    Ok((s, ()))
}

//mspace
fn mspace(input: Span) -> IResult<MathExpression> {
    let (s, element) = ws(delimited(tag("<mspace"), take_until("/>"), tag("/>")))(input)?;
    Ok((s, Mspace(&element)))
}

// Some xml have <mo .../>
fn mo_line(input: Span) -> IResult<MathExpression> {
    let (s, element) = ws(delimited(tag("<mo"), take_until("/>"), tag("/>")))(input)?;
    Ok((s, MoLine(&element)))
}

/// Math expressions
fn math_expression(input: Span) -> IResult<MathExpression> {
    ws(alt((
        mi, mo, mn, msup, msub, msqrt, mfrac, mrow, munder, mover, msubsup, mtext, mstyle, mspace,
        mo_line,
    )))(input)
}

/// testing MathML documents
fn math(input: Span) -> IResult<Math> {
    //let (s, elements) = elem_many0!("math")(input)?;
    let (s, elements) = preceded(opt(xml_declaration), elem_many0!("math"))(input)?;
    Ok((s, Math { content: elements }))
}

/// The `parse` function is part of the public API. It takes a string and returns a Math object.
pub fn parse(input: &str) -> IResult<Math> {
    let span = Span::new(input);
    let (remaining, math) = math(span)?;
    Ok((remaining, math))
}

/// A generic helper function for testing individual parsers.
#[cfg(test)]
fn test_parser<'a, P, O>(input: &'a str, mut parser: P, output: O)
where
    P: FnMut(Span<'a>) -> IResult<'a, O>,
    O: std::cmp::PartialEq + std::fmt::Debug,
{
    assert_eq!(parser(Span::new(input)).unwrap().1, output);
}

#[test]
fn test_mi() {
    test_parser("<mi k=\"v\" m1=\"n\">x</mi>", mi, Mi("x"))
}

#[test]
fn test_mo() {
    test_parser("<mo>=</mo>", mo, Mo("="))
}

#[test]
fn test_mn() {
    test_parser("<mn>1</mn>", mn, Mn("1"));
}

#[test]
fn test_mrow() {
    test_parser(
        "<mrow><mo>-</mo><mi>b</mi></mrow>",
        mrow,
        Mrow(vec![Mo("-"), Mi("b")]),
    )
}

#[test]
fn test_attribute() {
    test_parser("key=\"value\"", attribute, ("key", "value"))
}

#[test]
fn test_mfrac() {
    let frac = mfrac(Span::new("<mfrac><mn>1</mn><mn>2</mn></mfrac>"))
        .unwrap()
        .1;
    assert_eq!(frac, Mfrac(Box::new(Mn("1")), Box::new(Mn("2"))),)
}

#[test]
fn test_math_expression() {
    test_parser(
        "<mrow><mo>-</mo><mi>b</mi></mrow>",
        math_expression,
        Mrow(vec![Mo("-"), Mi("b")]),
    )
}

#[test]
fn test_mover() {
    test_parser(
        "<mover><mi>x</mi><mo>¯</mo></mover>",
        mover,
        Mover(vec![Mi("x"), Mo("¯")]),
    )
}

#[test]
fn test_munder() {
    test_parser(
        "<munder><mo>inf</mo><mn>0</mn><mo>≤</mo><mi>t</mi><mo>≤</mo></munder>",
        munder,
        Munder(vec![Mo("inf"), Mn("0"), Mo("≤"), Mi("t"), Mo("≤")]),
    )
}

#[test]
fn test_msubsup() {
    test_parser(
        "<msubsup><mi>L</mi><mi>t</mi><mi>∞</mi></msubsup>",
        msubsup,
        Msubsup(vec![Mi("L"), Mi("t"), Mi("∞")]),
    )
}

#[test]
fn test_mtext() {
    test_parser("<mtext>if</mtext>", mtext, Mtext("if"));
}

#[test]
fn test_mstyle() {
    test_parser(
        "<mstyle><mo>∑</mo><mi>I</mi></mstyle>",
        mstyle,
        Mstyle(vec![Mo("∑"), Mi("I")]),
    )
}

#[test]
fn test_mspace() {
    test_parser("<mspace width=\"1em\"/>", mspace, Mspace(" width=\"1em\""));
}

#[test]
fn test_moline() {
    test_parser(
        "<mo fence=\"true\" stretchy=\"true\" symmetric=\"true\"/>",
        mo_line,
        MoLine(" fence=\"true\" stretchy=\"true\" symmetric=\"true\""),
    );
}

#[test]
fn test_math() {
    test_parser(
        "<math>
                <mrow>
                    <mo>-</mo>
                    <mi>b</mi>
                </mrow>
            </math>",
        math,
        Math {
            content: vec![Mrow(vec![Mo("-"), Mi("b")])],
        },
    )
}

#[test]
fn test_mathml_parser() {
    let eqn = std::fs::read_to_string("tests/test01.xml").unwrap();
    test_parser(
        &eqn,
        math,
        Math {
            content: vec![
                Munder(vec![
                    Mo("sup"),
                    Mrow(vec![
                        Mn("0"),
                        Mo("≤"),
                        Mi("t"),
                        Mo("≤"),
                        Msub(Box::new(Mi("T")), Box::new(Mn("0"))),
                    ]),
                ]),
                Mo("‖"),
                Msup(
                    Box::new(Mrow(vec![Mover(vec![Mi("ρ"), Mo("~")])])),
                    Box::new(Mi("R")),
                ),
                Msup(
                    Box::new(Mrow(vec![Mover(vec![Mi("x"), Mo("¯")])])),
                    Box::new(Mi("a")),
                ),
                Msub(
                    Box::new(Mo("‖")),
                    Box::new(Mrow(vec![
                        Msup(Box::new(Mi("L")), Box::new(Mn("1"))),
                        Mo("∩"),
                        Msup(Box::new(Mi("L")), Box::new(Mi("∞"))),
                    ])),
                ),
                Mo("≤"),
                Mi("C"),
            ],
        },
    )
}
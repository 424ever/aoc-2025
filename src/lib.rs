use winnow::{Parser, combinator::delimited, error::ParserError, stream::Stream};

pub fn surrounded<Input, Output, Error, ParseNext, Border, Ignored>(
    parser: ParseNext,
    border: Border,
) -> impl Parser<Input, Output, Error>
where
    Input: Stream,
    Error: ParserError<Input>,
    ParseNext: Parser<Input, Output, Error>,
    Border: Parser<Input, Ignored, Error> + Clone,
{
    delimited(border.clone(), parser, border)
}

use nom::{
    bytes::complete::{tag, take_while},
    multi::many0,
    IResult,
};

use crate::dna::Dna;

pub fn parse_records(input: &str) -> IResult<&str, Vec<FastaDna>> {
    many0(parse_record)(input)
}

fn parse_record(input: &str) -> IResult<&str, FastaDna> {
    let (input, id) = parse_id(input)?;
    let (input, sequence) = parse_sequence(input)?;

    Ok((input, FastaDna { id, sequence }))
}

fn parse_id(input: &str) -> IResult<&str, String> {
    let (input, _) = tag(">")(input)?;
    let (input, id) = take_while(|c| c != '\n')(input)?;

    Ok((&input[1..], id.to_string()))
}

fn parse_sequence(input: &str) -> IResult<&str, Dna> {
    let (input, sequence) = take_while(|c| c != '>')(input)?;
    Ok((input, Dna::from_fasta_body(sequence)))
}

#[derive(Debug, PartialEq)]
pub struct FastaDna {
    pub id: String,
    pub sequence: Dna,
}

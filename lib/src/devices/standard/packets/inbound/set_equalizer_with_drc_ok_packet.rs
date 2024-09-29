use nom::error::{ContextError, ParseError};

use crate::devices::standard::packets::parsing::ParseResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetEqualizerWithDrcOkPacket {}

impl SetEqualizerWithDrcOkPacket {
    #[allow(dead_code)]
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<SetEqualizerWithDrcOkPacket, E> {
        Ok((input, SetEqualizerWithDrcOkPacket::default()))
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::packets::inbound::take_inbound_packet_body;

    use super::SetEqualizerWithDrcOkPacket;

    #[test]
    fn it_parses_an_example_ok_packet() {
        let input: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x02, 0x83, 0x0a, 0x00, 0x98];
        let (_, body) = take_inbound_packet_body(input).unwrap();
        SetEqualizerWithDrcOkPacket::take::<VerboseError<_>>(body).expect("should not error");
    }
}

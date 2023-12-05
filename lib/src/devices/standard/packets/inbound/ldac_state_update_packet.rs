use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
};

use crate::devices::standard::packets::parsing::{take_bool, ParseResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LdacStateUpdatePacket {
    pub is_enabled: bool,
}

pub fn take_ldac_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<LdacStateUpdatePacket, E> {
    context(
        "LdacStateUpdatePacket",
        all_consuming(map(take_bool, |is_enabled| LdacStateUpdatePacket {
            is_enabled,
        })),
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::packets::inbound::{
        take_inbound_packet_body, take_ldac_state_update_packet,
    };

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x7F, 0x0c, 0x00, 0x01, 0x96,
        ];
        let (_, body) = take_inbound_packet_body(input).unwrap();
        let packet = take_ldac_state_update_packet::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert_eq!(true, packet.is_enabled);
    }
}

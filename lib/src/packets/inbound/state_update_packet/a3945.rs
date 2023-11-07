use nom::{
    bytes::complete::take,
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};

use crate::packets::{
    parsing::{
        take_battery_level, take_bool, take_custom_button_model, take_dual_battery,
        take_equalizer_configuration, take_firmware_version, take_serial_number, ParseResult,
    },
    structures::{
        BatteryLevel, CustomButtonModel, DeviceFeatureFlags, DualBattery, EqualizerConfiguration,
        FirmwareVersion, SerialNumber,
    },
};

use super::StateUpdatePacket;

// A3945 only
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct A3945StateUpdatePacket {
    host_device: u8,
    tws_status: bool,
    battery: DualBattery,
    left_firmware: FirmwareVersion,
    right_firmware: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: EqualizerConfiguration, // 10 bands mono
    custom_button_model: CustomButtonModel,
    touch_tone_switch: bool,
    wear_detection_switch: bool,
    game_mode_switch: bool,
    charging_case_battery_level: BatteryLevel,
    bass_up_switch: bool,
    device_color: u8,
}

impl From<A3945StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3945StateUpdatePacket) -> Self {
        Self {
            feature_flags: DeviceFeatureFlags::EQUALIZER | DeviceFeatureFlags::CUSTOM_BUTTON_MODEL,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: None,
            age_range: None,
            gender: None,
            hear_id: None,
            custom_button_model: Some(packet.custom_button_model),
            firmware_version: Some(packet.left_firmware.min(packet.right_firmware)),
            serial_number: Some(packet.serial_number),
            dynamic_range_compression_min_firmware_version: None,
        }
    }
}

pub fn take_a3945_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<A3945StateUpdatePacket, E> {
    context(
        "StateUpdatePacket",
        all_consuming(map(
            tuple((
                le_u8,
                take_bool,
                take_dual_battery,
                take_firmware_version,
                take_firmware_version,
                take_serial_number,
                take_equalizer_configuration(10),
                take(10usize), // ??, maybe stereo eq?
                take_custom_button_model,
                take_bool,
                take_bool,
                take_bool,
                take_battery_level,
                take_bool,
                le_u8,
            )),
            |(
                host_device,
                tws_status,
                battery,
                left_firmware,
                right_firmware,
                serial_number,
                equalizer_configuration,
                _, // ??
                custom_button_model,
                touch_tone_switch,
                wear_detection_switch,
                game_mode_switch,
                charging_case_battery_level,
                bass_up_switch,
                device_color,
            )| {
                A3945StateUpdatePacket {
                    host_device,
                    tws_status,
                    battery,
                    left_firmware,
                    right_firmware,
                    serial_number,
                    equalizer_configuration,
                    custom_button_model,
                    touch_tone_switch,
                    wear_detection_switch,
                    game_mode_switch,
                    charging_case_battery_level,
                    bass_up_switch,
                    device_color,
                }
            },
        )),
    )(input)
}

use super::PacketReadWrite;
use crate::AsciiString;

// ----------------------------------------------------------------
// Settings packets
// ----------------------------------------------------------------

// 0x2B, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SaveSettingsPacket {
    #[VariableStr(0xCEF1, 0xB5)]
    pub settings: AsciiString,
}

// 0x2B, 0x02
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadSettingsPacket {
    #[VariableStr(0x54AF, 0x100)]
    pub settings: AsciiString,
}

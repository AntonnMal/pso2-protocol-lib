use super::{HelperReadWrite, PacketReadWrite};
use std::time::Duration;

// ----------------------------------------------------------------
// Mail packets
// ----------------------------------------------------------------

// 0x1A, 0x00
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x00)]
pub struct MailListRequestPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

// 0x1A, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct MailListPacket {
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: [u8; 4],
    pub unk6: u32,
    #[VariableStr(0x36A1, 0xBF)]
    pub name: String,
    #[VariableStr(0x36A1, 0xBF)]
    pub nickname: String,
    #[Magic(0x36A1, 0xBF)]
    pub headers: Vec<MailHeader>,
}

// 0x1A, 0x02
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct DeleteMailRequestPacket {
    #[Magic(0xBC5F, 0x0B)]
    pub ids: Vec<MailId>,
}

// 0x1A, 0x03
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct DeletedMailPacket {
    #[Magic(0x421C, 0x56)]
    pub ids: Vec<MailId>,
    pub unk: u32,
}

// 0x1A, 0x06
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x06)]
pub struct MailBodyRequestPacket {
    pub id: MailId,
}

// 0x1A, 0x07
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x07)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct MailBodyPacket {
    pub id: MailId,
    #[VariableStr(0x5913, 0x82)]
    pub message: String,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MailId {
    pub mail_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MailHeader {
    pub mail_id: u32,
    pub unk2: u32,
    pub user_id: u32,
    pub unk3: [u8; 0x14],
    pub unk4: u32,
    pub unk5: u32,
    pub receive_time: Duration,
    pub unk6: u32,
    #[FixedStr(0x22)]
    pub sender: String,
    #[FixedStr(0x2A)]
    pub subject: String,
}

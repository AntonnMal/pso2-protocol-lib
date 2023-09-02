use super::{
    models::{character::Character, FunValue, SGValue},
    EntityType, Flags, HelperReadWrite, ObjectHeader, PacketHeader, PacketReadWrite,
};
use crate::AsciiString;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{Read, Seek, Write},
    net::Ipv4Addr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// ----------------------------------------------------------------
// Login packets
// ----------------------------------------------------------------

// 0x11, 0x00
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x00)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SegaIDLoginPacket {
    //FIXME: fix data sizes
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub ver_id: [u8; 0x20],
    #[Magic(0x5E6, 0x6B)]
    pub interfaces: Vec<NetInterface>,
    #[Seek(0x14)]
    pub unk4: [u8; 0x90],
    #[Seek(0x10)]
    pub unk5: [u8; 0x10],
    #[Seek(0x10)]
    pub text_lang: u32,
    // 0 - jp, 1 - en
    pub voice_lang: u32,
    pub text_lang2: u32,
    pub lang_lang: u32,
    #[Seek(0x8)]
    #[FixedStr(0x10)]
    pub language: String,
    pub unk6: u32,
    pub unk7: u32,
    pub magic1: u32,
    pub unk8: [u8; 0x20],
    pub unk9: [u8; 0x44],
    #[Seek(0x104)]
    #[FixedStr(0x40)]
    pub username: AsciiString,
    #[Seek(0x20)]
    #[FixedStr(0x40)]
    pub password: AsciiString,
    #[Seek(0x4)]
    pub unk10: u32,
    #[SeekAfter(0x4)]
    #[VariableStr(0x5E6, 0x6B)]
    pub unk11: AsciiString,
}

// 0x11, 0x01
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoginResponsePacket {
    pub status: LoginStatus,
    #[VariableStr(0x8BA4, 0xB6)]
    pub error: String,
    pub player: ObjectHeader,
    #[FixedStr(0x20)]
    pub blockname: String,
    pub unk1: f32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: f32,
    pub unk6: f32,
    pub unk7: u32,
    pub unk8: f32,
    pub unk9: f32,
    pub unk10: u32,
    pub unk11: f32,
    pub unk12: u32,
    pub unk13: f32,
    pub unk14: [f32; 0xA],
    pub unk15: [f32; 0x15],
    pub unk16: u32,
    pub unk17: u32,
}

// 0x11, 0x03
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CharacterListPacket {
    pub is_global: bool,
    pub characters: Vec<Character>,
    pub play_times: [u32; 30],
    pub deletion_flags: [(u32, u32); 30],
    pub transfer_flags: [(u32, u32); 30],
    pub account_accessory: u16,
    pub login_survey: u32,
    pub ad: u32,
}

//0x11, 0x04
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x4)]
pub struct StartGamePacket {
    pub char_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

// 0x11, 0x05
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CharacterCreatePacket {
    pub character: Character,
    pub is_global: bool,
}

// 0x11, 0x0B
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EncryptionRequestPacket {
    pub rsa_data: Vec<u8>,
}

// 0x11, 0x0C
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EncryptionResponsePacket {
    pub data: Vec<u8>,
}

// 0x11, 0x0D
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x0D)]
pub struct ClientPingPacket {
    #[PSOTime]
    pub time: Duration,
}

// 0x11, 0x0E
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x0E)]
pub struct ClientPongPacket {
    #[PSOTime]
    pub client_time: Duration,
    #[PSOTime]
    pub server_time: Duration,
    pub unk1: u32,
}

// 0x11, 0x10
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x10)]
pub struct BlockListPacket {
    pub blocks: [BlockInfo; 200],
    pub unk: u32,
}

// 0x11, 0x1B
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1B)]
pub struct UserInfoNGSPacket {
    // i'm unsure about real types, just deriving from base version struct
    pub unk1: [u32; 22],
    pub unk2: u16,
    pub unk3: [u32; 16],
    pub fun: FunValue,
    pub unk4: [u32; 2],
    pub free_sg: SGValue,
    pub unk5: u16,
    pub unk6: [u32; 24],
    pub premium_expiration: Duration,
    pub unk7: u32,
    pub pq_expiration: Duration,
    pub pshop_expiration: Duration,
    pub unk8: [u32; 2],
    pub expand_max_orders_expiration: Duration,
    pub unk9: [u32; 19],
    pub material_storage_expiration: Duration,
    pub ex_storage4_expiration: Duration,
    pub ex_storage5_expiration: Duration,
    pub unk10: [u32; 4],
}

#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1B)]
pub struct UserInfoPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub ac1: u32,
    pub unk3: u32,
    pub ac2: u32,
    pub ac3: u32,
    pub ac4: u32,
    // also pso2es char id
    pub ac5: u32,
    pub ac6: u32,
    // also unlnked es account flag?
    pub ac7: u32,
    pub ac8: [u32; 11],
    pub fun: u32,
    pub unk4: u16,
    pub sg1: SGValue,
    pub free_sg: SGValue,
    pub sg2: [SGValue; 18],
    pub unk5: u16,
    pub unk6: [u32; 6],
    pub premium_expiration: Duration,
    pub unk7: u32,
    pub pq_expiration: Duration,
    pub pshop_expiration: Duration,
    pub unk8: [u32; 2],
    pub expand_max_orders_expiration: Duration,
    pub unk9: [u32; 19],
    pub material_storage_expiration: Duration,
    pub ex_storage4_expiration: Duration,
    pub ex_storage5_expiration: Duration,
}

// 0x11, 0x1E
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1E)]
pub struct NicknameRequestPacket {
    // Judging by Polaris Server this packet contains 0x44 byte long array of something
    #[SeekAfter(0x42)]
    pub error: u16,
}

// 0x11, 0x1D
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1D)]
pub struct NicknameResponsePacket {
    #[FixedStr(0x10)]
    #[SeekAfter(0x20)]
    pub nickname: String,
}

// 0x11, 0x2C
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x2C)]
pub struct BlockBalancePacket {
    pub unk1: [u8; 0x20],
    #[FixedStr(0x20)]
    pub blockname: String,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub unk2: [u8; 0x11A],
}

// 0x11, 0x2D
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x2D)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SystemInformationPacket {
    #[VariableStr(0x883D, 0x9F)]
    pub cpu_info: AsciiString,
    #[VariableStr(0x883D, 0x9F)]
    pub video_info: AsciiString,
    pub vram: u64,
    pub total_ram: u64,
    pub unk1: u32,
    pub unk2: u32,
    #[VariableStr(0x883D, 0x9F)]
    pub windows_version: String,
    #[VariableStr(0x883D, 0x9F)]
    pub window_size: AsciiString,
    #[VariableStr(0x883D, 0x9F)]
    pub unk3: String,
    #[VariableStr(0x883D, 0x9F)]
    pub unk4: String,
    #[VariableStr(0x883D, 0x9F)]
    pub video_driver: String,
    pub total_disk_space: u64,
    pub free_disk_space: u64,
}

// 0x11, 0x3D
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x3D)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct ShipListPacket {
    #[Magic(0xE418, 0x51)]
    pub ships: Vec<ShipEntry>,
    pub timestamp: Duration,
}

// 0x11, 0x42
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x42)]
pub struct CreateCharacter1ResponsePacket {
    pub status: u32,
    pub unk2: u32,
    pub used_smth: u32,
    pub req_ac: u32,
}

// 0x11, 0x55
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x55)]
pub struct CreateCharacter2ResponsePacket {
    pub unk: u32,
}

// 0x11, 0x63
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x63)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct VitaLoginPacket {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u16,
    pub unk4: u32,
    pub unk5: u32,
    pub ver_id: [u8; 0x20],
    #[Magic(0xBE3F, 0x77)]
    pub interfaces: Vec<NetInterface>,
    pub unk6: [u8; 0x10],
    #[Seek(0x4)]
    pub unk7: [u8; 0x90],
    #[Seek(0x10)]
    pub unk8: [u8; 0x10],
    #[Seek(0x10)]
    pub flag1: u32,
    pub flag2: u32,
    pub flag3: u32,
    pub flag4: u32,
    pub flag5: u32,
    pub flag6: u32,
    #[FixedStr(0x10)]
    pub language: String,
    pub unk9: u32,
    pub unk10: u32,
    pub magic1: u32,
    pub unk11: [u8; 0x20],
    pub unk12: [u8; 0x44],
    #[Seek(0xFC)]
    #[FixedStr(0x40)]
    pub username: AsciiString,
    #[Seek(0x20)]
    #[FixedStr(0x40)]
    pub password: AsciiString,
    #[Seek(0x4)]
    pub unk13: u8,
    pub unk14: u8,
    pub unk15: u16,
    #[VariableStr(0xBE3F, 0x77)]
    pub unk16: AsciiString,
    #[Magic(0xBE3F, 0x77)]
    pub unk17: Vec<u8>,
    pub unk18: [u8; 0x10],
}

// 0x11, 0x67
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x67)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SalonResponse {
    pub reedit_time: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    #[Magic(0xD536, 0xA4)]
    pub unk5: Vec<SalonThing1>,
    #[Magic(0xD536, 0xA4)]
    pub unk6: Vec<SalonThing2>,
    pub unk7: u32,
}

// 0x11, 0x71
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x71)]
pub struct NotificationStatusPacket {
    pub new_mail: u32,
    pub unk1: u32,
    pub campaigns: u32,
    pub unk3: u32,
}

// 0x11, 0x87
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x87)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoginHistoryPacket {
    #[Magic(0x8ceb, 8)]
    pub attempts: Vec<LoginAttempt>,
}

// 0x11, 0xEA
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xEA)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct NicknameErrorPacket {
    pub unk1: u32,
    #[VariableStr(0x4544, 0x14)]
    pub nickname: String,
}

// 0x11, 0xED
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xED)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct BannerListPacket {
    #[VariableStr(0xD67D, 0xF5)]
    pub banners: AsciiString,
}

// 0x11, 0xEE
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xEE)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct EmailCodeRequestPacket {
    pub unk1: u32,
    #[VariableStr(0x5C3B, 0x40)]
    pub message: String,
}

// 0x11, 0xFF
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xFF)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct Unk11FFPacket {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    #[VariableStr(0x3DD3, 0x3D)]
    pub unk5: String,
    pub unk6: [u8; 0xC],
    pub unk7: [u8; 0x40],
    pub unk8: [u8; 0x20],
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct NetInterface {
    pub state: u32,
    #[FixedStr(0x18)]
    pub mac: AsciiString,
}

#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct ShipEntry {
    pub id: u32,
    #[FixedStr(0x10)]
    pub name: String,
    pub ip: Ipv4Addr,
    #[Seek(4)]
    pub status: ShipStatus,
    #[SeekAfter(4)]
    pub order: u16,
}

#[repr(u16)]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
pub enum ShipStatus {
    #[default]
    Unknown,
    Online,
    Busy,
    Full,
    Offline,

    #[Read_default]
    Undefined = 0xFFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoginAttempt {
    pub ip: Ipv4Addr,
    pub status: LoginResult,
    pub timestamp: Duration,
    pub unk: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum LoginResult {
    #[default]
    Successful,
    EmailConfirmed,
    LoginError,
    EmailAuthError,
    AuthEmailSent,
    OTPError,
    InMaintenance,
    GenericError,

    #[Read_default]
    Undefined = 0xFFFF_FFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum LoginStatus {
    #[default]
    Success,
    Failure,

    #[Read_default]
    Undefined = 0xFFFF_FFFF,
}

#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct BlockInfo {
    pub unk1: u32,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u16,
    pub unk9: u16,
    #[FixedStr(0x20)]
    pub blockname: String,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub unk10: u16,
    pub unk11: u16,
    pub unk12: [u8; 10],
}

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct SalonThing1 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct SalonThing2 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl PacketReadWrite for CharacterListPacket {
    fn read(reader: &mut (impl Read + Seek), _: Flags) -> std::io::Result<Self> {
        let char_amount = reader.read_u32::<LittleEndian>()?.clamp(0, 30);
        reader.seek(std::io::SeekFrom::Current(4))?;
        let mut characters = vec![];
        for i in 0..30 {
            reader.seek(std::io::SeekFrom::Current(4))?;
            let character = Character::read(reader)?;
            if i < char_amount {
                characters.push(character);
            }
        }
        // ???
        reader.seek(std::io::SeekFrom::Current(0x41A4))?;
        let mut play_times = [0u32; 30];
        for item in &mut play_times {
            *item = reader.read_u32::<LittleEndian>()?;
        }
        reader.seek(std::io::SeekFrom::Current(32))?;
        let mut deletion_flags = [(0u32, 0u32); 30];
        for item in &mut deletion_flags {
            item.0 = reader.read_u32::<LittleEndian>()?;
            item.1 = reader.read_u32::<LittleEndian>()?;
        }
        let mut transfer_flags = [(0u32, 0u32); 30];
        for item in &mut transfer_flags {
            item.0 = reader.read_u32::<LittleEndian>()?;
            item.1 = reader.read_u32::<LittleEndian>()?;
        }
        let account_accessory = reader.read_u16::<LittleEndian>()?;
        reader.seek(std::io::SeekFrom::Current(6))?;
        let login_survey = reader.read_u32::<LittleEndian>()?;
        let ad = reader.read_u32::<LittleEndian>()?;

        Ok(Self {
            is_global: false,
            characters,
            play_times,
            deletion_flags,
            transfer_flags,
            account_accessory,
            login_survey,
            ad,
        })
    }
    fn write(&self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x03, Flags::default()).write(is_ngs);
        buf.write_u32::<LittleEndian>((self.characters.len() as u32).clamp(0, 30))
            .unwrap();
        buf.write_u32::<LittleEndian>(0).unwrap();

        let mut characters = &self.characters;
        let default_character = vec![Character::default()];
        if characters.is_empty() {
            characters = &default_character;
        }

        for character in characters.iter().cycle().take(30) {
            buf.write_u32::<LittleEndian>(0).unwrap();
            character.write(&mut buf, self.is_global).unwrap();
        }
        // ???
        for _ in 0..0x41A4 {
            buf.write_u8(0).unwrap();
        }
        for i in 0..30 {
            buf.write_u32::<LittleEndian>(self.play_times[i]).unwrap();
        }
        // ???
        for _ in 0..32 {
            buf.write_u8(0).unwrap();
        }
        for i in 0..30 {
            // deletion flag
            buf.write_u32::<LittleEndian>(self.deletion_flags[i].0)
                .unwrap();
            // timestamp
            buf.write_u32::<LittleEndian>(self.deletion_flags[i].1)
                .unwrap();
        }
        for i in 0..30 {
            // transfer flag
            buf.write_u32::<LittleEndian>(self.transfer_flags[i].0)
                .unwrap();
            // ??? prob target ship
            buf.write_u32::<LittleEndian>(self.transfer_flags[i].1)
                .unwrap();
        }
        buf.write_u16::<LittleEndian>(self.account_accessory)
            .unwrap();
        // ???
        buf.write_all(&[0u8; 6]).unwrap();
        buf.write_u32::<LittleEndian>(self.login_survey).unwrap();
        buf.write_u32::<LittleEndian>(self.ad).unwrap();
        // ???
        buf.write_u32::<LittleEndian>(0x00_00_00_00).unwrap();
        // ???
        buf.write_u32::<LittleEndian>(0x00_00_00_00).unwrap();
        buf
    }
}

impl PacketReadWrite for CharacterCreatePacket {
    fn read(reader: &mut (impl Read + Seek), _: Flags) -> std::io::Result<Self> {
        let character = Character::read(reader)?;
        Ok(Self {
            character,
            is_global: false,
        })
    }
    fn write(&self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x05, Flags::default()).write(is_ngs);
        self.character.write(&mut buf, self.is_global).unwrap();
        buf
    }
}

impl PacketReadWrite for EncryptionRequestPacket {
    fn read(reader: &mut impl Read, _: Flags) -> std::io::Result<Self> {
        let mut rsa_data = vec![];
        reader.read_to_end(&mut rsa_data)?;
        let mut tmp_data = vec![];
        let mut iter = rsa_data.into_iter().rev().skip(4);
        if let Some(x) = iter.find(|x| *x != 0x00) {
            tmp_data.push(x);
            tmp_data.extend(iter);
        }
        Ok(Self { rsa_data: tmp_data })
    }
    fn write(&self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x0B, Flags::default()).write(is_ngs);
        let mut data = self.rsa_data.clone();
        data.reverse();
        data.resize(0x104, 0);
        buf.extend(data);
        buf
    }
}

impl PacketReadWrite for EncryptionResponsePacket {
    fn read(reader: &mut impl Read, _: Flags) -> std::io::Result<Self> {
        let mut data = vec![];
        reader.read_to_end(&mut data)?;

        Ok(Self { data })
    }
    fn write(&self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x0C, Flags::default()).write(is_ngs);
        buf.extend(self.data.iter());
        buf
    }
}

// ----------------------------------------------------------------
// Default implementations
// ----------------------------------------------------------------

impl Default for SegaIDLoginPacket {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 9,
            unk3: 0,
            ver_id: [0u8; 0x20],
            interfaces: vec![],
            unk4: [0u8; 0x90],
            unk5: [0u8; 0x10],
            text_lang: 0,
            voice_lang: 0,
            text_lang2: 0,
            lang_lang: 0,
            language: String::new(),
            unk6: 7,
            unk7: 7,
            magic1: 0x0419,
            unk8: [0u8; 0x20],
            unk9: [0u8; 0x44],
            username: Default::default(),
            password: Default::default(),
            unk10: 512,
            unk11: Default::default(),
        }
    }
}

impl Default for LoginResponsePacket {
    fn default() -> Self {
        Self {
            status: LoginStatus::Success,
            error: String::new(),
            player: ObjectHeader {
                id: 0,
                unk: 0,
                unk2: 0,
                entity_type: EntityType::Player,
            },
            blockname: String::new(),
            unk1: 60.0,
            unk2: 7,
            unk3: 0xA,
            unk4: 1,
            unk5: 10.0,
            unk6: 5.0,
            unk7: 11,
            unk8: 1.0,
            unk9: 75.0,
            unk10: 40,
            unk11: 10.0,
            unk12: 1,
            unk13: 100.0,
            unk14: [1.0; 0xA],
            unk15: [100.0; 0x15],
            unk16: 0x91A2B,
            unk17: 0x91A2B,
        }
    }
}

impl Default for ClientPingPacket {
    fn default() -> Self {
        Self {
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        }
    }
}

impl Default for ClientPongPacket {
    fn default() -> Self {
        Self {
            client_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            server_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            unk1: 0,
        }
    }
}

impl Default for BlockBalancePacket {
    fn default() -> Self {
        Self {
            unk1: [0u8; 0x20],
            blockname: String::new(),
            ip: Ipv4Addr::UNSPECIFIED,
            port: 0,
            unk2: [0u8; 0x11A],
        }
    }
}

impl Default for ShipListPacket {
    fn default() -> Self {
        Self {
            ships: vec![],
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        }
    }
}

impl Default for VitaLoginPacket {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 9,
            unk5: 0,
            ver_id: [0u8; 0x20],
            interfaces: vec![],
            unk6: [0u8; 0x10],
            unk7: [0u8; 0x90],
            unk8: [0u8; 0x10],
            flag1: 0,
            flag2: 0,
            flag3: 0,
            flag4: 0,
            flag5: 0,
            flag6: 0,
            language: String::new(),
            unk9: 0,
            unk10: 0,
            magic1: 0,
            unk11: [0u8; 0x20],
            unk12: [0u8; 0x44],
            username: Default::default(),
            password: Default::default(),
            unk13: 0,
            unk14: 2,
            unk15: 0,
            unk16: Default::default(),
            unk17: vec![],
            unk18: [0u8; 0x10],
        }
    }
}

impl Default for NicknameErrorPacket {
    fn default() -> Self {
        Self {
            unk1: 2,
            nickname: String::new(),
        }
    }
}

impl Default for Unk11FFPacket {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 0,
            unk5: String::new(),
            unk6: [0; 0xC],
            unk7: [0; 0x40],
            unk8: [0; 0x20],
        }
    }
}

impl Default for ShipEntry {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            ip: Ipv4Addr::UNSPECIFIED,
            status: ShipStatus::Unknown,
            order: 0,
        }
    }
}

impl Default for LoginAttempt {
    fn default() -> Self {
        Self {
            ip: Ipv4Addr::UNSPECIFIED,
            status: LoginResult::Successful,
            timestamp: Duration::new(0, 0),
            unk: 9,
        }
    }
}

impl Default for BlockListPacket {
    fn default() -> Self {
        Self {
            blocks: vec![Default::default(); 200].try_into().unwrap(),
            unk: 0,
        }
    }
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 0,
            unk5: 0,
            unk6: 0,
            unk7: 0,
            unk8: 0,
            unk9: 0,
            blockname: String::new(),
            ip: Ipv4Addr::UNSPECIFIED,
            port: 0,
            unk10: 0,
            unk11: 0,
            unk12: [0; 10],
        }
    }
}

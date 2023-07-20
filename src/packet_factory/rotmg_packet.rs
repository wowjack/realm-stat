use crate::packet_factory::byte_buffer::ByteBuffer;

#[derive(Debug, Clone)]
pub enum RotmgPacket {
    Failure {
        rem: ByteBuffer
    },
    Teleport {
        rem: ByteBuffer
    },
    //Missing
    ClaimLoginReward {
        rem: ByteBuffer
    },
    DeletePet {
        rem: ByteBuffer
    },
    RequestTrade {
        rem: ByteBuffer
    },
    QuestFetchResponse {
        rem: ByteBuffer
    },
    JoinGuild {
        rem: ByteBuffer
    },
    Ping {
        rem: ByteBuffer
    },
    PlayerText {
        rem: ByteBuffer
    },
    NewTick {
        tick_id: u32,
        tick_time: u32,
        server_current_time: u32,
        server_prev_time: u16,
        rem: ByteBuffer
    },
    ShowEffect {
        rem: ByteBuffer
    },
    Text {
        name: String,
        object_id: u32,
        num_stars: u16,
        display_time: u8,
        recipient: String,
        content: String,
        clean_text: String,
        is_supporter: bool,
        star_background: u32
    },
    //BIG GAP
    Reconnect {
        name: String,
        host: String,
        unknown: u32,
        port: u32,
        game_id: u32,
        key: Vec<u8>,
    },
    Move {
        tick_id: u32,
        time: u32,
        rem: ByteBuffer
    },
    MapInfo {
        width: u32,
        height: u32,
        name: String,
        display_name: String,
        realm_name: String,
        difficulty: f32,
        seed: u32,
        background: u32,
        allow_teleport: bool,
        show_displays: bool,
        unknown_bool: bool,
        max_players: u16,
        game_opened_time: u32,
        build_version: String,
        unknown_int: u32,
        dungeon_mods: String
    },
    /**
     * ADD OTHER PACKET TYPES HERE
     */
    Other {
        type_num: u8,
        rem: ByteBuffer
    },
}
impl TryFrom<ByteBuffer> for RotmgPacket {
    type Error = ();

    fn try_from(mut buf: ByteBuffer) -> Result<Self, ()> {
        use RotmgPacket::*;
        let _packet_len = buf.read_u32()?;
        let packet_type = buf.read_u8()?;
        return Ok(match packet_type {
            0 => Failure { rem: buf },
            1 => Teleport { rem: buf },
            3 => ClaimLoginReward { rem: buf },
            4 => DeletePet { rem: buf },
            5 => RequestTrade { rem: buf },
            6 => QuestFetchResponse { rem: buf },
            7 => JoinGuild { rem: buf },
            8 => Ping { rem: buf },
            9 => PlayerText { rem: buf },
            10 => NewTick { tick_id: buf.read_u32()?, tick_time: buf.read_u32()?, server_current_time: buf.read_u32()?, server_prev_time: buf.read_u16()?, rem: buf },
            11 => ShowEffect { rem: buf },
            44 => Text { name: buf.read_string()?, object_id: buf.read_u32()?, num_stars: buf.read_u16()?, display_time: buf.read_u8()?, recipient: buf.read_string()?, content: buf.read_string()?, clean_text: buf.read_string()?, is_supporter: buf.read_bool()?, star_background: buf.read_u32()? },
            45 => Reconnect { name: buf.read_string()?, host: buf.read_string()?, unknown: buf.read_u32()?, port: buf.read_u32()?, game_id: buf.read_u32()?, key: buf.rem_to_vec() },
            62 => Move { tick_id: buf.read_u32()?, time: buf.read_u32()?, rem: buf },
            92 => MapInfo { width: buf.read_u32()?, height: buf.read_u32()?, name: buf.read_string()?, display_name: buf.read_string()?, realm_name: buf.read_string()?, difficulty: buf.read_f32()?, seed: buf.read_u32()?, background: buf.read_u32()?, allow_teleport: buf.read_bool()?, show_displays: buf.read_bool()?, unknown_bool: buf.read_bool()?, max_players: buf.read_u16()?, game_opened_time: buf.read_u32()?, build_version: buf.read_string()?, unknown_int: buf.read_u32()?, dungeon_mods: buf.read_string()? },
            _ => Other { type_num: packet_type, rem: buf},
        })
    }
}


pub struct NewTick {
    tick_id: u32,
    tick_time: u32,
    server_current_time: u32,
    server_prev_time: u16,
    status: Vec<u8>
}
impl TryFrom<ByteBuffer> for NewTick {
    type Error = ();

    fn try_from(mut buf: ByteBuffer) -> Result<Self, ()> {
        Ok(Self {
            tick_id: buf.read_u32()?,
            tick_time: buf.read_u32()?,
            server_current_time: buf.read_u32()?,
            server_prev_time: buf.read_u16()?,
            status: buf.rem_to_vec()
        })
    }
}
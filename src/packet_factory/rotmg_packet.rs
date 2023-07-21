#![allow(dead_code)]
use crate::packet_factory::byte_buffer::ByteBuffer;

#[repr(i16)]
#[derive(Debug, Clone)]
pub enum RotmgPacket {
    Failure {
        rem: ByteBuffer
    } = 0,
    Teleport {
        rem: ByteBuffer
    } = 1,
    //Missing
    ClaimLoginReward {
        rem: ByteBuffer
    } = 3,
    DeletePet {
        rem: ByteBuffer
    } = 4,
    RequestTrade {
        rem: ByteBuffer
    } = 5,
    QuestFetchResponse {
        rem: ByteBuffer
    } = 6,
    JoinGuild {
        rem: ByteBuffer
    } = 7,
    Ping {
        rem: ByteBuffer
    } = 8,
    PlayerText {
        rem: ByteBuffer
    } = 9,
    NewTick {
        tick_id: u32,
        tick_time: u32,
        server_current_time: u32,
        server_prev_time: u16,
        rem: ByteBuffer
    } = 10,
    ShowEffect {
        rem: ByteBuffer
    } = 11,
    ServerPlayerShoot {
        rem: ByteBuffer
    } = 12,
    UseItem {
        rem: ByteBuffer
    } = 13,
    TradeAccepted {
        rem: ByteBuffer
    } = 14,
    GuildRemove {
        rem: ByteBuffer
    } = 15,
    PetUpgradeRequest {
        rem: ByteBuffer
    } = 16,
    EnterArena {
        rem: ByteBuffer
    } = 17,
    GoTo {
        rem: ByteBuffer
    } = 18,
    InventoryDrop {
        rem: ByteBuffer
    } = 19,
    OtherHit {
        rem: ByteBuffer
    } = 20,
    NameResult {
        rem: ByteBuffer
    } = 21,
    BuyResult {
        rem: ByteBuffer
    } = 22,
    HatchPet {
        rem: ByteBuffer
    } = 23,
    ActivePetUpdateRequest {
        rem: ByteBuffer
    } = 24,
    EnemyHit {
        rem: ByteBuffer
    } = 25,
    GuildResult {
        rem: ByteBuffer
    } = 26,
    EditAccountList {
        rem: ByteBuffer
    } = 27,
    TradeChanged {
        rem: ByteBuffer
    } = 28,
    //missing
    PlayerShoot {
        rem: ByteBuffer
    } = 30,
    Pong {
        rem: ByteBuffer
    } = 31,
    //missing
    PetChangeSkinMessage {
        rem: ByteBuffer
    } = 33,
    TradeDone {
        rem: ByteBuffer
    } = 34,
    EnemyShoot {
        rem: ByteBuffer
    } = 35,
    AcceptTrade {
        rem: ByteBuffer
    } = 36,
    ChangeGuildRank {
        rem: ByteBuffer
    } = 37,
    PlaySound {
        rem: ByteBuffer
    } = 38,
    VerifyEmail {
        rem: ByteBuffer
    } = 39,
    SquareHit {
        rem: ByteBuffer
    } = 40,
    NewAbility {
        rem: ByteBuffer
    } = 41,
    Update {
        rem: ByteBuffer
    } = 42,
    //missing
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
    } = 44,
    Reconnect {
        name: String,
        host: String,
        unknown: u32,
        port: u32,
        game_id: u32,
        key: Vec<u8>,
    } = 45,
    Death {
        rem: ByteBuffer
    } = 46,
    UsePortal {
        rem: ByteBuffer
    } = 47,
    QuestRoomMessage {
        rem: ByteBuffer
    } = 48,
    AllyShoot {
        rem: ByteBuffer
    } = 49,
    ImminentArenaWave {
        rem: ByteBuffer
    } = 50,
    Reskin {
        rem: ByteBuffer
    } = 51,
    ResetDailyQuests {
        rem: ByteBuffer
    } = 52,
    PetChangeFormMsg {
        rem: ByteBuffer
    } = 53,
    //missing
    InvResult {
        rem: ByteBuffer
    } = 55,
    ChangeTrade {
        rem: ByteBuffer
    } = 56,
    Create {
        rem: ByteBuffer
    } = 57,
    QuestRedeem {
        rem: ByteBuffer
    } = 58,
    CreateGuild {
        rem: ByteBuffer
    } = 59,
    SetCondition {
        rem: ByteBuffer
    } = 60,
    Load {
        rem: ByteBuffer
    } = 61,
    Move {
        tick_id: u32,
        time: u32,
        rem: ByteBuffer
    } = 62,
    KeyInfoResponse {
        rem: ByteBuffer
    } = 63,
    AOE {
        rem: ByteBuffer
    } = 64,
    GoToAck {
        rem: ByteBuffer
    } = 65,
    GlobalNotification {
        rem: ByteBuffer
    } = 66,
    Notification {
        rem: ByteBuffer
    } = 67,
    ArenaDeath {
        rem: ByteBuffer
    } = 68,
    ClientStat {
        rem: ByteBuffer
    } = 69,
    //missing
    //missing
    //missing
    //missing
    Hello {
        rem: ByteBuffer
    } = 74,
    Damage {
        target_id: u32,
        effects: Vec<u8>,
        damage_amount: u16,
        killed: bool,
        armor_piercing: bool,
        bullet_id: u8,
        owner_id: u32
    } = 75,
    /**
     * ADD OTHER PACKET TYPES HERE
     */
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
    } = 92,
    /**
     * ADD OTHER PACKET TYPES HERE
     */
    Other {
        type_num: u8,
        rem: ByteBuffer
    } = 1000,
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
            12 => ServerPlayerShoot { rem: buf },
            13 => UseItem { rem: buf },
            14 => TradeAccepted { rem: buf },
            15 => GuildRemove { rem: buf },
            16 => PetUpgradeRequest { rem: buf },
            17 => EnterArena { rem: buf },
            18 => GoTo { rem: buf },
            19 => InventoryDrop { rem: buf },
            20 => OtherHit { rem: buf },
            21 => NameResult { rem: buf },
            22 => HatchPet { rem: buf },
            23 => HatchPet { rem: buf },
            24 => ActivePetUpdateRequest { rem: buf },
            25 => EnemyHit { rem: buf },
            26 => GuildResult { rem: buf },
            27 => EditAccountList { rem: buf },
            28 => TradeChanged { rem: buf },
            30 => PlayerShoot { rem: buf },
            31 => Pong { rem: buf },
            33 => PetChangeSkinMessage { rem: buf },
            34 => TradeDone { rem: buf },
            35 => EnemyShoot { rem: buf },
            36 => AcceptTrade { rem: buf },
            37 => ChangeGuildRank { rem: buf },
            38 => PlaySound { rem: buf },
            39 => VerifyEmail { rem: buf },
            40 => SquareHit { rem: buf },
            41 => NewAbility { rem: buf },
            42 => Update { rem: buf },
            44 => Text { name: buf.read_string()?, object_id: buf.read_u32()?, num_stars: buf.read_u16()?, display_time: buf.read_u8()?, recipient: buf.read_string()?, content: buf.read_string()?, clean_text: buf.read_string()?, is_supporter: buf.read_bool()?, star_background: buf.read_u32()? },
            45 => Reconnect { name: buf.read_string()?, host: buf.read_string()?, unknown: buf.read_u32()?, port: buf.read_u32()?, game_id: buf.read_u32()?, key: buf.rem_to_vec() },
            46 => Death { rem: buf },
            47 => UsePortal { rem: buf },
            48 => QuestRoomMessage { rem: buf },
            49 => AllyShoot { rem: buf },
            50 => ImminentArenaWave { rem: buf },
            51 => Reskin { rem: buf },
            52 => ResetDailyQuests { rem: buf },
            53 => PetChangeFormMsg { rem: buf },
            55 => InvResult { rem: buf },
            56 => ChangeTrade { rem: buf },
            57 => Create { rem: buf },
            58 => QuestRedeem { rem: buf },
            59 => CreateGuild { rem: buf },
            60 => SetCondition { rem: buf },
            61 => Load { rem: buf },
            62 => Move { tick_id: buf.read_u32()?, time: buf.read_u32()?, rem: buf },
            63 => KeyInfoResponse { rem: buf },
            64 => AOE { rem: buf },
            65 => GoToAck { rem: buf },
            66 => GlobalNotification { rem: buf },
            67 => Notification { rem: buf },
            68 => ArenaDeath { rem: buf },
            69 => ClientStat { rem: buf },
            74 => Hello { rem: buf },
            75 => {
                let target_id = buf.read_u32()?; let effect_len = buf.read_u8()?; let effects = buf.read_n_bytes(effect_len as usize)?.to_vec(); 
                Damage { target_id, effects, damage_amount: buf.read_u16()?, killed: buf.read_bool()?, armor_piercing: buf.read_bool()?, bullet_id: buf.read_u8()?, owner_id: buf.read_u32()? }
            },
            //more
            92 => MapInfo { width: buf.read_u32()?, height: buf.read_u32()?, name: buf.read_string()?, display_name: buf.read_string()?, realm_name: buf.read_string()?, difficulty: buf.read_f32()?, seed: buf.read_u32()?, background: buf.read_u32()?, allow_teleport: buf.read_bool()?, show_displays: buf.read_bool()?, unknown_bool: buf.read_bool()?, max_players: buf.read_u16()?, game_opened_time: buf.read_u32()?, build_version: buf.read_string()?, unknown_int: buf.read_u32()?, dungeon_mods: buf.read_string()? },
            //more
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
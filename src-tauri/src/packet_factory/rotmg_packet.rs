#![allow(dead_code)]
use crate::packet_factory::byte_buffer::ByteBuffer;
use super::{data_types::*, rotmg_packet_stitcher::StitchedPacket};

#[repr(u16)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
        //position: PositionData,
        //level: u8, //unknown
        //ground_tile_data: Vec<GroundTileData>,
        //object_data: Vec<ObjectData>,
        //drops: Vec<i32>,
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
    Aoe {
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
    ActivePetUpdate {
        rem: ByteBuffer
    } = 76,
    InvitedToGuild {
        rem: ByteBuffer
    } = 77,
    PetYardUpdate {
        rem: ByteBuffer
    } = 78,
    PasswordPrompt {
        rem: ByteBuffer
    } = 79,
    AcceptArenaDeath {
        rem: ByteBuffer
    } = 80,
    UpdateAck {
        rem: ByteBuffer
    } = 81,
    QuestObjectId {
        rem: ByteBuffer
    } = 82,
    Pic {
        rem: ByteBuffer
    } = 83,
    RealmHeroLeftMsg {
        rem: ByteBuffer
    } = 84,
    Buy {
        rem: ByteBuffer
    } = 85,
    TradeStart {
        rem: ByteBuffer
    } = 86,
    EvolvePet {
        rem: ByteBuffer
    } = 87,
    TradeRequested {
        rem: ByteBuffer
    } = 88,
    AoeAck {
        rem: ByteBuffer
    } = 89,
    PlayerHit {
        rem: ByteBuffer
    } = 90,
    CancelTrade {
        rem: ByteBuffer
    } = 91,
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
    LoginRewardMsg {
        rem: ByteBuffer
    } = 93,
    KeyInfoRequest {
        rem: ByteBuffer
    } = 94,
    InvSwap {
        rem: ByteBuffer
    } = 95,
    QuestRedeemResponse {
        rem: ByteBuffer
    } = 96,
    ChooseName {
        rem: ByteBuffer
    } = 97,
    QuestFetchAsk {
        rem: ByteBuffer
    } = 98,
    AccountList {
        rem: ByteBuffer
    } = 99,
    ShootAck {
        rem: ByteBuffer
    } = 100,
    CreateSuccess {
        rem: ByteBuffer
    } = 101,
    CheckCredits {
        rem: ByteBuffer
    } = 102,
    GroundDamage {
        rem: ByteBuffer
    } = 103,
    GuildInvite {
        rem: ByteBuffer
    } = 104,
    Escape {
        rem: ByteBuffer
    } = 105,
    File {
        rem: ByteBuffer
    } = 106,
    ReskinUnlock {
        rem: ByteBuffer
    } = 107,
    NewCharacterInfo {
        rem: ByteBuffer
    } = 108,
    UnlockInfo {
        rem: ByteBuffer
    } = 109,
    //missing
    //missing
    QueueInfo {
        rem: ByteBuffer
    } = 112,
    QueueCancel {
        rem: ByteBuffer
    } = 113,
    ExaltationBonusChanged {
        rem: ByteBuffer
    } = 114,
    RedeemExaltationReward {
        rem: ByteBuffer
    } = 115,
    //missing
    VaultUpdate {
        rem: ByteBuffer
    } = 117,
    ForgeRequest {
        rem: ByteBuffer
    } = 118,
    ForgeResult {
        rem: ByteBuffer
    } = 119,
    ForgeUnlockedBlueprints {
        rem: ByteBuffer
    } = 120,
    ShootAckCounter {
        rem: ByteBuffer
    } = 121,
    ChangeAllyShoot {
        rem: ByteBuffer
    } = 122,
    GetPlayersListMessage {
        rem: ByteBuffer
    } = 123,
    ModeratorActionMessage {
        rem: ByteBuffer
    } = 124,
    //missing
    CreepMoveMessage {
        rem: ByteBuffer
    } = 126,
    //missing
    //missing
    //missing
    //missing
    //missing
    //missing
    //missing
    Unknown134  {
        rem: ByteBuffer
    } = 134,
    //missing
    //missing
    Dash {
        rem: ByteBuffer
    } = 137,
    DashAck {
        rem: ByteBuffer
    } = 138,
    Unknown139 {
        rem: ByteBuffer
    } = 139,
    //missing
    //missing
    //missing
    //missing
    //missing
    Unknown145 {
        rem: ByteBuffer
    } = 145,
    Unknown146 {
        rem: ByteBuffer
    } = 146,
    Unknown147 {
        rem: ByteBuffer
    } = 147,
    //missing
    ClaimBattlePass {
        rem: ByteBuffer
    } = 149,
    ClaimBPMilestoneResult {
        rem: ByteBuffer
    } = 150,
    //missing
    //missing
    //missing
    ConvertSeasonal {
        rem: ByteBuffer
    } = 154,
    //missing
    //missing
    //missing
    //missing
    Emote {
        rem: ByteBuffer
    } = 159,
    //missing
    //missing
    //missing
    Unknown163 {
        rem: ByteBuffer
    } = 163,
    Unknown164 {
        rem: ByteBuffer
    } = 164,
    Unknown165 {
        rem: ByteBuffer
    } = 165,
    Stasis {
        rem: ByteBuffer
    } = 166,
    //missing
    //missing
    Unknown169 {
        rem: ByteBuffer
    } = 169,

    Other { //catch-all for any packets whose type number is not listed here
        type_num: u8,
        rem: ByteBuffer
    } = 1000,
}
impl TryFrom<StitchedPacket> for RotmgPacket {
    type Error = ();

    fn try_from(mut sp: StitchedPacket) -> Result<Self, ()> {
        use RotmgPacket::*;
        let mut buf = sp.buffer;
        return Ok(match sp.type_num {
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
            42 => Update { rem: buf },//{ position: PositionData::deserialize_from_buf(&mut buf)?, level: buf.read_u8()?, ground_tile_data: GroundTileData::deserialize_arr_from_buf(&mut buf)?, object_data: ObjectData::deserialize_arr_from_buf(&mut buf)?, drops: buf.read_compressed_i32_arr()?, rem: buf },
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
            64 => Aoe { rem: buf },
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
            76 => ActivePetUpdate { rem: buf },
            77 => InvitedToGuild { rem: buf },
            78 => PetYardUpdate { rem: buf },
            79 => PasswordPrompt { rem: buf },
            80 => AcceptArenaDeath { rem: buf },
            81 => UpdateAck { rem: buf },
            82 => QuestObjectId { rem: buf },
            83 => Pic { rem: buf },
            84 => RealmHeroLeftMsg { rem: buf },
            85 => Buy { rem: buf },
            86 => TradeStart { rem: buf },
            87 => EvolvePet { rem: buf },
            88 => TradeRequested { rem: buf },
            89 => AoeAck { rem: buf },
            90 => PlayerHit { rem: buf },
            91 => CancelTrade { rem: buf },
            92 => MapInfo { width: buf.read_u32()?, height: buf.read_u32()?, name: buf.read_string()?, display_name: buf.read_string()?, realm_name: buf.read_string()?, difficulty: buf.read_f32()?, seed: buf.read_u32()?, background: buf.read_u32()?, allow_teleport: buf.read_bool()?, show_displays: buf.read_bool()?, unknown_bool: buf.read_bool()?, max_players: buf.read_u16()?, game_opened_time: buf.read_u32()?, build_version: buf.read_string()?, unknown_int: buf.read_u32()?, dungeon_mods: buf.read_string()? },
            93 => LoginRewardMsg { rem: buf },
            94 => KeyInfoRequest { rem: buf },
            95 => InvSwap { rem: buf },
            96 => QuestRedeemResponse { rem: buf },
            97 => ChooseName { rem: buf },
            98 => QuestFetchAsk { rem: buf },
            99 => AccountList { rem: buf },
            100 => ShootAck { rem: buf },
            101 => CreateSuccess { rem: buf },
            102 => CheckCredits { rem: buf },
            103 => GroundDamage { rem: buf },
            104 => GuildInvite { rem: buf },
            105 => Escape { rem: buf },
            106 => File { rem: buf },
            107 => ReskinUnlock { rem: buf },
            108 => NewCharacterInfo { rem: buf },
            109 => UnlockInfo { rem: buf },
            112 => QueueInfo { rem: buf },
            113 => QueueCancel { rem: buf },
            114 => ExaltationBonusChanged { rem: buf },
            115 => RedeemExaltationReward { rem: buf },
            117 => VaultUpdate { rem: buf },
            118 => ForgeRequest { rem: buf },
            119 => ForgeResult { rem: buf },
            120 => ForgeUnlockedBlueprints { rem: buf },
            121 => ShootAckCounter { rem: buf },
            122 => ChangeAllyShoot { rem: buf },
            123 => GetPlayersListMessage { rem: buf },
            124 => ModeratorActionMessage { rem: buf },
            126 => CreepMoveMessage { rem: buf },
            134 => Unknown134 { rem: buf },
            137 => Dash { rem: buf },
            138 => DashAck { rem: buf },
            139 => Unknown139 { rem: buf },
            145 => Unknown145 { rem: buf },
            146 => Unknown146 { rem: buf },
            147 => Unknown147 { rem: buf },
            149 => ClaimBattlePass { rem: buf },
            150 => ClaimBPMilestoneResult { rem: buf },
            154 => ConvertSeasonal { rem: buf },
            159 => Emote { rem: buf },
            163 => Unknown163 { rem: buf },
            164 => Unknown164 { rem: buf },
            165 => Unknown165 { rem: buf },
            166 => Stasis { rem: buf },
            169 => Unknown169 { rem: buf },

            _ => Other { type_num: sp.type_num, rem: buf},
        })
    }
}

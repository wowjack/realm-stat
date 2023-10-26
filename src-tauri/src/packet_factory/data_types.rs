#![allow(unused)]

use super::byte_buffer::ByteBuffer;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PositionData {
    pub x: f32,
    pub y: f32,
}
impl PositionData {
    pub fn deserialize_from_buf(buf: &mut ByteBuffer) -> Result<Self, ()> {
        Ok(Self { x: buf.read_f32()?, y: buf.read_f32()? })
    }
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GroundTileData {
    pub x: u16,
    pub y: u16,
    pub type_num: u16
}
impl GroundTileData {
    pub fn deserialize_from_buf(buf: &mut ByteBuffer) -> Result<Self, ()> {
        Ok(Self {
            x: buf.read_u16()?,
            y: buf.read_u16()?,
            type_num: buf.read_u16()?
        })
    }
    pub fn deserialize_arr_from_buf(buf: &mut ByteBuffer) -> Result<Vec<Self>, ()> {
        let len = buf.read_compressed_i32()?;
        Self::deserialize_n_from_buf(buf, len as usize)
    }
    pub fn deserialize_n_from_buf(buf: &mut ByteBuffer, n: usize) -> Result<Vec<Self>, ()> {
        let mut ret = vec![];
        for _ in 0..n {
            ret.push(Self::deserialize_from_buf(buf)?);
        }
        Ok(ret)
    }
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ObjectData {
    pub type_num: u16,
    pub status_data: ObjectStatusData,
}
impl ObjectData {
    pub fn deserialize_from_buf(buf: &mut ByteBuffer) -> Result<Self, ()> {
        Ok(Self {
            type_num: buf.read_u16()?,
            status_data: ObjectStatusData::deserialize_from_buf(buf)?
        })
    }
    pub fn deserialize_arr_from_buf(buf: &mut ByteBuffer) -> Result<Vec<Self>, ()> {
        let len = buf.read_compressed_i32()?;
        Self::deserialize_n_from_buf(buf, len as usize)
    }
    pub fn deserialize_n_from_buf(buf: &mut ByteBuffer, n: usize) -> Result<Vec<Self>, ()> {
        let mut ret = vec![];
        for _ in 0..n {
            ret.push(Self::deserialize_from_buf(buf)?);
        }
        Ok(ret)
    }
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ObjectStatusData {
    pub object_id: i32,
    pub position: PositionData,
    pub stats: Vec<StatData>
}
impl ObjectStatusData {
    pub fn deserialize_from_buf(buf: &mut ByteBuffer) -> Result<Self, ()> {
        Ok(Self {
            object_id: buf.read_compressed_i32()?,
            position: PositionData::deserialize_from_buf(buf)?,
            stats: StatData::deserialize_arr_from_buf(buf)?
        })
    }
    pub fn deserialize_arr_from_buf(buf: &mut ByteBuffer) -> Result<Vec<Self>, ()> {
        let len = buf.read_compressed_i32()?;
        Self::deserialize_n_from_buf(buf, len as usize)
    }
    pub fn deserialize_n_from_buf(buf: &mut ByteBuffer, n: usize) -> Result<Vec<Self>, ()> {
        let mut ret = vec![];
        for _ in 0..n {
            ret.push(Self::deserialize_from_buf(buf)?);
        }
        Ok(ret)
    }
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct StatData {
    pub stat_type: StatType,
    pub stat_value: StatValue,
    pub stat_value_two: i32
}
impl StatData {
    pub fn deserialize_from_buf(buf: &mut ByteBuffer) -> Result<Self, ()> {
        let stat_type: StatType = unsafe {std::mem::transmute(buf.read_u8()?)};
        Ok(Self {
            stat_type: stat_type,
            stat_value: StatValue::deserialize_from_buf(buf, stat_type)?,
            stat_value_two: buf.read_compressed_i32()?
        })
    }
    pub fn deserialize_arr_from_buf(buf: &mut ByteBuffer) -> Result<Vec<Self>, ()> {
        let len = buf.read_compressed_i32()?;
        Self::deserialize_n_from_buf(buf, len as usize)
    }
    pub fn deserialize_n_from_buf(buf: &mut ByteBuffer, n: usize) -> Result<Vec<Self>, ()> {
        let mut ret = vec![];
        for _ in 0..n {
            ret.push(Self::deserialize_from_buf(buf)?);
        }
        Ok(ret)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum StatValue {
    StringValue(String),
    IntValue(i32)
}
impl StatValue {
    pub fn deserialize_from_buf(buf: &mut ByteBuffer, stat_type: StatType) -> Result<Self, ()> {
        use StatType::*;
        match stat_type {
            Exp | Name | AccountId | OwnerAccountId | GuildName | Texture | PetName | GraveAccountId | Unknown121 | Enchantment => {
                return Ok(StatValue::StringValue(buf.read_string()?))
            },
            _ => {
                return Ok(StatValue::IntValue(buf.read_compressed_i32()?))
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
#[repr(u8)]
pub enum StatType {
    MaxHp = 0,
    Hp = 1,
    Size = 2,
    MaxMp = 3,
    Mp = 4,
    NextLevelExp = 5,
    Exp = 6,
    Level = 7,
    Inv0 = 8,
    Inv1 = 9,
    Inv2 = 10,
    Inv3 = 11,
    Inv4 = 12,
    Inv5 = 13,
    Inv6 = 14,
    Inv7 = 15,
    Inv8 = 16,
    Inv9 = 17,
    Inv10 = 18,
    Inv11 = 19,
    Attack = 20,
    Defense = 21,
    Speed = 22,
    Unknown23 = 23,
    Seasonal = 24,
    SkinId = 25,
    Vitality = 26,
    Wisdom = 27,
    Dexterity = 28,
    Condition = 29,
    NumStars = 30,
    Name = 31,
    Tex1 = 32,
    Tex2 = 33,
    MerchType = 34,
    Credits = 35,
    MerchandisePrice = 36,
    Active = 37,
    AccountId = 38,
    Fame = 39,
    MerchCurrency = 40,
    Connect = 41,
    MerchCount = 42,
    MerchMinsLeft = 43,
    MerchDiscount = 44,
    MerchRankReq = 45,
    MaxHpBoost = 46,
    MaxMpBoost = 47,
    AttackBoost = 48,
    DefenseBoost = 49,
    SpeedBoost = 50,
    VitalityBoost = 51,
    WisdomBoost = 52,
    DesterityBoost = 53,
    OwnerAccountId = 54,
    RankRequired = 55,
    NameChosen = 56,
    CurrentFame = 57,
    NextClassQuestFame = 58,
    LegendaryRank = 59,
    SinkLevel = 60,
    AltTexture = 61,
    GuildName = 62,
    GuildRankStat = 63,
    Breath = 64,
    XpBoosted = 65,
    XpTimer = 66,
    LdTimer = 67,
    LtTimer = 68,
    HealthPotionStack = 69,
    MagicPotionStack = 70,
    Backpack0 = 71,
    Backpack1 = 72,
    Backpack2 = 73,
    Backpack3 = 74,
    Backpack4 = 75,
    Backpack5 = 76,
    Backpack6 = 77,
    Backpack7 = 78,
    HasBackpack = 79,
    Texture = 80,
    PetInstanceId = 81,
    PetName = 82,
    PetType = 83,
    PetRarity = 84,
    PetMaxAbilityPower = 85,
    PetFamily = 86,
    PetFirstAbilityPoint = 87,
    PetSecondAbilityPoint = 88,
    PetThirdAbilityPoint = 89,
    PetFirstAbilityPower = 90,
    PetSecondAbilityPower = 91,
    PetThirdAbilityPower = 92,
    PetFirstAbilityType = 93,
    PetSecondAbilityType = 94,
    PetThirdAbilityType = 95,
    NewCon = 96,
    FortuneToken = 97,
    SupporterPoints = 98,
    Supporter = 99,
    ChallengerStarBG = 100,
    PlayerId = 101,
    ProjectileSpeedMul = 102,
    ProjectileLifeMul = 103,
    OwnedAtTimestamp = 104,
    ExaltedAttack = 105,
    ExaltedDefense = 106,
    ExaltedSpeed = 107,
    ExaltedVitality = 108,
    ExaltedWisdom = 109,
    ExaltedDexterity = 110,
    ExaltedHp = 111,
    ExaltedMp = 112,
    ExaltationBonusDamage = 113,
    ExaltationIcReduction = 114,
    GraveAccountId = 115,
    PotionOneType = 116,
    PotionTwoType = 117,
    PotionThreeType = 118,
    PotionBelt = 119,
    ForgeFire = 120,
    Unknown121 = 121,
    Unknown122 = 122,
    Unknown123 = 123,
    Unknown124 = 124,
    AnimationId = 125,
    Unknown126 = 126,
    Enchantment = 127,
}
use serde::{
    Deserialize,
    de,
};
use bitflags::bitflags;


#[derive(Debug, Deserialize, Clone)]
pub struct User {
    username: String,
    #[serde(rename = "premium_type")]
    nitro_type: Nitro,
    email: Option<String>,
    phone: Option<String>,
    mfa_enabled: bool,
    #[serde(rename = "public_flags")]
    flags: UserFlags,
}

// source: https://github.com/twilight-rs/twilight/blob/main/twilight-model/src/user/flags.rs
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct UserFlags: u64 {
        /// Discord Employee.
        const STAFF = 1;
        /// Partnered server owner.
        const PARTNER = 1 << 1;
        /// HypeSquad events member.
        const HYPESQUAD = 1 << 2;
        /// Bug hunter level 1.
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        /// House Bravery member.
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        /// House Brilliance member.
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        /// House Balance member.
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        /// Early Nitro supporter.
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        /// User is in a team.
        const TEAM_PSEUDO_USER = 1 << 10;
        /// Bug hunter level 2.
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        /// Verified bot.
        const VERIFIED_BOT = 1 << 16;
        /// Early verified bot developer.
        const VERIFIED_DEVELOPER = 1 << 17;
        /// Moderator Programs Alumni
        const CERTIFIED_MODERATOR = 1 << 18;
        /// Moderator Programs Alumni
        const MODERATOR_PROGRAMS_ALUMNI = 1 << 18;
        /// Bot uses only HTTP interactions and is shown in the online member
        /// list.
        const BOT_HTTP_INTERACTIONS = 1 << 19;
        /// User is an [Active Developer].
        ///
        /// [Active Developer]: https://support-dev.discord.com/hc/articles/10113997751447
        const ACTIVE_DEVELOPER = 1 << 22;
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(from = "u8")]
pub enum Nitro {
    None,
    NitroClassic,
    Nitro,
    NitroBasic,
    Unknown(u8),
}

impl<'de> de::Deserialize<'de> for UserFlags {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl From<u8> for Nitro {
    fn from(value: u8) -> Self {
        match value {
            0 => Nitro::None,
            1 => Nitro::NitroClassic,
            2 => Nitro::Nitro,
            3 => Nitro::NitroBasic,
            other => Nitro::Unknown(other),
        }
    }
}
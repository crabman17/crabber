use serde::{
    Deserialize,
    de::{Deserializer, Error as DeError, Visitor}
};
use bitflags::bitflags;
use std::fmt::{Formatter, Result as FmtResult};


#[derive(Debug, Deserialize, Clone)]
pub struct Guild {
    name: String,
    permissions: Permissions,
    #[serde(rename = "approximate_member_count")]
    member_count: u64,
    owner: bool, 
}

impl Guild {
    #[inline]
    pub fn is_admin(&self) -> bool {
        self.permissions.contains(Permissions::ADMINISTRATOR) 
    }

    #[inline(always)] 
    pub fn is_owner(&self) -> bool {
        self.owner
    }
}

// soruce: https://github.com/twilight-rs/twilight/blob/main/twilight-model/src/guild/permissions.rs
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Permissions: u64 {
        const CREATE_INVITE = 1;
        const KICK_MEMBERS = 1 << 1;
        const BAN_MEMBERS = 1 << 2;
        const ADMINISTRATOR = 1 << 3;
        const MANAGE_CHANNELS = 1 << 4;
        const MANAGE_GUILD = 1 << 5;
        const ADD_REACTIONS = 1 << 6;
        const VIEW_AUDIT_LOG = 1 << 7;
        const PRIORITY_SPEAKER = 1 << 8;
        const STREAM = 1 << 9;
        const VIEW_CHANNEL = 1 << 10;
        /// Allows for sending messages and creating forum threads, but not
        /// sending messages in forum threads.
        const SEND_MESSAGES = 1 << 11;
        const SEND_TTS_MESSAGES = 1 << 12;
        const MANAGE_MESSAGES = 1 << 13;
        const EMBED_LINKS = 1 << 14;
        const ATTACH_FILES = 1 << 15;
        const READ_MESSAGE_HISTORY = 1 << 16;
        const MENTION_EVERYONE = 1 << 17;
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        const CONNECT = 1 << 20;
        const SPEAK = 1 << 21;
        const MUTE_MEMBERS = 1 << 22;
        const DEAFEN_MEMBERS = 1 << 23;
        const MOVE_MEMBERS = 1 << 24;
        const USE_VAD = 1 << 25;
        const CHANGE_NICKNAME = 1 << 26;
        const MANAGE_NICKNAMES = 1 << 27;
        const MANAGE_ROLES = 1 << 28;
        const MANAGE_WEBHOOKS = 1 << 29;
        #[deprecated(since = "0.15.2", note = "use `MANAGE_GUILD_EXPRESSIONS` instead")]
        const MANAGE_EMOJIS_AND_STICKERS = 1 << 30;
        /// Allows management and editing of emojis, stickers, and soundboard sounds.
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;
        const USE_SLASH_COMMANDS = 1 << 31;
        const REQUEST_TO_SPEAK = 1 << 32;
        /// Allows for creating, editing, and deleting scheduled events.
        const MANAGE_EVENTS = 1 << 33;
        /// Allows for deleting and archiving threads, and viewing all private threads.
        const MANAGE_THREADS = 1 << 34;
        /// Allows for creating public threads.
        const CREATE_PUBLIC_THREADS = 1 << 35;
        /// Allows for creating private threads.
        const CREATE_PRIVATE_THREADS = 1 << 36;
        /// Allows the usage of custom stickers from other servers.
        const USE_EXTERNAL_STICKERS = 1 << 37;
        /// Allows for sending messages in threads.
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        /// Allows for using activities (applications with the `EMBEDDED`
        /// flag) in a voice channel.
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        /// Allows for timing out users to prevent them from sending or reacting
        /// to messages in chat and threads, and from speaking in voice and
        /// stage channels.
        ///
        /// See Discord's article on [Guild Timeouts].
        ///
        /// [Guild Timeouts]: https://support.discord.com/hc/en-us/articles/4413305239191-Time-Out-FAQ
        const MODERATE_MEMBERS = 1 << 40;
        /// Allows for viewing role subscription insights.
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        /// Allows for using soundboard in a voice channel
        const USE_SOUNDBOARD = 1 << 42;
        /// Allows the usage of custom soundboard sounds from other servers
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        /// Allows sending voice messages
        const SEND_VOICE_MESSAGES = 1 << 46;
    }
}

struct PermissionsVisitor;

impl<'de> Visitor<'de> for PermissionsVisitor {
    type Value = Permissions;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("integer or string permissions")
    }

    fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Permissions::from_bits_truncate(v))
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        #[allow(clippy::map_err_ignore)]
        let num = v
            .parse()
            .map_err(|_| DeError::custom("permissions is not valid bitflags"))?;

        self.visit_u64(num)
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(PermissionsVisitor)
    }
}

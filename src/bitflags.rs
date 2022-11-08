use bitflags::bitflags;

use serde::{Deserialize, Serialize, Serializer};

bitflags! {
    pub struct CosmeticFlags: u32 {
        const DEVELOPER     = 0b00000001;
        const STAFF         = 0b00000010;
        const CONTRIBUTOR   = 0b00000100;
        const EARLY_USER    = 0b00001000;
        const BETA_TESTER   = 0b00010000;
        const SUPPORTER     = 0b00100000;
        const CLAIMED_ONE   = 0b01000000;
        const CLAIMED_TWOO  = 0b10000000;
    }
}

impl Default for CosmeticFlags {
    fn default() -> Self {
        Self::empty()
    }
}

/* source - https://github.com/novacrazy/serde_shims/blob/master/bitflags/src/lib.rs */
impl Serialize for CosmeticFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CosmeticFlags {
    fn deserialize<D>(deserializer: D) -> Result<CosmeticFlags, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <_ as Deserialize<'de>>::deserialize(deserializer)?;

        CosmeticFlags::from_bits(value).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid bits {:#X} for {}", value, stringify!(CosmeticFlags)))
        })
    }
}

#[allow(unused_imports)]
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, IntoStaticStr};

#[rustfmt::skip]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq,
    Deserialize, Serialize,
    EnumString, IntoStaticStr,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    Spot,
    Margin,
    Leveraged,

    #[serde(rename = "TRD_GRP_002")]
    #[strum(serialize = "TRD_GRP_002")]
    TrdGrp002,

    #[serde(rename = "TRD_GRP_003")]
    #[strum(serialize = "TRD_GRP_003")]
    TrdGrp003,

    #[serde(rename = "TRD_GRP_004")]
    #[strum(serialize = "TRD_GRP_004")]
    TrdGrp004,

    #[serde(rename = "TRD_GRP_005")]
    #[strum(serialize = "TRD_GRP_005")]
    TrdGrp005,

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_serializing)]
    Unknown,
}

#[rustfmt::skip]
impl Permission {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_permissions() {
        let permissions = vec![
            Permission::Spot,
            Permission::Margin,
            Permission::Leveraged,
            Permission::TrdGrp002,
            Permission::TrdGrp003,
            Permission::TrdGrp004,
            Permission::TrdGrp005,
        ];

        for permission in permissions.into_iter() {
            let permission_static_str = permission.as_str();
            assert_eq!(
                permission,
                Permission::from_str(permission_static_str).unwrap()
            );
            assert_eq!(permission, permission_static_str.try_into().unwrap());

            let permission_owned_str = permission_static_str.to_string();
            assert_eq!(
                permission,
                Permission::from_str(&permission_owned_str).unwrap()
            );
        }
    }
}

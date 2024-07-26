use std::fmt::{Debug, Formatter};

use bedrockrs_core::Vec3;
use serde::{Deserialize, Serialize};

/// A version used in Addons that is either a Vector [a, b, c] or SemVer String.
#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AddonDynamicVersion {
    Vector(Vec3<u32>),
    SemVer(semver::Version),
}

impl Debug for AddonDynamicVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AddonDynamicVersion::Vector(v) => f.debug_list().entries([v.x, v.y, v.z]).finish(),
            AddonDynamicVersion::SemVer(v) => v.fmt(f),
        }
    }
}

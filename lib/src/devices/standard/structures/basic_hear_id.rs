use serde::{Deserialize, Serialize};

use super::StereoVolumeAdjustments;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicHearId {
    pub is_enabled: bool,
    pub volume_adjustments: StereoVolumeAdjustments,
    pub time: i32,
}

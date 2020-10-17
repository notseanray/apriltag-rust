//! Pose types storing estimated rotation and translation parameters.

use crate::{common::*, MatdRef};

/// Estimated pose along with error.
pub struct PoseEstimation {
    pub pose: Pose,
    pub error: f64,
}

/// Estimated pose rotation and translation parameters.
#[repr(transparent)]
pub struct Pose(pub(crate) sys::apriltag_pose_t);

impl Pose {
    /// Gets the rotation matrix.
    pub fn rotation(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.0.R) }
    }

    /// Gets the translation matrix.
    pub fn translation(&self) -> MatdRef<'_> {
        unsafe { MatdRef::from_ptr(self.0.t) }
    }
}

impl Debug for Pose {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Pose")
            .field("rotation", &self.rotation())
            .field("translation", &self.translation())
            .finish()
    }
}

impl Drop for Pose {
    fn drop(&mut self) {
        unsafe {
            sys::matd_destroy(self.0.R);
            sys::matd_destroy(self.0.t);
        }
    }
}

/// Stores tag size and camera parameters.
#[derive(Debug, Clone)]
pub struct TagParams {
    pub tagsize: f64,
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
}

#[cfg(feature = "nalgebra")]
mod nalgebra_conv {
    use super::*;
    use nalgebra::{IsometryMatrix3, MatrixSlice, Rotation3, Translation3, U1, U3};
    impl Pose {
        pub fn to_isometry(self: &Pose) -> IsometryMatrix3<f64> {
            let rotation = self.rotation();
            let translation = self.translation();

            let rotation = Rotation3::from_matrix(
                &MatrixSlice::from_slice_generic(rotation.data(), U3, U3)
                    .into_owned()
                    .transpose(),
            );

            let translation: Translation3<f64> =
                MatrixSlice::from_slice_generic(translation.data(), U3, U1)
                    .into_owned()
                    .into();

            IsometryMatrix3::from_parts(translation, rotation)
        }
    }
}

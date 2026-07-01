//! Control identifier types and bounds.

/// Marker used by `ControlsSchema` implementations without a control kind.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NoControl;

/// Common bounds for generated control identifiers.
pub trait ControlId: Clone + Copy + Default + Send + Sync + Unpin + 'static {}

impl<T> ControlId for T where T: Clone + Copy + Default + Send + Sync + Unpin + 'static {}

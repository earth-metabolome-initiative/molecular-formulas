//! Submodule providing typesetting markers for things like
//! baseline, superscripts and subscripts.

/// Marker trait for typesetting.
pub trait TypeSetting {}

/// Marker for superscript typesetting.
pub struct Superscript;

/// Marker for subscript typesetting.
pub struct Subscript;

/// Marker for baseline typesetting.
pub struct Baseline;

impl TypeSetting for Superscript {}

impl TypeSetting for Subscript {}

impl TypeSetting for Baseline {}

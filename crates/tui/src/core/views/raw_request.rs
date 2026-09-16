//! An arbitrary method and path, and whatever came back.

#[derive(Default)]
pub struct RawRequestView {
    pub method: tui_input::Input,
    pub path: tui_input::Input,
    pub body: Option<String>,
    pub focus: RawFocus,
    /// A response from an older generation is dropped rather than shown.
    pub generation: crate::core::Generation,
    pub response: Option<crate::core::collection::Delivered>,
    pub unavailable: Option<crate::core::failure::Failure>,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum RawFocus {
    #[default]
    Method,
    Path,
    Body,
    Response,
}

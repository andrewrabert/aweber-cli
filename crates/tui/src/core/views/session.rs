//! The session: what it is, and the PKCE login that replaces it.

pub struct SessionView {
    pub status: crate::ports::SessionStatus,
    pub authorization: Option<crate::ports::Authorization>,
    pub code: tui_input::Input,
    pub error: Option<String>,
}

impl SessionView {
    pub fn of(status: crate::ports::SessionStatus) -> SessionView {
        SessionView {
            status,
            authorization: None,
            code: tui_input::Input::default(),
            error: None,
        }
    }
}

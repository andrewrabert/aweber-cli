//! The confirmation, naming the exact HTTP effect.

use aweber::catalog::{Args, Operation};

pub struct ConfirmView {
    pub operation: Operation,
    pub args: Args,
    /// The exact HTTP effect, as `DELETE /1.0/accounts/1/lists/2/subscribers/3`.
    pub effect: String,
    pub tier: crate::catalog::ConfirmationTier,
    pub typed: tui_input::Input,
}

impl ConfirmView {
    pub fn satisfied(&self) -> bool {
        match self.tier {
            crate::catalog::ConfirmationTier::None => true,
            crate::catalog::ConfirmationTier::YesNo => true,
            crate::catalog::ConfirmationTier::Phrase(phrase) => self.typed.value() == phrase,
        }
    }
}

//! Everything the core asks the outside world to do.

#[derive(Clone, Debug)]
pub enum Effect {
    Send {
        request: crate::core::RequestId,
        generation: crate::core::Generation,
        plan: aweber::catalog::RequestPlan,
        purpose: Purpose,
    },
    StartWatch {
        watch: crate::core::WatchId,
        plan: aweber::catalog::RequestPlan,
        interval: std::time::Duration,
    },
    CancelWatch {
        watch: crate::core::WatchId,
    },
    Copy {
        text: String,
    },
    Write {
        path: std::path::PathBuf,
        text: String,
    },
    Edit {
        purpose: EditPurpose,
        seed: String,
        extension: &'static str,
    },
    OpenBrowser {
        url: String,
    },
    Login {
        code: String,
        verifier: crate::Secret,
    },
    Logout,
    ReadSession,
    Quit,
}

#[derive(Clone, Debug)]
pub enum Purpose {
    Launch,
    Collection {
        key: crate::core::cache::CacheKey,
    },
    Page {
        key: crate::core::cache::CacheKey,
    },
    Document {
        operation: aweber::catalog::Operation,
    },
    Mutation {
        operation: aweber::catalog::Operation,
        args: aweber::catalog::Args,
        /// The entity the mutation was issued against, read when it was issued.
        target: Option<crate::core::state::EntityKey>,
    },
    /// Best-effort: message subjects and visualization, never blocking a view.
    Enrichment {
        of: crate::core::state::EntityKey,
    },
    /// The re-read after a lost precondition race.
    Precondition {
        operation: aweber::catalog::Operation,
        args: aweber::catalog::Args,
    },
    /// The candidates of a builder row that names another entity.
    ListPicker {
        row: usize,
    },
    Raw,
}

#[derive(Clone, Debug)]
pub enum EditPurpose {
    Ruleset {
        workflow: uuid::Uuid,
        precondition: Option<i64>,
    },
    JsonPatch {
        workflow: uuid::Uuid,
        precondition: Option<i64>,
    },
    JsonBody {
        operation: aweber::catalog::Operation,
        args: aweber::catalog::Args,
    },
}

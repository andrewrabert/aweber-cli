//! Every operation of the API as data: its clap command, its arguments, and the
//! request it becomes.

mod args;
mod command_line;
mod commands;
mod groups;
mod operation;
mod plan;

pub use args::{ArgRole, ArgSpec, ArgValue, Args, ValueKind};
pub use command_line::{command_line, plan_command_line, raw_command_line};
pub use commands::{command_tree, group_command, raw_command};
pub use groups::{Group, groups, resolve};
pub use operation::Operation;
pub use plan::{CursorStyle, PRECONDITION_ARG, PlanBody, PlanError, RequestPlan, Scope};

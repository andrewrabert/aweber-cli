use aweber::ids::MessageId;

use crate::cli::Cli;
use crate::workflows::object;
use crate::workflows::Failure;

impl Cli {
    pub(crate) async fn execute_get_message(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let message_ids: Vec<MessageId> = matches
            .get_many::<MessageId>("message")
            .expect("message is required")
            .cloned()
            .collect();
        let messages = aweber::message::get_messages(&self.client, &message_ids)
            .await
            .map_err(Failure::api)?;
        let document: Vec<_> = messages.into_values().collect();
        object::print(&document)?;
        Ok(())
    }
}

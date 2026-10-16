use aweber::ids::{AttachmentId, BroadcastUid, MessageId, WorkflowId};
use aweber::message::{BoundResource, MessageChanges};

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

    fn single_message(matches: &clap::ArgMatches) -> MessageId {
        matches
            .get_one::<MessageId>("message")
            .expect("message is required")
            .clone()
    }

    pub(crate) async fn execute_update_message(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let message = Self::single_message(matches);
        let body_json = match matches.get_one::<std::path::PathBuf>("body-json") {
            Some(path) => {
                let text = std::fs::read_to_string(path).map_err(|error| {
                    anyhow::anyhow!("could not read {}: {error}", path.display())
                })?;
                match serde_json::from_str::<serde_json::Value>(&text)? {
                    serde_json::Value::Object(map) => Some(map),
                    _ => anyhow::bail!("{} does not hold a JSON object", path.display()),
                }
            }
            None => None,
        };
        let mut changes = MessageChanges {
            subject: matches.get_one::<String>("subject").cloned(),
            body_text: matches.get_one::<String>("body-text").cloned(),
            body_html: matches.get_one::<String>("body-html").cloned(),
            body_json,
            attachment_ids: None,
        };
        let added: Vec<AttachmentId> = matches
            .get_many::<AttachmentId>("add-attachment")
            .map(|values| values.cloned().collect())
            .unwrap_or_default();
        let removed: Vec<AttachmentId> = matches
            .get_many::<AttachmentId>("remove-attachment")
            .map(|values| values.cloned().collect())
            .unwrap_or_default();
        if !added.is_empty() || !removed.is_empty() {
            let current =
                aweber::message::get_messages(&self.client, std::slice::from_ref(&message))
                    .await
                    .map_err(Failure::api)?;
            let existing = current
                .get(&message)
                .map(|found| found.attachment_ids.clone())
                .unwrap_or_default();
            let mut merged: Vec<AttachmentId> = existing
                .iter()
                .map(|id| id.parse::<AttachmentId>())
                .collect::<Result<_, _>>()?;
            merged.retain(|id| !removed.contains(id));
            for id in added {
                if !merged.contains(&id) {
                    merged.push(id);
                }
            }
            changes.attachment_ids = Some(merged);
        }
        let updated = aweber::message::update_message(
            &self.client,
            self.account_id,
            list_id,
            &message,
            &changes,
        )
        .await
        .map_err(Failure::api)?;
        object::print(&updated)?;
        Ok(())
    }

    pub(crate) async fn execute_bind_message(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let message = Self::single_message(matches);
        let resource = if let Some(uid) = matches.get_one::<BroadcastUid>("broadcast") {
            BoundResource::Broadcast(*uid)
        } else {
            let id = matches
                .get_one::<WorkflowId>("campaign")
                .expect("broadcast or campaign is required");
            BoundResource::Campaign(*id)
        };
        let bound = aweber::message::bind_message(
            &self.client,
            self.account_id,
            list_id,
            &message,
            &resource,
        )
        .await
        .map_err(Failure::api)?;
        object::print(&bound)?;
        Ok(())
    }

    pub(crate) async fn execute_unbind_message(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let message = Self::single_message(matches);
        let unbound =
            aweber::message::unbind_message(&self.client, self.account_id, list_id, &message)
                .await
                .map_err(Failure::api)?;
        object::print(&unbound)?;
        Ok(())
    }

    pub(crate) async fn execute_delete_message(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let message = Self::single_message(matches);
        aweber::message::delete_message(&self.client, self.account_id, list_id, &message)
            .await
            .map_err(Failure::api)?;
        Ok(())
    }
}

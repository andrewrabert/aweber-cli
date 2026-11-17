use aweber::ids::{AttachmentId, BroadcastUid, MessageId, WorkflowId};
use aweber::message::{
    Binding, BoundResource, MessageChanges, MessageField, MessageSearch, NewMessage,
    LIST_MESSAGES_PAGE,
};

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

    fn body_json(
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<Option<serde_json::Map<String, serde_json::Value>>> {
        let Some(path) = matches.get_one::<std::path::PathBuf>("body-json") else {
            return Ok(None);
        };
        let text = std::fs::read_to_string(path)
            .map_err(|error| anyhow::anyhow!("could not read {}: {error}", path.display()))?;
        let document = serde_json::from_str(&text).map_err(|error| {
            anyhow::anyhow!("{} does not hold a JSON object: {error}", path.display())
        })?;
        Ok(Some(document))
    }

    pub(crate) async fn execute_list_messages(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let bindings: Vec<Binding> = matches
            .get_many::<Binding>("binding")
            .expect("binding has a default")
            .copied()
            .collect();
        let subject = matches.get_one::<String>("subject").map(String::as_str);
        let mut offset = matches.get_one::<usize>("offset").copied().unwrap_or(0);
        let limit = matches.get_one::<usize>("limit").copied();
        let mut messages = Vec::new();
        loop {
            let wanted = match limit {
                Some(limit) => (limit - messages.len()).min(LIST_MESSAGES_PAGE),
                None => LIST_MESSAGES_PAGE,
            };
            if wanted == 0 {
                break;
            }
            let page = aweber::message::list_messages(
                &self.client,
                self.account_id,
                list_id,
                &bindings,
                subject,
                offset,
                wanted,
            )
            .await
            .map_err(Failure::api)?;
            let received = page.len();
            offset += received;
            messages.extend(page);
            if received < wanted {
                break;
            }
        }
        object::print(&messages)?;
        Ok(())
    }

    pub(crate) async fn execute_find_messages(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let search = MessageSearch {
            binding: matches.get_one::<Binding>("binding").copied(),
            bound_resource_id: matches.get_one::<uuid::Uuid>("bound-resource-id").copied(),
            subject: matches.get_one::<String>("subject").cloned(),
        };
        let fields: Vec<MessageField> = matches
            .get_many::<MessageField>("fields")
            .map(|fields| fields.copied().collect())
            .unwrap_or_default();
        let account = self.account_uid().await?;
        let found = aweber::message::find_messages(&self.client, account, &search, &fields)
            .await
            .map_err(Failure::api)?;
        object::print(&found)?;
        Ok(())
    }

    pub(crate) async fn execute_create_message(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let text = |name: &str| matches.get_one::<String>(name).cloned().unwrap_or_default();
        let message = NewMessage {
            subject: text("subject"),
            body_text: text("body-text"),
            body_html: text("body-html"),
            body_json: Self::body_json(matches)?.unwrap_or_default(),
            attachment_ids: matches
                .get_many::<AttachmentId>("attachment")
                .map(|values| values.cloned().collect())
                .unwrap_or_default(),
        };
        let created =
            aweber::message::create_message(&self.client, self.account_id, list_id, &message)
                .await
                .map_err(Failure::api)?;
        object::print(&created)?;
        Ok(())
    }

    pub(crate) async fn execute_update_message(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let message = Self::single_message(matches);
        let body_json = Self::body_json(matches)?;
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
            let mut merged: Vec<AttachmentId> = current
                .get(&message)
                .map(|found| found.attachment_ids.clone())
                .unwrap_or_default();
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

use tracing::{error, warn};
use twilight_model::application::interaction::{InteractionData, InteractionType};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::channel::message::{Component, MessageFlags};
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle};
use twilight_model::gateway::event::Event;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::id::Id;
use twilight_model::id::marker::UserMarker;
use twilight_util::builder::InteractionResponseDataBuilder;
use crate::{Context, ensure_user_db_presence as ensure_user};

pub(crate) async fn handle_event(ctx: &Context, event: &Event) -> anyhow::Result<()> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!ping" => {
            ctx.client.create_message(msg.channel_id)
                .content("Pong!")?
                .await?;
        }
        Event::InteractionCreate(interaction) => {
            match interaction.kind {
                // This is a command
                InteractionType::ApplicationCommand => {
                    // if it doesn't have any data, we don't care
                    if interaction.data.is_none() { return Ok(()); }

                    // In case we can extract the command's data, we can identify the command
                    if let InteractionData::ApplicationCommand(command_data) = interaction.data.as_ref().unwrap() {
                        command_received(ctx, interaction, command_data).await?
                    }
                }
                _ => unimplemented!("Received an interaction that wasn't a command")
            }
        }
        _ => {}
    }

    Ok(())
}

async fn command_received(ctx: &Context, interaction: &InteractionCreate, command_data: &CommandData) -> anyhow::Result<()> {
    if interaction.author_id().is_none() {
        warn!(target: "CommandReceiver", "Command {} received with no user", command_data.name);
        return Ok(());
    }
    let user_id = interaction.author_id().unwrap();
    dbg!(&command_data.name);

    {
        let _ = ensure_user!(*ctx.database, &user_id);
    }

    // Check the user accepted the TOS
    if !verify_user_tos_acceptance(ctx, &user_id).await {
        let interface = ctx.client.interaction(ctx.application_id);
        let data = InteractionResponseDataBuilder::new()
            .content("You didn't accepted the TOS. See [url]")
            .components([
                Component::ActionRow(
                    ActionRow {
                        components: vec![
                            Component::Button(Button {
                                custom_id: Some("NATIVE_ACCEPT_TOS".to_string()),
                                disabled: false,
                                emoji: None,
                                label: Some("Accept".to_string()),
                                style: ButtonStyle::Primary,
                                url: None
                            })
                        ]
                    }
                )
            ])
            .build();


        interface.create_response(
            interaction.id,
            &interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(data),
            },
        ).await?;

        return Ok(());
    }

    let interface = ctx.client.interaction(ctx.application_id);

    let interaction_res_data = InteractionResponseDataBuilder::new()
        .content("Pong!")
        .flags(MessageFlags::empty())
        .build();

    dbg!(&interaction_res_data);

    let res = interface.create_response(
        interaction.id,
        &interaction.token,
        &InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(interaction_res_data),
        },
    ).await?;

    dbg!(&res.status());

    Ok(())
}

/// Call the 'ENSURE_USER' procedure in the database
///
/// This macro should be expected to be called in an asynchronous context.
/// The return value is
#[macro_export]
macro_rules! ensure_user_db_presence {
    ($db:expr, $user_id:expr) => {
        sqlx::query!("CALL ensure_user(?);", $user_id.to_string())
            .execute(&$db)
            .await
    };
}

/// Verify if the user has accepted the TOS
///
/// If not, it'll send a message and return false
async fn verify_user_tos_acceptance(ctx: &Context, user_id: &Id<UserMarker>) -> bool {
    let query = sqlx::query!("SELECT accepted_tos FROM users WHERE id=?;", user_id.to_string())
        .fetch_one(ctx.database.as_ref())
        .await;

    if let Err(e) = query {
        error!(target: "CommandReceiver", "Failed to fetch user from database: {}", e);
        return false;
    }

    query.unwrap().accepted_tos == 1
}
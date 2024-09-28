use twilight_model::application::interaction::{Interaction, InteractionData, InteractionType};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::event::Event;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;
use crate::{Context};

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
    dbg!(&command_data.name);

    // Check the user accepted the TOS
    if !verify_user_tos_acceptance(ctx, interaction).await { return Ok(()); }

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

/// Verify if the user has accepted the TOS
///
/// If not, it'll send a message and return false
async fn verify_user_tos_acceptance(ctx: &Context, interaction: &Interaction) -> bool {
    // TODO call database
    true
}
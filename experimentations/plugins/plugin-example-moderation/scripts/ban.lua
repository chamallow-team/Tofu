name                 = "ban"
interaction_type     = 1 -- SlashCommand
-- Not sure how to do the permissions system
-- If the permissions is equals to 0, it means that the dev know what he's doing
-- Otherwise, the permissions will be checked before-end
permissions_required = Permissions.Ban
guild_only           = true -- Not useful for community-plugins, but


description = "Ban a user"
-- Use a builder?
options = {
    {
        type = 1,
        required = false,
        name = "reason",
        description = "The reason of the ban",
        -- There are localizations for the options, it's pretty straight-forward
        localizations = {
            { lang = "fr", name = "raison", description = "La raison du ban" }
        }
    }
}
-- Add localizations for things such as the name and the description
localizations = {
    {
        lang = "fr",
        name = "ban",
        description = "Bannir"
    },
    {
        lang = "zh",
        name = "ban",
        description = "封禁用户"
    }
}


-- When this is an interaction, 
function interaction(ctx, interaction)
    local user = interaction.args.get(0) -- get the user
    
    if user == nil then
        -- Reply to the interaction by sending a message back
        -- The method `ctx.translate` takes the path to the traduction and arguments
        -- Starting the path to the traduction by `::` will tell the API to use the global translations
        interaction.reply({
            content = ctx.translate("::invalid_user", user.id)
        })
        return
    end
    
    local guild_member = interaction.guild.get_member(user.id)
    
    if guild_member == nil then
       interaction.reply({
           content = ctx.translate("::not_in_guild", user.id)
       })
       return
    end
    
    interaction.defer()
    local result = guild_member.ban()
    
    if result == 0 then
        interaction.update_reply({
            content = ctx.translate("ban::cannot_ban")
        })
    else
        interaction.update_reply({
            content = ctx.translate("ban::success")
        })
    end
end
# Plugins

*Je te conseille ce fichier avec la vue formatée*

J'ai pensé à séparer en plusieurs dossiers (je sais que tu n'aime pas ça, laisse moi t'expliquer)
```
root
├───lang
└───scripts
```

Il y a également des fichiers qui se baladent au milieu, voici un exemple simple de plugin complet: (que tu pourra directement retrouver dans le .zip)

```
root
├─── plugin.toml
├─── README-fr.md
├─── README-en-US.md
├─── lang
│    ├─── fr.json
│    └─── en-US.json
└─── scripts
     ├─── utils
     │    └─── plugin_utils.lua
     ├─── ban.lua
     └─── message_create.event.lua
```

On va découper ca en plusieurs parties et je vais expliquer mes choix

*Tu trouvera pleins de commentaires dans les fichiers dans `plugin-example-moderation`*

## Plugin.toml

<details>
    <summary>plugin.toml</summary>

```toml
version = "0.0.1"
author  = ["native/kady-team"]
type = "app"
another translation system
tags = ["moderation", "ai"]

# Contains the name, description and the localizations associated
[name]
default = "Automod"
description = "The auto-moderator plugin"

[name.localizations]
fr = { name = "Auto-Modération", description = "Le plugin de modération natif" }

[scripts]
ignored = ["utils/**/*"]

[dependencies]
kady-native = "latest"
jsp = "0.0.1"
```
</details>

En gros, il faut pleins d'infos pour le plugin, tel que sa version, son nom etc...

J'ai aussi pensé à ajouter un système de librairies, ce qui pourrait être très utile, nous permettant par exemple de mettre des dépendances.

On pourrait aussi créer un plugin natif avec pleins de petits trucs et petites fonctionnalités, qui pourraient être utilisées simplement en ajoutant la dépendance :)

L'avantage d'un système de librairies est aussi que ducoup, les gens pourront se dire "ok, j'ai de la logique répétitive entre les deux, je peux le mettre dans une librairie"

Les fichiers *ignorés* ne seront pas *ignorés* par le système, mais seront plutôt considérés comme des fichiers qui ne contiennent pas d'interactions ou d'events.

Et pour le reste, il n'y a pas grand chose à ajouter, on retrouve le nom, les traductions du nom + description etc... 

## Les `README-{lang}.md`

Alors ça je penses que tu vas aimer :)

Si pour les plugins natifs on n'a pas besoins de donner plein de détails, il faut quand même que les plugins de la communauté puissent se "vendre", alors ces fichiers sont là pour répondre à ce besoin.

Ces fichiers markdown pourront être utilisés pour fournir une description du plugin aux utilisateurs.

## Le dossier `lang`

Dans ce dossier, on aura toutes les traductions locales du plugin.

Malgré tout, il y a des trucs comme "pas les permissions requises" qui sont redondantes dans tout les plugins, alors je me suis dit: Pourquoi pas rajouter des traductions "globales".

Pour accéder à des traductions locales, on pourra faire `a::b::c`<br>
et pour les traductions globales, on fera juste `::a::b::c`

La structure des fichiers JSON est avantageuse pour le système de traductions, car c'est un arbre (facile à implémenter, et facile à parcourir), mais en plus c'est "nested", ce qui permet de structurer les langues de diverses manières

Pour les noms des langues, on utilisera le codage international des langues (fr, en-US, en-UK, zh etc...), ca sera juste chiant de basculer en `en-US` si il n'y a `en-UK`, mais je suis sûr qu'on trouvera comment faire.

## Le dossier `scripts`

Le plus fun!

### Events

Pour les events, on retrouvera par exemple:
```lua
function event(ctx, message)
    -- ...
end
```

L'avantage est qu'un plugin n'as besoin que d'un seul listener par event, donc on peut directement nommer les fichiers de cette manière:

`{nom_event}.event.lua`

C'est explicit pour nous et pour l'API


Au niveau du code, on n'as besoin que d'une fonction `event` avec les arguments, rien de sorcier de ce coté.

### Scripts

J'ai repensé à ce que tu m'avais proposé pour les commandes

Faire un seul fichier avec toutes les commandes est une mauvaise idée de mon point de vue, personnellement, quand le fichier commence à être lourd, je passe plus de temps à chercher où je veux aller que de coder/réfléchir, alors imagine avec des personnes qui ne sont pas des devs aguéris, ca va faire mal, même avec un éditeur créé par nos soins.

Au lieu de ca, je propose plutôt cette structure:
<details>
    <summary>ban.lua</summary>

```lua
name                 = "ban"
interaction_type     = 1 -- SlashCommand
-- ca je ne suis vraiment pas sûr
permissions_required = Permissions.Ban
guild_only           = true -- Not useful for community-plugins, but

description = "Ban a user"
options = {
    {
        type = 1,
        required = false,
        name = "reason",
        description = "The reason of the ban",
        localizations = {
            { lang = "fr", name = "raison", description = "La raison du ban" }
        }
    }
}
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


function interaction(ctx, interaction)
    local user = interaction.args.get(0) -- get the user
    
    if user == nil do
        interaction.reply({
            content = ctx.translate("::invalid_user", user.id)
        })
        return
    end
    
    local guild_member = interaction.guild.get_member(user.id)
    
    if guild_member == nil do
       interaction.reply({
           content = ctx.translate("::not_in_guild", user.id)
       })
       return
    end
    
    interaction.defer()
    local result = guild_member.ban()
    
    if result == 0 do
        interaction.update_reply({
            content = ctx.translate("ban::cannot_ban")
        })
    else do
        interaction.update_reply({
            content = ctx.translate("ban::success")
        })
    end
end
```
</details>

En gros on retrouve deux choses:
- Des variables pour les infos de la commande
- Une fonction `interaction(ctx, interaction)` quand l'interaction est utilisée

L'avantage de ma structure (de mon point de vue), c'est que c'est facile de comprendre et déboguer ca et que on se retrouve avec des fichiers raisonnables (j'y reviendrais sur la question du stockage.

L'API lua sera à réfléchir après, alors j'ai utilisé des méthodes très explicites et à peu près similaires avec *Discord.JS*

## Stockage des plugins

Comme tu l'avais si bien dit, Linux a un nombre limité de fichiers, alors, je propose que les plugins soient des .zip où on appliqué un algorithme de compression, mais pas que ca.

On pourra regrouper les plugins par paquets, encore une fois dans un .zip compressé

L'avantage sera qu'on aura aucun problème avec le nombre de fichiers car les fichiers seront "virtuels" (dans un sens)

Et personnellement je ne me fait pas de soucis si ca prend du temps, il ne faut pas oublier que quand ca sera juste lus au démarrage de Kady.

Les paquets de plugins pourront être "pleins" avec un nombre de plugins, une taille en mémoire etc..., on est totalement libres là-dessus.

Il faudra juste associer dans la base de donnée SQL une ID à chaque plugin et assigner ce plugin à un paquet.
use ::serenity::{
    all::{
        CacheHttp, Colour, CreateEmbed, CreateEmbedAuthor, CreateMessage, EventHandler, Http,
        Message, ReactionType,
    },
    async_trait,
    prelude::*,
};
use chrono::Datelike;
use poise::{serenity_prelude as serenity, CreateReply};
use rand::seq::SliceRandom;
use shuttle_runtime::SecretStore;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const ICON_LINK: &str = "https://i.imgur.com/mPXUvUZ.png";

const DEMOMAN_ICON: &str = "<:demoman:1333727378370723851>";
const SCOUT_ICON: &str = "<:scout:1333727211815174235>";
const SOLDIER_ICON: &str = "<:soldier:1333727354505265235>";
const MEDIC_ICON: &str = "<:medic:1333727312583331874>";

#[poise::command(slash_command)]
async fn confirm_event(
    ctx: Context<'_>,
    #[description = "Event type"] event_type: Option<String>,
    #[description = "Day"] day: Option<String>,
    #[description = "Time"] time: Option<String>,
    #[description = "Opponent"] opponent: Option<String>,
    #[description = "Maps"] maps: Option<String>,
    #[description = "Combo Scout"] combo_scout: Option<String>,
    #[description = "Flank Scout"] flank_scout: Option<String>,
    #[description = "Pocket Soldier"] pocket_soldier: Option<String>,
    #[description = "Roamer Soldier"] roamer_soldier: Option<String>,
    #[description = "Demoman"] demoman: Option<String>,
    #[description = "Medic"] medic: Option<String>,
) -> Result<(), Error> {
    let event_type = event_type.unwrap_or_else(|| "[Event type not specified]".to_string());
    let day = day.unwrap_or_else(|| "[Day not specified]".to_string());
    let time = time.unwrap_or_else(|| "[Time not specified]".to_string());
    let opponent = opponent.unwrap_or_else(|| "[Opponent not specified]".to_string());
    let maps = maps.unwrap_or_else(|| "[Maps not specified]".to_string());
    let combo_scout = combo_scout.unwrap_or("[Combo Scout not listed]".to_string());
    let flank_scout = flank_scout.unwrap_or("[Flank Scout not listed]".to_string());
    let pocket_soldier = pocket_soldier.unwrap_or("[Pocket Soldier not listed]".to_string());
    let roamer_soldier = roamer_soldier.unwrap_or("[Roamer Soldier not listed]".to_string());
    let demoman = demoman.unwrap_or("[Demoman not listed]".to_string());
    let medic = medic.unwrap_or("[Medic not listed]".to_string());

    ctx.send(
        CreateReply::default()
            .content("Confirming event")
            .ephemeral(true),
    )
    .await?;
    let embed = default_embed()
        .title(format!("**{event_type} Confirmation**"))
        .description(format!(
            ":calendar: **Day**: {day}\n
            :alarm_clock: **Time**: {time}\n
            :busts_in_silhouette: **Who**: {opponent}\n
            :map: **Maps**: {maps}\n
            :gun: **Roster**:
            > {SCOUT_ICON} Combo Scout: {combo_scout}
            > {SCOUT_ICON} Flank Scout: {flank_scout}
            > {SOLDIER_ICON} Pocket Soldier: {pocket_soldier}
            > {SOLDIER_ICON} Roamer Soldier: {roamer_soldier}
            > {DEMOMAN_ICON} Demoman: {demoman}
            > {MEDIC_ICON} Medic: {medic}\n
            *React with :white_check_mark: if you can make it*"
        ));
    let msg_handle = ctx
        .channel_id()
        .send_message(
            ctx.http(),
            CreateMessage::default()
                .embed(embed)
                .content("[ <@&1244624494731984906> ]"),
        )
        .await?;

    let reactions = ["✅", "❌"];

    add_reactions(&reactions, &msg_handle, ctx.http()).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn announce_event(
    ctx: Context<'_>,
    #[description = "Event type"] event_type: Option<String>,
    #[description = "Day"] day: Option<String>,
    #[description = "Opponent"] opponent: Option<String>,
) -> Result<(), Error> {
    let event_type = event_type.unwrap_or_else(|| "[Event type not specified]".to_string());
    let day = day.unwrap_or_else(|| "[Day not specified]".to_string());
    let opponent = opponent.unwrap_or_else(|| "[Opponent not specified]".to_string());
    ctx.send(
        CreateReply::default()
            .content("Announcing event")
            .ephemeral(true),
    )
    .await?;

    let embed = default_embed().title(format!("**{event_type} Announcment**"))
        .description(format!(":calendar: **Day**: {day}\n\n:busts_in_silhouette: **Who**: {opponent}\n\n*React below with time availability*"));
    let msg_handle = ctx
        .channel_id()
        .send_message(
            ctx.http(),
            CreateMessage::default()
                .embed(embed)
                .content("[ <@&1244624494731984906> ]"),
        )
        .await?;
    let reactions = ["6️⃣", "7️⃣", "8️⃣", "9️⃣", "🔟", "❌"];
    add_reactions(&reactions, &msg_handle, ctx.http()).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn poll_availability(ctx: Context<'_>) -> Result<(), Error> {
    let offset = chrono::offset::FixedOffset::east_opt(3600 * 10).unwrap();
    let timezone: chrono::DateTime<chrono::FixedOffset> =
        chrono::DateTime::from_naive_utc_and_offset(
            chrono::prelude::Utc::now().naive_utc(),
            offset,
        );
    let mut curr_day = timezone.weekday();
    let mut str_build = "".to_string();

    let icons = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "❌"];

    (0..7).for_each(|i| {
        str_build += &(icons[i].to_string() + " → " + &curr_day.to_string() + "\n");
        curr_day = curr_day.succ();
    });
    str_build += ":x: →  No Availability\n";

    ctx.send(
        CreateReply::default()
            .content("Creating availability poll")
            .ephemeral(true),
    )
    .await?;

    let embed = default_embed()
        .title("**Availability Poll**".to_string())
        .description(format!("*Polling availability for the next week.*\nReact with **all** of the days during which you are availabile **for at least an hour** at some point between 6pm – 10pm.\n\n{str_build}\n\n"));

    let msg_handle = ctx
        .channel_id()
        .send_message(
            ctx.http(),
            CreateMessage::default()
                .embed(embed)
                .content("[ <@&1244624494731984906> ]"),
        )
        .await?;

    add_reactions(&icons, &msg_handle, ctx.http()).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn request_merc(
    ctx: Context<'_>,
    #[description = "Day"] day: Option<String>,
    #[description = "Time"] time: Option<String>,
    #[description = "Role"] role: Option<String>,
    #[description = "Opponent"] opponent: Option<String>,
) -> Result<(), Error> {
    let day = day.unwrap_or_else(|| "[Day not specified]".to_string());
    let time = time.unwrap_or_else(|| "[Time not specified]".to_string());
    let role = role.unwrap_or_else(|| "[Role not specified]".to_string());
    let opponent = opponent.unwrap_or_else(|| "[Opponent not specified]".to_string());
    ctx.send(
        CreateReply::default()
            .content("Requesting merc")
            .ephemeral(true),
    )
    .await?;

    let embed = default_embed().title("**Mercenary Request**".to_string())
        .description(format!(":performing_arts: **Role**: {role}\n\n:calendar: **Day**: {day}\n\n:alarm_clock: **Time**: {time}\n\n:busts_in_silhouette: **Who**: {opponent}\n\n*React with :white_check_mark: if you can make it*"));
    let msg_handle = ctx
        .channel_id()
        .send_message(
            ctx.http(),
            CreateMessage::default()
                .embed(embed)
                .content("[ <@&1245310052294987846> ]"),
        )
        .await?;
    let reactions = ["✅", "❌"];
    add_reactions(&reactions, &msg_handle, ctx.http()).await?;

    Ok(())
}

fn default_embed() -> CreateEmbed {
    CreateEmbed::new()
        .colour(Colour::from_rgb(230, 29, 213))
        .author(
            CreateEmbedAuthor::new("The 7th Reason")
                .icon_url(ICON_LINK)
                .url("https://github.com/AuroraEchoes/The7thReason"),
        )
}

async fn add_reactions(reactions: &[&str], message: &Message, http: &Http) -> Result<(), Error> {
    for reaction in reactions {
        add_reaction(reaction, message, http).await?;
    }
    Ok(())
}

async fn add_reaction(reaction: &str, message: &Message, http: &Http) -> Result<(), Error> {
    message
        .react(http, ReactionType::Unicode(reaction.to_string()))
        .await?;
    Ok(())
}

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets.get("BOT_TOKEN").unwrap();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![poll_availability(), announce_event(), confirm_event()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    Ok(client.into())
}

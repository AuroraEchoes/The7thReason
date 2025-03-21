use chrono_tz::Australia::Sydney;
use ::serenity::all::{
        Colour, CreateEmbed, CreateEmbedAuthor, CreateMessage, Http,
        Message, ReactionType,
    };
use chrono::{DateTime, Days, NaiveDateTime, NaiveTime, TimeDelta, TimeZone};
use poise::{serenity_prelude as serenity, CreateReply};
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
    let demoman_title = if demoman.to_lowercase().contains("aurora") {
        "Demoma’am".to_string()
    } else {
        "Demoman".to_string()
    };

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
> {DEMOMAN_ICON} {demoman_title}: {demoman}
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
async fn poll_availability(
    ctx: Context<'_>,
    #[description = "Start Date (YYYY-MM-DD)"] start_date: Option<String>,
) -> Result<(), Error> {
    let naive_date = parse_date(&start_date.unwrap_or_default());
    let icons = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "❌"];

    if let Ok(naive_date) = naive_date {
        // let mut tz_date = chrono::DateTime::from_naive_utc_and_offset(naive_date.into(), sydney);
        let date_time = NaiveDateTime::new(naive_date, NaiveTime::from_hms_opt(12, 0, 0).unwrap());
        let mut tz_date = Sydney.from_local_datetime(&date_time).unwrap();
        let mut str_build = "".to_string();

        (0..7).for_each(|i| {
            let timestamp = tz_date.timestamp();
            str_build += &(icons[i].to_string() + " → " + format!("<t:{timestamp}:D>").as_str() + "\n");
            tz_date = tz_date.checked_add_days(Days::new(1)).unwrap()
        });

        ctx.send(
            CreateReply::default()
                .content("Creating availability poll")
                .ephemeral(true),
        )
        .await?;

        let embed = default_embed()
            .title(format!("**Availability Poll**"))
            .description(format!("*Polling availability.*\nReact with **all** of the days during which you are availabile **for at least an hour** at some point between 6pm – 10pm.\n\n{str_build}\n\n"));

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

    }
    else {
        ctx.send(
            CreateReply::default()
                .content("Invalid date. Please send a valid date in the format YYYY-MM-DD.")
                .ephemeral(true),
        ).await?;
    }
   
    Ok(())
}

fn parse_date(input: &String) -> Result<chrono::NaiveDate, ()> {
    let ymd = input
        .split("-")
        .filter_map(|s| match s.parse::<u32>() {
            Err(e) => None,
            Ok(i) => Some(i)
        })
        .collect::<Vec<_>>();
    if ymd.len() != 3 {
        return Err(())
    }
    let naive_date = chrono::NaiveDate::from_ymd_opt(ymd[0] as i32, ymd[1], ymd[2]);
    if let Some(naive_date) = naive_date {
        return Ok(naive_date)
    } else {
        return Err(())
    }
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

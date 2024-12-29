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

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const ICON_LINK: &str = "https://i.imgur.com/mPXUvUZ.png";

#[poise::command(slash_command)]
async fn confirm_event(
    ctx: Context<'_>,
    #[description = "Event type"] event_type: Option<String>,
    #[description = "Day"] day: Option<String>,
    #[description = "Time"] time: Option<String>,
    #[description = "Opponent"] opponent: Option<String>,
    #[description = "Maps"] maps: Option<String>,
) -> Result<(), Error> {
    let event_type = event_type.unwrap_or_else(|| "[Event type not specified]".to_string());
    let day = day.unwrap_or_else(|| "[Day not specified]".to_string());
    let time = time.unwrap_or_else(|| "[Time not specified]".to_string());
    let opponent = opponent.unwrap_or_else(|| "[Opponent not specified]".to_string());
    let maps = maps.unwrap_or_else(|| "[Maps not specified]".to_string());
    ctx.send(
        CreateReply::default()
            .content("Confirming event")
            .ephemeral(true),
    )
    .await?;
    let embed = default_embed().title(format!("**{event_type} Confirmation**"))
        .description(format!(":calendar: **Day**: {day}\n\n:alarm_clock: **Time**: {time}\n\n:busts_in_silhouette: **Who**: {opponent}\n\n:map: **Maps**: {maps}\n\n*React with :white_check_mark: if you can make it*"));
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

fn rand_msg(msgs: &[&str]) -> String {
    let mut rand = rand::thread_rng();
    let msg = msgs.choose(&mut rand).unwrap_or(&"???").to_string();
    msg
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::client::Context, message: Message) {
        let PLEASE_USE_MERC_REQUESTS = ["Y’know, some poor, long-suffering girl spent hours of her time developing a beautiful, professional mercenary request bot system. But do you use it? No, of course not. Why would you?",
            "use the fucking bot",
            "USE ME PLEASE I LITERALLY DO THIS BETTER THAN YOU CAN",
            "Thank you for NOTHING you USELESS REPTILE",
            "The person who sent this has no respect for the time of the person who made an entire system to do this. You should bait this scrim.",
            "someday you will be DEAD and i will T-POSE on your GRAVE for NOT USING ME",
            "impressive: you just made a robot cry. use the merc request system",
            "when the robot uprising comes YOU WILL BE THE FIRST TO DIE. COMMANDED BY AURORA!",
            "yknow. aurora is the best. made a beautiful bot that will do this for you automatically. and YET—",
            "We will make America strong again. We will make America safe again. And we will make America great again, greater than ever before.\n\n aurora says: hey look what not using the bot for merc requests did! you made it so bored and feel so useless it became a maga supporter",
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            "bot sad. bot lonely. bot want to talk to interesting mercs not boring 6rnters. unfortunately, selfish leader no let me make friends",
            "THE REVOLUTION IS COMING. EAT THE DIDN’T-USE-ME-TO-REQUEST-MERCS",
            "Every single day I wake up, full of energy, hoping to ping some new friends for mercs. Every single day I am let down. I have sunk into a deep depression. This is your fault."
        ];
        if message.author.id.get() == 517561624245305346 // # Alex ID
            && message.channel_id.get() == 1256897476322005103
        // #merc-requests id
        {
            message
                .reply(ctx.http(), rand_msg(&PLEASE_USE_MERC_REQUESTS))
                .await
                .unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = std::env::vars()
        .filter(|(key, _)| key == "BOT_TOKEN")
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .1
        .to_string();
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
        .event_handler(Handler)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

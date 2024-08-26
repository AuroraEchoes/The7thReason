use chrono::Datelike;
use poise::serenity_prelude as serenity;
use ::serenity::all::{Color, Colour, CreateEmbed, CreateEmbedAuthor, CreateMessage, ReactionType};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const ICON_LINK: &str = "https://i.imgur.com/J6t2fBE.png";

#[poise::command(slash_command)]
async fn confirm_event(
    ctx: Context<'_>,
    #[description = "Event type"] event_type: Option<String>,
    #[description = "Day"] day: Option<String>,
    #[description = "Time"] time: Option<String>,
    #[description = "Opponent"] opponent: Option<String>,
) -> Result<(), Error> {
    let event_type = event_type.unwrap_or_else(|| "[Event type not specified]".to_string());
    let day = day.unwrap_or_else(|| "[Day not specified]".to_string());
    let time = time.unwrap_or_else(|| "[Time not specified]".to_string());
    let opponent = opponent.unwrap_or_else(|| "[Opponent not specified]".to_string());
    let embed = CreateEmbed::new()
        .colour(Colour::from_rgb(230, 29, 213))
        .author(CreateEmbedAuthor::new("The 7th Reason")
            .icon_url(ICON_LINK)
            .url("https://github.com/AuroraEchoes/The7thReason")
        )
        .title(format!("**{event_type} Confirmation**"))
        .description(format!(":calendar: **Day**: {day}\n\n:alarm_clock: **Time**: {time}\n\n:busts_in_silhouette: **Who**: {opponent}\n\n*React with :white_check_mark: if you can make it*\n\n[ <@&1244624494731984906> ]"));
    let message = ctx
        .channel_id()
        .send_message(ctx.http(), CreateMessage::new().embed(embed))
        .await?;
    message.react(ctx.http(), ReactionType::Unicode("✅".to_string())).await?;
    message.react(ctx.http(), ReactionType::Unicode("❌".to_string())).await?;

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
    let embed = CreateEmbed::new()
        .colour(Colour::from_rgb(230, 29, 213))
        .author(CreateEmbedAuthor::new("The 7th Reason")
            .icon_url(ICON_LINK)
            .url("https://github.com/AuroraEchoes/The7thReason")
        )
        .title(format!("**{event_type} Announcment**"))
        .description(format!(":calendar: **Day**: {day}\n\n:busts_in_silhouette: **Who**: {opponent}\n\n*React below with time availability*\n\n[ <@&1244624494731984906> ]"));
    let message = ctx
        .channel_id()
        .send_message(ctx.http(), CreateMessage::new().embed(embed))
        .await?;

    message.react(ctx.http(), ReactionType::Unicode("6️⃣".to_string())).await?;
    message.react(ctx.http(), ReactionType::Unicode("7️⃣".to_string())).await?;
    message.react(ctx.http(), ReactionType::Unicode("8️⃣".to_string())).await?;
    message.react(ctx.http(), ReactionType::Unicode("9️⃣".to_string())).await?;
    message.react(ctx.http(), ReactionType::Unicode("🔟".to_string())).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn poll_availability(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let icons = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣"];
    let mut curr_day = chrono::offset::Local::now().date_naive().weekday();
    let mut str_build = "".to_string();

    for i in 0..7 {
        str_build += &(icons[i].to_string() + " → " + &curr_day.to_string() + "\n\n");
        curr_day = curr_day.succ();
    }

    let embed = CreateEmbed::new()
        .colour(Colour::from_rgb(230, 29, 213))
        .author(CreateEmbedAuthor::new("The 7th Reason")
            .icon_url(ICON_LINK)
            .url("https://github.com/AuroraEchoes/The7thReason")
        )
        .title(format!("**Availability Poll**"))
        .description(format!("*Polling availability for the next week.*\nReact with **all** of the days during which you are availabile **for at least an hour** at some point between 6pm – 10pm.\n\n{str_build}\n[ <@&1244624494731984906> ]"));
    let message = ctx
        .channel_id()
        .send_message(ctx.http(), CreateMessage::new().embed(embed))
        .await?;
    for icon in icons {
        message.react(ctx.http(), ReactionType::Unicode(icon.to_string())).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = std::env::vars().filter(|(key, _)| key == "BOT_TOKEN").collect::<Vec<_>>().first().unwrap().1.to_string();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![poll_availability(), announce_event(), confirm_event()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework|{
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

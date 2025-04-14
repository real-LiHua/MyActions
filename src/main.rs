use clap::Parser;
use grammers_client::session::{PackedType, Session};
use grammers_client::types::PackedChat;
use grammers_client::{Client, Config, InitParams, InputMessage};
use tokio;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

const SESSION_FILE: &str = "bot.session";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    id: i32,

    #[arg(long)]
    hash: String,

    #[arg(long)]
    token: String,

    #[arg(long)]
    chat: i64,

    #[arg(long)]
    file: String,

    #[arg(long)]
    message: String,
}

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse();

    println!("Connecting to Telegram...");
    let client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE)?,
        api_id: args.id,
        api_hash: args.hash,
        params: InitParams {
            catch_up: true,
            ..Default::default()
        },
    })
    .await?;
    println!("Connected!");

    if !client.is_authorized().await? {
        println!("Signing in...");
        client.bot_sign_in(&args.token).await?;
        client.session().save_to_file(SESSION_FILE)?;
        println!("Signed in!");
    }

    let uploaded_file = client.upload_file(&args.file).await?;

    client
        .send_message(
            PackedChat {
                ty: PackedType::Chat,
                id: args.chat,
                access_hash: Some(0),
            },
            InputMessage::text(&args.message).file(uploaded_file),
        )
        .await?;
    Ok(())
}

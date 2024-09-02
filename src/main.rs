use std::{thread::sleep, time};

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

use reqwest;
use serde_json;

mod jellyfin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let jellyfin_url = std::env::var("JELLYFIN_URL").expect("No jellyfin url");
    let jellyfin_token = std::env::var("JELLYFIN_TOKEN").expect("No jellyfin token");
    let discord_app_id = std::env::var("DISCORD_APP_ID").expect("No Discord app id");

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("http://{}/Sessions", jellyfin_url))
        .header(
            "Authorization",
            format!("MediaBrowser Token=\"{}\"", jellyfin_token),
        )
        .send()?;

    let text = format!("{{\"data\": {}}}", response.text()?);
    let json_response = serde_json::from_str::<serde_json::Value>(&text)?;

    // println!("{:?}", json_response);

    let author = json_response["data"][0]["NowPlayingItem"]["Artists"][0].to_string();
    let title = json_response["data"][0]["NowPlayingItem"]["Name"].to_string();

    println!("author: {}", author);
    println!("title: {}", title);

    let mut discord_client = DiscordIpcClient::new(&discord_app_id)?;
    let mut i = 0;

    discord_client.connect()?;
    while i != 30 {
        let response = client
            .get(format!("http://{}/Sessions", jellyfin_url))
            .header(
                "Authorization",
                format!("MediaBrowser Token=\"{}\"", jellyfin_token),
            )
            .send()?;
        let text = format!("{{\"data\": {}}}", response.text()?);
        let json_response = serde_json::from_str::<serde_json::Value>(&text)?;

        let playtime = json_response["data"][0]["PlayState"]["PositionTicks"]
            .to_string()
            .parse::<u64>()?
            / 10000000;
        println!("playtime: {}", playtime);
        let playtime = format!("{}:{:02}", playtime / 60, playtime % 60);

        discord_client.set_activity(
            activity::Activity::new()
                .state(&author[1..author.len() - 1])
                .details(&format!("{}\n({})", &title[1..title.len() - 1], playtime))
                .assets(
                    activity::Assets::new()
                        .large_text("Large text")
                        .large_image("logo")
                        .small_text("small text")
                        .small_image("logo"),
                ),
        )?;
        i += 1;
        sleep(time::Duration::from_secs(1));
    }
    discord_client.close()?;

    Ok(())
}

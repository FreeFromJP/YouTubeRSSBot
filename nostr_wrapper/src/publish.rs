use crate::Config;
use nostr_sdk::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub async fn publish_text_note(
    my_keys: &Keys,
    username: &str,
    avatar: &str,
    message: &str,
) -> Result<()> {
    let file = File::open("./conf/test/config.yaml").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader).expect("Failed to read config");
    // show keys
    let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
    log::info!("Bech32 PubKey: {}", bech32_pubkey);

    // create client
    let client = Client::new(my_keys);

    // add relays
    client.add_relays(config.nostr.relays).await?;

    // connect to the network
    client.connect().await;

    let metadata = Metadata::new()
        .name(username)
        .display_name(username)
        .about("Description")
        .picture(Url::parse(avatar)?)
        .banner(Url::parse(avatar)?)
        .nip05("username@example.com")
        .lud16("yuki@getalby.com")
        .custom_field("custom_field", "my value");

    // Update metadata
    client.set_metadata(&metadata).await?;

    // post a text note
    client.publish_text_note(message, []).await?;
    // disconnect
    match client.disconnect().await {
        Ok(_) => (),
        Err(e) => log::error!("Failed to disconnect: {}", e),
    }
    Ok(())
}

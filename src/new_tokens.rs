
use crate::{ consts::DISCORD_URL, event::CreateEvent, utlis::{embed::{send_embed, Embed, EmbedField, EmbedThumbnail, WebhookMessage}, user_data::TokenMetadata}};
use mpl_token_metadata::accounts::Metadata;



pub async fn new_tokens_prog(
    create_event: CreateEvent,
    token_data: TokenMetadata,
    user_prev_tokens: Vec<Metadata>,
) {

let amount=create_event.buy_event.unwrap().amount;
    let percentage = (amount as f64) / 1000000000000000000.0;

    // let token_market_cap =
    //     ((trade_event.virtual_sol_reserves as f64) / (trade_event.virtual_token_reserves as f64)) *
    //     1_000_000.0 *
    //     sol_to_usd_rate;

    let tokens_section: Vec<String> = if !user_prev_tokens.is_empty() {
        let tokens_result = (async {
            let mut tokens = Vec::new();
            for item in &user_prev_tokens {
                if item.mint == create_event.mint {
                    continue;
                } else if !item.mint.to_string().is_empty() {
        

                    tokens.push(
                        format!(
                            "- [ {} $({})]({}) \n",
                            item.name,
                            item.symbol,
                            format!("https://dexscreener.com/solana/{}", item.mint)
                        )
                    );
                }
            }
            tokens
        }).await;

        tokens_result
    } else {
        Vec::new()
    };


    let mut embed_fields = vec![];

    for chunk in tokens_section.chunks(10) {
        let chunk_value = chunk.join(""); 
        embed_fields.push(EmbedField {
            name: "".to_string(), 
            value: format!("{}", chunk_value),
            inline: false,
        });
    }

    let mut fields = vec![EmbedField {
        name: "".to_string(),
        value: format!(
            "**Contract Address**\n`{}`\n\n**Description**\n{}\n\n**Dev Information:**\n* Dev Holdings: `{:.2}%`\n\n**Creator Launched Tokens** \n{}",
            create_event.mint,
            token_data.description,
            percentage,
            if tokens_section.is_empty() {
                "There is no previously launched tokens"
            } else {
                ""
            }
        ),
        inline: true,
    }];

    fields.extend(embed_fields);

    fields.push(EmbedField {
        name: "".to_string(),
        value: "".to_string(), 
        inline: true,
    });

    // Construct the embed object
    let embed = Embed {
        title: format!(
            "{} $({}) ",
            create_event.name,
            create_event.symbol,
        ),
        url: format!("https://dexscreener.com/solana/{}", create_event.mint),
        fields,
        thumbnail: EmbedThumbnail {
            url: token_data.image.clone(),
        },
    };


    let message = WebhookMessage {
        content: "".to_string(), 
        embeds: vec![embed],
    };
    let webhook_url =
    DISCORD_URL;
    let _ = send_embed(webhook_url, message).await;
}

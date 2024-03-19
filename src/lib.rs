use tg_flows::{listen_to_update, Telegram, Update, UpdateKind, update_handler, ChatId};
use flowsnet_platform_sdk::logger;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    let telegram_token = std::env::var("TELEGRAM_TOKEN").unwrap();
    listen_to_update(telegram_token).await;
}

fn is_user_allowed(user_id: &str, allowed_user_ids: Vec<&str>) -> bool {
    allowed_user_ids.contains(&user_id)
}

#[update_handler]
async fn handler(update: Update) {
    logger::init();
    let telegram_token = std::env::var("TELEGRAM_TOKEN").unwrap();
    let tele:Telegram = Telegram::new(telegram_token.to_string());

    let target_channel_id = std::env::var("TARGET_CHANNEL_ID").unwrap();
    let allowed_user_ids_str = std::env::var("ALLOWED_USER_IDS").unwrap();
    let allowed_user_ids: Vec<&str> = allowed_user_ids_str.split(',').collect();

    if let UpdateKind::Message(msg) = update.kind {
        let chat_id_from = msg.chat.id;

        let chat_id_target = ChatId(target_channel_id.parse().unwrap());

        if let Some(user) = msg.from() {
            if is_user_allowed(&user.id.to_string(), allowed_user_ids) {
                let text = msg.text().unwrap_or("");
                let _sended_msg = tele.send_message(chat_id_target, text);
            }
        }
    }
}
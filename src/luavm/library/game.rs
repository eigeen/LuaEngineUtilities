use mhw_toolkit::game_util::{ChatMessageSender, SystemMessageColor};
use mlua::prelude::*;
use mlua::UserData;
use once_cell::sync::Lazy;

static CHAT_MESSAGE_SENDER: Lazy<ChatMessageSender> = Lazy::new(ChatMessageSender::new);

pub struct Game;

impl UserData for Game {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("sendMessage", |_, arg: String| {
            CHAT_MESSAGE_SENDER.send(&arg);
            Ok(())
        });

        methods.add_function(
            "systemMessage",
            |_, (msg, color): (String, Option<String>)| {
                let color_value = match color {
                    Some(c) => match c.to_lowercase().as_str() {
                        "blue" | "general" => SystemMessageColor::Blue,
                        "purple" | "primary" => SystemMessageColor::Purple,
                        _ => {
                            return Err(LuaError::runtime(format!(
                                "Unsupported color: {}, expect `blue|general` or `purple|primary`",
                                c
                            )))
                        }
                    },
                    None => SystemMessageColor::Blue,
                };
                mhw_toolkit::game_util::show_system_message(&msg, color_value);

                Ok(())
            },
        );
    }
}

pub mod channel;
pub mod get_trailer;
pub mod load_channel_list;
pub mod login;
pub mod user;

use futures_loco_protocol::session::LocoSession;

use crate::{request, RequestResult};
use async_stream::try_stream;
use futures_lite::Stream;

use self::channel::TalkChannel;

#[derive(Debug, Clone, Copy)]
pub struct TalkSession<'a>(pub &'a LocoSession);

impl<'a> TalkSession<'a> {
    pub async fn ping(self) -> RequestResult<()> {
        request!(self.0, "PING", bson {}).await
    }

    pub async fn set_status(self, status: i32) -> RequestResult<()> {
        request!(self.0, "SETST", bson { "st": status }).await
    }

    pub async fn login(
        self,
        req: login::Request<'a>,
    ) -> RequestResult<(
        login::Response,
        impl Stream<Item = RequestResult<load_channel_list::Response>> + 'a,
    )> {
        let res = request!(self.0, "LOGINLIST", &req, login::Response).await?;

        let mut req = load_channel_list::Request {
            chat_ids: req.chat_list.chat_ids,
            max_ids: req.chat_list.max_ids,
            last_token_id: res.chat_list.last_token_id.unwrap_or(0),
            last_chat_id: res.chat_list.last_chat_id,
        };

        let stream = try_stream!({
            loop {
                let res = request!(self.0, "LCHATLIST", &req, load_channel_list::Response).await?;

                if let Some(id) = res.last_token_id {
                    req.last_token_id = id;
                }
                req.last_chat_id = res.last_chat_id;

                let eof = res.eof;

                yield res;

                if eof {
                    break;
                }
            }
        });

        Ok((res, stream))
    }

    pub async fn get_trailer(
        self,
        chat_type: i32,
        key: &str,
    ) -> RequestResult<get_trailer::Response> {
        request!(self.0, "GETTRAILER", bson {
            "k": key,
            "t": chat_type,
        }, get_trailer::Response)
        .await
    }

    pub const fn channel(self, id: i64) -> TalkChannel<'a> {
        TalkChannel {
            session: self.0,
            id,
        }
    }
}

/*
impl_session!(
        pub fn chat_on_channel("CHATONROOM", bson ChatOnChannelReq) -> ChatOnChannelRes;

        pub fn get_all_users("GETMEM", bson struct GetAllUsersReq {
            /// Chatroom id
            #[serde(rename = "chatId")]
            pub chat_id: i64,
        }) -> struct GetAllUsersRes {
            /// User list
            pub members: Vec<UserVariant>,
        };

        pub fn get_users("MEMBER", bson struct GetUsersReq<'a> {
            /// Chatroom id
            #[serde(rename = "chatId")]
            pub chat_id: i64,

            /// List of requesting user id list
            #[serde(rename = "memberIds")]
            pub user_ids: &'a [i64],
        }) -> struct GetUsersRes {
            /// Chatroom id
            #[serde(rename = "chatId")]
            pub chat_id: i64,

            /// List of requested user list
            #[serde(rename = "members")]
            pub members: Vec<UserVariant>,
        };
    }
);
*/

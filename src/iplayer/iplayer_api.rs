use rocket::{
    catch, delete, get,
    http::Status,
    post, put,
    request::{FromRequest, Outcome},
    Request, Responder,
};

use crate::{
    iplayer::data::IPlayer,
    util::httpGetResponder_tools::{null_200_http_get_responder, HttpGetResponder},
    CONFIG_VAR,
};
#[derive(Debug)]
pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            let ser_config = CONFIG_VAR
                .lock()
                .unwrap()
                .as_ref()
                .expect("CONFIG_VAR not initialized")
                .clone();
            key == ser_config.x_api_key
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

//添加新玩家数据
#[post("/", format = "application/json", data = "<player_str>")]
pub fn addiplayer(player_str: String, api_key: ApiKey) -> HttpGetResponder {
    let iplayer: IPlayer = serde_yaml::from_str(&player_str).unwrap_or(IPlayer::default());
    println!("{:#?}", iplayer);
    println!("{:?}", api_key);
    null_200_http_get_responder()
}

//删除玩家数据
#[delete("/?<nick_name>")]
pub fn deleteiplayer(nick_name: String, api_key: ApiKey) -> HttpGetResponder {
    println!("{:?}", nick_name);
    null_200_http_get_responder()
}

//修改玩家基础数据
#[put("/?<nick_name>", format = "application/json", data = "<player_str>")]
pub fn updataiplayer(nick_name: String, player_str: String, api_key: ApiKey) -> HttpGetResponder {
    // Your logic here...
    println!("x-api-key: {}", api_key.0);
    println!("Nick Name: {}", nick_name);
    println!("Player String: {}", player_str);

    null_200_http_get_responder()
}
//获取玩家基础数据数据
#[get("/?<nick_name>")]
pub fn getiplayerdate(nick_name: String, api_key: ApiKey) -> HttpGetResponder {
    println!("{:?}", nick_name);
    null_200_http_get_responder()
}

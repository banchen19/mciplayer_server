// 异步时锁
use std::sync::Mutex;

use crate::iplayer::data::Config;

mod iplayer;
mod util;

use rocket::catchers;
use rocket::futures::stream::PollNext;
//rocket框架
use rocket::routes;
use rocket::Config as OtherConfig;

use rocket::yansi::Paint; //转换u数组为字符串

// 基础库
use std::io;

// 全局静态变量
#[macro_use]
extern crate lazy_static;
lazy_static! {
    // static ref POOL: Mutex<Option<Pool>> = Mutex::new(None);  //Pool mysql的连接池  use mysql::Pool;
    static ref CONFIG_VAR: Mutex<Option<Config>> = Mutex::new(None);
}
#[tokio::main]
async fn main() {
    let config = Config::generate_file();
    *CONFIG_VAR.lock().unwrap() = Some(config.clone());
    let http_server_task: tokio::task::JoinHandle<tokio::task::JoinHandle<()>> =
        tokio::spawn(start_http_server(config));
    let _handle_input: () = handle_input(http_server_task).await.to_owned();
}

async fn handle_input(
    // ws_server_task: tokio::task::JoinHandle<tokio::task::JoinHandle<()>>,
    http_server_task: tokio::task::JoinHandle<tokio::task::JoinHandle<()>>,
) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("请输入正确指令");
        // 去除输入字符串两端的空格和换行符
        let command = input.trim();
        match command {
            "stop" => {
                // ws_server_task.abort();
                http_server_task.abort();
                // println!("{}", Paint::red("Rust_Syn服务端: 已经停止"));
                std::process::exit(0);
            }
            "help" => {
                let mut text = format!("{}: 帮助\n", Paint::yellow("help"));
                text += &format!("{}: 停止服务端\n", Paint::yellow("stop"));
                text += &format!(
                    "{}: 添加玩家数据（格式：addplayer <玩家名字>  <权限等级>）\n",
                    Paint::yellow("addplayer")
                );
                text += &format!(
                    "{}: 删除玩家数据（格式：delplayer <玩家名字>）\n",
                    Paint::yellow("delplayer")
                );
                text += &format!(
                    "{}: 修改玩家权限等级（格式：changeplevel <玩家名字>  <权限等级>）\n",
                    Paint::yellow("changeplevel")
                );
                println!("{}", Paint::green(text));
            }
            _ => {
                println!("其他指令{}",input)
                //  match com_mg(input) {
                //     Ok(_) => {
                //         println!("{}", Paint::green("执行成功"));
                //     }
                //     Err(_str) => {
                //         println!("{}", Paint::red(_str))
                //     }
                // },
            }
        }
    }
}

use crate::iplayer::iplayer_api::*;
use crate::util::httpGetResponder_tools::not_found;
// 启动http端
async fn start_http_server(config: Config) -> tokio::task::JoinHandle<()> {
    // 启动 HTTP 服务器
    let http_server_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let httpconfig = OtherConfig::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", config.api_port));
        let _ = rocket::custom(httpconfig)
            .mount("/addiplayer", routes![addiplayer])
            .mount("/deleteiplayer", routes![deleteiplayer])
            .mount("/updataiplayer", routes![updataiplayer])
            .mount("/getiplayerdate", routes![getiplayerdate])
            .register("/", catchers![not_found])
            .launch()
            .await;
    });
    http_server_task
}

// 异步
use std::{error::Error, fs};

// 工具
use crate::util::yml_util::{generate_random_key, read_yml, write_config_to_yml};

// 基础库
use serde::{Deserialize, Serialize};

/// 玩家默认数据
#[derive(Clone, Serialize, Deserialize,Default,Debug)]
pub struct IPlayer {
    /// 年龄
    age: String,

    /// 性别
    gender: String,

    /// 地址
    ip_address: String,

    /// java玩家名
    je_name: String,

    /// 邮箱地址
    mail: String,

    /// 昵称
    nick_name: String,

    /// 密码
    password: String,

    /// pe玩家名
    pe_name: String,

    /// QQ号
    qq: f64,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    /// api端口
    pub api_port: u32,

    /// 默认新玩家具有多少经济
    pub default_money: i64,

    /// 加密方式，AES-128
    pub key_mode: String,

    /// 多元经济，moneys/单经济名
    pub multiple_economies: String,

    /// 存储数据方式，sqlite3/mysql/json
    pub sql_mode: String,

    /// 白名单，默认true
    pub whitelist: bool,

    /// ws端口
    pub ws_prot: u32,

    /// 长度为十六的密钥
    #[serde(rename = "x-api-key")]
    pub x_api_key: String,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            api_port: 20103,
            default_money: 0,
            key_mode: "AES-128".to_string(),
            multiple_economies: "滑稽币".to_string(),
            sql_mode: "sqlite3".to_string(),
            whitelist: true,
            ws_prot: 20104,
            x_api_key: generate_random_key(16),
        }
    }
}

impl Config {
    pub fn generate_file() -> Config {
        let file_path = "config.yml";
        match fs::metadata(&file_path) {
            Err(_) => {
                println!("文件不存在, 开始写入");
                if let Err(err) = write_config_to_yml(&Config::default(), &file_path) {
                    println!("无法写入配置文件：{}", err);
                }
            }
            Ok(_) => {
                println!("检测到配置文件存在");
            }
        }
        match read_yml(&file_path) {
            Ok(config) => config,
            Err(err) => Config::default(),
        }
    }
}

use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};

use crate::iplayer::data::Config;

//随机生成指定长度的密钥
pub fn generate_random_key(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let characters: Vec<char> = "QWERTYUIOPASDFGHJKLZXCVBNMabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let key: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..characters.len());
            characters[idx]
        })
        .collect();
    key
}
// 写入到yml文件
pub fn write_config_to_yml(
    config: &Config,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_string = serde_yaml::to_string(config)?;
    let mut file = File::create(file_path)?;
    file.write_all(yaml_string.as_bytes())?;
    Ok(())
}

// 读取文件并转换为config
pub fn read_yml(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

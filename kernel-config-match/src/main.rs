// 功能说明：
// 支持将架构相关默认开启config配置选项过滤出来
// 参数 1：内核源码目录下arch/<arch-name>/Kconfig文件路径
// 参数 2：包含裁剪的内核fragments的目录

use anyhow::{anyhow, Ok, Result};
use std::fs;
use std::{collections::HashMap, env};

fn parse_configs(text: &str) -> HashMap<String, String> {
    let mut configs = HashMap::new();

    let lines: Vec<&str> = text.lines().collect();

    let mut key = String::new();
    let mut value = String::new();
    for line in lines {
        if line.starts_with("config ") {
            // insert the last config block
            if !key.is_empty() {
                if configs.contains_key(key.as_str()) {
                    println!("key {:?} is duplicate!!!", key);
                }

                // filter the config with the `def_bool y` && no `depends on` configuration
                if value.contains("def_bool y") && !value.contains("depends on") {
                    configs.insert(key.trim_end().to_string(), value.trim_end().to_string());
                }

                value.clear();
            }
            let start_line: Vec<&str> = line.split(" ").collect();
            key = start_line[1].to_string();
        } else {
            value.push_str(line);
            value.push('\n');
        }
    }

    // process the last config block
    if !key.is_empty() {
        configs.insert(key, value);
    }

    configs
}

fn parse_configs_vec(text: &str) -> Vec<String> {
    let mut configs = Vec::new();

    let lines: Vec<&str> = text.lines().collect();

    let mut key = String::new();
    let mut value = String::new();
    for line in lines {
        if line.starts_with("config ") {
            // insert the last config block
            if !key.is_empty() {
                configs.push(key.trim_end().to_string());
                value.clear();
            }
            let start_line: Vec<&str> = line.split(" ").collect();
            key = start_line[1].to_string();
        } else {
            value.push_str(line);
            value.push('\n');
        }
    }

    // process the last config block
    if !key.is_empty() {
        configs.push(key);
    }

    configs
}

fn filter_auto_select_config(map: &HashMap<String, String>) -> Result<HashMap<String, String>> {
    let mut select_configs = HashMap::new();

    for (_, v) in map.iter() {
        let lines: Vec<&str> = v.lines().collect();
        for line in lines {
            if line.contains("select") && !line.contains("if") {
                let result: Vec<&str> = line.trim().split(" ").collect();
                if result.len() != 2 {
                    println!("invalid select config value");
                    continue;
                }
                let new_key = result[1];
                select_configs.insert(new_key.to_string(), "".to_string());
            }
        }
    }

    Ok(select_configs)
}

fn main() {
    //let text = "config X86_32\n\tdef_bool y\n\tdepends on !64BIT\n\t...\nconfig X86_64\n\tdef_bool y\n\tdepends on 64BIT\n\t...";
    let args: Vec<String> = env::args().collect();
    let kconfig_file = &args[1];
    let text = fs::read_to_string(kconfig_file).expect("failed to read contents from file");

    //let configs = parse_configs_vec(&text);
    let configs = parse_configs(&text);

    println!("len of configs: {}", configs.len());
    println!("auto enabled kernel config: ");
    for (key, _) in &configs {
        println!("{}", key);
    }

    println!("-----------------------------------");

    let selectd_configs = filter_auto_select_config(&configs).unwrap();
    println!("len of auto selected configs: {}", selectd_configs.len());
    println!("auto selected kernel config: ");
    for (key, _) in &selectd_configs {
        println!("{}", key);
    }
}

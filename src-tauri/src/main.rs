// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io::Read};
use std::io::BufReader;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DPMessage {
    #[serde(rename = "ID")]
    message_id: String,
    #[serde(rename = "Timestamp")]
    message_timestamp: String,
    #[serde(rename = "Contents")]
    message_content: String,
    #[serde(rename = "Attachments")]
    message_attachments: String
}

#[derive(Serialize, Deserialize)]
struct DPChannel {
    #[serde(rename = "id")]
    channel_id: String,
    #[serde(rename = "type")]
    channel_type: u8,
    #[serde(rename = "name")]
    channel_name: Option<String>
}

#[derive(Serialize, Deserialize)]
struct DPUser {
    #[serde(rename = "id")]
    user_id: String,
    #[serde(rename = "username")]
    user_name: String,
    #[serde(rename = "discriminator")]
    user_discriminator: u8,
    #[serde(rename = "global_name")]
    user_globalname: String,
    #[serde(rename = "email")]
    user_email: String,
    #[serde(rename = "verified")]
    user_verified: bool,
    #[serde(rename = "has_mobile")]
    user_has_phone: bool,
    #[serde(rename = "phone")]
    user_phone: String,
    #[serde(rename = "ip")]
    user_ip: String
}

#[derive(Serialize, Deserialize)]
struct DPIndex {
    account: DPUser,
    channel_list: Vec<DPChannel>
}

#[tauri::command]
fn load_message(zip_path: &str, channel_id: &str) -> Vec<DPMessage> {
    println!("Attempting to load message id: {} from {}",channel_id, zip_path);
    let file = fs::File::open(zip_path).unwrap();
    let reader = BufReader::new(file);

    let mut message_list: Vec<DPMessage> = Vec::new();

    let mut data_package = zip::ZipArchive::new(reader).unwrap();

    let mut file = match data_package.by_name(format!("messages/c{}/messages.csv",channel_id).as_str()) {
        Ok(file)=>file,
        Err(_) => todo!(), };
    
    //read file to string
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("Message data expected...");

    println!("{}", contents);

    let mut csv_reader = csv::Reader::from_reader(contents.as_bytes()); 
    for result in csv_reader.deserialize() {
        let message: DPMessage = result.expect("message data...");
        println!("Loading message... - {},{},{},{}", message.message_id, message.message_timestamp, message.message_content, message.message_attachments);
        message_list.push(message);
    }
    
    return message_list;
}

#[tauri::command]
fn index_package(zip_path: &str) -> DPIndex {
    let file = fs::File::open(zip_path).unwrap();
    let reader = BufReader::new(file);

    let mut user_data: DPUser = DPUser { user_id: ("").to_string(), user_name: ("").to_string(), user_discriminator: 0, user_globalname: ("").to_string(), user_email: ("").to_string(), user_verified: false, user_has_phone: false, user_phone: ("").to_string(), user_ip: ("").to_string() };
    let mut channel_list: Vec<DPChannel> = Vec::new();

    let mut data_package = zip::ZipArchive::new(reader).unwrap();

    for i in 0..data_package.len() {
        let mut file = data_package.by_index(i).unwrap();

        //Account folder
        if file.name().starts_with("account/") {
            //user data
            if file.name().ends_with("user.json") {
                println!("{} found, attempting to parse data...", file.name());
                //read file to string
                let mut contents: String = String::new();
                file.read_to_string(&mut contents).expect("User data expected...");
                user_data = serde_json::from_str(&contents).expect("Error parsing string...");
                println!("finished parsing {}", file.name());
            }
        }
        //Messages folder
        if file.name().starts_with("messages/") {
            //channel data
            if file.name().ends_with("channel.json"){
                println!("Channel data [{}] found, attempting to parse data...", file.name());
                //read file to string
                let mut contents: String = String::new();
                file.read_to_string(&mut contents).expect("User data expected...");
                let mut channel_data: DPChannel = serde_json::from_str(&contents).expect("Error parsing string...");
                if channel_data.channel_type == 1 {
                    //add function to compare relation list with found IDs
                    let channelname: String = format!("DM with {}", channel_data.channel_id);
                    channel_data.channel_name= Some(channelname);
                }
                channel_list.push(channel_data);
                println!("finished parsing {}", file.name());
            }
        }
    }
    let package_index: DPIndex = DPIndex { account: user_data, channel_list: channel_list };
    return package_index;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![index_package, load_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod js_bridge;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Serialize, Deserialize)]
struct MemberJSON<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub you: bool
}

pub fn register_member(room_id: &str,name: &str) {
    js_bridge::register_member(room_id,name)
}

pub fn sync_members(room_id: &str,callback:Box<dyn Fn(Vec<Member>) -> ()>) {
    let json_callback : Box<dyn Fn(String)>= Box::new(move |json:String| {
        let members_json : Vec<MemberJSON> = serde_json::from_str(json.as_str()).expect("JSON Parse Error");
        let members = members_json
            .iter()
            .map(|member| Member {id:member.id,name: member.name, you: member.you})
            .collect::<Vec<Member>>();
        callback(members);
    });
    js_bridge::sync_member(
        room_id, 
        Closure::into_js_value(Closure::wrap(json_callback))
    )
    
}

pub struct Member<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub you: bool
}

pub fn create_room(hostname: &str,callback : Box<dyn FnOnce(String)>) {
    js_bridge::create_room(
        hostname,
        Closure::into_js_value(Closure::once (callback))
    )
}

pub enum Phase {
    Meeting,
    Started
}


pub struct Room {
    pub phase: Phase,
    pub is_host: bool  
}

#[derive(Serialize, Deserialize)]
pub struct RoomJSON<'a> {
    pub phase: &'a str,
    pub is_host: bool  
}

pub fn sync_room(room_id: &str,callback:Box<dyn Fn(Option<Room>) -> ()>) {
    let callback = Box::new(move |room: Option<String>| {
        let room = room.map(|room| -> Room {
            let room: RoomJSON = serde_json::from_str(room.as_str()).expect("JSON Parse Error");
            Room { 
                phase: match room.phase {
                    "MEETING" => Phase::Meeting,
                    "STARTED" => Phase::Started,
                    invalid => panic!("Invalid Value {}",invalid)
                }, 
                is_host: room.is_host 
            }
        });
        callback(room);
    });
    js_bridge::sync_room(
        room_id, 
        callback
    );
}

pub fn start_room(room_id: &str) {
    js_bridge::start_room(room_id);
}
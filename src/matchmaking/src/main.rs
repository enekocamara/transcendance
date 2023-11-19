#[allow(unused_imports)]

use warp::Filter;
use warp::filters::multipart;
use warp::reply::reply;

use core::fmt;
use std::convert::Infallible;
use std::fmt::Debug;
use std::thread::sleep;
use std::{thread, clone};
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::collections::{VecDeque};
use std::time::{Duration, Instant};

use serde::Deserialize;

use array_init::array_init;

mod ServerSettings;

use ServerSettings::ServerSettings::Settings;



#[repr(u16)]
#[derive(Deserialize,Copy,Clone)]
enum LobbyType{
    Uninitialized = 0,
    Duel = 2,
    Group = 4,
    Battalion = 10,
}
impl Default for LobbyType {
    fn default() -> Self {
        LobbyType::Uninitialized
    }
}

impl fmt::Debug for Lobby {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Lobby");
        for (index, name) in self.players.iter().enumerate() {
            debug_struct.field(&format!("Player{}", index), name);
        }
        debug_struct.finish()
    }
}


#[derive(Deserialize)]
struct Input {
    #[serde(deserialize_with = "deserialize_char_array")]
    username: [char;10],
    lobby_type : LobbyType,
}

#[derive(Clone, Default)]
struct Lobby{
    lobby_type : u16,
    players : Vec<[char;10]>
}

struct Data{
    lobbies : [Lobby; Settings::MaxLobbyNum as usize],
    free_lobbies : VecDeque<usize>,
    num_of_active_lobbies : u16,
    last_lobby : u16,//like a custom memory manager use the lobby_type u16 same size
    num_of_active_players : u16
}

impl Data{
    fn getLastLobby(&mut self) -> Result<u16,()>{
        let mut index : u16 = self.last_lobby;
        loop {
            if self.lobbies[index as usize].lobby_type == u16::MAX{
                return Ok(self.last_lobby);
            }
            index = self.lobbies[index as usize].lobby_type;
        }
    }
    
}

struct QueuesSys{
    duel_queue:  VecDeque<[char;10]>,
    group_queueu: VecDeque<[char;10]>,
    batallion_queueu: VecDeque<[char;10]>,
}

fn deserialize_char_array<'de, D>(deserializer: D) -> Result<[char; 10], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let mut array = ['\0'; 10];
    let chars: Vec<char> = s.chars().collect();
    array[..chars.len()].copy_from_slice(&chars);
    Ok(array)
}

fn join(player : Input, queues : Arc<Mutex<QueuesSys>>) -> String{
    match queues.lock() {
        Ok(mut guard) => {
            match player.lobby_type {
                LobbyType::Duel => guard.duel_queue.push_back(player.username),
                LobbyType::Group => guard.group_queueu.push_back(player.username),
                LobbyType::Battalion => guard.batallion_queueu.push_back(player.username),
                _ => return String::from("Error queue type")
            }
            String::from("added to the queue")},
        Err(_) => {String::from("Something went wrong")},
    }
}

async fn warp_function(queue :  Arc<Mutex<QueuesSys>>){
    let hello 
        = warp::path!("matchmaking" / "join")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |data: Input| join(data, queue.clone()));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 5555))
        .await;
}

fn frame(data : &mut Data){

}

fn handle_queue(data : &mut Data, queue : &mut VecDeque<[char;10]>, lobbyType : LobbyType){
    if data.free_lobbies.len() == 0 {return;}

    let lobby_index : usize = data.free_lobbies.pop_front().unwrap();
    for i in 0..lobbyType as usize{
        println!("i{}",i);
        let username = queue.pop_front().unwrap();
        data.lobbies[lobby_index].players.push(username);
    }
    data.lobbies[lobby_index].lobby_type = lobbyType as u16;
    println!("{:?}", data.lobbies[lobby_index]);
}

fn check_queue(queues :  Arc<Mutex<QueuesSys>>){
    let mut data : Data = Data{
        lobbies : array_init(|_| Lobby::default()),
        free_lobbies : (0..Settings::MaxLobbyNum as usize).collect(),
        num_of_active_lobbies : 0,
        num_of_active_players : 0,
        last_lobby : 0,
    };
    let tic_rate = Settings::TicRate as u64;
    let tic_time = Duration::from_millis(1000 / tic_rate);
    let mut last_frame = Instant::now();
    let mut tic_count = 0;
    loop {
        if tic_count >= tic_rate{
            match queues.lock() {
                Ok(mut guard) => {
                    println!("Duel:{},Group{},Battalion{}",guard.duel_queue.len(),guard.group_queueu.len(),guard.batallion_queueu.len());
                    if guard.duel_queue.len() >= 2 && data.num_of_active_players + 2 < Settings::MaxPlayerNum as u16{
                        handle_queue(&mut data, &mut guard.duel_queue, LobbyType::Duel);}
                    else if guard.group_queueu.len() >= 4 && data.num_of_active_players + 4 < Settings::MaxPlayerNum as u16{
                        handle_queue(&mut data, &mut guard.group_queueu, LobbyType::Group);}
                    else if guard.batallion_queueu.len() >= 10 && data.num_of_active_players + 10 < Settings::MaxPlayerNum as u16{
                        handle_queue(&mut data, &mut guard.batallion_queueu, LobbyType::Battalion);}
                },
                Err(_)=>{}
            }
            tic_count = 0;
        }
        tic_count += 1;
        frame(&mut data);
        let elapsed_frame = last_frame.elapsed();
        if elapsed_frame < tic_time {
            std::thread::sleep(tic_time - elapsed_frame);
        }else{
            println!("Load too heavy...");
        }
        last_frame = Instant::now();
    }
}

fn connections_function(){
    let tic_time = Duration::from_millis(1000);
    let mut last = Instant::now();
    loop {
        match queues.lock() {
            Ok(mut guard) => {
                println!("Duel:{},Group{},Battalion{}",guard.duel_queue.len(),guard.group_queueu.len(),guard.batallion_queueu.len());
                if guard.duel_queue.len() >= 2 && data.num_of_active_players + 2 < Settings::MaxPlayerNum as u16{
                    handle_queue(&mut data, &mut guard.duel_queue, LobbyType::Duel);}
                else if guard.group_queueu.len() >= 4 && data.num_of_active_players + 4 < Settings::MaxPlayerNum as u16{
                    handle_queue(&mut data, &mut guard.group_queueu, LobbyType::Group);}
                else if guard.batallion_queueu.len() >= 10 && data.num_of_active_players + 10 < Settings::MaxPlayerNum as u16{
                    handle_queue(&mut data, &mut guard.batallion_queueu, LobbyType::Battalion);}
            },
            Err(_)=>{}
        }
        let elapsed_frame = last.elapsed();
        if elapsed_frame < tic_time {
            std::thread::sleep(tic_time - elapsed_frame);
        }
    }
}

fn lauch(){
    let queues : Arc<Mutex<QueuesSys>> = 
        Arc::new(Mutex::new(QueuesSys {
            duel_queue: VecDeque::<[char;10]>::new(),
            group_queueu:VecDeque::<[char;10]>::new(),
            batallion_queueu: VecDeque::<[char;10]>::new()}));
    //let queue = Arc::new(Mutex::new(VecDeque::<[char;10]>::new()));
    let cloned_queue1 = queues.clone();
    let cloned_queue2 = queues.clone();
    let warp_handle 
    = thread::spawn(move || tokio::runtime::Runtime::new().unwrap().block_on(warp_function(cloned_queue1)));
    let queue_handle = thread::spawn( move || check_queue(cloned_queue2));
    let reqwest_handle = thread::spawn (move || )
    warp_handle.join().unwrap();
    queue_handle.join().unwrap();
}



fn main()  {
    lauch();
}

// curl  -X POST -H "Content-Type: application/json" -d "{\"username\": \"abcde\",\"lobby_type\":1}"  http://localhost:5555/matchmaking/join
// Invoke-RestMethod -Uri "http://localhost:5555/matchmaking/join" -Method Post -Headers @{"Content-Type"="application/json"} -Body '{"username": "papa", "lobby_type": "Duel"}'
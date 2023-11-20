use tokio::runtime::Handle;
#[allow(unused_imports)]

/*when somebody connects through http, they receive a key that is paired with their user, if somebody else tryes to connect
  with their username they key pair will be invalidated, restarting the proccess*/

use warp::filters::multipart;
use warp::reply::WithStatus;
use warp::{Filter, reply::with_status, reply::json, http::StatusCode, Reply};

use serde_json::Value;
use serde::Deserialize;

use core::fmt;
use std::alloc::System;
use std::os::windows::fs::OpenOptionsExt;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::{VecDeque,HashMap, HashSet};
use std::time::{Duration, Instant};


use array_init::array_init;

mod ServerSettings;

use ServerSettings::ServerSettings::Settings;

use std::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token, net};
use mio::net::{TcpListener as MioTcpListener, TcpStream as MioTcpStream};

const SERVER: Token = Token(0);

#[repr(u8)]
#[derive(Deserialize,Copy,Clone,PartialEq)]
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
            debug_struct.field(&format!("\nPlayer{}", index), name);
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
    lobby_type : LobbyType,
    players : Vec<[char;10]>
}

struct Data{
    lobbies : [Lobby; Settings::MaxLobbyNum as usize],
    free_lobbies : VecDeque<usize>,
    number_of_active_lobbies : u16,
}


struct Server {
    listener :  MioTcpListener,
    clients: HashMap<Token, MioTcpStream>,
    token_counter: usize,
}

impl Server {
    fn new(listener: MioTcpListener)-> Self{
        let mut server =  Self {
            listener: listener,
            clients: HashMap::new(),
            token_counter: 1,
        };
        return server;
    }
    fn accept_connection(&mut self, poll: &Poll) -> Option<Token>{
        let connection = self.listener.accept();
        if let Ok((mut stream ,_)) = self.listener.accept(){
            let token = Token(self.token_counter);
            self.token_counter += 1;
            self.clients.insert(token, stream);
            Some(token)
        } else {
            None
        }
    }
}

struct QueuesSys{
    connection_pending: HashMap::<[char;10], LobbyType>,
    duel_queue:  VecDeque<[char;10]>,
    group_queueu: VecDeque<[char;10]>,
    batallion_queueu: VecDeque<[char;10]>,
    num_of_active_lobbies : u16,
    num_of_active_players : u16,
    server: Server,
}

#[repr(u8)]
enum ClientState{
    Unidentifiend,//a token will be send back to the client with the http request, it will have the ip,port,and their key. Key and Username are paired
    Identified, //authorised will continue to queues
    Unauthorised,//username found, key missmatch
}

#[repr(u8)]
enum MessageType{
    MatchmakingConnectionInfo,
    LobbyCreationSuccessful,
    LobbyCreationError,
    LobbyCreationPending,
    LobbyCreationPendingNeedMorePlayers,
    LobbyCreationPendingNeedFreeLobby,
}

struct Message{
    operation: MessageType,
    message: String,
}

#[derive(serde::Serialize)]
struct Response {
    message: &'static str,
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

fn join(player : Input, queues : Arc<Mutex<QueuesSys>>) -> impl Reply{
    match queues.lock() {
        Ok(mut guard) => {
            guard.connection_pending.insert(player.username, player.lobby_type)
                .unwrap();
           /* match player.lobby_type {
                LobbyType::Duel => guard.duel_queue.push_back(player.username),
                LobbyType::Group => guard.group_queueu.push_back(player.username),
                LobbyType::Battalion => guard.batallion_queueu.push_back(player.username),
                _ => {
                    let response = Response{
                        message : "LobbyType invalid",
                    };
                    let json_response = json(&response);
                    return warp::reply::with_status(json_response, StatusCode::INTERNAL_SERVER_ERROR);
                }
            }*/
            //String::from("added to the queue");
            let response = Response{
                message : "{\"ConnectKey\":\"SECRET_KEY\"}",
            };
            let json_response = json(&response);
            warp::reply::with_status(json_response, StatusCode::OK)
        },
        Err(_) => {
            let response = Response{
                message : "Server Error, tray again"
            };
            let json_response = json(&response);
            warp::reply::with_status(json_response, StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn warp_manager(queue :  Arc<Mutex<QueuesSys>>){
    let hello = warp::path!("matchmaking" / "join")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |data: Input| join(data, queue.clone()));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 5555))
        .await;
}

fn frame(data : Arc<Mutex<Data>>){
    match data.lock() {
        Ok(mut guarded_data) => {
            if guarded_data.number_of_active_lobbies == 0 {
                return ;
            }
            let mut num_of_lobbies_framed : u16 = 0;
            let number_of_active_lobbies = guarded_data.number_of_active_lobbies;
            for lobby in guarded_data.lobbies.iter_mut() {
                if lobby.lobby_type != LobbyType::Uninitialized{
                    num_of_lobbies_framed += 1;
                }
                if num_of_lobbies_framed == number_of_active_lobbies{
                    break ;
                }
            }
        },
        Err(_) => {}
    }
}

fn logic_manager(data :  Arc<Mutex<Data>>){
    let tic_rate = Settings::TicRate as u64;
    let tic_time = Duration::from_millis(1000 / tic_rate);
    let mut last_frame = Instant::now();
    loop {
        frame(data.clone());
        let elapsed_frame = last_frame.elapsed();
        if elapsed_frame < tic_time {
            std::thread::sleep(tic_time - elapsed_frame);
        }else{
            println!("Load too heavy...");
        }
        last_frame = Instant::now();
    }
}

fn queues_manager(data : Arc<Mutex<Data>>, queues : Arc<Mutex<QueuesSys>>){
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(128);
    match  queues.lock(){
        Ok(mut guarded_queues) => {poll.registry()
            .register(&mut guarded_queues.server.listener, SERVER, Interest::READABLE).unwrap();},
        Err(_) => { panic!("No register");}
    }

    let tic_time = Duration::from_millis(1000);
    let mut last = Instant::now();
    loop {
        match queues.lock() {
            Ok(mut guarded_queues) => {
                println!("Duel:{},Group{},Battalion{}",guarded_queues.duel_queue.len(),guarded_queues.group_queueu.len(),guarded_queues.batallion_queueu.len());

                if guarded_queues.num_of_active_lobbies < Settings::MaxLobbyNum as u16 && guarded_queues.num_of_active_players + 2 < Settings::MaxPlayerNum as u16 && guarded_queues.duel_queue.len() >= 2 {
                    if handle_queue(data.clone(), &mut guarded_queues.duel_queue, LobbyType::Duel){
                        guarded_queues.num_of_active_players += 2;
                        guarded_queues.num_of_active_lobbies += 1;
                    }
                }
                else if guarded_queues.num_of_active_lobbies < Settings::MaxLobbyNum as u16 && guarded_queues.num_of_active_players + 4 < Settings::MaxPlayerNum as u16 && guarded_queues.group_queueu.len() >= 4{
                    if handle_queue(data.clone(), &mut guarded_queues.group_queueu, LobbyType::Group){
                        guarded_queues.num_of_active_players += 4;
                        guarded_queues.num_of_active_lobbies += 1;
                    }
                }
                else if guarded_queues.num_of_active_lobbies < Settings::MaxLobbyNum as u16 && guarded_queues.batallion_queueu.len() >= 10 && guarded_queues.num_of_active_players + 10 < Settings::MaxPlayerNum as u16{
                    if handle_queue(data.clone(), &mut guarded_queues.batallion_queueu, LobbyType::Battalion){
                        guarded_queues.num_of_active_players += 10;
                        guarded_queues.num_of_active_lobbies += 1;
                    }
                }
                poll.poll(&mut events, Some(Duration::from_millis(50))).unwrap();
                for event in events.iter(){
                    match event.token() {
                        SERVER => {
                            if let Some(token) =  guarded_queues.server.accept_connection(&poll){

                            }
                        },
                        other => {
                            
                        } 
                    }
                }
            },
            Err(_)=>{}
        }

        let elapsed_frame = last.elapsed();
        if elapsed_frame < tic_time {
            std::thread::sleep(tic_time - elapsed_frame);
        }
        last = Instant::now();
    }
}

fn handle_queue(data : Arc<Mutex<Data>>, queue : &mut VecDeque<[char;10]>, lobbyType : LobbyType) -> bool{
    match data.lock(){
        Ok (mut guarded_data) => {
            if guarded_data.free_lobbies.len() == 0 {return false}

            let lobby_index : usize = guarded_data.free_lobbies.pop_front().unwrap();
            for i in 0..lobbyType as usize{
                println!("i{}",i);
                let username = queue.pop_front().unwrap();
                guarded_data.lobbies[lobby_index].players.push(username);
            }
            guarded_data.lobbies[lobby_index].lobby_type = lobbyType;
            println!("{:?}", guarded_data.lobbies[lobby_index]);
            true
        },
        Err (_) => {false}
    }    
}

fn lauch(){
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(128);
    let addr = "127.0.0.1:5556".parse().unwrap();
    let listener = MioTcpListener::bind(addr).unwrap();
    let queues : Arc<Mutex<QueuesSys>> = 
        Arc::new(Mutex::new(QueuesSys {
            connection_pending : HashMap::<[char;10], LobbyType>::new(),
            duel_queue: VecDeque::<[char;10]>::new(),
            group_queueu:VecDeque::<[char;10]>::new(),
            batallion_queueu: VecDeque::<[char;10]>::new(),
            num_of_active_lobbies : 0,
            num_of_active_players : 0,
            server : Server::new(listener)
        }
    ));

    let data : Arc<Mutex<Data>> = 
        Arc::new(Mutex::new(Data{
            lobbies : array_init(|_| Lobby::default()),
            free_lobbies : (0..Settings::MaxLobbyNum as usize).collect(),
            number_of_active_lobbies : 0,

        } 
    ));

    let cloned_queue1 = queues.clone();
    let cloned_queue2 = queues.clone();
    let cloned_data1 = data.clone();
    let cloned_data2 = data.clone();

    let warp_handle = thread::spawn(move || tokio::runtime::Runtime::new().unwrap().block_on(warp_manager(cloned_queue1)));
    let queue_handle = thread::spawn( move || queues_manager(cloned_data1, cloned_queue2));
    let logic_handle = thread::spawn (move || logic_manager(cloned_data2));
    /*have a cli to control the app through the main thread, make it close correctly, hadle signals like control-C etc */
    warp_handle.join().unwrap();
    queue_handle.join().unwrap();
    logic_handle.join().unwrap();
}

fn main()  {
    lauch();
}

// curl  -X POST -H "Content-Type: application/json" -d "{\"username\": \"abcde\",\"lobby_type\":1}"  http://localhost:5555/matchmaking/join
// Invoke-RestMethod -Uri "http://localhost:5555/matchmaking/join" -Method Post -Headers @{"Content-Type"="application/json"} -Body '{"username": "papa", "lobby_type": "Duel"}'
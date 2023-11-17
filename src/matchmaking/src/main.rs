#[allow(unused_imports)]

use warp::Filter;
use warp::filters::multipart;
use warp::reply::reply;

use std::convert::Infallible;
use std::thread::sleep;
use std::{thread, clone};
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::collections::{VecDeque};
use std::time::{Duration, Instant};


use serde::Deserialize;

use array_init::array_init;

#[repr(u8)]
#[derive(Deserialize,Copy,Clone)]
enum LobbyType{
    Uninitialized = 0,
    Duel = 0b0001,
    Group = 0b0010,
    Battalion = 0b0100,
}
impl Default for LobbyType {
    fn default() -> Self {
        LobbyType::Uninitialized
    }
}


#[derive(Deserialize)]
struct Input {
    #[serde(deserialize_with = "deserialize_char_array")]
    username: [char;10],
    lobby_type : u8,
}

#[derive(Clone, Default)]
struct Lobby{
    lobby_type : LobbyType,
    players : Vec<[char;10]>
}

struct Data{
    lobbies : [Lobby;100],
    num_of_active_lobbies : u8,
    last_lobby : u8,//like a custom memory manager use the lobby_type u8 same size
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

fn join(data : Input, queue : Arc<Mutex<VecDeque<[char;10]>>>) -> String{
    match queue.lock() {
        Ok(mut queue) => {
            queue.push_back(data.username);
            String::from("added to the queue")},
        Err(_) => {String::from("Something went wrong")},
    }
}

async fn warp_function(queue : Arc<Mutex<VecDeque<[char;10]>>>){
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

fn check_queue(queue : Arc<Mutex<VecDeque<[char;10]>>>){
    let mut data : Data = Data{
        lobbies : array_init(|_| Lobby::default()),
        num_of_active_lobbies : 0,
        last_lobby : 0,
    };
    let tic_rate = 30;
    let tic_time = Duration::from_millis(1000 / tic_rate);
    let mut last_frame = Instant::now();
    let mut tic_count = 0;
    loop {
        let elapsed_frame = last_frame.elapsed();
        if tic_count >= tic_rate{
            match queue.lock() {
                Ok(mut guard) => {
                    match guard.len() {
                        0 => {println!("No players in the queue.");},
                        1 => {println!("One player waiting in the queue");},
                        n if n >= 2 => {
                            println!("Players {} and {} will battle now", String::from_iter(guard.pop_front().unwrap().iter()), String::from_iter(guard.pop_front().unwrap().iter()));
                            let lobby = &mut data.lobbies[usize::from(data.last_lobby)];
                            println!("{} players remaining",n - 2);
                        },
                        _ => {println!("Something went wrong")}
                    }
                },
                Err(_)=>{}
            }
            tic_count = 0;
        }
        tic_count += 1;
        frame(&mut data);
        if elapsed_frame < tic_time {
            std::thread::sleep(tic_time - elapsed_frame);
        }
        last_frame = Instant::now();
    }
}

fn lauch(){
    let queue = Arc::new(Mutex::new(VecDeque::<[char;10]>::new()));
    let cloned_queue1 = queue.clone();
    let cloned_queue2 = queue.clone();
    let warp_handle 
    = thread::spawn(move || tokio::runtime::Runtime::new().unwrap().block_on(warp_function(cloned_queue1)));
    let queue_handle = thread::spawn( move || check_queue(cloned_queue2));
    warp_handle.join().unwrap();
    queue_handle.join().unwrap();
}



fn main()  {
    lauch();
}

// curl  -X POST -H "Content-Type: application/json" -d "{\"username\": \"abcde\",\"lobby_type\":1}"  http://localhost:5555/matchmaking/join
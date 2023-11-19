
pub mod ServerSettings{
    pub enum Settings{
        MaxPlayerNum = 200,
        MaxLobbyNum = 100,
        TicRate = 30,
    }
    pub const BACKEND_URL : &'static str = "http://localhost:5000";
}
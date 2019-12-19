use std::error::Error;
use power4::game_config::Config;
use power4::game_master::GameMaster;

fn main() -> Result <(), Box<dyn Error>>{
    let config = Config::run()?;
    GameMaster::run(config)
}

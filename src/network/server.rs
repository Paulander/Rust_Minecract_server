use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use log::{info, error};

use crate::world::World;
use crate::player::Player;
use super::protocol::handle_connection;

pub struct MinecraftServer {
    bind_address: String,
    world: Arc<Mutex<World>>,
    players: Arc<Mutex<Vec<Player>>>,
}

impl MinecraftServer {
    pub fn new(bind_address: String, world: Arc<Mutex<World>>) -> Self {
        Self {
            bind_address,
            world,
            players: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.bind_address).await?;
        info!("Server listening on {}", self.bind_address);

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    info!("New connection from: {}", addr);
                    let world = Arc::clone(&self.world);
                    let players = Arc::clone(&self.players);
                    
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(socket, world, players).await {
                            error!("Error handling connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }
} 
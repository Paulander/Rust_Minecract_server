use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{info, error, debug};

use crate::world::World;
use crate::player::Player;
use super::packet::{Packet, PacketReader, PacketWriter};

pub async fn handle_connection(
    mut socket: TcpStream,
    world: Arc<Mutex<World>>,
    players: Arc<Mutex<Vec<Player>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = vec![0u8; 4096];
    let mut reader = PacketReader::new();
    let mut writer = PacketWriter::new();

    // Handshaking and login sequence
    while let Ok(n) = socket.read(&mut buffer).await {
        if n == 0 {
            return Ok(());
        }
        
        reader.push(&buffer[..n]);
        
        while let Some(packet) = reader.next_packet()? {
            debug!("Received packet: {:?}", packet);
            
            match process_packet(packet, &world, &players).await? {
                Some(response) => {
                    let data = writer.write_packet(response)?;
                    socket.write_all(&data).await?;
                },
                None => {}
            }
        }
    }
    
    Ok(())
}

async fn process_packet(
    packet: Packet,
    world: &Arc<Mutex<World>>,
    players: &Arc<Mutex<Vec<Player>>>,
) -> Result<Option<Packet>, Box<dyn std::error::Error>> {
    // In a real implementation, you would handle the packet based on its type
    // For now, we'll just log it and return a dummy response
    info!("Processing packet: {:?}", packet);
    
    // Dummy implementation - return a simple response packet
    Ok(Some(Packet::new(0, vec![0, 1, 2, 3])))
} 
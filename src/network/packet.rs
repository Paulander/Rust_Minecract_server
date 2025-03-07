use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: u8,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn new(id: u8, data: Vec<u8>) -> Self {
        Self { id, data }
    }
}

pub struct PacketReader {
    buffer: Vec<u8>,
}

impl PacketReader {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn push(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn next_packet(&mut self) -> Result<Option<Packet>, Box<dyn std::error::Error>> {
        if self.buffer.is_empty() {
            return Ok(None);
        }

        // A real implementation would read VarInt length and packet ID
        // This is simplified for the example
        let mut cursor = Cursor::new(&self.buffer);
        let length = match cursor.read_u32::<BigEndian>() {
            Ok(len) => len as usize,
            Err(_) => return Ok(None),  // Not enough data
        };

        let header_size = 4;  // 4 bytes for length
        let total_size = header_size + length;

        if self.buffer.len() < total_size {
            return Ok(None);  // Not enough data
        }

        let packet_id = self.buffer[header_size];
        let packet_data = self.buffer[header_size+1..total_size].to_vec();
        
        // Remove the processed packet from the buffer
        self.buffer.drain(0..total_size);

        Ok(Some(Packet::new(packet_id, packet_data)))
    }
}

pub struct PacketWriter {
    buffer: Vec<u8>,
}

impl PacketWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn write_packet(&mut self, packet: Packet) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.buffer.clear();
        
        // Write packet length
        let length = 1 + packet.data.len() as u32;  // 1 byte for packet ID
        self.buffer.write_u32::<BigEndian>(length)?;
        
        // Write packet ID
        self.buffer.push(packet.id);
        
        // Write packet data
        self.buffer.extend_from_slice(&packet.data);
        
        Ok(self.buffer.clone())
    }
} 
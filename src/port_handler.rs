use serialport::SerialPort;
use std::time::{Duration, Instant};
use crate::values::*;

pub struct PortHandler {
    port: Option<Box<dyn SerialPort>>,
    port_name: String,
    baudrate: u32,
    packet_start_time: Instant,
    packet_timeout: Duration,
    tx_time_per_byte: f64,
    pub is_using: bool,
}

impl PortHandler {
    pub fn new(port_name: &str) -> Self {
        Self {
            port: None,
            port_name: port_name.to_string(),
            baudrate: DEFAULT_BAUDRATE,
            packet_start_time: Instant::now(),
            packet_timeout: Duration::from_millis(0),
            tx_time_per_byte: 0.0,
            is_using: false,
        }
    }

    pub fn open_port(&mut self) -> Result<(), String> {
        self.setup_port()
    }

    pub fn close_port(&mut self) {
        self.port = None;
    }

    pub fn clear_port(&mut self) -> Result<(), String> {
        if let Some(ref mut port) = self.port {
            port.flush().map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn get_bytes_available(&mut self) -> Result<u32, String> {
        if let Some(ref mut port) = self.port {
            port.bytes_to_read().map_err(|e| e.to_string())
        } else {
            Ok(0)
        }
    }

    pub fn read_port(&mut self, length: usize) -> Result<Vec<u8>, String> {
        if let Some(ref mut port) = self.port {
            let mut buffer = vec![0u8; length];
            let bytes_read = port.read(&mut buffer).map_err(|e| e.to_string())?;
            buffer.truncate(bytes_read);
            Ok(buffer)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn write_port(&mut self, packet: &[u8]) -> Result<usize, String> {
        if let Some(ref mut port) = self.port {
            port.write(packet).map_err(|e| e.to_string())
        } else {
            Err("Port not open".to_string())
        }
    }

    pub fn set_packet_timeout(&mut self, packet_length: usize) {
        self.packet_start_time = Instant::now();
        let timeout_ms = (self.tx_time_per_byte * packet_length as f64)
            + (self.tx_time_per_byte * 3.0)
            + LATENCY_TIMER;
        self.packet_timeout = Duration::from_millis(timeout_ms as u64);
    }

    pub fn set_packet_timeout_millis(&mut self, msec: u64) {
        self.packet_start_time = Instant::now();
        self.packet_timeout = Duration::from_millis(msec);
    }

    pub fn is_packet_timeout(&mut self) -> bool {
        if self.get_time_since_start() > self.packet_timeout {
            self.packet_timeout = Duration::from_millis(0);
            true
        } else {
            false
        }
    }

    fn get_time_since_start(&self) -> Duration {
        Instant::now().duration_since(self.packet_start_time)
    }

    fn setup_port(&mut self) -> Result<(), String> {
        let port = serialport::new(&self.port_name, self.baudrate)
            .timeout(Duration::from_millis(0))
            .open()
            .map_err(|e| format!("Could not open port {}: {}", self.port_name, e))?;

        self.port = Some(port);

        // Clear input buffer
        if let Some(ref mut p) = self.port {
            let _ = p.clear(serialport::ClearBuffer::Input);
        }

        self.tx_time_per_byte = (1000.0 / self.baudrate as f64) * 10.0;

        Ok(())
    }
}

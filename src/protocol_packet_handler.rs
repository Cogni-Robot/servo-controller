use crate::port_handler::PortHandler;
use crate::values::*;

pub struct ProtocolPacketHandler<'a> {
    port_handler: &'a mut PortHandler,
    sts_end: u8,
}

impl<'a> ProtocolPacketHandler<'a> {
    pub fn new(port_handler: &'a mut PortHandler) -> Self {
        Self {
            port_handler,
            sts_end: 0,
        }
    }

    // Fonctions utilitaires de manipulation de bytes
    pub fn sts_makeword(&self, a: u8, b: u8) -> u16 {
        if self.sts_end == 0 {
            (a as u16) | ((b as u16) << 8)
        } else {
            (b as u16) | ((a as u16) << 8)
        }
    }

    pub fn sts_makedword(&self, a: u16, b: u16) -> u32 {
        (a as u32) | ((b as u32) << 16)
    }

    pub fn sts_lobyte(&self, w: u16) -> u8 {
        if self.sts_end == 0 {
            (w & 0xFF) as u8
        } else {
            ((w >> 8) & 0xFF) as u8
        }
    }

    pub fn sts_hibyte(&self, w: u16) -> u8 {
        if self.sts_end == 0 {
            ((w >> 8) & 0xFF) as u8
        } else {
            (w & 0xFF) as u8
        }
    }

    pub fn sts_loword(&self, l: u32) -> u16 {
        (l & 0xFFFF) as u16
    }

    pub fn sts_hiword(&self, h: u32) -> u16 {
        ((h >> 16) & 0xFFFF) as u16
    }

    pub fn sts_tohost(&self, a: u16, b: u8) -> i16 {
        if (a & (1 << b)) != 0 {
            -((a & !(1 << b)) as i16)
        } else {
            a as i16
        }
    }

    // Transmission de paquet
    pub fn tx_packet(&mut self, txpacket: &mut Vec<u8>) -> CommResult {
        let total_packet_length = txpacket[PKT_LENGTH] as usize + 4;

        if self.port_handler.is_using {
            return CommResult::PortBusy;
        }
        self.port_handler.is_using = true;

        if total_packet_length > TXPACKET_MAX_LEN {
            self.port_handler.is_using = false;
            return CommResult::TxError;
        }

        // En-tête de paquet
        txpacket[PKT_HEADER_0] = 0xFF;
        txpacket[PKT_HEADER_1] = 0xFF;

        // Calcul de la somme de contrôle
        let mut checksum: u8 = 0;
        for i in 2..total_packet_length - 1 {
            checksum = checksum.wrapping_add(txpacket[i]);
        }
        txpacket[total_packet_length - 1] = !checksum;

        // Envoi du paquet
        let _ = self.port_handler.clear_port();
        match self.port_handler.write_port(&txpacket[..total_packet_length]) {
            Ok(written) if written == total_packet_length => CommResult::Success,
            _ => {
                self.port_handler.is_using = false;
                CommResult::TxFail
            }
        }
    }

    // Réception de paquet
    pub fn rx_packet(&mut self) -> (Vec<u8>, CommResult) {
        let mut rxpacket = Vec::new();
        let mut wait_length = 6;

        loop {
            match self.port_handler.read_port(wait_length - rxpacket.len()) {
                Ok(mut data) => rxpacket.append(&mut data),
                Err(_) => break,
            }

            let rx_length = rxpacket.len();
            if rx_length >= wait_length {
                // Recherche de l'en-tête du paquet
                let mut idx = 0;
                for i in 0..rx_length - 1 {
                    if rxpacket[i] == 0xFF && rxpacket[i + 1] == 0xFF {
                        idx = i;
                        break;
                    }
                }

                if idx == 0 {
                    if rx_length >= PKT_LENGTH + 1 {
                        let id = rxpacket[PKT_ID];
                        let length = rxpacket[PKT_LENGTH];
                        let error = rxpacket[PKT_ERROR];

                        if id > 0xFD || length as usize > RXPACKET_MAX_LEN || error > 0x7F {
                            rxpacket.remove(0);
                            continue;
                        }

                        let new_wait_length = length as usize + PKT_LENGTH + 1;
                        if wait_length != new_wait_length {
                            wait_length = new_wait_length;
                            continue;
                        }

                        if rx_length < wait_length {
                            if self.port_handler.is_packet_timeout() {
                                self.port_handler.is_using = false;
                                return (
                                    rxpacket,
                                    if rx_length == 0 {
                                        CommResult::RxTimeout
                                    } else {
                                        CommResult::RxCorrupt
                                    },
                                );
                            }
                            continue;
                        }

                        // Vérification de la somme de contrôle
                        let mut checksum: u8 = 0;
                        for i in 2..wait_length - 1 {
                            checksum = checksum.wrapping_add(rxpacket[i]);
                        }
                        checksum = !checksum;

                        self.port_handler.is_using = false;
                        if rxpacket[wait_length - 1] == checksum {
                            return (rxpacket, CommResult::Success);
                        } else {
                            return (rxpacket, CommResult::RxCorrupt);
                        }
                    }
                } else {
                    rxpacket.drain(0..idx);
                }
            } else {
                if self.port_handler.is_packet_timeout() {
                    self.port_handler.is_using = false;
                    return (
                        rxpacket,
                        if rx_length == 0 {
                            CommResult::RxTimeout
                        } else {
                            CommResult::RxCorrupt
                        },
                    );
                }
            }
        }

        self.port_handler.is_using = false;
        (rxpacket, CommResult::RxFail)
    }

    // Transmission et réception
    pub fn tx_rx_packet(&mut self, txpacket: &mut Vec<u8>) -> (Option<Vec<u8>>, CommResult, u8) {
        let result = self.tx_packet(txpacket);
        if !result.is_success() {
            return (None, result, 0);
        }

        if txpacket[PKT_ID] == BROADCAST_ID {
            self.port_handler.is_using = false;
            return (None, result, 0);
        }

        // Définition du délai d'attente
        if txpacket[PKT_INSTRUCTION] == INST_READ {
            self.port_handler
                .set_packet_timeout(txpacket[PKT_PARAMETER0 + 1] as usize + 6);
        } else {
            self.port_handler.set_packet_timeout(6);
        }

        loop {
            let (rxpacket, rx_result) = self.rx_packet();
            if !rx_result.is_success() || rxpacket.get(PKT_ID) == Some(&txpacket[PKT_ID]) {
                let error = if rx_result.is_success() && !rxpacket.is_empty() {
                    rxpacket[PKT_ERROR]
                } else {
                    0
                };
                return (Some(rxpacket), rx_result, error);
            }
        }
    }

    // Ping
    pub fn ping(&mut self, sts_id: u8) -> (u16, CommResult, u8) {
        if sts_id >= BROADCAST_ID {
            return (0, CommResult::NotAvailable, 0);
        }

        let mut txpacket = vec![0u8; 6];
        txpacket[PKT_ID] = sts_id;
        txpacket[PKT_LENGTH] = 2;
        txpacket[PKT_INSTRUCTION] = INST_PING;

        let (_rxpacket, result, error) = self.tx_rx_packet(&mut txpacket);

        if result.is_success() {
            let (data, read_result, read_error) = self.read_tx_rx(sts_id, 3, 2);
            if read_result.is_success() && data.len() >= 2 {
                let model_number = self.sts_makeword(data[0], data[1]);
                return (model_number, read_result, read_error);
            }
        }

        (0, result, error)
    }

    // Lecture
    pub fn read_tx_rx(&mut self, sts_id: u8, address: u8, length: u8) -> (Vec<u8>, CommResult, u8) {
        if sts_id >= BROADCAST_ID {
            return (Vec::new(), CommResult::NotAvailable, 0);
        }

        let mut txpacket = vec![0u8; 8];
        txpacket[PKT_ID] = sts_id;
        txpacket[PKT_LENGTH] = 4;
        txpacket[PKT_INSTRUCTION] = INST_READ;
        txpacket[PKT_PARAMETER0] = address;
        txpacket[PKT_PARAMETER0 + 1] = length;

        let (rxpacket, result, error) = self.tx_rx_packet(&mut txpacket);

        if result.is_success() {
            if let Some(packet) = rxpacket {
                let start = PKT_PARAMETER0;
                let end = PKT_PARAMETER0 + length as usize;
                if packet.len() >= end {
                    return (packet[start..end].to_vec(), result, error);
                }
            }
        }

        (Vec::new(), result, error)
    }

    pub fn read_1byte_tx_rx(&mut self, sts_id: u8, address: u8) -> (u8, CommResult, u8) {
        let (data, result, error) = self.read_tx_rx(sts_id, address, 1);
        let data_read = if result.is_success() && !data.is_empty() {
            data[0]
        } else {
            0
        };
        (data_read, result, error)
    }

    pub fn read_2byte_tx_rx(&mut self, sts_id: u8, address: u8) -> (u16, CommResult, u8) {
        let (data, result, error) = self.read_tx_rx(sts_id, address, 2);
        let data_read = if result.is_success() && data.len() >= 2 {
            self.sts_makeword(data[0], data[1])
        } else {
            0
        };
        (data_read, result, error)
    }

    // Écriture
    pub fn write_tx_rx(
        &mut self,
        sts_id: u8,
        address: u8,
        data: &[u8],
    ) -> (CommResult, u8) {
        let length = data.len();
        let mut txpacket = vec![0u8; length + 7];

        txpacket[PKT_ID] = sts_id;
        txpacket[PKT_LENGTH] = (length + 3) as u8;
        txpacket[PKT_INSTRUCTION] = INST_WRITE;
        txpacket[PKT_PARAMETER0] = address;

        txpacket[PKT_PARAMETER0 + 1..PKT_PARAMETER0 + 1 + length].copy_from_slice(data);

        let (_, result, error) = self.tx_rx_packet(&mut txpacket);
        (result, error)
    }

    pub fn write_tx_only(&mut self, sts_id: u8, address: u8, data: &[u8]) -> CommResult {
        let length = data.len();
        let mut txpacket = vec![0u8; length + 7];

        txpacket[PKT_ID] = sts_id;
        txpacket[PKT_LENGTH] = (length + 3) as u8;
        txpacket[PKT_INSTRUCTION] = INST_WRITE;
        txpacket[PKT_PARAMETER0] = address;

        txpacket[PKT_PARAMETER0 + 1..PKT_PARAMETER0 + 1 + length].copy_from_slice(data);

        let result = self.tx_packet(&mut txpacket);
        self.port_handler.is_using = false;
        result
    }

    pub fn write_1byte_tx_only(&mut self, sts_id: u8, address: u8, data: u8) -> CommResult {
        self.write_tx_only(sts_id, address, &[data])
    }

    pub fn write_1byte_tx_rx(&mut self, sts_id: u8, address: u8, data: u8) -> (CommResult, u8) {
        self.write_tx_rx(sts_id, address, &[data])
    }

    pub fn write_2byte_tx_only(&mut self, sts_id: u8, address: u8, data: u16) -> CommResult {
        let bytes = [self.sts_lobyte(data), self.sts_hibyte(data)];
        self.write_tx_only(sts_id, address, &bytes)
    }

    pub fn write_2byte_tx_rx(&mut self, sts_id: u8, address: u8, data: u16) -> (CommResult, u8) {
        let bytes = [self.sts_lobyte(data), self.sts_hibyte(data)];
        self.write_tx_rx(sts_id, address, &bytes)
    }

    // Sync Write
    pub fn sync_write_tx_only(
        &mut self,
        start_address: u8,
        data_length: u8,
        param: &[u8],
    ) -> CommResult {
        let param_length = param.len();
        let mut txpacket = vec![0u8; param_length + 8];

        txpacket[PKT_ID] = BROADCAST_ID;
        txpacket[PKT_LENGTH] = (param_length + 4) as u8;
        txpacket[PKT_INSTRUCTION] = INST_SYNC_WRITE;
        txpacket[PKT_PARAMETER0] = start_address;
        txpacket[PKT_PARAMETER0 + 1] = data_length;

        txpacket[PKT_PARAMETER0 + 2..PKT_PARAMETER0 + 2 + param_length].copy_from_slice(param);

        let (_, result, _) = self.tx_rx_packet(&mut txpacket);
        result
    }

    // Sync Read
    pub fn sync_read_tx(
        &mut self,
        start_address: u8,
        data_length: u8,
        param: &[u8],
    ) -> CommResult {
        let param_length = param.len();
        let mut txpacket = vec![0u8; param_length + 8];

        txpacket[PKT_ID] = BROADCAST_ID;
        txpacket[PKT_LENGTH] = (param_length + 4) as u8;
        txpacket[PKT_INSTRUCTION] = INST_SYNC_READ;
        txpacket[PKT_PARAMETER0] = start_address;
        txpacket[PKT_PARAMETER0 + 1] = data_length;

        txpacket[PKT_PARAMETER0 + 2..PKT_PARAMETER0 + 2 + param_length].copy_from_slice(param);

        self.tx_packet(&mut txpacket)
    }

    pub fn sync_read_rx(&mut self, data_length: usize, param_length: usize) -> (CommResult, Vec<u8>) {
        let wait_length = (6 + data_length) * param_length;
        self.port_handler.set_packet_timeout(wait_length);
        
        let mut rxpacket = Vec::new();

        loop {
            match self.port_handler.read_port(wait_length - rxpacket.len()) {
                Ok(mut data) => rxpacket.append(&mut data),
                Err(_) => break,
            }

            let rx_length = rxpacket.len();
            if rx_length >= wait_length {
                self.port_handler.is_using = false;
                return (CommResult::Success, rxpacket);
            } else {
                if self.port_handler.is_packet_timeout() {
                    self.port_handler.is_using = false;
                    return (
                        if rx_length == 0 {
                            CommResult::RxTimeout
                        } else {
                            CommResult::RxCorrupt
                        },
                        rxpacket,
                    );
                }
            }
        }

        self.port_handler.is_using = false;
        (CommResult::RxFail, rxpacket)
    }
}

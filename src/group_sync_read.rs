use crate::protocol_packet_handler::ProtocolPacketHandler;
use crate::values::*;
use std::collections::HashMap;

pub struct GroupSyncRead {
    start_address: u8,
    data_length: usize,
    last_result: bool,
    is_param_changed: bool,
    param: Vec<u8>,
    data_dict: HashMap<u8, Vec<u8>>,
}

impl GroupSyncRead {
    pub fn new(start_address: u8, data_length: usize) -> Self {
        Self {
            start_address,
            data_length,
            last_result: false,
            is_param_changed: false,
            param: Vec::new(),
            data_dict: HashMap::new(),
        }
    }

    fn make_param(&mut self) {
        if self.data_dict.is_empty() {
            return;
        }

        self.param.clear();
        for scs_id in self.data_dict.keys() {
            self.param.push(*scs_id);
        }
    }

    pub fn add_param(&mut self, sts_id: u8) -> bool {
        if self.data_dict.contains_key(&sts_id) {
            return false;
        }

        self.data_dict.insert(sts_id, Vec::new());
        self.is_param_changed = true;
        true
    }

    pub fn remove_param(&mut self, sts_id: u8) {
        if self.data_dict.remove(&sts_id).is_some() {
            self.is_param_changed = true;
        }
    }

    pub fn clear_param(&mut self) {
        self.data_dict.clear();
    }

    pub fn tx_packet(&mut self, ph: &mut ProtocolPacketHandler) -> CommResult {
        if self.data_dict.is_empty() {
            return CommResult::NotAvailable;
        }

        if self.is_param_changed || self.param.is_empty() {
            self.make_param();
        }

        ph.sync_read_tx(self.start_address, self.data_length as u8, &self.param)
    }

    pub fn rx_packet(&mut self, ph: &mut ProtocolPacketHandler) -> CommResult {
        self.last_result = true;

        if self.data_dict.is_empty() {
            return CommResult::NotAvailable;
        }

        let (result, rxpacket) = ph.sync_read_rx(self.data_length, self.data_dict.len());

        if rxpacket.len() >= (self.data_length + 6) {
            for sts_id in self.data_dict.keys().copied().collect::<Vec<_>>() {
                match self.read_rx(&rxpacket, sts_id, self.data_length) {
                    (Some(data), CommResult::Success) => {
                        self.data_dict.insert(sts_id, data);
                    }
                    _ => {
                        self.last_result = false;
                    }
                }
            }
        } else {
            self.last_result = false;
        }

        result
    }

    pub fn tx_rx_packet(&mut self, ph: &mut ProtocolPacketHandler) -> CommResult {
        let result = self.tx_packet(ph);
        if !result.is_success() {
            return result;
        }
        self.rx_packet(ph)
    }

    fn read_rx(&self, rxpacket: &[u8], sts_id: u8, data_length: usize) -> (Option<Vec<u8>>, CommResult) {
        let rx_length = rxpacket.len();
        let mut rx_index = 0;

        while (rx_index + 6 + data_length) <= rx_length {
            let mut headpacket = [0u8; 3];
            
            while rx_index < rx_length {
                headpacket[2] = headpacket[1];
                headpacket[1] = headpacket[0];
                headpacket[0] = rxpacket[rx_index];
                rx_index += 1;
                
                if headpacket[2] == 0xFF && headpacket[1] == 0xFF && headpacket[0] == sts_id {
                    break;
                }
            }

            if (rx_index + 3 + data_length) > rx_length {
                break;
            }

            if rxpacket[rx_index] != (data_length + 2) as u8 {
                rx_index += 1;
                continue;
            }
            rx_index += 1;

            let error = rxpacket[rx_index];
            rx_index += 1;

            let mut cal_sum = sts_id.wrapping_add((data_length + 2) as u8).wrapping_add(error);
            let mut data = vec![error];
            
            for i in 0..data_length {
                if rx_index + i >= rx_length {
                    return (None, CommResult::RxCorrupt);
                }
                data.push(rxpacket[rx_index + i]);
                cal_sum = cal_sum.wrapping_add(rxpacket[rx_index + i]);
            }
            rx_index += data_length;

            cal_sum = !cal_sum;

            if rx_index >= rx_length {
                return (None, CommResult::RxCorrupt);
            }

            if cal_sum != rxpacket[rx_index] {
                return (None, CommResult::RxCorrupt);
            }

            return (Some(data), CommResult::Success);
        }

        (None, CommResult::RxCorrupt)
    }

    pub fn is_available(&self, sts_id: u8, address: u8, data_length: usize) -> (bool, u8) {
        if !self.data_dict.contains_key(&sts_id) {
            return (false, 0);
        }

        if address < self.start_address
            || self.start_address + self.data_length as u8 - (data_length as u8) < address
        {
            return (false, 0);
        }

        if let Some(data) = self.data_dict.get(&sts_id) {
            if data.is_empty() || data.len() < (data_length + 1) {
                return (false, 0);
            }
            return (true, data[0]);
        }

        (false, 0)
    }

    pub fn get_data(&self, ph: &ProtocolPacketHandler, sts_id: u8, address: u8, data_length: usize) -> u32 {
        if let Some(data) = self.data_dict.get(&sts_id) {
            let offset = (address - self.start_address) as usize + 1;
            
            if data.len() <= offset {
                return 0;
            }

            match data_length {
                1 => data[offset] as u32,
                2 => {
                    if data.len() <= offset + 1 {
                        return 0;
                    }
                    ph.sts_makeword(data[offset], data[offset + 1]) as u32
                }
                4 => {
                    if data.len() <= offset + 3 {
                        return 0;
                    }
                    ph.sts_makedword(
                        ph.sts_makeword(data[offset], data[offset + 1]),
                        ph.sts_makeword(data[offset + 2], data[offset + 3]),
                    )
                }
                _ => 0,
            }
        } else {
            0
        }
    }
}

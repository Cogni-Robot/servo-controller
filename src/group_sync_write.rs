use crate::protocol_packet_handler::ProtocolPacketHandler;
use crate::values::*;
use std::collections::HashMap;

pub struct GroupSyncWrite {
    start_address: u8,
    data_length: usize,
    is_param_changed: bool,
    param: Vec<u8>,
    data_dict: HashMap<u8, Vec<u8>>,
}

impl GroupSyncWrite {
    pub fn new(start_address: u8, data_length: usize) -> Self {
        Self {
            start_address,
            data_length,
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

        for (sts_id, data) in &self.data_dict {
            if data.is_empty() {
                return;
            }
            self.param.push(*sts_id);
            self.param.extend(data);
        }
    }

    pub fn add_param(&mut self, sts_id: u8, data: Vec<u8>) -> bool {
        if self.data_dict.contains_key(&sts_id) {
            return false;
        }

        if data.len() > self.data_length {
            return false;
        }

        self.data_dict.insert(sts_id, data);
        self.is_param_changed = true;
        true
    }

    pub fn remove_param(&mut self, sts_id: u8) {
        if self.data_dict.remove(&sts_id).is_some() {
            self.is_param_changed = true;
        }
    }

    pub fn change_param(&mut self, sts_id: u8, data: Vec<u8>) -> bool {
        if !self.data_dict.contains_key(&sts_id) {
            return false;
        }

        if data.len() > self.data_length {
            return false;
        }

        self.data_dict.insert(sts_id, data);
        self.is_param_changed = true;
        true
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

        ph.sync_write_tx_only(self.start_address, self.data_length as u8, &self.param)
    }
}

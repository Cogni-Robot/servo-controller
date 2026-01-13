use crate::group_sync_write::GroupSyncWrite;
use crate::port_handler::PortHandler;
use crate::protocol_packet_handler::ProtocolPacketHandler;
use crate::values::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ST3215 {
    port_handler: Arc<Mutex<PortHandler>>,
    #[allow(dead_code)]
    group_sync_write: Arc<Mutex<GroupSyncWrite>>,
}

impl ST3215 {
    /// Créer une nouvelle instance ST3215
    pub fn new(device: &str) -> Result<Self, String> {
        let mut port_handler = PortHandler::new(device);
        port_handler.open_port()?;

        let group_sync_write = GroupSyncWrite::new(STS_ACC, 7);

        Ok(Self {
            port_handler: Arc::new(Mutex::new(port_handler)),
            group_sync_write: Arc::new(Mutex::new(group_sync_write)),
        })
    }

    /// Vérifier la présence d'un servo
    pub fn ping_servo(&self, sts_id: u8) -> bool {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (model, comm, error) = handler.ping(sts_id);
        comm.is_success() && model != 0 && error == 0
    }

    /// Scanner le bus pour déterminer tous les servos présents
    pub fn list_servos(&self) -> Vec<u8> {
        let mut servos = Vec::new();
        for id in 0..254 {
            if self.ping_servo(id) {
                servos.push(id);
            }
            thread::sleep(Duration::from_millis(10)); // Augmenté de 1ms à 10ms
        }
        servos
    }

    /// Lire la charge du servo (en pourcentage)
    pub fn read_load(&self, sts_id: u8) -> Option<f32> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (load, comm, error) = handler.read_1byte_tx_rx(sts_id, STS_PRESENT_LOAD_L);
        if comm.is_success() && error == 0 {
            Some(load as f32 * 0.1)
        } else {
            None
        }
    }

    /// Lire la tension actuelle du servo (en V)
    pub fn read_voltage(&self, sts_id: u8) -> Option<f32> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (voltage, comm, error) = handler.read_1byte_tx_rx(sts_id, STS_PRESENT_VOLTAGE);
        if comm.is_success() && error == 0 {
            Some(voltage as f32 * 0.1)
        } else {
            None
        }
    }

    /// Lire le courant actuel du servo (en mA)
    pub fn read_current(&self, sts_id: u8) -> Option<f32> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (current, comm, error) = handler.read_1byte_tx_rx(sts_id, STS_PRESENT_CURRENT_L);
        if comm.is_success() && error == 0 {
            Some(current as f32 * 6.5)
        } else {
            None
        }
    }

    /// Lire la température actuelle du servo (en °C)
    pub fn read_temperature(&self, sts_id: u8) -> Option<u8> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (temperature, comm, error) = handler.read_1byte_tx_rx(sts_id, STS_PRESENT_TEMPERATURE);
        if comm.is_success() && error == 0 {
            Some(temperature)
        } else {
            None
        }
    }

    /// Lire la valeur d'accélération actuelle du servo
    pub fn read_acceleration(&self, sts_id: u8) -> Option<u8> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (acc, comm, error) = handler.read_1byte_tx_rx(sts_id, STS_ACC);
        if comm.is_success() && error == 0 {
            Some(acc)
        } else {
            None
        }
    }

    /// Lire le mode actuel du servo
    /// - 0: Mode position
    /// - 1: Mode vitesse constante
    /// - 2: Mode PWM
    /// - 3: Mode servo pas à pas
    pub fn read_mode(&self, sts_id: u8) -> Option<u8> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (mode, comm, error) = handler.read_1byte_tx_rx(sts_id, STS_MODE);
        if comm.is_success() && error == 0 {
            Some(mode)
        } else {
            None
        }
    }

    /// Lire la correction de position actuelle du servo
    pub fn read_correction(&self, sts_id: u8) -> Option<i16> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (correction, comm, error) = handler.read_2byte_tx_rx(sts_id, STS_OFS_L);
        if comm.is_success() && error == 0 {
            let mask = 0x07FFF;
            let mut bits = correction & mask;
            if (correction & 0x0800) != 0 {
                bits = bits & 0x7FF;
                Some(-(bits as i16))
            } else {
                Some(bits as i16)
            }
        } else {
            None
        }
    }

    /// Le servo est-il en mouvement ?
    pub fn is_moving(&self, sts_id: u8) -> Option<bool> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (moving, comm, error) = handler.read_1byte_tx_rx(sts_id, STS_MOVING);
        if comm.is_success() && error == 0 {
            Some(moving != 0)
        } else {
            None
        }
    }

    /// Configurer la valeur d'accélération pour le servo
    /// acc: Valeur d'accélération (0-254). Unité: 100 step/s²
    pub fn set_acceleration(&self, sts_id: u8, acc: u8) -> Option<bool> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (comm, error) = handler.write_tx_rx(sts_id, STS_ACC, &[acc]);
        if comm.is_success() && error == 0 {
            Some(true)
        } else {
            None
        }
    }

    /// Configurer la valeur de vitesse pour le servo
    /// speed: Valeur de vitesse (0-3400). Unité: Step/s
    pub fn set_speed(&self, sts_id: u8, speed: u16) -> Option<bool> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (comm, error) = handler.write_2byte_tx_rx(sts_id, STS_GOAL_SPEED_L, speed);
        if comm.is_success() && error == 0 {
            Some(true)
        } else {
            None
        }
    }

    /// Désactiver le torque du servo (Mettre le couple à 0)
    pub fn disable_torque(&self, sts_id: u8) -> Result<(), String> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (comm, error) = handler.write_tx_rx(sts_id, STS_TORQUE_ENABLE, &[0]);
        if comm.is_success() && error == 0 {
            Ok(())
        } else {
            Err(format!("Failed to disable torque for servo {}: comm={:?}, error={}", sts_id, comm, error))
        }
    }

    /// Activer le torque du servo (Mettre le couple à 1)
    pub fn enable_torque(&self, sts_id: u8) -> Result<(), String> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (comm, error) = handler.write_tx_rx(sts_id, STS_TORQUE_ENABLE, &[1]);
        if comm.is_success() && error == 0 {
            Ok(())
        } else {
            Err(format!("Failed to enable torque for servo {}: comm={:?}, error={}", sts_id, comm, error))
        }
    }

    /// Arrêter le servo (Mettre le couple à 0)
    /// 
    /// **Deprecated:** Utilisez `disable_torque` à la place
    #[deprecated(since = "0.1.0", note = "Utilisez disable_torque à la place")]
    pub fn stop_servo(&self, sts_id: u8) -> Option<bool> {
        self.disable_torque(sts_id).ok().map(|_| true)
    }

    /// Démarrer le servo (Mettre le couple à 1)
    /// 
    /// **Deprecated:** Utilisez `enable_torque` à la place
    #[deprecated(since = "0.1.0", note = "Utilisez enable_torque à la place")]
    pub fn start_servo(&self, sts_id: u8) -> Result<(), String> {
        self.enable_torque(sts_id)
    }

    /// Configurer le mode opérationnel du servo
    /// mode: ID du mode (0, 1, 2 ou 3)
    pub fn set_mode(&self, sts_id: u8, mode: u8) -> Result<(), String> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (comm, _error) = handler.write_tx_rx(sts_id, STS_MODE, &[mode]);
        if comm.is_success() {
            Ok(())
        } else {
            Err("Failed to set mode".to_string())
        }
    }

    /// Ajouter une correction de position
    /// correction: correction (en steps, peut être négatif)
    pub fn correct_position(&self, sts_id: u8, correction: i16) -> Result<(), String> {
        let mut corr = correction.abs() as u16;
        if corr > MAX_CORRECTION {
            corr = MAX_CORRECTION;
        }

        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        
        let lo = handler.sts_lobyte(corr);
        let mut hi = handler.sts_hibyte(corr);

        if correction < 0 {
            hi |= 1 << 3;
        }

        let (comm, _error) = handler.write_tx_rx(sts_id, STS_OFS_L, &[lo, hi]);
        if comm.is_success() {
            Ok(())
        } else {
            Err("Failed to correct position".to_string())
        }
    }

    /// Commencer la rotation
    /// speed: vitesse du servo (peut être négatif, si oui rotation dans le sens inverse)
    pub fn rotate(&self, sts_id: u8, speed: i16) -> Result<(), String> {
        self.set_mode(sts_id, 1)?;

        let abs_speed = speed.abs() as u16;
        let abs_speed = if abs_speed > MAX_SPEED {
            MAX_SPEED
        } else {
            abs_speed
        };

        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        
        let lo = handler.sts_lobyte(abs_speed);
        let mut hi = handler.sts_hibyte(abs_speed);

        if speed < 0 {
            hi |= 1 << 7;
        }

        let (comm, _error) = handler.write_tx_rx(sts_id, STS_GOAL_SPEED_L, &[lo, hi]);
        if comm.is_success() {
            Ok(())
        } else {
            Err("Failed to rotate".to_string())
        }
    }

    /// Obtenir la prochaine position bloquante
    fn get_block_position(&self, sts_id: u8) -> Option<u16> {
        let mut stop_matches = 0;
        loop {
            let moving = self.is_moving(sts_id)?;

            if !moving {
                let position = self.read_position(sts_id);
                let _ = self.set_mode(sts_id, 0);
                let _ = self.disable_torque(sts_id);

                if let Some(pos) = position {
                    stop_matches += 1;
                    if stop_matches > 4 {
                        return Some(pos);
                    }
                } else {
                    return None;
                }
            } else {
                stop_matches = 0;
            }

            thread::sleep(Duration::from_millis(20));
        }
    }

    /// Définir la position 2048 (Mettre le couple à 128)
    pub fn define_middle(&self, sts_id: u8) -> Option<bool> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (comm, error) = handler.write_tx_rx(sts_id, STS_TORQUE_ENABLE, &[128]);
        if comm.is_success() && error == 0 {
            Some(true)
        } else {
            None
        }
    }

    /// Étalonner un servo: Trouver ses positions min et max, puis configurer la nouvelle position 0
    /// ATTENTION: Ne doit être utilisé que pour un servo ayant au moins une position bloquante
    pub fn tare_servo(&self, sts_id: u8) -> (Option<u16>, Option<u16>) {
        if self.correct_position(sts_id, 0).is_err() {
            return (None, None);
        }

        thread::sleep(Duration::from_millis(500));

        self.set_acceleration(sts_id, 100);
        let _ = self.rotate(sts_id, -250);
        thread::sleep(Duration::from_millis(500));

        let min_position = self.get_block_position(sts_id);

        let _ = self.rotate(sts_id, 250);
        thread::sleep(Duration::from_millis(500));

        let max_position = self.get_block_position(sts_id);

        if let (Some(mut min_pos), Some(mut max_pos)) = (min_position, max_position) {
            let distance = if min_pos >= max_pos {
                ((MAX_POSITION - min_pos + max_pos) / 2) as i16
            } else {
                ((max_pos - min_pos) / 2) as i16
            };

            let corr = if min_pos > MAX_POSITION / 2 {
                min_pos as i16 - MAX_POSITION as i16 - 1
            } else {
                min_pos as i16
            };

            if self.correct_position(sts_id, corr).is_ok() {
                min_pos = 0;
                max_pos = (distance * 2) as u16;
                thread::sleep(Duration::from_millis(500));

                self.move_to(sts_id, distance as u16, 2400, 50, false);
            }

            return (Some(min_pos), Some(max_pos));
        }

        (None, None)
    }

    /// Déplacer le servo vers une position prédéfinie
    /// position: Nouvelle position du servo
    /// speed: Vitesse de déplacement en step/s (facultatif, 2400 par défaut)
    /// acc: Vitesse d'accélération en step/s² (facultatif, 50 par défaut)
    /// wait: Attendre que la position soit atteinte avant le retour de la fonction
    pub fn move_to(&self, sts_id: u8, position: u16, speed: u16, acc: u8, wait: bool) -> Option<bool> {
        self.set_mode(sts_id, 0).ok()?;
        self.set_acceleration(sts_id, acc)?;
        self.set_speed(sts_id, speed)?;

        let curr_pos = self.read_position(sts_id)?;

        self.write_position(sts_id, position)?;

        if wait {
            let distance = (position as i32 - curr_pos as i32).abs() as f64;
            let time_to_speed = speed as f64 / (acc as f64 * 100.0);
            let distance_acc = 0.5 * (acc as f64 * 100.0) * time_to_speed.powi(2);

            let time_wait = if distance_acc >= distance {
                (2.0 * distance / acc as f64).sqrt()
            } else {
                let remain_distance = distance - distance_acc;
                time_to_speed + (remain_distance / speed as f64)
            };

            thread::sleep(Duration::from_secs_f64(time_wait));
        }

        Some(true)
    }

    /// Écrire la position
    pub fn write_position(&self, sts_id: u8, position: u16) -> Option<bool> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (comm, error) = handler.write_2byte_tx_rx(sts_id, STS_GOAL_POSITION_L, position);
        if comm.is_success() && error == 0 {
            Some(true)
        } else {
            None
        }
    }

    /// Obtenir le statut des capteurs
    pub fn read_status(&self, sts_id: u8) -> Option<HashMap<String, bool>> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (status_byte, comm, error) = handler.read_1byte_tx_rx(sts_id, STS_STATUS);
        
        if !comm.is_success() || error != 0 {
            return None;
        }

        let status_bits = [
            "Voltage",
            "Sensor",
            "Temperature",
            "Current",
            "Angle",
            "Overload",
        ];

        let mut status = HashMap::new();
        for (i, name) in status_bits.iter().enumerate() {
            status.insert(name.to_string(), (status_byte & (1 << i)) == 0);
        }

        Some(status)
    }

    /// Obtenir la position actuelle
    pub fn read_position(&self, sts_id: u8) -> Option<u16> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (position, comm, error) = handler.read_2byte_tx_rx(sts_id, STS_PRESENT_POSITION_L);
        if comm.is_success() && error == 0 {
            Some(position)
        } else {
            None
        }
    }

    /// Obtenir la vitesse actuelle
    pub fn read_speed(&self, sts_id: u8) -> Option<i16> {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        let (speed, comm, error) = handler.read_2byte_tx_rx(sts_id, STS_PRESENT_SPEED_L);
        if comm.is_success() && error == 0 {
            Some(handler.sts_tohost(speed, 15))
        } else {
            None
        }
    }

    /// Verrouiller l'EEPROM du servo
    pub fn lock_eprom(&self, sts_id: u8) -> CommResult {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        handler.write_1byte_tx_only(sts_id, STS_LOCK, 1)
    }

    /// Déverrouiller l'EEPROM du servo
    pub fn unlock_eprom(&self, sts_id: u8) -> CommResult {
        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        handler.write_1byte_tx_only(sts_id, STS_LOCK, 0)
    }

    /// Changer l'ID d'un servo
    /// sts_id: ID actuel du servo (1 pour un servo neuf)
    /// new_id: Nouvel ID pour le servo (0-253)
    pub fn change_id(&self, sts_id: u8, new_id: u8) -> Result<(), String> {
        if new_id > 253 {
            return Err("new_id must be between 0 and 253".to_string());
        }

        if !self.ping_servo(sts_id) {
            return Err(format!("Could not find servo: {}", sts_id));
        }

        if !self.unlock_eprom(sts_id).is_success() {
            return Err("Could not unlock Eprom".to_string());
        }

        let mut port = self.port_handler.lock().unwrap();
        let mut handler = ProtocolPacketHandler::new(&mut *port);
        if !handler.write_1byte_tx_only(sts_id, STS_ID, new_id).is_success() {
            return Err("Could not change Servo ID".to_string());
        }

        drop(port);
        let _ = self.lock_eprom(sts_id);
        
        Ok(())
    }
}
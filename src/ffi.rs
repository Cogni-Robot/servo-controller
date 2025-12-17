//! Interface FFI pour l'utilisation depuis C/C++
//!
//! Ce module expose les fonctions de la bibliothèque ST3215 via une interface C
//! compatible, permettant l'utilisation depuis C++ et d'autres langages.

use crate::st3215::ST3215;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Handle opaque pour ST3215
pub struct ST3215Handle {
    inner: ST3215,
}

/// Créer une nouvelle instance ST3215
/// 
/// # Arguments
/// * `device` - Chemin du port série (ex: "/dev/ttyUSB0" ou "COM3")
///
/// # Retour
/// Un pointeur vers ST3215Handle, ou NULL en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_new(device: *const c_char) -> *mut ST3215Handle {
    if device.is_null() {
        return ptr::null_mut();
    }

    let device_str = unsafe {
        match CStr::from_ptr(device).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };

    match ST3215::new(device_str) {
        Ok(st) => Box::into_raw(Box::new(ST3215Handle { inner: st })),
        Err(_) => ptr::null_mut(),
    }
}

/// Libérer une instance ST3215
///
/// # Arguments
/// * `handle` - Handle ST3215 à libérer
#[unsafe(no_mangle)]
pub extern "C" fn st3215_free(handle: *mut ST3215Handle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        }
    }
}

/// Vérifier la présence d'un servo
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo (0-253)
///
/// # Retour
/// 1 si le servo répond, 0 sinon
#[unsafe(no_mangle)]
pub extern "C" fn st3215_ping_servo(handle: *mut ST3215Handle, servo_id: u8) -> i32 {
    if handle.is_null() {
        return 0;
    }

    let st = unsafe { &(*handle).inner };
    if st.ping_servo(servo_id) {
        1
    } else {
        0
    }
}

/// Lister tous les servos connectés
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `out_ids` - Buffer pour stocker les IDs trouvés
/// * `max_ids` - Taille maximale du buffer
///
/// # Retour
/// Nombre de servos trouvés
#[unsafe(no_mangle)]
pub extern "C" fn st3215_list_servos(
    handle: *mut ST3215Handle,
    out_ids: *mut u8,
    max_ids: usize,
) -> usize {
    if handle.is_null() || out_ids.is_null() {
        return 0;
    }

    let st = unsafe { &(*handle).inner };
    let servos = st.list_servos();
    let count = servos.len().min(max_ids);

    unsafe {
        let slice = std::slice::from_raw_parts_mut(out_ids, count);
        slice.copy_from_slice(&servos[..count]);
    }

    count
}

/// Déplacer un servo vers une position cible
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo
/// * `position` - Position cible (0-4095)
/// * `speed` - Vitesse de déplacement (0-4095)
/// * `acceleration` - Accélération (0-254)
///
/// # Retour
/// 0 en cas de succès, -1 en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_move_to(
    handle: *mut ST3215Handle,
    servo_id: u8,
    position: u16,
    speed: u16,
    acceleration: u8,
) -> i32 {
    if handle.is_null() {
        return -1;
    }

    let st = unsafe { &(*handle).inner };
    match st.move_to(servo_id, position, speed, acceleration, false) {
        Some(_) => 0,
        None => -1,
    }
}

/// Lire la position actuelle d'un servo
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo
/// * `out_position` - Pointeur pour stocker la position lue
///
/// # Retour
/// 0 en cas de succès, -1 en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_read_position(
    handle: *mut ST3215Handle,
    servo_id: u8,
    out_position: *mut u16,
) -> i32 {
    if handle.is_null() || out_position.is_null() {
        return -1;
    }

    let st = unsafe { &(*handle).inner };
    match st.read_position(servo_id) {
        Some(pos) => {
            unsafe {
                *out_position = pos;
            }
            0
        }
        None => -1,
    }
}

/// Lire la vitesse actuelle d'un servo
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo
/// * `out_speed` - Pointeur pour stocker la vitesse lue
///
/// # Retour
/// 0 en cas de succès, -1 en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_read_speed(
    handle: *mut ST3215Handle,
    servo_id: u8,
    out_speed: *mut u16,
) -> i32 {
    if handle.is_null() || out_speed.is_null() {
        return -1;
    }

    let st = unsafe { &(*handle).inner };
    match st.read_speed(servo_id) {
        Some(speed) => {
            unsafe {
                *out_speed = speed.abs() as u16;
            }
            0
        }
        None => -1,
    }
}

/// Lire la charge d'un servo (en pourcentage)
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo
/// * `out_load` - Pointeur pour stocker la charge lue
///
/// # Retour
/// 0 en cas de succès, -1 en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_read_load(
    handle: *mut ST3215Handle,
    servo_id: u8,
    out_load: *mut f32,
) -> i32 {
    if handle.is_null() || out_load.is_null() {
        return -1;
    }

    let st = unsafe { &(*handle).inner };
    match st.read_load(servo_id) {
        Some(load) => {
            unsafe {
                *out_load = load;
            }
            0
        }
        None => -1,
    }
}

/// Lire la tension d'un servo (en volts)
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo
/// * `out_voltage` - Pointeur pour stocker la tension lue
///
/// # Retour
/// 0 en cas de succès, -1 en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_read_voltage(
    handle: *mut ST3215Handle,
    servo_id: u8,
    out_voltage: *mut f32,
) -> i32 {
    if handle.is_null() || out_voltage.is_null() {
        return -1;
    }

    let st = unsafe { &(*handle).inner };
    match st.read_voltage(servo_id) {
        Some(voltage) => {
            unsafe {
                *out_voltage = voltage;
            }
            0
        }
        None => -1,
    }
}

/// Lire le courant d'un servo (en mA)
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo
/// * `out_current` - Pointeur pour stocker le courant lu
///
/// # Retour
/// 0 en cas de succès, -1 en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_read_current(
    handle: *mut ST3215Handle,
    servo_id: u8,
    out_current: *mut f32,
) -> i32 {
    if handle.is_null() || out_current.is_null() {
        return -1;
    }

    let st = unsafe { &(*handle).inner };
    match st.read_current(servo_id) {
        Some(current) => {
            unsafe {
                *out_current = current;
            }
            0
        }
        None => -1,
    }
}

/// Lire la température d'un servo (en °C)
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo
/// * `out_temperature` - Pointeur pour stocker la température lue
///
/// # Retour
/// 0 en cas de succès, -1 en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_read_temperature(
    handle: *mut ST3215Handle,
    servo_id: u8,
    out_temperature: *mut u8,
) -> i32 {
    if handle.is_null() || out_temperature.is_null() {
        return -1;
    }

    let st = unsafe { &(*handle).inner };
    match st.read_temperature(servo_id) {
        Some(temp) => {
            unsafe {
                *out_temperature = temp;
            }
            0
        }
        None => -1,
    }
}

/// Vérifier si un servo est en mouvement
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo
///
/// # Retour
/// 1 si en mouvement, 0 si arrêté, -1 en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_is_moving(handle: *mut ST3215Handle, servo_id: u8) -> i32 {
    if handle.is_null() {
        return -1;
    }

    let st = unsafe { &(*handle).inner };
    match st.is_moving(servo_id) {
        Some(true) => 1,
        Some(false) => 0,
        None => -1,
    }
}

/// Activer le couple d'un servo
///
/// # Arguments
/// * `handle` - Handle ST3215
/// * `servo_id` - ID du servo
/// * `enable` - 1 pour activer, 0 pour désactiver
///
/// # Retour
/// 0 en cas de succès, -1 en cas d'erreur
#[unsafe(no_mangle)]
pub extern "C" fn st3215_enable_torque(
    handle: *mut ST3215Handle,
    servo_id: u8,
    enable: i32,
) -> i32 {
    if handle.is_null() {
        return -1;
    }

    let st = unsafe { &(*handle).inner };
    let result = if enable != 0 {
        st.start_servo(servo_id)
    } else {
        st.stop_servo(servo_id).map(|_| ()).ok_or_else(|| "Failed to stop servo".to_string())
    };
    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Obtenir la version de la bibliothèque
///
/// # Retour
/// Chaîne de caractères contenant la version (doit être libérée avec st3215_free_string)
#[unsafe(no_mangle)]
pub extern "C" fn st3215_version() -> *mut c_char {
    let version = env!("CARGO_PKG_VERSION");
    match CString::new(version) {
        Ok(s) => s.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Libérer une chaîne de caractères allouée par la bibliothèque
///
/// # Arguments
/// * `s` - Pointeur vers la chaîne à libérer
#[unsafe(no_mangle)]
pub extern "C" fn st3215_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

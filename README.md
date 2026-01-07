# ST3215 Servo Controller

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Bibliothèque Rust complète pour contrôler les servomoteurs ST3215 via communication série. Cette bibliothèque offre une interface simple et sûre pour gérer tous les aspects des servos ST3215.

## Caractéristiques

- **API Rust complète** - Interface type-safe et ergonomique
- **Bindings C/C++** - Utilisation depuis C++ via FFI
- **Multi-plateforme** - Windows, Linux, macOS
- **Communication série optimisée** - Support de multiples ports
- **Gestion du torque** - Activation/désactivation précise
- **Multiples modes** - Position, vitesse, PWM, pas-à-pas
- **Lecture des capteurs** - Tension, courant, température, charge
- **Étalonnage automatique** - Détection des limites min/max
- **Thread-safe** - Utilisation sécurisée en multi-threading

## Installation

### Depuis Cargo

Ajoutez cette dépendance dans votre `Cargo.toml` :

```toml
[dependencies]
st3215 = { path = "." }
```

### Depuis Git

```bash
git clone https://github.com/Cogni-Robot/servo-controller
cd servo-controller
cargo build --release
```

## Démarrage rapide

```rust
use st3215::ST3215;

fn main() -> Result<(), String> {
    // Connexion au port série
    let controller = ST3215::new("/dev/ttyUSB0")?;
    
    // Lister tous les servos connectés
    let servos = controller.list_servos();
    println!("Servos trouvés: {:?}", servos);
    
    // Contrôler un servo
    let servo_id = 1;
    controller.enable_torque(servo_id)?;
    controller.move_to(servo_id, 2048, 2400, 50, false);
    
    Ok(())
}
```

## Documentation complète

### Table des matières

- [Initialisation](#initialisation)
- [Détection et connexion](#détection-et-connexion)
- [Contrôle du torque](#contrôle-du-torque)
- [Contrôle de position](#contrôle-de-position)
- [Contrôle de vitesse](#contrôle-de-vitesse)
- [Lecture des capteurs](#lecture-des-capteurs)
- [Configuration avancée](#configuration-avancée)
- [Étalonnage](#étalonnage)
- [Exemples](#exemples)

---

## Initialisation

### `new(device: &str) -> Result<Self, String>`

Crée une nouvelle instance du contrôleur ST3215.

**Paramètres:**
- `device`: Chemin du port série

**Retour:** `Result<ST3215, String>`

**Exemples:**

```rust
// Windows
let controller = ST3215::new("COM3")?;

// Linux
let controller = ST3215::new("/dev/ttyUSB0")?;
let controller = ST3215::new("/dev/ttyACM0")?;

// MacOS
let controller = ST3215::new("/dev/cu.usbserial-1234")?;
```

---

## Détection et connexion

### `ping_servo(sts_id: u8) -> bool`

Vérifie si un servo est présent et répond.

**Paramètres:**
- `sts_id`: ID du servo (0-253)

**Retour:** `true` si le servo répond, `false` sinon

**Exemple:**

```rust
if controller.ping_servo(1) {
    println!("Servo 1 est connecté");
}
```

### `list_servos() -> Vec<u8>`

Scanne tous les IDs possibles (0-253) et retourne la liste des servos trouvés.

**Retour:** Vecteur contenant les IDs des servos détectés

**Exemple:**

```rust
let servos = controller.list_servos();
println!("Servos trouvés: {:?}", servos);
// Output: Servos trouvés: [1, 2, 5, 8]
```

---

## Contrôle du torque

### `enable_torque(sts_id: u8) -> Result<(), String>`

Active le torque du servo. Le servo maintiendra sa position et pourra être contrôlé.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Result<(), String>`

**Exemple:**

```rust
controller.enable_torque(1)?;
println!("Torque activé");
```

### `disable_torque(sts_id: u8) -> Result<(), String>`

Désactive le torque du servo. Le servo peut être déplacé manuellement.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Result<(), String>`

**Exemple:**

```rust
controller.disable_torque(1)?;
println!("Le servo peut être déplacé manuellement");
```

---

## Contrôle de position

### `move_to(sts_id: u8, position: u16, speed: u16, acc: u8, wait: bool) -> Option<bool>`

Déplace le servo vers une position cible avec vitesse et accélération spécifiées.

**Paramètres:**
- `sts_id`: ID du servo
- `position`: Position cible (0-4095)
- `speed`: Vitesse de déplacement en step/s (0-3400)
- `acc`: Accélération en 100 step/s² (0-254)
- `wait`: Si `true`, bloque jusqu'à ce que la position soit atteinte

**Retour:** `Some(true)` en cas de succès, `None` en cas d'erreur

**Exemple:**

```rust
// Déplacement rapide sans attente
controller.move_to(1, 2048, 2400, 50, false);

// Déplacement lent avec attente
controller.move_to(1, 1024, 500, 20, true);
println!("Position atteinte!");
```

### `write_position(sts_id: u8, position: u16) -> Option<bool>`

Écrit directement une position cible sans modifier vitesse/accélération.

**Paramètres:**
- `sts_id`: ID du servo
- `position`: Position cible (0-4095)

**Retour:** `Some(true)` en cas de succès, `None` en cas d'erreur

**Exemple:**

```rust
controller.set_speed(1, 2000);
controller.set_acceleration(1, 50);
controller.write_position(1, 2048);
```

### `read_position(sts_id: u8) -> Option<u16>`

Lit la position actuelle du servo.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(position)` si réussi, `None` sinon

**Exemple:**

```rust
if let Some(pos) = controller.read_position(1) {
    println!("Position actuelle: {}", pos);
}
```

### `is_moving(sts_id: u8) -> Option<bool>`

Vérifie si le servo est en mouvement.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(true)` si en mouvement, `Some(false)` si arrêté, `None` en cas d'erreur

**Exemple:**

```rust
controller.move_to(1, 3000, 1500, 50, false);

while controller.is_moving(1) == Some(true) {
    println!("En mouvement...");
    std::thread::sleep(std::time::Duration::from_millis(100));
}
println!("Position atteinte!");
```

---

## Contrôle de vitesse

### `rotate(sts_id: u8, speed: i16) -> Result<(), String>`

Active le mode rotation continue avec une vitesse spécifiée.

**Paramètres:**
- `sts_id`: ID du servo
- `speed`: Vitesse de rotation en step/s (-3400 à +3400)
  - Positif: rotation horaire
  - Négatif: rotation anti-horaire

**Retour:** `Result<(), String>`

**Exemple:**

```rust
// Rotation horaire à 500 step/s
controller.rotate(1, 500)?;

// Rotation anti-horaire à 1000 step/s
controller.rotate(1, -1000)?;

// Arrêter
controller.disable_torque(1)?;
```

### `set_speed(sts_id: u8, speed: u16) -> Option<bool>`

Configure la vitesse pour les déplacements en mode position.

**Paramètres:**
- `sts_id`: ID du servo
- `speed`: Vitesse en step/s (0-3400)

**Retour:** `Some(true)` en cas de succès, `None` en cas d'erreur

**Exemple:**

```rust
controller.set_speed(1, 2400);
```

### `read_speed(sts_id: u8) -> Option<i16>`

Lit la vitesse actuelle du servo.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(speed)` si réussi, `None` sinon. La vitesse peut être négative.

**Exemple:**

```rust
if let Some(speed) = controller.read_speed(1) {
    println!("Vitesse actuelle: {} step/s", speed);
}
```

### `set_acceleration(sts_id: u8, acc: u8) -> Option<bool>`

Configure l'accélération du servo.

**Paramètres:**
- `sts_id`: ID du servo
- `acc`: Accélération (0-254), unité: 100 step/s²

**Retour:** `Some(true)` en cas de succès, `None` en cas d'erreur

**Exemple:**

```rust
// Accélération rapide (5000 step/s²)
controller.set_acceleration(1, 50);

// Accélération lente (1000 step/s²)
controller.set_acceleration(1, 10);
```

### `read_acceleration(sts_id: u8) -> Option<u8>`

Lit la valeur d'accélération configurée.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(acc)` si réussi, `None` sinon

**Exemple:**

```rust
if let Some(acc) = controller.read_acceleration(1) {
    println!("Accélération: {} (× 100 step/s²)", acc);
}
```

---

## Lecture des capteurs

### `read_voltage(sts_id: u8) -> Option<f32>`

Lit la tension d'alimentation du servo.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(voltage)` en volts, `None` en cas d'erreur

**Exemple:**

```rust
if let Some(voltage) = controller.read_voltage(1) {
    println!("Tension: {:.1} V", voltage);
    
    if voltage < 6.0 {
        println!("Attention: Tension faible!");
    }
}
```

### `read_current(sts_id: u8) -> Option<f32>`

Lit le courant consommé par le servo.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(current)` en milliampères, `None` en cas d'erreur

**Exemple:**

```rust
if let Some(current) = controller.read_current(1) {
    println!("Courant: {:.1} mA", current);
}
```

### `read_temperature(sts_id: u8) -> Option<u8>`

Lit la température interne du servo.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(temperature)` en degrés Celsius, `None` en cas d'erreur

**Exemple:**

```rust
if let Some(temp) = controller.read_temperature(1) {
    println!("Température: {} °C", temp);
    
    if temp > 70 {
        println!("Attention: Température élevée!");
        controller.disable_torque(1)?;
    }
}
```

### `read_load(sts_id: u8) -> Option<f32>`

Lit la charge actuelle sur le servo.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(load)` en pourcentage, `None` en cas d'erreur

**Exemple:**

```rust
if let Some(load) = controller.read_load(1) {
    println!("Charge: {:.1}%", load);
}
```

### `read_status(sts_id: u8) -> Option<HashMap<String, bool>>`

Lit l'état de tous les capteurs du servo.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `HashMap` avec les états des capteurs (`true` = OK, `false` = Erreur)
- `"Voltage"`: État de la tension
- `"Sensor"`: État du capteur
- `"Temperature"`: État de la température
- `"Current"`: État du courant
- `"Angle"`: État de l'angle
- `"Overload"`: État de surcharge

**Exemple:**

```rust
if let Some(status) = controller.read_status(1) {
    for (sensor, ok) in status {
        let icon = if ok { "OK" } else { "ERR" };
        println!("[{}] {}: {}", icon, sensor, if ok { "OK" } else { "ERROR" });
    }
}
```

---

## Configuration avancée

### `set_mode(sts_id: u8, mode: u8) -> Result<(), String>`

Change le mode opérationnel du servo.

**Paramètres:**
- `sts_id`: ID du servo
- `mode`: Mode à activer
  - `0`: Mode position (contrôle de position précis)
  - `1`: Mode vitesse constante (rotation continue)
  - `2`: Mode PWM (contrôle direct du PWM)
  - `3`: Mode pas-à-pas (contrôle stepper)

**Retour:** `Result<(), String>`

**Exemple:**

```rust
// Mode position (par défaut)
controller.set_mode(1, 0)?;

// Mode rotation continue
controller.set_mode(1, 1)?;
```

### `read_mode(sts_id: u8) -> Option<u8>`

Lit le mode actuel du servo.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(mode)` si réussi, `None` sinon

**Exemple:**

```rust
if let Some(mode) = controller.read_mode(1) {
    let mode_name = match mode {
        0 => "Position",
        1 => "Vitesse",
        2 => "PWM",
        3 => "Pas-à-pas",
        _ => "Inconnu",
    };
    println!("Mode actuel: {}", mode_name);
}
```

### `correct_position(sts_id: u8, correction: i16) -> Result<(), String>`

Applique une correction de position (offset).

**Paramètres:**
- `sts_id`: ID du servo
- `correction`: Valeur de correction en steps (-2047 à +2047)

**Retour:** `Result<(), String>`

**Exemple:**

```rust
// Ajouter un offset de +100 steps
controller.correct_position(1, 100)?;

// Soustraire 50 steps
controller.correct_position(1, -50)?;

// Réinitialiser
controller.correct_position(1, 0)?;
```

### `read_correction(sts_id: u8) -> Option<i16>`

Lit la correction de position actuelle.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(correction)` si réussi, `None` sinon

**Exemple:**

```rust
if let Some(corr) = controller.read_correction(1) {
    println!("Correction actuelle: {} steps", corr);
}
```

### `change_id(sts_id: u8, new_id: u8) -> Result<(), String>`

Change l'ID d'un servo.

**Paramètres:**
- `sts_id`: ID actuel du servo
- `new_id`: Nouvel ID (0-253)

**Retour:** `Result<(), String>`

**Attention:** Cette opération modifie l'EEPROM du servo.

**Exemple:**

```rust
// Changer l'ID de 1 à 5
controller.change_id(1, 5)?;
println!("ID changé: le servo répond maintenant à l'ID 5");

// Vérification
if controller.ping_servo(5) {
    println!("Nouveau ID confirmé");
}
```

### `lock_eprom(sts_id: u8) -> CommResult`

Verrouille l'EEPROM du servo pour éviter les modifications accidentelles.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `CommResult`

**Exemple:**

```rust
controller.lock_eprom(1);
```

### `unlock_eprom(sts_id: u8) -> CommResult`

Déverrouille l'EEPROM du servo pour permettre les modifications.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `CommResult`

**Exemple:**

```rust
controller.unlock_eprom(1);
controller.change_id(1, 5)?;
controller.lock_eprom(5);
```

---

## Étalonnage

### `tare_servo(sts_id: u8) -> (Option<u16>, Option<u16>)`

Étalonne automatiquement un servo en trouvant ses positions min et max.

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** Tuple `(min_position, max_position)`

**Important:** 
- Ne fonctionne que sur des servos avec butées mécaniques
- Le servo va effectuer une rotation complète
- Assurez-vous qu'il n'y a pas d'obstacles

**Exemple:**

```rust
println!("Démarrage de l'étalonnage...");
let (min, max) = controller.tare_servo(1);

match (min, max) {
    (Some(min_pos), Some(max_pos)) => {
        println!("Étalonnage réussi!");
        println!("  Position min: {}", min_pos);
        println!("  Position max: {}", max_pos);
        println!("  Course totale: {} steps", max_pos - min_pos);
    }
    _ => println!("Échec de l'étalonnage"),
}
```

### `define_middle(sts_id: u8) -> Option<bool>`

Définit la position actuelle comme position 2048 (milieu).

**Paramètres:**
- `sts_id`: ID du servo

**Retour:** `Some(true)` en cas de succès, `None` en cas d'erreur

**Exemple:**

```rust
// Placer manuellement le servo à la position souhaitée
controller.disable_torque(1)?;
println!("Placez le servo à la position centrale...");
std::thread::sleep(std::time::Duration::from_secs(5));

// Définir cette position comme 2048
controller.define_middle(1);
controller.enable_torque(1)?;
```

---

## Exemples

### Exemple 1: Scanner et lister les servos

```rust
use st3215::ST3215;

fn main() -> Result<(), String> {
    let controller = ST3215::new("/dev/ttyUSB0")?;
    
    println!("Scan des servos...");
    let servos = controller.list_servos();
    
    println!("\n{} servo(s) trouvé(s):", servos.len());
    for id in servos {
        println!("  - Servo ID: {}", id);
    }
    
    Ok(())
}
```

### Exemple 2: Contrôle simple de position

```rust
use st3215::ST3215;

fn main() -> Result<(), String> {
    let controller = ST3215::new("/dev/ttyUSB0")?;
    let servo_id = 1;
    
    // Activer le torque
    controller.enable_torque(servo_id)?;
    
    // Déplacer vers différentes positions
    let positions = [1024, 2048, 3072, 2048];
    
    for &pos in &positions {
        println!("Déplacement vers {}", pos);
        controller.move_to(servo_id, pos, 2000, 50, true);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    
    // Désactiver le torque
    controller.disable_torque(servo_id)?;
    
    Ok(())
}
```

### Exemple 3: Surveillance des capteurs

```rust
use st3215::ST3215;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), String> {
    let controller = ST3215::new("/dev/ttyUSB0")?;
    let servo_id = 1;
    
    controller.enable_torque(servo_id)?;
    
    // Monitoring en boucle
    for _ in 0..10 {
        println!("\n--- État du servo {} ---", servo_id);
        
        if let Some(pos) = controller.read_position(servo_id) {
            println!("Position: {}", pos);
        }
        
        if let Some(voltage) = controller.read_voltage(servo_id) {
            println!("Tension: {:.1} V", voltage);
        }
        
        if let Some(current) = controller.read_current(servo_id) {
            println!("Courant: {:.1} mA", current);
        }
        
        if let Some(temp) = controller.read_temperature(servo_id) {
            println!("Température: {} °C", temp);
        }
        
        if let Some(load) = controller.read_load(servo_id) {
            println!("Charge: {:.1}%", load);
        }
        
        thread::sleep(Duration::from_secs(1));
    }
    
    Ok(())
}
```

### Exemple 4: Rotation continue

```rust
use st3215::ST3215;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), String> {
    let controller = ST3215::new("/dev/ttyUSB0")?;
    let servo_id = 1;
    
    // Rotation horaire pendant 3 secondes
    println!("Rotation horaire...");
    controller.rotate(servo_id, 500)?;
    thread::sleep(Duration::from_secs(3));
    
    // Rotation anti-horaire pendant 3 secondes
    println!("Rotation anti-horaire...");
    controller.rotate(servo_id, -500)?;
    thread::sleep(Duration::from_secs(3));
    
    // Arrêt
    println!("Arrêt...");
    controller.disable_torque(servo_id)?;
    
    Ok(())
}
```

### Exemple 5: Contrôle multi-servos

```rust
use st3215::ST3215;

fn main() -> Result<(), String> {
    let controller = ST3215::new("/dev/ttyUSB0")?;
    
    let servos = controller.list_servos();
    println!("Contrôle de {} servos", servos.len());
    
    // Activer tous les servos
    for &id in &servos {
        controller.enable_torque(id)?;
    }
    
    // Déplacer tous les servos vers la position centrale
    for &id in &servos {
        controller.move_to(id, 2048, 2000, 50, false);
    }
    
    // Attendre que tous soient en position
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Lire les positions finales
    for &id in &servos {
        if let Some(pos) = controller.read_position(id) {
            println!("Servo {}: position = {}", id, pos);
        }
    }
    
    Ok(())
}
```

---

## Compilation et exécution

### Compilation

```bash
# Mode debug
cargo build

# Mode release (optimisé)
cargo build --release
```

### Exécuter les exemples

```bash
# Exemple basique
cargo run --example basic --release

# Exemple de contrôle du torque
cargo run --example torque_control --release

# Programme principal
cargo run --release
```

### Tests

```bash
cargo test
```

---

## Utilisation depuis C/C++

Cette bibliothèque peut être utilisée depuis C/C++ via les bindings FFI.

Voir la documentation complète: [CPP_INTEROP.md](docs/CPP_INTEROP.md)

### Exemple C++

```cpp
#include "st3215.h"

int main() {
    // Créer le contrôleur
    ST3215Handle* controller = st3215_new("/dev/ttyUSB0");
    
    // Activer le torque
    st3215_enable_torque(controller, 1, 1);
    
    // Déplacer le servo
    st3215_move_to(controller, 1, 2048, 2400, 50, 0);
    
    // Libérer les ressources
    st3215_free(controller);
    
    return 0;
}
```

---

## Spécifications techniques

### Limites du servo ST3215

| Paramètre | Valeur min | Valeur max | Unité |
|-----------|------------|------------|-------|
| Position | 0 | 4095 | steps |
| Vitesse | 0 | 3400 | step/s |
| Accélération | 0 | 254 | × 100 step/s² |
| Tension | 6.0 | 8.4 | V |
| Température | -5 | 75 | °C |
| ID | 0 | 253 | - |

### Registres de la mémoire

#### EEPROM (lecture seule)
- `STS_MODEL_L/H` (3-4): Numéro de modèle

#### EEPROM (lecture/écriture) - Persistant
- `STS_ID` (5): ID du servo
- `STS_BAUD_RATE` (6): Vitesse de communication
- `STS_MIN_ANGLE_LIMIT_L/H` (9-10): Limite min d'angle
- `STS_MAX_ANGLE_LIMIT_L/H` (11-12): Limite max d'angle
- `STS_OFS_L/H` (31-32): Offset de position
- `STS_MODE` (33): Mode opérationnel

#### SRAM (lecture/écriture) - Volatile
- `STS_TORQUE_ENABLE` (40): Activation du couple
- `STS_ACC` (41): Accélération
- `STS_GOAL_POSITION_L/H` (42-43): Position cible
- `STS_GOAL_TIME_L/H` (44-45): Temps pour atteindre la position
- `STS_GOAL_SPEED_L/H` (46-47): Vitesse cible
- `STS_LOCK` (55): Verrouillage EEPROM

#### SRAM (lecture seule) - État actuel
- `STS_PRESENT_POSITION_L/H` (56-57): Position actuelle
- `STS_PRESENT_SPEED_L/H` (58-59): Vitesse actuelle
- `STS_PRESENT_LOAD_L/H` (60-61): Charge actuelle
- `STS_PRESENT_VOLTAGE` (62): Tension actuelle
- `STS_PRESENT_TEMPERATURE` (63): Température actuelle
- `STS_STATUS` (65): Bits d'état des capteurs
- `STS_MOVING` (66): Statut de mouvement
- `STS_PRESENT_CURRENT_L/H` (69-70): Courant actuel

### Modes opérationnels

| Mode | Valeur | Description |
|------|--------|-------------|
| Position | 0 | Contrôle de position précis (0-4095) |
| Vitesse | 1 | Rotation continue à vitesse constante |
| PWM | 2 | Contrôle direct du signal PWM |
| Stepper | 3 | Mode pas-à-pas |

---

## Débogage

### Activer les logs

```bash
# Logs de base
RUST_LOG=info cargo run

# Logs détaillés
RUST_LOG=debug cargo run

# Logs très détaillés
RUST_LOG=trace cargo run
```

### Problèmes courants

#### "Permission denied" sous Linux

```bash
# Ajouter l'utilisateur au groupe dialout
sudo usermod -a -G dialout $USER

# Ou donner les permissions au port
sudo chmod 666 /dev/ttyUSB0
```

#### Le servo ne répond pas

1. Vérifier la connexion physique
2. Vérifier le câblage (TX/RX, alimentation)
3. Vérifier le baudrate (par défaut: 1000000)
4. Tester avec `ping_servo()`

#### Position incorrecte

1. Vérifier la correction de position: `read_correction()`
2. Réinitialiser la correction: `correct_position(id, 0)`
3. Effectuer un étalonnage: `tare_servo(id)`

---

## Dépendances

- `serialport` (4.3) - Communication série multiplateforme
- `thiserror` (1.0) - Gestion élégante des erreurs
- `serde` (1.0) - Sérialisation (optionnel)
- `serde_json` (1.0) - JSON (optionnel)

---

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

---

## Auteur

**NotPunchnox**

---

## Liens utiles

- [Repository GitHub](https://github.com/Cogni-Robot/servo-controller)
- [Issues & Bugs](https://github.com/Cogni-Robot/servo-controller/issues)
- [Documentation C++](docs/CPP_INTEROP.md)
- [Cogni-Robot](https://github.com/Cogni-Robot)

---

## Remerciements

Merci à tous les contributeurs et utilisateurs de cette bibliothèque!

---

**Made with ❤️ and 🦀 Rust**
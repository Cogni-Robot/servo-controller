# Servo Controller - ST3215

Bibliothèque Rust pour contrôler les servomoteurs ST3215 via communication série.

Ce projet a été écrit en Rust pour bénéficier de :
- **Performance** : Compilation native et optimisations
- **Sécurité** : Gestion mémoire sûre et système de types robuste
- **Concurrence** : Support natif du multithreading sécurisé
- **Fiabilité** : Détection des erreurs à la compilation

## Installation

Ajoutez cette dépendance dans votre `Cargo.toml` :

```toml
[dependencies]
st3215 = { path = "." }
```

Ou clonez le dépôt :

```bash
git clone https://github.com/Cogni-Robot/servo-controller
cd servo-controller
```

## Utilisation

### Exemple basique

```rust
use st3215::ST3215;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connexion au port série
    let servo = ST3215::new("COM3")?;

    // Lister tous les servos
    let ids = servo.list_servos();
    println!("Servos trouvés: {:?}", ids);

    // Déplacer un servo vers la position 2048
    for id in ids {
        servo.move_to(id, 2048, 2400, 50, false);
    }

    Ok(())
}
```

### Compilation et exécution

```bash
# Compiler le projet
cargo build --release

# Exécuter l'exemple principal
cargo run --release

# Exécuter l'exemple basique
cargo run --example basic --release
```

## API Principale

### Création et connexion

```rust
// Windows
let servo = ST3215::new("COMx")?;

// Linux
let servo = ST3215::new("/dev/ttyUSBx")?;
let servo = ST3215::new("/dev/ttyACMx")?;

// MacOS
let servo = ST3215::new("/dev/cu.usbserial...")?;
let servo = ST3215::new("/dev/tty.usbserial...")?;
```

### Détection des servos

```rust
// Scanner tous les servos
let ids = servo.list_servos();

// Vérifier un servo spécifique
let exists = servo.ping_servo(1);
```

### Contrôle de position

```rust
// Déplacer vers une position
servo.move_to(id, position, speed, acceleration, wait);

// Lire la position actuelle
if let Some(pos) = servo.read_position(id) {
    println!("Position: {}", pos);
}
```

### Contrôle de vitesse

```rust
// Rotation continue
servo.rotate(id, 500)?;   // Rotation clockwise
servo.rotate(id, -500)?;  // Rotation counter-clockwise

// Configurer la vitesse
servo.set_speed(id, 2400);
```

### Lecture des capteurs

```rust
// Tension
if let Some(voltage) = servo.read_voltage(id) {
    println!("Tension: {:.1} V", voltage);
}

// Température
if let Some(temp) = servo.read_temperature(id) {
    println!("Température: {} °C", temp);
}

// Courant
if let Some(current) = servo.read_current(id) {
    println!("Courant: {:.1} mA", current);
}

// Charge
if let Some(load) = servo.read_load(id) {
    println!("Charge: {:.1}%", load);
}
```

### Configuration

```rust
// Changer l'ID
servo.change_id(1, 5)?;

// Configurer le mode (0=Position, 1=Vitesse, 2=PWM, 3=Pas à pas)
servo.set_mode(id, 0)?;

// Configurer l'accélération
servo.set_acceleration(id, 50);

// Correction de position
servo.correct_position(id, 100)?;
```

### Étalonnage

```rust
// Étalonner un servo (trouver min/max)
let (min, max) = servo.tare_servo(id);
println!("Min: {:?}, Max: {:?}", min, max);
```


## Dépendances

- `serialport` (4.3) - Communication série
- `thiserror` (1.0) - Gestion des erreurs

## Registres ST3215

### EEPROM (lecture seule)
- `STS_MODEL_L/H` (3-4) : Numéro de modèle

### EEPROM (lecture/écriture)
- `STS_ID` (5) : ID du servo
- `STS_BAUD_RATE` (6) : Vitesse de communication
- `STS_MODE` (33) : Mode opérationnel

### SRAM (lecture/écriture)
- `STS_TORQUE_ENABLE` (40) : Activation du couple
- `STS_ACC` (41) : Accélération
- `STS_GOAL_POSITION_L/H` (42-43) : Position cible
- `STS_GOAL_SPEED_L/H` (46-47) : Vitesse cible

### SRAM (lecture seule)
- `STS_PRESENT_POSITION_L/H` (56-57) : Position actuelle
- `STS_PRESENT_SPEED_L/H` (58-59) : Vitesse actuelle
- `STS_PRESENT_VOLTAGE` (62) : Tension actuelle
- `STS_PRESENT_TEMPERATURE` (63) : Température actuelle
- `STS_MOVING` (66) : Statut de mouvement
- `STS_PRESENT_CURRENT_L/H` (69-70) : Courant actuel

## Modes opérationnels

- **Mode 0** : Position - Contrôle de position précis
- **Mode 1** : Vitesse constante - Rotation continue
- **Mode 2** : PWM - Contrôle direct du PWM
- **Mode 3** : Pas à pas - Contrôle en mode stepper

## Notes importantes

1. Le port série doit être accessible (droits appropriés sous Linux/macOS)
2. La vitesse de communication par défaut est 1 000 000 bauds
3. Les positions valides vont de 0 à 4095
4. La fonction `tare_servo()` ne doit être utilisée que sur des servos avec positions bloquantes

## Débogage

Pour activer les logs de débogage :

```bash
RUST_LOG=debug cargo run
```

## 📄 Licence
MIT

## 👥 Auteurs
NotPunchnox

## 🔗 Liens

- Repository: https://github.com/Cogni-Robot/servo-controller
- Issues: https://github.com/Cogni-Robot/servo-controller/issues
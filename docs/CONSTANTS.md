# Constantes et valeurs du ST3215

Ce document décrit toutes les constantes, limites et valeurs importantes pour le contrôle des servomoteurs ST3215.

## Sommaire

- [Limites physiques](#limites-physiques)
- [Registres mémoire](#registres-mémoire)
- [Modes opérationnels](#modes-opérationnels)
- [Baudrates disponibles](#baudrates-disponibles)
- [Bits d'erreur](#bits-derreur)
- [Instructions protocole](#instructions-protocole)

---

## Limites physiques

### Position
```rust
pub const MIN_POSITION: u16 = 0;
pub const MAX_POSITION: u16 = 4095;
```
- **Plage**: 0 à 4095 steps
- **Résolution**: ~0.088° par step (360° / 4096)
- **Course totale**: 360° (une rotation complète)

### Vitesse
```rust
pub const MAX_SPEED: u16 = 3400;
```
- **Plage**: 0 à 3400 step/s
- **Unité**: steps par seconde
- **Vitesse max**: ~300 RPM (rotations par minute)

### Accélération
- **Plage**: 0 à 254
- **Unité**: × 100 step/s²
- **Accélération max**: 25400 step/s²

### Correction de position
```rust
pub const MAX_CORRECTION: u16 = 2047;
```
- **Plage**: -2047 à +2047 steps
- **Usage**: Calibration et offset de position

### Tension
- **Tension nominale**: 7.4V
- **Plage de fonctionnement**: 6.0V à 8.4V
- **Résolution de lecture**: 0.1V

### Température
- **Plage de fonctionnement**: -5°C à 75°C
- **Température d'alarme**: Typiquement 70°C
- **Résolution**: 1°C

### Courant
- **Résolution de lecture**: 6.5 mA par unité
- **Courant max**: Dépend du modèle

---

## Registres mémoire

### EEPROM - Lecture seule (RO)

#### Modèle
```rust
pub const STS_MODEL_L: u8 = 3;
pub const STS_MODEL_H: u8 = 4;
```
- **Description**: Numéro de modèle du servo
- **Taille**: 2 bytes (16 bits)

---

### EEPROM - Lecture/Écriture (RW)

Ces valeurs sont persistantes et conservées après extinction.

#### ID du servo
```rust
pub const STS_ID: u8 = 5;
```
- **Description**: Identifiant unique du servo
- **Plage**: 0 à 253
- **Valeur par défaut**: 1 (servo neuf)
- **Note**: 254 (0xFE) est réservé pour broadcast

#### Baudrate
```rust
pub const STS_BAUD_RATE: u8 = 6;
```
- **Description**: Vitesse de communication
- **Valeurs possibles**: Voir [Baudrates disponibles](#baudrates-disponibles)

#### Limites d'angle
```rust
pub const STS_MIN_ANGLE_LIMIT_L: u8 = 9;
pub const STS_MIN_ANGLE_LIMIT_H: u8 = 10;
pub const STS_MAX_ANGLE_LIMIT_L: u8 = 11;
pub const STS_MAX_ANGLE_LIMIT_H: u8 = 12;
```
- **Description**: Limites min/max de position
- **Taille**: 2 bytes chacune

#### Zone morte (Dead Zone)
```rust
pub const STS_CW_DEAD: u8 = 26;    // Sens horaire
pub const STS_CCW_DEAD: u8 = 27;   // Sens anti-horaire
```
- **Description**: Zone morte pour éviter les oscillations

#### Offset de position
```rust
pub const STS_OFS_L: u8 = 31;
pub const STS_OFS_H: u8 = 32;
```
- **Description**: Correction de position (calibration)
- **Plage**: -2047 à +2047

#### Mode opérationnel
```rust
pub const STS_MODE: u8 = 33;
```
- **Description**: Mode de fonctionnement du servo
- **Valeurs**: Voir [Modes opérationnels](#modes-opérationnels)

---

### SRAM - Lecture/Écriture (RW)

Ces valeurs sont volatiles et perdues après extinction.

#### Activation du torque
```rust
pub const STS_TORQUE_ENABLE: u8 = 40;
```
- **Valeurs**:
  - `0`: Torque désactivé (servo libre)
  - `1`: Torque activé (servo sous contrôle)
  - `128`: Définir position 2048 comme centrale

#### Accélération
```rust
pub const STS_ACC: u8 = 41;
```
- **Description**: Accélération du mouvement
- **Plage**: 0 à 254
- **Unité**: × 100 step/s²

#### Position cible
```rust
pub const STS_GOAL_POSITION_L: u8 = 42;
pub const STS_GOAL_POSITION_H: u8 = 43;
```
- **Description**: Position à atteindre
- **Plage**: 0 à 4095

#### Temps pour atteindre la position
```rust
pub const STS_GOAL_TIME_L: u8 = 44;
pub const STS_GOAL_TIME_H: u8 = 45;
```
- **Description**: Temps prévu pour le mouvement

#### Vitesse cible
```rust
pub const STS_GOAL_SPEED_L: u8 = 46;
pub const STS_GOAL_SPEED_H: u8 = 47;
```
- **Description**: Vitesse de déplacement
- **Plage**: 0 à 3400 step/s

#### Verrouillage EEPROM
```rust
pub const STS_LOCK: u8 = 55;
```
- **Valeurs**:
  - `0`: EEPROM déverrouillée (modifications possibles)
  - `1`: EEPROM verrouillée (protection)

---

### SRAM - Lecture seule (RO)

#### Position actuelle
```rust
pub const STS_PRESENT_POSITION_L: u8 = 56;
pub const STS_PRESENT_POSITION_H: u8 = 57;
```
- **Description**: Position actuelle du servo
- **Plage**: 0 à 4095

#### Vitesse actuelle
```rust
pub const STS_PRESENT_SPEED_L: u8 = 58;
pub const STS_PRESENT_SPEED_H: u8 = 59;
```
- **Description**: Vitesse actuelle (peut être négative)

#### Charge actuelle
```rust
pub const STS_PRESENT_LOAD_L: u8 = 60;
pub const STS_PRESENT_LOAD_H: u8 = 61;
```
- **Description**: Charge sur le servo
- **Résolution**: 0.1% par unité

#### Tension actuelle
```rust
pub const STS_PRESENT_VOLTAGE: u8 = 62;
```
- **Description**: Tension d'alimentation
- **Résolution**: 0.1V par unité

#### Température actuelle
```rust
pub const STS_PRESENT_TEMPERATURE: u8 = 63;
```
- **Description**: Température interne
- **Unité**: °C

#### Statut des capteurs
```rust
pub const STS_STATUS: u8 = 65;
```
- **Description**: État des capteurs (bit field)
- **Bits**: Voir [Bits d'erreur](#bits-derreur)

#### Mouvement en cours
```rust
pub const STS_MOVING: u8 = 66;
```
- **Valeurs**:
  - `0`: Servo arrêté
  - `1`: Servo en mouvement

#### Courant actuel
```rust
pub const STS_PRESENT_CURRENT_L: u8 = 69;
pub const STS_PRESENT_CURRENT_H: u8 = 70;
```
- **Description**: Courant consommé
- **Résolution**: 6.5 mA par unité

---

## Modes opérationnels

Le registre `STS_MODE` (33) définit le comportement du servo:

| Mode | Valeur | Nom | Description |
|------|--------|-----|-------------|
| Position | `0` | Mode position | Contrôle de position classique (0-4095) |
| Vitesse | `1` | Mode vitesse | Rotation continue à vitesse constante |
| PWM | `2` | Mode PWM | Contrôle direct du signal PWM |
| Stepper | `3` | Mode pas-à-pas | Fonctionnement en mode stepper |

### Mode 0 - Position (par défaut)
- Permet de déplacer le servo vers une position cible
- Utilise `STS_GOAL_POSITION_L/H` pour la cible
- Utilise `STS_GOAL_SPEED_L/H` pour la vitesse
- Utilise `STS_ACC` pour l'accélération

### Mode 1 - Vitesse constante
- Rotation continue dans un sens ou l'autre
- Utilise `STS_GOAL_SPEED_L/H` (le bit de signe indique le sens)
- Pas de limite de position
- Idéal pour les roues ou rotation continue

### Mode 2 - PWM
- Contrôle direct du PWM moteur
- Plus bas niveau, plus de contrôle
- Nécessite une bonne compréhension du matériel

### Mode 3 - Pas-à-pas
- Simule un moteur pas-à-pas
- Contrôle précis step par step

---

## Baudrates disponibles

```rust
pub const STS_1M: u8 = 0;        // 1 000 000 bauds (défaut)
pub const STS_0_5M: u8 = 1;      // 500 000 bauds
pub const STS_250K: u8 = 2;      // 250 000 bauds
pub const STS_128K: u8 = 3;      // 128 000 bauds
pub const STS_115200: u8 = 4;    // 115 200 bauds
pub const STS_76800: u8 = 5;     // 76 800 bauds
pub const STS_57600: u8 = 6;     // 57 600 bauds
pub const STS_38400: u8 = 7;     // 38 400 bauds
```

**Baudrate par défaut**: 1 000 000 bauds

**Note**: Le changement de baudrate nécessite de modifier l'EEPROM (déverrouiller d'abord).

---

## Bits d'erreur

Le registre `STS_STATUS` (65) contient les bits d'état:

```rust
pub const ERRBIT_VOLTAGE: u8 = 1;      // Bit 0: Erreur de tension
pub const ERRBIT_ANGLE: u8 = 2;        // Bit 1: Erreur d'angle
pub const ERRBIT_OVERHEAT: u8 = 4;     // Bit 2: Surchauffe
pub const ERRBIT_OVERELE: u8 = 8;      // Bit 3: Erreur électrique
pub const ERRBIT_OVERLOAD: u8 = 32;    // Bit 5: Surcharge
```

### Interprétation

| Bit | Nom | Description |
|-----|-----|-------------|
| 0 | Voltage | Tension hors de la plage acceptable |
| 1 | Angle | Position hors limites |
| 2 | Overheat | Température trop élevée |
| 3 | Electric | Problème électrique |
| 5 | Overload | Charge excessive sur le servo |

**Valeur 0** = OK, **Valeur 1** = Erreur

---

## Instructions protocole

```rust
pub const INST_PING: u8 = 1;           // Ping un servo
pub const INST_READ: u8 = 2;           // Lire des données
pub const INST_WRITE: u8 = 3;          // Écrire des données
pub const INST_REG_WRITE: u8 = 4;      // Écriture différée
pub const INST_ACTION: u8 = 5;         // Exécuter les écritures différées
pub const INST_SYNC_WRITE: u8 = 131;   // Écriture synchronisée (0x83)
pub const INST_SYNC_READ: u8 = 130;    // Lecture synchronisée (0x82)
```

### Structure des paquets

```
[Header0][Header1][ID][Length][Instruction][Params...][Checksum]
```

#### Positions dans le paquet
```rust
pub const PKT_HEADER_0: usize = 0;      // 0xFF
pub const PKT_HEADER_1: usize = 1;      // 0xFF
pub const PKT_ID: usize = 2;            // ID du servo
pub const PKT_LENGTH: usize = 3;        // Longueur des données
pub const PKT_INSTRUCTION: usize = 4;   // Code instruction
pub const PKT_ERROR: usize = 4;         // Code erreur (réponse)
pub const PKT_PARAMETER0: usize = 5;    // Premier paramètre
```

---

## IDs spéciaux

```rust
pub const BROADCAST_ID: u8 = 0xFE;  // 254 - Broadcast à tous les servos
pub const MAX_ID: u8 = 0xFC;        // 252 - ID maximum assignable
```

**Attention**: Le broadcast ne génère pas de réponse des servos.

---

## Constantes de communication

```rust
pub const DEFAULT_BAUDRATE: u32 = 1_000_000;  // 1 Mbaud
pub const LATENCY_TIMER: f64 = 50.0;          // Latence en ms

pub const TXPACKET_MAX_LEN: usize = 250;      // Taille max paquet TX
pub const RXPACKET_MAX_LEN: usize = 250;      // Taille max paquet RX
```

---

## Codes de résultat de communication

```rust
pub enum CommResult {
    Success = 0,           // Succès
    PortBusy = -1,        // Port occupé
    TxFail = -2,          // Échec transmission
    RxFail = -3,          // Échec réception
    TxError = -4,         // Erreur transmission
    RxWaiting = -5,       // En attente de réception
    RxTimeout = -6,       // Timeout réception
    RxCorrupt = -7,       // Données corrompues
    NotAvailable = -9,    // Non disponible
}
```

---

## Exemples d'utilisation des constantes

### Vérifier les erreurs

```rust
use st3215::values::*;

if let Some(status) = controller.read_status(1) {
    if !status["Voltage"] {
        println!("⚠️ Erreur de tension!");
    }
    if !status["Temperature"] {
        println!("⚠️ Surchauffe détectée!");
    }
}
```

### Limiter les valeurs

```rust
use st3215::values::*;

let mut position = 5000;
if position > MAX_POSITION {
    position = MAX_POSITION;
}

let mut speed = 4000;
if speed > MAX_SPEED {
    speed = MAX_SPEED;
}
```

### Calculer l'angle réel

```rust
let position: u16 = 2048;
let angle_deg = (position as f32 / 4096.0) * 360.0;
println!("Angle: {:.2}°", angle_deg);
```

---

## Notes importantes

1. **EEPROM**: Limitée en nombre d'écritures (~100,000 cycles)
2. **Verrouillage**: Toujours verrouiller l'EEPROM après modification
3. **Broadcast**: Ne pas attendre de réponse après un broadcast
4. **Baudrate**: Vérifier que le PC et le servo utilisent le même baudrate
5. **Limites**: Toujours vérifier les limites avant d'envoyer des commandes

---

**Documentation à jour pour la version 0.1.1**

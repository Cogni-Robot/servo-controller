# Interopérabilité C/C++ avec ST3215

Ce document explique comment utiliser la bibliothèque ST3215 Rust depuis du code C/C++.

## Vue d'ensemble

La bibliothèque ST3215 expose une interface FFI (Foreign Function Interface) compatible C, permettant son utilisation depuis C++ et d'autres langages supportant les ABI C.

## Architecture

### Composants principaux

1. **Module FFI** (`src/ffi.rs`) - Expose les fonctions Rust via une interface C
2. **Header C** (`include/st3215.h`) - Déclarations de fonctions pour C/C++
3. **Bibliothèque dynamique** - Compilée en `.dll` (Windows), `.so` (Linux) ou `.dylib` (macOS)

## Build

### Prérequis

- **Rust** : Installation via [rustup](https://rustup.rs/)
- **CMake** : Version 3.15 ou supérieure
- **Compilateur C++** :
  - Windows : Visual Studio 2019+ ou MinGW
  - Linux : GCC ou Clang
  - macOS : Xcode Command Line Tools

### Compilation rapide

#### Windows
```batch
build.bat
```

#### Linux/macOS
```bash
chmod +x build.sh
./build.sh
```

### Compilation manuelle

1. **Compiler la bibliothèque Rust** :
   ```bash
   cargo build --release
   ```

2. **Le header est généré automatiquement** lors du build dans `include/st3215.h`

3. **Compiler l'exemple C++** :
   ```bash
   mkdir build && cd build
   cmake -DCMAKE_BUILD_TYPE=Release ../examples/cpp
   cmake --build . --config Release
   ```

## Utilisation dans votre projet C++

### 1. Inclure le header

```cpp
#include "st3215.h"
```

### 2. Créer une instance

```cpp
ST3215Handle* handle = st3215_new("COM3");  // Windows
// ou
ST3215Handle* handle = st3215_new("/dev/ttyUSB0");  // Linux
```

### 3. Utiliser les fonctions

```cpp
// Ping un servo
int32_t found = st3215_ping_servo(handle, 1);

// Lire la position
uint16_t position;
if (st3215_read_position(handle, 1, &position) == 0) {
    std::cout << "Position: " << position << std::endl;
}

// Déplacer le servo
st3215_move_to(handle, 1, 2048, 1000, 50);

// Vérifier si en mouvement
int32_t moving = st3215_is_moving(handle, 1);
```

### 4. Libérer les ressources

```cpp
st3215_free(handle);
```

## API Reference

### Gestion de l'instance

#### `st3215_new`
```c
ST3215Handle* st3215_new(const char* device);
```
Crée une nouvelle instance ST3215. Retourne `NULL` en cas d'erreur.

#### `st3215_free`
```c
void st3215_free(ST3215Handle* handle);
```
Libère une instance ST3215.

### Scan et diagnostic

#### `st3215_ping_servo`
```c
int32_t st3215_ping_servo(ST3215Handle* handle, uint8_t servo_id);
```
Vérifie la présence d'un servo. Retourne 1 si trouvé, 0 sinon.

#### `st3215_list_servos`
```c
size_t st3215_list_servos(ST3215Handle* handle, uint8_t* out_ids, size_t max_ids);
```
Liste tous les servos connectés. Retourne le nombre de servos trouvés.

### Contrôle de mouvement

#### `st3215_move_to`
```c
int32_t st3215_move_to(ST3215Handle* handle, uint8_t servo_id, 
                        uint16_t position, uint16_t speed, uint8_t acceleration);
```
Déplace un servo vers une position cible.
- `position` : 0-4095
- `speed` : 0-4095
- `acceleration` : 0-254

#### `st3215_is_moving`
```c
int32_t st3215_is_moving(ST3215Handle* handle, uint8_t servo_id);
```
Vérifie si un servo est en mouvement. Retourne 1 si en mouvement, 0 si arrêté, -1 en cas d'erreur.

#### `st3215_enable_torque`
```c
int32_t st3215_enable_torque(ST3215Handle* handle, uint8_t servo_id, int32_t enable);
```
Active ou désactive le couple d'un servo.

### Lecture de données

#### `st3215_read_position`
```c
int32_t st3215_read_position(ST3215Handle* handle, uint8_t servo_id, uint16_t* out_position);
```
Lit la position actuelle (0-4095).

#### `st3215_read_speed`
```c
int32_t st3215_read_speed(ST3215Handle* handle, uint8_t servo_id, uint16_t* out_speed);
```
Lit la vitesse actuelle.

#### `st3215_read_voltage`
```c
int32_t st3215_read_voltage(ST3215Handle* handle, uint8_t servo_id, float* out_voltage);
```
Lit la tension en volts.

#### `st3215_read_current`
```c
int32_t st3215_read_current(ST3215Handle* handle, uint8_t servo_id, float* out_current);
```
Lit le courant en mA.

#### `st3215_read_temperature`
```c
int32_t st3215_read_temperature(ST3215Handle* handle, uint8_t servo_id, uint8_t* out_temperature);
```
Lit la température en °C.

#### `st3215_read_load`
```c
int32_t st3215_read_load(ST3215Handle* handle, uint8_t servo_id, float* out_load);
```
Lit la charge en pourcentage.

### Utilitaires

#### `st3215_version`
```c
char* st3215_version(void);
```
Retourne la version de la bibliothèque. La chaîne doit être libérée avec `st3215_free_string`.

#### `st3215_free_string`
```c
void st3215_free_string(char* s);
```
Libère une chaîne allouée par la bibliothèque.

## Exemple complet

Voir `examples/cpp/example.cpp` pour un exemple complet démontrant :
- Connexion au port série
- Scan des servos
- Lecture des informations
- Contrôle de mouvement
- Gestion des erreurs

## Configuration CMake

Pour intégrer la bibliothèque dans votre projet CMake :

```cmake
# Définir le chemin vers la bibliothèque
set(ST3215_ROOT "/path/to/servo-controller")
set(ST3215_INCLUDE "${ST3215_ROOT}/include")
set(ST3215_LIB "${ST3215_ROOT}/target/release")

# Ajouter l'include
target_include_directories(your_target PRIVATE ${ST3215_INCLUDE})

# Lier la bibliothèque
if(WIN32)
    target_link_libraries(your_target PRIVATE "${ST3215_LIB}/st3215.dll.lib")
else()
    target_link_libraries(your_target PRIVATE "${ST3215_LIB}/libst3215.so")
endif()
```

## Gestion des erreurs

Les fonctions retournent généralement :
- `0` en cas de succès
- `-1` en cas d'erreur
- `NULL` pour les pointeurs en cas d'échec

Toujours vérifier les valeurs de retour avant d'utiliser les données.

## Thread Safety

L'instance `ST3215Handle` utilise des mutex en interne pour protéger l'accès au port série. Vous pouvez appeler les fonctions depuis plusieurs threads, mais gardez à l'esprit que les opérations série sont séquentielles.

## Notes importantes

1. **Port série** : Assurez-vous d'avoir les permissions nécessaires sur le port série (Linux/macOS)
2. **Baudrate** : Le baudrate par défaut est 1000000 bps
3. **Timeout** : Les opérations ont un timeout de 50ms par défaut
4. **Ressources** : Toujours appeler `st3215_free()` pour libérer les ressources

## Dépannage

### Windows : DLL introuvable
Copiez `st3215.dll` dans le même répertoire que votre exécutable, ou ajoutez le chemin à la variable PATH.

### Linux : Erreur de permission
```bash
sudo usermod -a -G dialout $USER
# Déconnexion/reconnexion nécessaire
```

### macOS : Bibliothèque non signée
```bash
xattr -d com.apple.quarantine libst3215.dylib
```

## Support

Pour plus d'informations, consultez :
- README.md du projet
- Documentation Rust : `cargo doc --open`
- Exemples dans `examples/`

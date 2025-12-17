#include <iostream>
#include <cstdint>
#include <cstring>
#include <thread>
#include <chrono>
#include "../../include/st3215.h"

int main() {
    std::cout << "ST3215 Servo Controller - Exemple C++" << std::endl;
    
    // Obtenir la version de la bibliothèque
    char* version = st3215_version();
    std::cout << "Version: " << version << std::endl;
    st3215_free_string(version);
    
    // Créer une nouvelle instance
    // Modifier le port série selon votre configuration:
    // - Windows: "COM3", "COM4", etc.
    // - Linux: "/dev/ttyUSB0", "/dev/ttyACM0", etc.
    const char* port = "COM3";
    
    ST3215Handle* handle = st3215_new(port);
    if (handle == nullptr) {
        std::cerr << "Erreur: Impossible d'ouvrir le port " << port << std::endl;
        return 1;
    }
    
    std::cout << "Port " << port << " ouvert avec succès" << std::endl;
    
    // Scanner les servos disponibles
    std::cout << "\nRecherche des servos connectés..." << std::endl;
    uint8_t servo_ids[253];
    size_t servo_count = st3215_list_servos(handle, servo_ids, 253);
    
    if (servo_count == 0) {
        std::cout << "Aucun servo trouvé!" << std::endl;
        st3215_free(handle);
        return 1;
    }
    
    std::cout << "Servos trouvés (" << servo_count << "): ";
    for (size_t i = 0; i < servo_count; i++) {
        std::cout << (int)servo_ids[i];
        if (i < servo_count - 1) std::cout << ", ";
    }
    std::cout << std::endl;
    
    // Utiliser le premier servo trouvé
    uint8_t servo_id = servo_ids[0];
    std::cout << "\nUtilisation du servo ID: " << (int)servo_id << std::endl;
    
    // Activer le couple
    if (st3215_enable_torque(handle, servo_id, 1) == 0) {
        std::cout << "Couple activé" << std::endl;
    } else {
        std::cerr << "Erreur lors de l'activation du couple" << std::endl;
    }
    
    // Lire les informations du servo
    std::cout << "\n=== Informations du servo ===" << std::endl;
    
    uint16_t position;
    if (st3215_read_position(handle, servo_id, &position) == 0) {
        std::cout << "Position: " << position << " / 4095" << std::endl;
    }
    
    float voltage;
    if (st3215_read_voltage(handle, servo_id, &voltage) == 0) {
        std::cout << "Tension: " << voltage << " V" << std::endl;
    }
    
    uint8_t temperature;
    if (st3215_read_temperature(handle, servo_id, &temperature) == 0) {
        std::cout << "Température: " << (int)temperature << " °C" << std::endl;
    }
    
    float load;
    if (st3215_read_load(handle, servo_id, &load) == 0) {
        std::cout << "Charge: " << load << " %" << std::endl;
    }
    
    float current;
    if (st3215_read_current(handle, servo_id, &current) == 0) {
        std::cout << "Courant: " << current << " mA" << std::endl;
    }
    
    // Effectuer quelques mouvements
    std::cout << "\n=== Test de mouvement ===" << std::endl;
    
    // Position 1: Centre
    std::cout << "Déplacement vers la position centrale (2048)..." << std::endl;
    if (st3215_move_to(handle, servo_id, 2048, 1000, 50) == 0) {
        // Attendre que le mouvement soit terminé
        int32_t moving;
        do {
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
            moving = st3215_is_moving(handle, servo_id);
        } while (moving == 1);
        
        if (st3215_read_position(handle, servo_id, &position) == 0) {
            std::cout << "Position atteinte: " << position << std::endl;
        }
    }
    
    std::this_thread::sleep_for(std::chrono::seconds(1));
    
    // Position 2: Gauche
    std::cout << "\nDéplacement vers la gauche (1024)..." << std::endl;
    if (st3215_move_to(handle, servo_id, 1024, 1500, 100) == 0) {
        int32_t moving;
        do {
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
            moving = st3215_is_moving(handle, servo_id);
        } while (moving == 1);
        
        if (st3215_read_position(handle, servo_id, &position) == 0) {
            std::cout << "Position atteinte: " << position << std::endl;
        }
    }
    
    std::this_thread::sleep_for(std::chrono::seconds(1));
    
    // Position 3: Droite
    std::cout << "\nDéplacement vers la droite (3072)..." << std::endl;
    if (st3215_move_to(handle, servo_id, 3072, 1500, 100) == 0) {
        int32_t moving;
        do {
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
            moving = st3215_is_moving(handle, servo_id);
        } while (moving == 1);
        
        if (st3215_read_position(handle, servo_id, &position) == 0) {
            std::cout << "Position atteinte: " << position << std::endl;
        }
    }
    
    // Retour au centre
    std::cout << "\nRetour au centre..." << std::endl;
    st3215_move_to(handle, servo_id, 2048, 1000, 50);
    std::this_thread::sleep_for(std::chrono::seconds(2));
    
    // Désactiver le couple
    std::cout << "\nDésactivation du couple..." << std::endl;
    st3215_enable_torque(handle, servo_id, 0);
    
    // Libérer les ressources
    st3215_free(handle);
    
    std::cout << "\nTest terminé avec succès!" << std::endl;
    return 0;
}

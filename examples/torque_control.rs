/// Exemple d'utilisation du contrôle du torque des servos ST3215
/// 
/// Cet exemple montre comment activer et désactiver le torque d'un servo

use st3215::ST3215;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), String> {
    // Remplacer "/dev/ttyUSB0" par le port série approprié
    let device = std::env::var("SERVO_PORT").unwrap_or_else(|_| "/dev/ttyUSB0".to_string());
    
    println!("Connexion au port série: {}", device);
    let controller = ST3215::new(&device)?;
    
    // ID du servo à contrôler (modifier selon votre configuration)
    let servo_id: u8 = 1;
    
    // Vérifier que le servo est présent
    println!("Vérification de la présence du servo {}...", servo_id);
    if !controller.ping_servo(servo_id) {
        return Err(format!("Servo {} non trouvé!", servo_id));
    }
    println!("✓ Servo {} trouvé", servo_id);
    
    // Exemple 1: Activer le torque
    println!("\n--- Activation du torque ---");
    controller.enable_torque(servo_id)?;
    println!("✓ Torque activé pour le servo {}", servo_id);
    
    // Le servo maintient maintenant sa position
    thread::sleep(Duration::from_secs(2));
    
    // Lire la position actuelle
    if let Some(position) = controller.read_position(servo_id) {
        println!("Position actuelle: {}", position);
    }
    
    // Exemple 2: Désactiver le torque
    println!("\n--- Désactivation du torque ---");
    controller.disable_torque(servo_id)?;
    println!("✓ Torque désactivé pour le servo {}", servo_id);
    println!("Le servo peut maintenant être déplacé manuellement");
    
    // Attendre quelques secondes
    thread::sleep(Duration::from_secs(3));
    
    // Exemple 3: Réactiver le torque et déplacer le servo
    println!("\n--- Réactivation et déplacement ---");
    controller.enable_torque(servo_id)?;
    println!("✓ Torque réactivé");
    
    // Déplacer vers la position 2048 (milieu de la course)
    let target_position = 2048;
    let speed = 1000;
    let acceleration = 50;
    println!("Déplacement vers la position {}...", target_position);
    controller.move_to(servo_id, target_position, speed, acceleration, false);
    
    // Attendre que le mouvement soit terminé
    thread::sleep(Duration::from_secs(2));
    
    // Vérifier la position finale
    if let Some(final_position) = controller.read_position(servo_id) {
        println!("Position finale: {}", final_position);
    }
    
    // Exemple 4: Cycle d'activation/désactivation
    println!("\n--- Cycle d'activation/désactivation ---");
    for i in 1..=3 {
        println!("Cycle {}/3", i);
        
        println!("  Désactivation...");
        controller.disable_torque(servo_id)?;
        thread::sleep(Duration::from_millis(500));
        
        println!("  Activation...");
        controller.enable_torque(servo_id)?;
        thread::sleep(Duration::from_millis(500));
    }
    
    // Désactiver le torque à la fin
    println!("\n--- Fin du programme ---");
    controller.disable_torque(servo_id)?;
    println!("✓ Torque désactivé");
    
    Ok(())
}
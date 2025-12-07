
use st3215::ST3215;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Créer une instance ST3215 connectée au port COM3
    let servo = ST3215::new("COM3")?;

    // Lister tous les servos disponibles
    let ids = servo.list_servos();
    println!("Servos trouvés: {:?}", ids);

    // Pour chaque servo trouvé
    for id in ids {
        println!("\n=== Servo {} ===", id);
        
        // Lire et afficher les informations du servo
        if let Some(position) = servo.read_position(id) {
            println!("Position actuelle: {}", position);
        }
        
        if let Some(voltage) = servo.read_voltage(id) {
            println!("Tension: {:.1} V", voltage);
        }
        
        if let Some(temp) = servo.read_temperature(id) {
            println!("Température: {} °C", temp);
        }
        
        if let Some(current) = servo.read_current(id) {
            println!("Courant: {:.1} mA", current);
        }

        // Déplacer le servo vers la position
        let deg: u16 = 65;
        let pos: u16 = deg * (4096.0 / 360.0) as u16;

        println!("Déplacement vers la position {}...", deg);

        servo.move_to(id, pos, 2400, 50, true);
        
        println!("Mouvement terminé!");
    }

    Ok(())
}

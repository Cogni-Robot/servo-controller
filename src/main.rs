use st3215::ST3215;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Créer une instance ST3215 connectée au port COM3
    let servo = ST3215::new("COM3")?;

    // Lister tous les servos disponibles
    let ids = servo.list_servos();
    println!("Servos trouvés: {:?}", ids);

    // Déplacer chaque servo vers la position 2048
    for id in ids {
        println!("Servo: {}", id);
        servo.move_to(id, 2048, 2400, 50, false);
    }

    Ok(())
}

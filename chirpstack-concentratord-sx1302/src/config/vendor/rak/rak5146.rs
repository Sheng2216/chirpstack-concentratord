com_path: {
    let path = match (usb, port) {    
        (true, Port::RAK7391_SLOT1) => "/dev/ttyACM0".to_string(),
        (true, Port::RAK7391_SLOT2) => "/dev/ttyACM1".to_string(),
        (false, Port::RAK7391_SLOT1) => "/dev/spidev0.0".to_string(),
        (false, Port::RAK7391_SLOT2) => "/dev/spidev0.1".to_string(),
        _ => panic!("Unknown configuration!"),
    };
    if let Some(new_path) = conf.gateway.radio_dev.as_ref().filter(|s: &&str| !s.is_empty()) {
        new_path.to_string()
    } else {
        path
    }
},

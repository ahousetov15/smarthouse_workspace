use smarthouse::*;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn get_dev_info_log(sm: &Smarthouse, room_name: &str, device_name: &str, storage: &DeviceStorage) {
    match sm.get_device_info(room_name, device_name, storage) {
        Ok(resp) => info!("{}", resp),
        Err(DeviceStorageGetInfoErrors::NoDevice(error)) => error!("{}", error),
        Err(DeviceStorageGetInfoErrors::NoRoom(error)) => error!("{}", error),
    }
}

fn main() {
    pretty_env_logger::init_timed();
    // Создаем имена для наших устройств в разных комнатах
    let socket_bedroom_name = String::from("Розетка в спальне");
    let socket_bedroom_bathroom_name = String::from("Розетка у ванны в спальне");
    let thermo_bedroom_name = String::from("Термометр в спальне");
    let socket_kitchen_name = String::from("Розетка над столом кухни");
    let socket_kitchen_stove_name = String::from("Розетка у плиты");

    // Создаем имена наших комнат
    let bedroom_name = String::from("Спальня");
    let kitchen_name = String::from("Кухня");

    let mut storage: DeviceStorage = DeviceStorage::new(DeviceStorage {
        room_map: HashMap::new(),
    });

    let socket_bedroom = Arc::new(Socket::new(Socket {
        name: socket_bedroom_name.clone(),
        power: 220.0,
        state: SocketState::IsOff,
    }));
    let socket_bedroom_bathroom = Arc::new(Socket::new(Socket {
        name: socket_bedroom_bathroom_name.clone(),
        power: 210.0,
        state: SocketState::IsOn,
    }));
    let thermo_bedroom = Arc::new(Thermometer::new(Thermometer {
        name: thermo_bedroom_name.clone(),
        temperature: 22,
    }));
    let socket_kitchen = Arc::new(Socket::new(Socket {
        name: socket_kitchen_name.clone(),
        power: 220.0,
        state: SocketState::IsOff,
    }));
    let socket_kitchen_stove = Arc::new(Socket::new(Socket {
        name: socket_kitchen_stove_name.clone(),
        power: 210.0,
        state: SocketState::IsOn,
    }));

    // Создали умный дом
    let mut smarthouse = Smarthouse::new("Домашная работа 3", &storage);

    match smarthouse.add_device_in_room(&bedroom_name, socket_bedroom, &mut storage) {
        Ok(msg) => info!("{}", msg),
        Err(error) => error!("{:?}", error),
    }
    match smarthouse.add_device_in_room(&bedroom_name, socket_bedroom_bathroom, &mut storage) {
        Ok(msg) => info!("{}", msg),
        Err(error) => error!("{:?}", error),
    }
    match smarthouse.add_device_in_room(&bedroom_name, thermo_bedroom, &mut storage) {
        Ok(msg) => info!("{}", msg),
        Err(error) => error!("{:?}", error),
    }
    match smarthouse.add_device_in_room(&kitchen_name, socket_kitchen, &mut storage) {
        Ok(msg) => info!("{}", msg),
        Err(error) => error!("{:?}", error),
    }
    match smarthouse.add_device_in_room(&kitchen_name, socket_kitchen_stove, &mut storage) {
        Ok(msg) => info!("{}", msg),
        Err(error) => error!("{:?}", error),
    }
    // Проверяем список комнат
    log::info!("{}", smarthouse.get_roooms_list());

    info!("{}", smarthouse.get_rooms_devices_list(&bedroom_name));

    // Проверяем список устройств на кухне
    info!("{}", smarthouse.get_rooms_devices_list(&kitchen_name));

    // Проверяем отчеты по устройствам
    get_dev_info_log(&smarthouse, &bedroom_name, &socket_bedroom_name, &storage);
    get_dev_info_log(
        &smarthouse,
        &bedroom_name,
        &socket_bedroom_bathroom_name,
        &storage,
    );
    get_dev_info_log(&smarthouse, &bedroom_name, &thermo_bedroom_name, &storage);
    get_dev_info_log(
        &smarthouse,
        &bedroom_name,
        "Неведомая хрень".to_string().as_str(),
        &storage,
    );

    warn!("Начинаем проверку УДАЛЕНИЯ устройств и комнат!!!");
    match smarthouse.remove(&bedroom_name, Some(&socket_bedroom_name), &mut storage) {
        Ok(msg) => info!("{}", msg),
        Err(error) => error!("{:?}", error),
    }
    match smarthouse.remove(
        &kitchen_name,
        Some(&socket_kitchen_stove_name),
        &mut storage,
    ) {
        Ok(msg) => info!("{}", msg),
        Err(error) => error!("{:?}", error),
    }
    match smarthouse.remove(&kitchen_name, None, &mut storage) {
        Ok(msg) => {
            info!("{}", msg);
            if smarthouse.get_roooms_list().contains(&kitchen_name) {
                error!("Комана '{}' не удалена", kitchen_name);
                // assert!(false);
            } else {
                info!("Комана '{}' удалена успешно", kitchen_name);
                // assert!(false);
            }
        }
        Err(error) => error!("{:?}", error),
    }
}

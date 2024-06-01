use clap::{App, Arg};
use fast_qr::qr::QRBuilder;
use mac_conditions;
use prettytable::table;
use systemd_info;

mod redis;

fn main() {
    let matches = App::new("macgui")
        .version("1.0.0")
        .author("h13317136163@163.com")
        .about("MAC地址采集程序")
        .arg(
            Arg::with_name("ip")
                .short("i")
                .long("ip")
                .value_name("IP_ADDRESS")
                .help("Redis数据库地址")
                .default_value("redis://127.0.0.1:6379/0"),
        )
        .get_matches();
    let ip_address = matches.value_of("ip").unwrap();
    let serial_number = systemd_info::get_bios_serial_number().unwrap();
    let (wired_mac, wireless_mac, bluetooth_mac) = mac_conditions::get_mac_addresses();

    let redis_status = redis::write_mac_to_redis(
        &ip_address,
        &serial_number,
        &wired_mac,
        &wireless_mac,
        &bluetooth_mac,
    );

    let mac_qr: String = format!(
        "{},{},{},{}",
        serial_number, wired_mac, wireless_mac, bluetooth_mac
    );
    let qrcode = QRBuilder::new(&*mac_qr).build().unwrap();

    let mac_qr_str = qrcode.to_str();
    let table1 = table!(
        ["类型", "MAC地址"],
        ["有线", wired_mac],
        ["无线", wireless_mac],
        ["蓝牙", bluetooth_mac],
        ["序列号", serial_number]
    );

    let table2 = table!([table1, mac_qr_str]);

    table2.printstd();
    println!("{}", redis_status)
}

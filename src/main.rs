extern crate libappindicator;
extern crate gtk;
extern crate zbus;

mod upower;
mod device;

use upower::UPowerProxy;
use device::DeviceProxy;

use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};
use zbus::Connection;

fn main() {
    let system_bus = Connection::new_system().unwrap();
    let upower_proxy = UPowerProxy::new(&system_bus).unwrap();
    let devices = upower_proxy.enumerate_devices().unwrap();
    let device_proxies: Vec<DeviceProxy> = devices.into_iter().map(|d| {
        return DeviceProxy::new_for_owned(system_bus.clone(),
                                   String::from("org.freedesktop.UPower"),
                                   String::from(d.into_inner().as_str())).unwrap();
    } ).collect();

    gtk::init().unwrap();
    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::Active);
    indicator.set_icon_full("battery", "icon");
    let mut m = gtk::Menu::new();
    let quit = gtk::MenuItem::with_label("Quit");
    for proxy in device_proxies {
        let p: DeviceProxy = proxy;
        let mi = gtk::MenuItem::with_label(format!("{} - {}%",
                                                   &p.model().unwrap().to_string(),
                                                   &p.percentage().unwrap()).as_str());
        m.append(&mi);
    }
    quit.connect_activate(|_| {
        gtk::main_quit();
    });
    m.append(&quit);
    indicator.set_menu(&mut m);
    m.show_all();
    gtk::main();
}

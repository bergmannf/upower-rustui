extern crate zbus;

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.DBus.Properties")]
trait Properties {
    /// Get method
    fn get(&self, interface_name: &str, property_name: &str) -> zbus::Result<zvariant::OwnedValue>;

    /// GetAll method
    fn get_all(
        &self,
        interface_name: &str,
    ) -> zbus::Result<std::collections::HashMap<String, zvariant::OwnedValue>>;

    /// Set method
    fn set(
        &self,
        interface_name: &str,
        property_name: &str,
        value: &zvariant::Value,
    ) -> zbus::Result<()>;
}
#[dbus_proxy(interface = "org.freedesktop.DBus.Introspectable")]
trait Introspectable {
    /// Introspect method
    fn introspect(&self) -> zbus::Result<String>;
}
#[dbus_proxy(interface = "org.freedesktop.DBus.Peer")]
trait Peer {
    /// GetMachineId method
    fn get_machine_id(&self) -> zbus::Result<String>;

    /// Ping method
    fn ping(&self) -> zbus::Result<()>;
}
#[dbus_proxy(interface = "org.freedesktop.UPower.Device")]
trait Device {
    /// GetHistory method
    fn get_history(
        &self,
        r#type: &str,
        timespan: u32,
        resolution: u32,
    ) -> zbus::Result<Vec<(u32, f64, u32)>>;

    /// GetStatistics method
    fn get_statistics(&self, r#type: &str) -> zbus::Result<Vec<(f64, f64)>>;

    /// Refresh method
    fn refresh(&self) -> zbus::Result<()>;

    /// BatteryLevel property
    #[dbus_proxy(property)]
    fn battery_level(&self) -> zbus::fdo::Result<u32>;

    /// Capacity property
    #[dbus_proxy(property)]
    fn capacity(&self) -> zbus::fdo::Result<f64>;

    /// Energy property
    #[dbus_proxy(property)]
    fn energy(&self) -> zbus::fdo::Result<f64>;

    /// EnergyEmpty property
    #[dbus_proxy(property)]
    fn energy_empty(&self) -> zbus::fdo::Result<f64>;

    /// EnergyFull property
    #[dbus_proxy(property)]
    fn energy_full(&self) -> zbus::fdo::Result<f64>;

    /// EnergyFullDesign property
    #[dbus_proxy(property)]
    fn energy_full_design(&self) -> zbus::fdo::Result<f64>;

    /// EnergyRate property
    #[dbus_proxy(property)]
    fn energy_rate(&self) -> zbus::fdo::Result<f64>;

    /// HasHistory property
    #[dbus_proxy(property)]
    fn has_history(&self) -> zbus::fdo::Result<bool>;

    /// HasStatistics property
    #[dbus_proxy(property)]
    fn has_statistics(&self) -> zbus::fdo::Result<bool>;

    /// IconName property
    #[dbus_proxy(property)]
    fn icon_name(&self) -> zbus::fdo::Result<String>;

    /// IsPresent property
    #[dbus_proxy(property)]
    fn is_present(&self) -> zbus::fdo::Result<bool>;

    /// IsRechargeable property
    #[dbus_proxy(property)]
    fn is_rechargeable(&self) -> zbus::fdo::Result<bool>;

    /// Luminosity property
    #[dbus_proxy(property)]
    fn luminosity(&self) -> zbus::fdo::Result<f64>;

    /// Model property
    #[dbus_proxy(property)]
    fn model(&self) -> zbus::fdo::Result<String>;

    /// NativePath property
    #[dbus_proxy(property)]
    fn native_path(&self) -> zbus::fdo::Result<String>;

    /// Online property
    #[dbus_proxy(property)]
    fn online(&self) -> zbus::fdo::Result<bool>;

    /// Percentage property
    #[dbus_proxy(property)]
    fn percentage(&self) -> zbus::fdo::Result<f64>;

    /// PowerSupply property
    #[dbus_proxy(property)]
    fn power_supply(&self) -> zbus::fdo::Result<bool>;

    /// Serial property
    #[dbus_proxy(property)]
    fn serial(&self) -> zbus::fdo::Result<String>;

    /// State property
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::fdo::Result<u32>;

    /// Technology property
    #[dbus_proxy(property)]
    fn technology(&self) -> zbus::fdo::Result<u32>;

    /// Temperature property
    #[dbus_proxy(property)]
    fn temperature(&self) -> zbus::fdo::Result<f64>;

    /// TimeToEmpty property
    #[dbus_proxy(property)]
    fn time_to_empty(&self) -> zbus::fdo::Result<i64>;

    /// TimeToFull property
    #[dbus_proxy(property)]
    fn time_to_full(&self) -> zbus::fdo::Result<i64>;

    /// UpdateTime property
    #[dbus_proxy(property)]
    fn update_time(&self) -> zbus::fdo::Result<u64>;

    /// Vendor property
    #[dbus_proxy(property)]
    fn vendor(&self) -> zbus::fdo::Result<String>;

    /// Voltage property
    #[dbus_proxy(property)]
    fn voltage(&self) -> zbus::fdo::Result<f64>;

    /// WarningLevel property
    #[dbus_proxy(property)]
    fn warning_level(&self) -> zbus::fdo::Result<u32>;
}

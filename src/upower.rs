extern crate zbus;
extern crate zvariant;

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
#[dbus_proxy(interface = "org.freedesktop.UPower")]
trait UPower {
    /// EnumerateDevices method
    fn enumerate_devices(&self) -> zbus::Result<Vec<zvariant::OwnedObjectPath>>;

    /// GetCriticalAction method
    fn get_critical_action(&self) -> zbus::Result<String>;

    /// GetDisplayDevice method
    fn get_display_device(&self) -> zbus::Result<zvariant::OwnedObjectPath>;

    /// DaemonVersion property
    #[dbus_proxy(property)]
    fn daemon_version(&self) -> zbus::fdo::Result<String>;

    /// LidIsClosed property
    #[dbus_proxy(property)]
    fn lid_is_closed(&self) -> zbus::fdo::Result<bool>;

    /// LidIsPresent property
    #[dbus_proxy(property)]
    fn lid_is_present(&self) -> zbus::fdo::Result<bool>;

    /// OnBattery property
    #[dbus_proxy(property)]
    fn on_battery(&self) -> zbus::fdo::Result<bool>;
}

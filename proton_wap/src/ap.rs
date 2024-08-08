//! Wireless access point abstraction.

use network_manager::{
    Device as NmDevice,
    DeviceType,
    NetworkManager,
    Connection,
    ConnectionState,
};

use proton_cfg::HotspotConfig;

use proton_dev::{
    Device,
    DeviceManager,
};

use proton_err::{
    ProtonResult,
    ProtonError,
};

/// A wireless access point.
pub struct AccessPoint {
    /// Device discovery manager.
    device_manager: DeviceManager,

    /// Hotspot configuration information.
    pub config: HotspotConfig,
}

impl AccessPoint {
    /// Constructs a new wireless access point.
    /// 
    /// # Parameters
    /// - `wlifname` (`&str`): the name of the wireless interface over which
    /// this access point connects to remote devices (e.g. "wlan0")
    /// - `config` (`HotspotConfig`): hotspot configuration options
    /// 
    /// # Returns
    /// A `ProtonResult<AccessPoint>` containing a new `AccessPoint` if
    /// initialization was successful.
    pub async fn new(
        wlifname: &str,
        config: HotspotConfig,
    ) -> ProtonResult<Self> {
        // Initialize NetworkManager API
        let network_manager = NetworkManager::new();

        // Is this device a Wi-Fi device?
        let check_if_wifi_device = |device: &NmDevice| *device.device_type() == DeviceType::WiFi;

        // Get Wi-Fi device
        let device = network_manager.get_devices()
            .unwrap_or_default()
            .into_iter()
            .find(check_if_wifi_device)
            .ok_or(ProtonError::CouldNotFindWirelessInterface)?;

        // Convert to Wi-Fi device
        let wifi_device = device.as_wifi_device()
            .ok_or(ProtonError::CouldNotFindWirelessInterface)?;

        // Make sure gateway is in the CIDR range
        if !config.cidr.contains(&config.gateway) {
            return Err (ProtonError::CidrMustContainGateway {
                cidr: config.cidr.to_string(),
                gateway: config.gateway.to_string(),
            });
        }

        // Create a hotspot on the selected device
        let (connection, _state) = wifi_device.create_hotspot_advanced::<str>(
            config.ssid.as_str(),
            Some (config.pass.as_str()),
            config.gateway,
            config.security.as_str(),
            config.band.as_str(),
        )?;

        // Activate the hotspot
        if ConnectionState::Activated != connection.activate()? {
            return Err (ProtonError::CouldNotActivateHotspot);
        }

        Ok (Self {
            device_manager: DeviceManager::new(config.cidr, wlifname)?,
            config,
        })
    }

    /// Get the NetworkManager hotspot abstraction.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `ProtonResult<Connection>` containing the connection,
    /// if the method could successfully construct it.
    fn get_hotspot(&mut self) -> ProtonResult<Connection> {
        // Initialize NetworkManager API
        let nm = NetworkManager::new();

        // Check for hotspot
        let check_if_ap = |connection: &Connection| connection.settings().mode.as_str() == "ap";

        nm.get_connections()
            .unwrap_or_default()
            .into_iter()
            .find(check_if_ap)
            .ok_or(ProtonError::HotspotNotInitialized)
    }

    /// Activate the hotspot.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `ProtonResult<()>` indicating whether or not the activation
    /// was successful.
    pub async fn activate(&mut self) -> ProtonResult<()> {
        self.get_hotspot()?.activate()?;

        Ok (())
    }

    /// Deactivate the hotspot.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `ProtonResult<()>` indicating whether or not the deactivation
    /// was successful.
    pub async fn deactivate(&mut self) -> ProtonResult<()> {
        self.get_hotspot()?.deactivate()?;

        Ok (())
    }

    /// Delete the hotspot.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `ProtonResult<()>` indicating whether or not the deletion
    /// was successful.
    pub async fn delete(&mut self) -> ProtonResult<()> {
        self.get_hotspot()?.delete()?;

        Ok (())
    }

    /// Get a list of all connected devices.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `ProtonResult<Vec<Device>>` wrappping the list of devices, if
    /// the network scan was successful.
    pub async fn scan(&mut self) -> ProtonResult<Vec<Device>> {
        Ok (self.device_manager.scan().await?)
    }
}
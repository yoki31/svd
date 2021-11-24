use super::{
    BuildError, Cpu, EmptyToNone, Peripheral, RegisterProperties, SvdError, ValidateLevel,
};

/// Errors for [`Device::validate`]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Device has no peripherals
    #[error("Device must contain at least one peripheral")]
    EmptyDevice,
}

/// The top element in a SVD file. Describes information specific to a device.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Device {
    /// Specify the vendor of the device using the full name
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub vendor: Option<String>,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none", rename = "vendorID")
    )]
    pub vendor_id: Option<String>,

    /// The string identifies the device or device series. Device names are required to be unique
    pub name: String,

    /// Specify the name of the device series
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub series: Option<String>,

    /// Define the version of the SVD file
    pub version: String,

    /// Describe the main features of the device (for example CPU, clock frequency, peripheral overview)
    pub description: String,

    /// The text will be copied into the header section of the generated device header file and shall contain the legal disclaimer
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub license_text: Option<String>,

    /// Describe the processor included in the device
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub cpu: Option<Cpu>,

    /// Specify the file name (without extension) of the device-specific system include file
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub header_system_filename: Option<String>,

    /// This string is prepended to all type definition names generated in the CMSIS-Core device header file
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub header_definitions_prefix: Option<String>,

    /// Define the number of data bits uniquely selected by each address
    pub address_unit_bits: u32,

    /// Define the number of data bit-width of the maximum single data transfer supported by the bus infrastructure
    pub width: u32,

    /// Default properties for all registers
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub default_register_properties: RegisterProperties,

    /// Group to define peripherals
    pub peripherals: Vec<Peripheral>,

    /// Specify the underlying XML schema to which the CMSIS-SVD schema is compliant.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub xmlns_xs: String,

    /// Specify the file path and file name of the CMSIS-SVD Schema
    #[cfg_attr(feature = "serde", serde(skip))]
    pub no_namespace_schema_location: String,

    /// Specify the compliant CMSIS-SVD schema version
    #[cfg_attr(feature = "serde", serde(skip))]
    pub schema_version: String,
}

/// Builder for [`Device`]
#[derive(Clone, Debug, Default)]
pub struct DeviceBuilder {
    vendor: Option<String>,
    vendor_id: Option<String>,
    name: Option<String>,
    series: Option<String>,
    version: Option<String>,
    description: Option<String>,
    license_text: Option<String>,
    cpu: Option<Cpu>,
    header_system_filename: Option<String>,
    header_definitions_prefix: Option<String>,
    address_unit_bits: Option<u32>,
    width: Option<u32>,
    default_register_properties: RegisterProperties,
    peripherals: Option<Vec<Peripheral>>,
    xmlns_xs: Option<String>,
    no_namespace_schema_location: Option<String>,
    schema_version: Option<String>,
}

impl From<Device> for DeviceBuilder {
    fn from(d: Device) -> Self {
        Self {
            vendor: d.vendor,
            vendor_id: d.vendor_id,
            name: Some(d.name),
            series: d.series,
            version: Some(d.version),
            description: Some(d.description),
            cpu: d.cpu,
            address_unit_bits: Some(d.address_unit_bits),
            width: Some(d.width),
            default_register_properties: d.default_register_properties,
            peripherals: Some(d.peripherals),
            xmlns_xs: Some(d.xmlns_xs),
            no_namespace_schema_location: Some(d.no_namespace_schema_location),
            schema_version: Some(d.schema_version),
        }
    }
}

impl DeviceBuilder {
    /// Set the vendor of the device.
    pub fn vendor(mut self, value: Option<String>) -> Self {
        self.vendor = value;
        self
    }
    /// Set the vendor_id of the device.
    pub fn vendor_id(mut self, value: Option<String>) -> Self {
        self.vendor_id = value;
        self
    }
    /// Set the name of the device.
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the series of the device.
    pub fn series(mut self, value: Option<String>) -> Self {
        self.series = value;
        self
    }
    /// Set the version of the device.
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
    /// Set the description of the device.
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    /// Set the cpu of the device.
    pub fn cpu(mut self, value: Option<Cpu>) -> Self {
        self.cpu = value;
        self
    }
    /// Set the address unit bits of the device.
    pub fn address_unit_bits(mut self, value: u32) -> Self {
        self.address_unit_bits = Some(value);
        self
    }
    /// Set the width of the device.
    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(value);
        self
    }
    /// Set the default register properties of the device.
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
        self
    }
    /// Set the peripherals of the device.
    pub fn peripherals(mut self, value: Vec<Peripheral>) -> Self {
        self.peripherals = Some(value);
        self
    }
    /// Set the xmlns_xs version of the device.
    pub fn xmlns_xs(mut self, value: String) -> Self {
        self.xmlns_xs = Some(value);
        self
    }
    /// Set the no_namespace_schema_location version of the device.
    pub fn no_namespace_schema_location(mut self, value: String) -> Self {
        self.no_namespace_schema_location = Some(value);
        self
    }
    /// Set the schema version of the device.
    pub fn schema_version(mut self, value: String) -> Self {
        self.schema_version = Some(value);
        self
    }
    /// Validate and build a [`Device`].
    pub fn build(self, lvl: ValidateLevel) -> Result<Device, SvdError> {
        let mut device = Device {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            version: self.version,
            description: self.description,
            cpu: self.cpu,
            address_unit_bits: self.address_unit_bits,
            width: self.width,
            default_register_properties: self.default_register_properties.build(lvl)?,
            peripherals: self
                .peripherals
                .ok_or_else(|| BuildError::Uninitialized("peripherals".to_string()))?,
            schema_version: self.schema_version,
        };
        if !lvl.is_disabled() {
            device.validate(lvl)?;
        }
        Ok(device)
    }
}

impl Device {
    /// Make a builder for [`Device`]
    pub fn builder() -> DeviceBuilder {
        DeviceBuilder::default()
    }
    /// Modify an existing [`Device`] based on a [builder](DeviceBuilder).
    pub fn modify_from(
        &mut self,
        builder: DeviceBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.version.is_some() {
            self.version = builder.version.empty_to_none();
        }
        if builder.description.is_some() {
            self.description = builder.description.empty_to_none();
        }
        if builder.cpu.is_some() {
            self.cpu = builder.cpu;
        }
        if builder.address_unit_bits.is_some() {
            self.address_unit_bits = builder.address_unit_bits;
        }
        if builder.width.is_some() {
            self.width = builder.width;
        }
        self.default_register_properties
            .modify_from(builder.default_register_properties, lvl)?;
        if let Some(peripherals) = builder.peripherals {
            self.peripherals = peripherals;
        }
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }
    /// Validate the [`Device`]
    pub fn validate(&mut self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        // TODO
        if self.peripherals.is_empty() {
            return Err(Error::EmptyDevice.into());
        }
        Ok(())
    }
}

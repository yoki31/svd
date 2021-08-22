use super::{
    bitrange, Access, BitRange, BuildError, EnumeratedValues, ModifiedWriteValues, SvdError, Usage,
    ValidateLevel, WriteConstraint,
};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("You can have 0, 1 or 2 enumeratedValues with different usage")]
    IncompatibleEnumeratedValues,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct FieldInfo {
    /// Name string used to identify the field.
    /// Field names must be unique within a register
    pub name: String,

    /// String describing the details of the register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Bit position of the field within the register
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub bit_range: BitRange,

    /// Predefined strings set the access type.
    /// The element can be omitted if access rights get inherited from parent elements
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub access: Option<Access>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub modified_write_values: Option<ModifiedWriteValues>,

    /// Specifies the subset of allowed write values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub write_constraint: Option<WriteConstraint>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub enumerated_values: Vec<EnumeratedValues>,

    /// Specify the field name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldInfoBuilder {
    name: Option<String>,
    description: Option<String>,
    bit_range: Option<BitRange>,
    access: Option<Access>,
    modified_write_values: Option<ModifiedWriteValues>,
    write_constraint: Option<WriteConstraint>,
    enumerated_values: Option<Vec<EnumeratedValues>>,
    derived_from: Option<String>,
}

impl From<FieldInfo> for FieldInfoBuilder {
    fn from(f: FieldInfo) -> Self {
        Self {
            name: Some(f.name),
            description: f.description,
            bit_range: Some(f.bit_range),
            access: f.access,
            modified_write_values: f.modified_write_values,
            write_constraint: f.write_constraint,
            enumerated_values: Some(f.enumerated_values),
            derived_from: f.derived_from,
        }
    }
}

impl FieldInfoBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn bit_range(mut self, value: BitRange) -> Self {
        self.bit_range = Some(value);
        self
    }
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.access = value;
        self
    }
    pub fn modified_write_values(mut self, value: Option<ModifiedWriteValues>) -> Self {
        self.modified_write_values = value;
        self
    }
    pub fn write_constraint(mut self, value: Option<WriteConstraint>) -> Self {
        self.write_constraint = value;
        self
    }
    pub fn enumerated_values(mut self, value: Vec<EnumeratedValues>) -> Self {
        self.enumerated_values = Some(value);
        self
    }
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    pub fn build(self, lvl: ValidateLevel) -> Result<FieldInfo, SvdError> {
        let mut field = FieldInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            description: self.description,
            bit_range: self
                .bit_range
                .ok_or_else(|| BuildError::Uninitialized("bit_range".to_string()))?,
            access: self.access,
            modified_write_values: self.modified_write_values,
            write_constraint: self.write_constraint,
            enumerated_values: self.enumerated_values.unwrap_or_default(),
            derived_from: self.derived_from,
        };
        if !lvl.is_disabled() {
            field.validate(lvl)?;
        }
        Ok(field)
    }
}

impl FieldInfo {
    pub fn builder() -> FieldInfoBuilder {
        FieldInfoBuilder::default()
    }
    pub fn modify_from(
        &mut self,
        builder: FieldInfoBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.description.is_some() {
            self.description = builder.description;
        }
        if let Some(bit_range) = builder.bit_range {
            self.bit_range = bit_range;
        }
        if builder.access.is_some() {
            self.access = builder.access;
        }
        if builder.derived_from.is_some() {
            self.derived_from = builder.derived_from;
            self.modified_write_values = None;
            self.write_constraint = None;
            self.enumerated_values = Vec::new();
        } else {
            if builder.modified_write_values.is_some() {
                self.modified_write_values = builder.modified_write_values;
            }
            if builder.write_constraint.is_some() {
                self.write_constraint = builder.write_constraint;
            }
            if let Some(enumerated_values) = builder.enumerated_values {
                self.enumerated_values = enumerated_values;
            }
        }
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }
    pub fn validate(&mut self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if lvl.is_strict() {
            super::check_dimable_name(&self.name, "name")?;
            if let Some(name) = self.derived_from.as_ref() {
                super::check_derived_name(name, "derivedFrom")?;
            }
        }

        if self.bit_range.width == 0 {
            return Err(bitrange::Error::ZeroWidth.into());
        }

        // If the bit_range has its maximum width, all enumerated values will of
        // course fit in so we can skip validation.
        if self.bit_range.width < 64 {
            for ev in &self.enumerated_values {
                ev.check_range(0..2_u64.pow(self.bit_range.width))?;
            }
        }

        if lvl.is_strict() {
            match self.enumerated_values.as_slice() {
                [] | [_] => {}
                [ev1, ev2]
                    if (ev1.usage() == Usage::Read && ev2.usage() == Usage::Write)
                        || (ev2.usage() == Usage::Read && ev1.usage() == Usage::Write) => {}
                _ => return Err(Error::IncompatibleEnumeratedValues.into()),
            }
        }
        Ok(())
    }
}

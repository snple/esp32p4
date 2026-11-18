//! Data types for representing SVD register definitions
#![allow(dead_code)]

/// Access type for a register or field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl Access {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "read-only" | "ro" => Access::ReadOnly,
            "write-only" | "wo" => Access::WriteOnly,
            "read-write" | "read-writeonce" | "rw" => Access::ReadWrite,
            _ => Access::ReadWrite,
        }
    }

    pub fn rust_type(&self) -> &'static str {
        match self {
            Access::ReadOnly => "RO",
            Access::WriteOnly => "WO",
            Access::ReadWrite => "RW",
        }
    }
}

/// A bit field within a register
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub description: String,
    pub msb: u32,
    pub lsb: u32,
    pub access: Access,
    pub enum_values: Vec<EnumValue>,
}

impl Field {
    /// Get the bit mask for this field
    pub fn mask(&self) -> u32 {
        let width = self.msb - self.lsb + 1;
        (((1u64 << width) - 1) as u32) << self.lsb
    }

    /// Get the width of this field in bits
    pub fn width(&self) -> u32 {
        self.msb - self.lsb + 1
    }
}

/// An enumerated value for a field
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: String,
    pub value: u32,
    pub description: String,
}

/// A hardware register
#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
    pub description: String,
    pub offset: u32,
    pub reset_value: Option<u32>,
    pub access: Access,
    pub fields: Vec<Field>,
    /// Number of array elements, if this register is a `<dim>` array.
    pub dim: Option<u32>,
    /// Address stride between array elements.
    pub dim_increment: Option<u32>,
    /// Inclusive `(first, last)` indices from `<dimIndex>`, if present.
    pub dim_index: Option<(u32, u32)>,
}

/// An interrupt definition
#[derive(Debug, Clone)]
pub struct Interrupt {
    pub name: String,
    pub value: u32,
    pub description: String,
}

/// A complete peripheral (register block)
#[derive(Debug, Clone)]
pub struct Peripheral {
    pub name: String,
    pub description: String,
    pub base_address: Option<u32>,
    pub derived_from: Option<String>,
    pub registers: Vec<Register>,
    pub interrupts: Vec<Interrupt>,
}

impl Peripheral {
    pub fn module_name(&self) -> String {
        self.name.to_lowercase()
    }
}

impl Register {
    /// Expand this register into `(offset, name, description)` tuples.
    ///
    /// ESP SVDs encode arrays as `<dim>N</dim>` plus `%s` (or `[%s]`) in the
    /// register name, e.g. `GPIO%s` or `IV_MEM[%s]`. A non-array register
    /// returns exactly one entry.
    pub fn expanded(&self) -> Vec<(u32, String, String)> {
        match self.dim {
            None => vec![(self.offset, self.name.clone(), self.description.clone())],
            Some(dim) => {
                let stride = self.dim_increment.unwrap_or(4);
                let (first, last) = self.dim_index.unwrap_or((0, dim.saturating_sub(1)));
                (first..=last)
                    .map(|i| {
                        let suffix = i.to_string();
                        let name = if self.name.contains("%s") {
                            self.name.replace("%s", &suffix)
                        } else {
                            format!("{}{}", self.name, suffix)
                        };
                        let desc = self.description.replace("%s", &suffix);
                        (self.offset + (i - first) * stride, name, desc)
                    })
                    .collect()
            }
        }
    }
}

/// The complete SVD device
#[derive(Debug)]
pub struct SvdDevice {
    pub name: String,
    pub description: String,
    pub peripherals: Vec<Peripheral>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_dim_index_from_one() {
        let reg = Register {
            name: "FUNC%s_IN_SEL_CFG".to_string(),
            description: "input %s".to_string(),
            offset: 0x15c,
            reset_value: None,
            access: Access::ReadWrite,
            fields: Vec::new(),
            dim: Some(254),
            dim_increment: Some(4),
            dim_index: Some((1, 254)),
        };

        let expanded = reg.expanded();
        assert_eq!(expanded.len(), 254);
        assert_eq!(
            expanded[0],
            (0x15c, "FUNC1_IN_SEL_CFG".to_string(), "input 1".to_string())
        );
        assert_eq!(
            expanded[253],
            (
                0x15c + 253 * 4,
                "FUNC254_IN_SEL_CFG".to_string(),
                "input 254".to_string()
            )
        );
    }

    #[test]
    fn expands_dim_index_defaults_to_zero() {
        let reg = Register {
            name: "GPIO%s".to_string(),
            description: String::new(),
            offset: 0x4,
            reset_value: None,
            access: Access::ReadWrite,
            fields: Vec::new(),
            dim: Some(3),
            dim_increment: Some(4),
            dim_index: None,
        };

        let expanded = reg.expanded();
        assert_eq!(expanded[0].0, 0x4);
        assert_eq!(expanded[0].1, "GPIO0");
        assert_eq!(expanded[2].0, 0xc);
        assert_eq!(expanded[2].1, "GPIO2");
    }
}

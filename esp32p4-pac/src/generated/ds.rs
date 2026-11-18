//! DS
//! Digital Signature

#![allow(dead_code)]

pub const BASE: u32 = 0x50094000;

/// Y_MEM[0] Register
///
/// memory that stores Y
///
/// Address offset: 0x0000
#[repr(transparent)]
pub struct YMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem0 {}

/// Y_MEM[1] Register
///
/// memory that stores Y
///
/// Address offset: 0x0004
#[repr(transparent)]
pub struct YMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem1 {}

/// Y_MEM[2] Register
///
/// memory that stores Y
///
/// Address offset: 0x0008
#[repr(transparent)]
pub struct YMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem2 {}

/// Y_MEM[3] Register
///
/// memory that stores Y
///
/// Address offset: 0x000c
#[repr(transparent)]
pub struct YMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem3 {}

/// Y_MEM[4] Register
///
/// memory that stores Y
///
/// Address offset: 0x0010
#[repr(transparent)]
pub struct YMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem4 {}

/// Y_MEM[5] Register
///
/// memory that stores Y
///
/// Address offset: 0x0014
#[repr(transparent)]
pub struct YMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem5 {}

/// Y_MEM[6] Register
///
/// memory that stores Y
///
/// Address offset: 0x0018
#[repr(transparent)]
pub struct YMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem6 {}

/// Y_MEM[7] Register
///
/// memory that stores Y
///
/// Address offset: 0x001c
#[repr(transparent)]
pub struct YMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem7 {}

/// Y_MEM[8] Register
///
/// memory that stores Y
///
/// Address offset: 0x0020
#[repr(transparent)]
pub struct YMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem8 {}

/// Y_MEM[9] Register
///
/// memory that stores Y
///
/// Address offset: 0x0024
#[repr(transparent)]
pub struct YMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem9 {}

/// Y_MEM[10] Register
///
/// memory that stores Y
///
/// Address offset: 0x0028
#[repr(transparent)]
pub struct YMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem10 {}

/// Y_MEM[11] Register
///
/// memory that stores Y
///
/// Address offset: 0x002c
#[repr(transparent)]
pub struct YMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem11 {}

/// Y_MEM[12] Register
///
/// memory that stores Y
///
/// Address offset: 0x0030
#[repr(transparent)]
pub struct YMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem12 {}

/// Y_MEM[13] Register
///
/// memory that stores Y
///
/// Address offset: 0x0034
#[repr(transparent)]
pub struct YMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem13 {}

/// Y_MEM[14] Register
///
/// memory that stores Y
///
/// Address offset: 0x0038
#[repr(transparent)]
pub struct YMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem14 {}

/// Y_MEM[15] Register
///
/// memory that stores Y
///
/// Address offset: 0x003c
#[repr(transparent)]
pub struct YMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem15 {}

/// Y_MEM[16] Register
///
/// memory that stores Y
///
/// Address offset: 0x0040
#[repr(transparent)]
pub struct YMem16 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem16 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem16 {}

/// Y_MEM[17] Register
///
/// memory that stores Y
///
/// Address offset: 0x0044
#[repr(transparent)]
pub struct YMem17 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem17 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem17 {}

/// Y_MEM[18] Register
///
/// memory that stores Y
///
/// Address offset: 0x0048
#[repr(transparent)]
pub struct YMem18 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem18 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem18 {}

/// Y_MEM[19] Register
///
/// memory that stores Y
///
/// Address offset: 0x004c
#[repr(transparent)]
pub struct YMem19 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem19 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem19 {}

/// Y_MEM[20] Register
///
/// memory that stores Y
///
/// Address offset: 0x0050
#[repr(transparent)]
pub struct YMem20 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem20 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem20 {}

/// Y_MEM[21] Register
///
/// memory that stores Y
///
/// Address offset: 0x0054
#[repr(transparent)]
pub struct YMem21 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem21 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem21 {}

/// Y_MEM[22] Register
///
/// memory that stores Y
///
/// Address offset: 0x0058
#[repr(transparent)]
pub struct YMem22 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem22 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem22 {}

/// Y_MEM[23] Register
///
/// memory that stores Y
///
/// Address offset: 0x005c
#[repr(transparent)]
pub struct YMem23 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem23 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem23 {}

/// Y_MEM[24] Register
///
/// memory that stores Y
///
/// Address offset: 0x0060
#[repr(transparent)]
pub struct YMem24 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem24 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem24 {}

/// Y_MEM[25] Register
///
/// memory that stores Y
///
/// Address offset: 0x0064
#[repr(transparent)]
pub struct YMem25 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem25 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem25 {}

/// Y_MEM[26] Register
///
/// memory that stores Y
///
/// Address offset: 0x0068
#[repr(transparent)]
pub struct YMem26 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem26 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem26 {}

/// Y_MEM[27] Register
///
/// memory that stores Y
///
/// Address offset: 0x006c
#[repr(transparent)]
pub struct YMem27 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem27 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem27 {}

/// Y_MEM[28] Register
///
/// memory that stores Y
///
/// Address offset: 0x0070
#[repr(transparent)]
pub struct YMem28 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem28 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem28 {}

/// Y_MEM[29] Register
///
/// memory that stores Y
///
/// Address offset: 0x0074
#[repr(transparent)]
pub struct YMem29 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem29 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem29 {}

/// Y_MEM[30] Register
///
/// memory that stores Y
///
/// Address offset: 0x0078
#[repr(transparent)]
pub struct YMem30 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem30 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem30 {}

/// Y_MEM[31] Register
///
/// memory that stores Y
///
/// Address offset: 0x007c
#[repr(transparent)]
pub struct YMem31 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem31 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem31 {}

/// Y_MEM[32] Register
///
/// memory that stores Y
///
/// Address offset: 0x0080
#[repr(transparent)]
pub struct YMem32 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem32 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem32 {}

/// Y_MEM[33] Register
///
/// memory that stores Y
///
/// Address offset: 0x0084
#[repr(transparent)]
pub struct YMem33 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem33 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem33 {}

/// Y_MEM[34] Register
///
/// memory that stores Y
///
/// Address offset: 0x0088
#[repr(transparent)]
pub struct YMem34 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem34 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem34 {}

/// Y_MEM[35] Register
///
/// memory that stores Y
///
/// Address offset: 0x008c
#[repr(transparent)]
pub struct YMem35 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem35 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem35 {}

/// Y_MEM[36] Register
///
/// memory that stores Y
///
/// Address offset: 0x0090
#[repr(transparent)]
pub struct YMem36 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem36 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem36 {}

/// Y_MEM[37] Register
///
/// memory that stores Y
///
/// Address offset: 0x0094
#[repr(transparent)]
pub struct YMem37 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem37 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem37 {}

/// Y_MEM[38] Register
///
/// memory that stores Y
///
/// Address offset: 0x0098
#[repr(transparent)]
pub struct YMem38 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem38 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem38 {}

/// Y_MEM[39] Register
///
/// memory that stores Y
///
/// Address offset: 0x009c
#[repr(transparent)]
pub struct YMem39 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem39 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem39 {}

/// Y_MEM[40] Register
///
/// memory that stores Y
///
/// Address offset: 0x00a0
#[repr(transparent)]
pub struct YMem40 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem40 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem40 {}

/// Y_MEM[41] Register
///
/// memory that stores Y
///
/// Address offset: 0x00a4
#[repr(transparent)]
pub struct YMem41 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem41 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem41 {}

/// Y_MEM[42] Register
///
/// memory that stores Y
///
/// Address offset: 0x00a8
#[repr(transparent)]
pub struct YMem42 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem42 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem42 {}

/// Y_MEM[43] Register
///
/// memory that stores Y
///
/// Address offset: 0x00ac
#[repr(transparent)]
pub struct YMem43 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem43 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem43 {}

/// Y_MEM[44] Register
///
/// memory that stores Y
///
/// Address offset: 0x00b0
#[repr(transparent)]
pub struct YMem44 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem44 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem44 {}

/// Y_MEM[45] Register
///
/// memory that stores Y
///
/// Address offset: 0x00b4
#[repr(transparent)]
pub struct YMem45 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem45 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem45 {}

/// Y_MEM[46] Register
///
/// memory that stores Y
///
/// Address offset: 0x00b8
#[repr(transparent)]
pub struct YMem46 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem46 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem46 {}

/// Y_MEM[47] Register
///
/// memory that stores Y
///
/// Address offset: 0x00bc
#[repr(transparent)]
pub struct YMem47 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem47 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem47 {}

/// Y_MEM[48] Register
///
/// memory that stores Y
///
/// Address offset: 0x00c0
#[repr(transparent)]
pub struct YMem48 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem48 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem48 {}

/// Y_MEM[49] Register
///
/// memory that stores Y
///
/// Address offset: 0x00c4
#[repr(transparent)]
pub struct YMem49 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem49 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem49 {}

/// Y_MEM[50] Register
///
/// memory that stores Y
///
/// Address offset: 0x00c8
#[repr(transparent)]
pub struct YMem50 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem50 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem50 {}

/// Y_MEM[51] Register
///
/// memory that stores Y
///
/// Address offset: 0x00cc
#[repr(transparent)]
pub struct YMem51 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem51 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem51 {}

/// Y_MEM[52] Register
///
/// memory that stores Y
///
/// Address offset: 0x00d0
#[repr(transparent)]
pub struct YMem52 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem52 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem52 {}

/// Y_MEM[53] Register
///
/// memory that stores Y
///
/// Address offset: 0x00d4
#[repr(transparent)]
pub struct YMem53 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem53 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem53 {}

/// Y_MEM[54] Register
///
/// memory that stores Y
///
/// Address offset: 0x00d8
#[repr(transparent)]
pub struct YMem54 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem54 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem54 {}

/// Y_MEM[55] Register
///
/// memory that stores Y
///
/// Address offset: 0x00dc
#[repr(transparent)]
pub struct YMem55 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem55 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem55 {}

/// Y_MEM[56] Register
///
/// memory that stores Y
///
/// Address offset: 0x00e0
#[repr(transparent)]
pub struct YMem56 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem56 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem56 {}

/// Y_MEM[57] Register
///
/// memory that stores Y
///
/// Address offset: 0x00e4
#[repr(transparent)]
pub struct YMem57 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem57 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem57 {}

/// Y_MEM[58] Register
///
/// memory that stores Y
///
/// Address offset: 0x00e8
#[repr(transparent)]
pub struct YMem58 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem58 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem58 {}

/// Y_MEM[59] Register
///
/// memory that stores Y
///
/// Address offset: 0x00ec
#[repr(transparent)]
pub struct YMem59 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem59 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem59 {}

/// Y_MEM[60] Register
///
/// memory that stores Y
///
/// Address offset: 0x00f0
#[repr(transparent)]
pub struct YMem60 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem60 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem60 {}

/// Y_MEM[61] Register
///
/// memory that stores Y
///
/// Address offset: 0x00f4
#[repr(transparent)]
pub struct YMem61 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem61 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem61 {}

/// Y_MEM[62] Register
///
/// memory that stores Y
///
/// Address offset: 0x00f8
#[repr(transparent)]
pub struct YMem62 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem62 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem62 {}

/// Y_MEM[63] Register
///
/// memory that stores Y
///
/// Address offset: 0x00fc
#[repr(transparent)]
pub struct YMem63 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem63 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem63 {}

/// Y_MEM[64] Register
///
/// memory that stores Y
///
/// Address offset: 0x0100
#[repr(transparent)]
pub struct YMem64 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem64 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem64 {}

/// Y_MEM[65] Register
///
/// memory that stores Y
///
/// Address offset: 0x0104
#[repr(transparent)]
pub struct YMem65 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem65 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem65 {}

/// Y_MEM[66] Register
///
/// memory that stores Y
///
/// Address offset: 0x0108
#[repr(transparent)]
pub struct YMem66 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem66 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem66 {}

/// Y_MEM[67] Register
///
/// memory that stores Y
///
/// Address offset: 0x010c
#[repr(transparent)]
pub struct YMem67 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem67 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem67 {}

/// Y_MEM[68] Register
///
/// memory that stores Y
///
/// Address offset: 0x0110
#[repr(transparent)]
pub struct YMem68 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem68 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem68 {}

/// Y_MEM[69] Register
///
/// memory that stores Y
///
/// Address offset: 0x0114
#[repr(transparent)]
pub struct YMem69 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem69 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem69 {}

/// Y_MEM[70] Register
///
/// memory that stores Y
///
/// Address offset: 0x0118
#[repr(transparent)]
pub struct YMem70 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem70 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem70 {}

/// Y_MEM[71] Register
///
/// memory that stores Y
///
/// Address offset: 0x011c
#[repr(transparent)]
pub struct YMem71 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem71 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem71 {}

/// Y_MEM[72] Register
///
/// memory that stores Y
///
/// Address offset: 0x0120
#[repr(transparent)]
pub struct YMem72 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem72 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem72 {}

/// Y_MEM[73] Register
///
/// memory that stores Y
///
/// Address offset: 0x0124
#[repr(transparent)]
pub struct YMem73 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem73 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem73 {}

/// Y_MEM[74] Register
///
/// memory that stores Y
///
/// Address offset: 0x0128
#[repr(transparent)]
pub struct YMem74 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem74 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem74 {}

/// Y_MEM[75] Register
///
/// memory that stores Y
///
/// Address offset: 0x012c
#[repr(transparent)]
pub struct YMem75 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem75 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem75 {}

/// Y_MEM[76] Register
///
/// memory that stores Y
///
/// Address offset: 0x0130
#[repr(transparent)]
pub struct YMem76 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem76 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem76 {}

/// Y_MEM[77] Register
///
/// memory that stores Y
///
/// Address offset: 0x0134
#[repr(transparent)]
pub struct YMem77 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem77 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem77 {}

/// Y_MEM[78] Register
///
/// memory that stores Y
///
/// Address offset: 0x0138
#[repr(transparent)]
pub struct YMem78 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem78 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem78 {}

/// Y_MEM[79] Register
///
/// memory that stores Y
///
/// Address offset: 0x013c
#[repr(transparent)]
pub struct YMem79 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem79 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem79 {}

/// Y_MEM[80] Register
///
/// memory that stores Y
///
/// Address offset: 0x0140
#[repr(transparent)]
pub struct YMem80 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem80 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem80 {}

/// Y_MEM[81] Register
///
/// memory that stores Y
///
/// Address offset: 0x0144
#[repr(transparent)]
pub struct YMem81 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem81 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem81 {}

/// Y_MEM[82] Register
///
/// memory that stores Y
///
/// Address offset: 0x0148
#[repr(transparent)]
pub struct YMem82 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem82 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem82 {}

/// Y_MEM[83] Register
///
/// memory that stores Y
///
/// Address offset: 0x014c
#[repr(transparent)]
pub struct YMem83 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem83 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem83 {}

/// Y_MEM[84] Register
///
/// memory that stores Y
///
/// Address offset: 0x0150
#[repr(transparent)]
pub struct YMem84 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem84 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem84 {}

/// Y_MEM[85] Register
///
/// memory that stores Y
///
/// Address offset: 0x0154
#[repr(transparent)]
pub struct YMem85 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem85 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem85 {}

/// Y_MEM[86] Register
///
/// memory that stores Y
///
/// Address offset: 0x0158
#[repr(transparent)]
pub struct YMem86 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem86 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem86 {}

/// Y_MEM[87] Register
///
/// memory that stores Y
///
/// Address offset: 0x015c
#[repr(transparent)]
pub struct YMem87 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem87 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem87 {}

/// Y_MEM[88] Register
///
/// memory that stores Y
///
/// Address offset: 0x0160
#[repr(transparent)]
pub struct YMem88 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem88 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem88 {}

/// Y_MEM[89] Register
///
/// memory that stores Y
///
/// Address offset: 0x0164
#[repr(transparent)]
pub struct YMem89 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem89 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem89 {}

/// Y_MEM[90] Register
///
/// memory that stores Y
///
/// Address offset: 0x0168
#[repr(transparent)]
pub struct YMem90 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem90 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem90 {}

/// Y_MEM[91] Register
///
/// memory that stores Y
///
/// Address offset: 0x016c
#[repr(transparent)]
pub struct YMem91 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem91 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem91 {}

/// Y_MEM[92] Register
///
/// memory that stores Y
///
/// Address offset: 0x0170
#[repr(transparent)]
pub struct YMem92 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem92 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem92 {}

/// Y_MEM[93] Register
///
/// memory that stores Y
///
/// Address offset: 0x0174
#[repr(transparent)]
pub struct YMem93 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem93 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem93 {}

/// Y_MEM[94] Register
///
/// memory that stores Y
///
/// Address offset: 0x0178
#[repr(transparent)]
pub struct YMem94 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem94 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem94 {}

/// Y_MEM[95] Register
///
/// memory that stores Y
///
/// Address offset: 0x017c
#[repr(transparent)]
pub struct YMem95 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem95 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem95 {}

/// Y_MEM[96] Register
///
/// memory that stores Y
///
/// Address offset: 0x0180
#[repr(transparent)]
pub struct YMem96 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem96 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem96 {}

/// Y_MEM[97] Register
///
/// memory that stores Y
///
/// Address offset: 0x0184
#[repr(transparent)]
pub struct YMem97 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem97 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem97 {}

/// Y_MEM[98] Register
///
/// memory that stores Y
///
/// Address offset: 0x0188
#[repr(transparent)]
pub struct YMem98 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem98 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem98 {}

/// Y_MEM[99] Register
///
/// memory that stores Y
///
/// Address offset: 0x018c
#[repr(transparent)]
pub struct YMem99 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem99 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem99 {}

/// Y_MEM[100] Register
///
/// memory that stores Y
///
/// Address offset: 0x0190
#[repr(transparent)]
pub struct YMem100 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem100 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem100 {}

/// Y_MEM[101] Register
///
/// memory that stores Y
///
/// Address offset: 0x0194
#[repr(transparent)]
pub struct YMem101 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem101 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem101 {}

/// Y_MEM[102] Register
///
/// memory that stores Y
///
/// Address offset: 0x0198
#[repr(transparent)]
pub struct YMem102 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem102 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem102 {}

/// Y_MEM[103] Register
///
/// memory that stores Y
///
/// Address offset: 0x019c
#[repr(transparent)]
pub struct YMem103 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem103 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem103 {}

/// Y_MEM[104] Register
///
/// memory that stores Y
///
/// Address offset: 0x01a0
#[repr(transparent)]
pub struct YMem104 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem104 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem104 {}

/// Y_MEM[105] Register
///
/// memory that stores Y
///
/// Address offset: 0x01a4
#[repr(transparent)]
pub struct YMem105 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem105 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem105 {}

/// Y_MEM[106] Register
///
/// memory that stores Y
///
/// Address offset: 0x01a8
#[repr(transparent)]
pub struct YMem106 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem106 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem106 {}

/// Y_MEM[107] Register
///
/// memory that stores Y
///
/// Address offset: 0x01ac
#[repr(transparent)]
pub struct YMem107 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem107 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem107 {}

/// Y_MEM[108] Register
///
/// memory that stores Y
///
/// Address offset: 0x01b0
#[repr(transparent)]
pub struct YMem108 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem108 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem108 {}

/// Y_MEM[109] Register
///
/// memory that stores Y
///
/// Address offset: 0x01b4
#[repr(transparent)]
pub struct YMem109 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem109 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem109 {}

/// Y_MEM[110] Register
///
/// memory that stores Y
///
/// Address offset: 0x01b8
#[repr(transparent)]
pub struct YMem110 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem110 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem110 {}

/// Y_MEM[111] Register
///
/// memory that stores Y
///
/// Address offset: 0x01bc
#[repr(transparent)]
pub struct YMem111 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem111 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem111 {}

/// Y_MEM[112] Register
///
/// memory that stores Y
///
/// Address offset: 0x01c0
#[repr(transparent)]
pub struct YMem112 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem112 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem112 {}

/// Y_MEM[113] Register
///
/// memory that stores Y
///
/// Address offset: 0x01c4
#[repr(transparent)]
pub struct YMem113 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem113 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem113 {}

/// Y_MEM[114] Register
///
/// memory that stores Y
///
/// Address offset: 0x01c8
#[repr(transparent)]
pub struct YMem114 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem114 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem114 {}

/// Y_MEM[115] Register
///
/// memory that stores Y
///
/// Address offset: 0x01cc
#[repr(transparent)]
pub struct YMem115 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem115 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem115 {}

/// Y_MEM[116] Register
///
/// memory that stores Y
///
/// Address offset: 0x01d0
#[repr(transparent)]
pub struct YMem116 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem116 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem116 {}

/// Y_MEM[117] Register
///
/// memory that stores Y
///
/// Address offset: 0x01d4
#[repr(transparent)]
pub struct YMem117 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem117 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem117 {}

/// Y_MEM[118] Register
///
/// memory that stores Y
///
/// Address offset: 0x01d8
#[repr(transparent)]
pub struct YMem118 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem118 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem118 {}

/// Y_MEM[119] Register
///
/// memory that stores Y
///
/// Address offset: 0x01dc
#[repr(transparent)]
pub struct YMem119 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem119 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem119 {}

/// Y_MEM[120] Register
///
/// memory that stores Y
///
/// Address offset: 0x01e0
#[repr(transparent)]
pub struct YMem120 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem120 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem120 {}

/// Y_MEM[121] Register
///
/// memory that stores Y
///
/// Address offset: 0x01e4
#[repr(transparent)]
pub struct YMem121 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem121 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem121 {}

/// Y_MEM[122] Register
///
/// memory that stores Y
///
/// Address offset: 0x01e8
#[repr(transparent)]
pub struct YMem122 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem122 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem122 {}

/// Y_MEM[123] Register
///
/// memory that stores Y
///
/// Address offset: 0x01ec
#[repr(transparent)]
pub struct YMem123 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem123 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem123 {}

/// Y_MEM[124] Register
///
/// memory that stores Y
///
/// Address offset: 0x01f0
#[repr(transparent)]
pub struct YMem124 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem124 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem124 {}

/// Y_MEM[125] Register
///
/// memory that stores Y
///
/// Address offset: 0x01f4
#[repr(transparent)]
pub struct YMem125 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem125 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem125 {}

/// Y_MEM[126] Register
///
/// memory that stores Y
///
/// Address offset: 0x01f8
#[repr(transparent)]
pub struct YMem126 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem126 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem126 {}

/// Y_MEM[127] Register
///
/// memory that stores Y
///
/// Address offset: 0x01fc
#[repr(transparent)]
pub struct YMem127 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem127 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem127 {}

/// M_MEM[0] Register
///
/// memory that stores M
///
/// Address offset: 0x0200
#[repr(transparent)]
pub struct MMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem0 {}

/// M_MEM[1] Register
///
/// memory that stores M
///
/// Address offset: 0x0204
#[repr(transparent)]
pub struct MMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem1 {}

/// M_MEM[2] Register
///
/// memory that stores M
///
/// Address offset: 0x0208
#[repr(transparent)]
pub struct MMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem2 {}

/// M_MEM[3] Register
///
/// memory that stores M
///
/// Address offset: 0x020c
#[repr(transparent)]
pub struct MMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem3 {}

/// M_MEM[4] Register
///
/// memory that stores M
///
/// Address offset: 0x0210
#[repr(transparent)]
pub struct MMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem4 {}

/// M_MEM[5] Register
///
/// memory that stores M
///
/// Address offset: 0x0214
#[repr(transparent)]
pub struct MMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem5 {}

/// M_MEM[6] Register
///
/// memory that stores M
///
/// Address offset: 0x0218
#[repr(transparent)]
pub struct MMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem6 {}

/// M_MEM[7] Register
///
/// memory that stores M
///
/// Address offset: 0x021c
#[repr(transparent)]
pub struct MMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem7 {}

/// M_MEM[8] Register
///
/// memory that stores M
///
/// Address offset: 0x0220
#[repr(transparent)]
pub struct MMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem8 {}

/// M_MEM[9] Register
///
/// memory that stores M
///
/// Address offset: 0x0224
#[repr(transparent)]
pub struct MMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem9 {}

/// M_MEM[10] Register
///
/// memory that stores M
///
/// Address offset: 0x0228
#[repr(transparent)]
pub struct MMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem10 {}

/// M_MEM[11] Register
///
/// memory that stores M
///
/// Address offset: 0x022c
#[repr(transparent)]
pub struct MMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem11 {}

/// M_MEM[12] Register
///
/// memory that stores M
///
/// Address offset: 0x0230
#[repr(transparent)]
pub struct MMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem12 {}

/// M_MEM[13] Register
///
/// memory that stores M
///
/// Address offset: 0x0234
#[repr(transparent)]
pub struct MMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem13 {}

/// M_MEM[14] Register
///
/// memory that stores M
///
/// Address offset: 0x0238
#[repr(transparent)]
pub struct MMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem14 {}

/// M_MEM[15] Register
///
/// memory that stores M
///
/// Address offset: 0x023c
#[repr(transparent)]
pub struct MMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem15 {}

/// M_MEM[16] Register
///
/// memory that stores M
///
/// Address offset: 0x0240
#[repr(transparent)]
pub struct MMem16 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem16 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem16 {}

/// M_MEM[17] Register
///
/// memory that stores M
///
/// Address offset: 0x0244
#[repr(transparent)]
pub struct MMem17 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem17 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem17 {}

/// M_MEM[18] Register
///
/// memory that stores M
///
/// Address offset: 0x0248
#[repr(transparent)]
pub struct MMem18 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem18 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem18 {}

/// M_MEM[19] Register
///
/// memory that stores M
///
/// Address offset: 0x024c
#[repr(transparent)]
pub struct MMem19 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem19 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem19 {}

/// M_MEM[20] Register
///
/// memory that stores M
///
/// Address offset: 0x0250
#[repr(transparent)]
pub struct MMem20 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem20 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem20 {}

/// M_MEM[21] Register
///
/// memory that stores M
///
/// Address offset: 0x0254
#[repr(transparent)]
pub struct MMem21 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem21 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem21 {}

/// M_MEM[22] Register
///
/// memory that stores M
///
/// Address offset: 0x0258
#[repr(transparent)]
pub struct MMem22 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem22 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem22 {}

/// M_MEM[23] Register
///
/// memory that stores M
///
/// Address offset: 0x025c
#[repr(transparent)]
pub struct MMem23 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem23 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem23 {}

/// M_MEM[24] Register
///
/// memory that stores M
///
/// Address offset: 0x0260
#[repr(transparent)]
pub struct MMem24 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem24 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem24 {}

/// M_MEM[25] Register
///
/// memory that stores M
///
/// Address offset: 0x0264
#[repr(transparent)]
pub struct MMem25 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem25 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem25 {}

/// M_MEM[26] Register
///
/// memory that stores M
///
/// Address offset: 0x0268
#[repr(transparent)]
pub struct MMem26 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem26 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem26 {}

/// M_MEM[27] Register
///
/// memory that stores M
///
/// Address offset: 0x026c
#[repr(transparent)]
pub struct MMem27 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem27 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem27 {}

/// M_MEM[28] Register
///
/// memory that stores M
///
/// Address offset: 0x0270
#[repr(transparent)]
pub struct MMem28 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem28 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem28 {}

/// M_MEM[29] Register
///
/// memory that stores M
///
/// Address offset: 0x0274
#[repr(transparent)]
pub struct MMem29 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem29 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem29 {}

/// M_MEM[30] Register
///
/// memory that stores M
///
/// Address offset: 0x0278
#[repr(transparent)]
pub struct MMem30 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem30 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem30 {}

/// M_MEM[31] Register
///
/// memory that stores M
///
/// Address offset: 0x027c
#[repr(transparent)]
pub struct MMem31 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem31 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem31 {}

/// M_MEM[32] Register
///
/// memory that stores M
///
/// Address offset: 0x0280
#[repr(transparent)]
pub struct MMem32 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem32 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem32 {}

/// M_MEM[33] Register
///
/// memory that stores M
///
/// Address offset: 0x0284
#[repr(transparent)]
pub struct MMem33 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem33 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem33 {}

/// M_MEM[34] Register
///
/// memory that stores M
///
/// Address offset: 0x0288
#[repr(transparent)]
pub struct MMem34 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem34 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem34 {}

/// M_MEM[35] Register
///
/// memory that stores M
///
/// Address offset: 0x028c
#[repr(transparent)]
pub struct MMem35 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem35 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem35 {}

/// M_MEM[36] Register
///
/// memory that stores M
///
/// Address offset: 0x0290
#[repr(transparent)]
pub struct MMem36 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem36 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem36 {}

/// M_MEM[37] Register
///
/// memory that stores M
///
/// Address offset: 0x0294
#[repr(transparent)]
pub struct MMem37 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem37 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem37 {}

/// M_MEM[38] Register
///
/// memory that stores M
///
/// Address offset: 0x0298
#[repr(transparent)]
pub struct MMem38 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem38 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem38 {}

/// M_MEM[39] Register
///
/// memory that stores M
///
/// Address offset: 0x029c
#[repr(transparent)]
pub struct MMem39 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem39 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem39 {}

/// M_MEM[40] Register
///
/// memory that stores M
///
/// Address offset: 0x02a0
#[repr(transparent)]
pub struct MMem40 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem40 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem40 {}

/// M_MEM[41] Register
///
/// memory that stores M
///
/// Address offset: 0x02a4
#[repr(transparent)]
pub struct MMem41 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem41 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem41 {}

/// M_MEM[42] Register
///
/// memory that stores M
///
/// Address offset: 0x02a8
#[repr(transparent)]
pub struct MMem42 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem42 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem42 {}

/// M_MEM[43] Register
///
/// memory that stores M
///
/// Address offset: 0x02ac
#[repr(transparent)]
pub struct MMem43 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem43 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem43 {}

/// M_MEM[44] Register
///
/// memory that stores M
///
/// Address offset: 0x02b0
#[repr(transparent)]
pub struct MMem44 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem44 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem44 {}

/// M_MEM[45] Register
///
/// memory that stores M
///
/// Address offset: 0x02b4
#[repr(transparent)]
pub struct MMem45 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem45 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem45 {}

/// M_MEM[46] Register
///
/// memory that stores M
///
/// Address offset: 0x02b8
#[repr(transparent)]
pub struct MMem46 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem46 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem46 {}

/// M_MEM[47] Register
///
/// memory that stores M
///
/// Address offset: 0x02bc
#[repr(transparent)]
pub struct MMem47 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem47 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem47 {}

/// M_MEM[48] Register
///
/// memory that stores M
///
/// Address offset: 0x02c0
#[repr(transparent)]
pub struct MMem48 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem48 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem48 {}

/// M_MEM[49] Register
///
/// memory that stores M
///
/// Address offset: 0x02c4
#[repr(transparent)]
pub struct MMem49 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem49 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem49 {}

/// M_MEM[50] Register
///
/// memory that stores M
///
/// Address offset: 0x02c8
#[repr(transparent)]
pub struct MMem50 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem50 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem50 {}

/// M_MEM[51] Register
///
/// memory that stores M
///
/// Address offset: 0x02cc
#[repr(transparent)]
pub struct MMem51 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem51 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem51 {}

/// M_MEM[52] Register
///
/// memory that stores M
///
/// Address offset: 0x02d0
#[repr(transparent)]
pub struct MMem52 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem52 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem52 {}

/// M_MEM[53] Register
///
/// memory that stores M
///
/// Address offset: 0x02d4
#[repr(transparent)]
pub struct MMem53 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem53 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem53 {}

/// M_MEM[54] Register
///
/// memory that stores M
///
/// Address offset: 0x02d8
#[repr(transparent)]
pub struct MMem54 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem54 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem54 {}

/// M_MEM[55] Register
///
/// memory that stores M
///
/// Address offset: 0x02dc
#[repr(transparent)]
pub struct MMem55 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem55 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem55 {}

/// M_MEM[56] Register
///
/// memory that stores M
///
/// Address offset: 0x02e0
#[repr(transparent)]
pub struct MMem56 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem56 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem56 {}

/// M_MEM[57] Register
///
/// memory that stores M
///
/// Address offset: 0x02e4
#[repr(transparent)]
pub struct MMem57 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem57 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem57 {}

/// M_MEM[58] Register
///
/// memory that stores M
///
/// Address offset: 0x02e8
#[repr(transparent)]
pub struct MMem58 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem58 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem58 {}

/// M_MEM[59] Register
///
/// memory that stores M
///
/// Address offset: 0x02ec
#[repr(transparent)]
pub struct MMem59 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem59 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem59 {}

/// M_MEM[60] Register
///
/// memory that stores M
///
/// Address offset: 0x02f0
#[repr(transparent)]
pub struct MMem60 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem60 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem60 {}

/// M_MEM[61] Register
///
/// memory that stores M
///
/// Address offset: 0x02f4
#[repr(transparent)]
pub struct MMem61 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem61 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem61 {}

/// M_MEM[62] Register
///
/// memory that stores M
///
/// Address offset: 0x02f8
#[repr(transparent)]
pub struct MMem62 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem62 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem62 {}

/// M_MEM[63] Register
///
/// memory that stores M
///
/// Address offset: 0x02fc
#[repr(transparent)]
pub struct MMem63 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem63 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem63 {}

/// M_MEM[64] Register
///
/// memory that stores M
///
/// Address offset: 0x0300
#[repr(transparent)]
pub struct MMem64 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem64 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem64 {}

/// M_MEM[65] Register
///
/// memory that stores M
///
/// Address offset: 0x0304
#[repr(transparent)]
pub struct MMem65 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem65 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem65 {}

/// M_MEM[66] Register
///
/// memory that stores M
///
/// Address offset: 0x0308
#[repr(transparent)]
pub struct MMem66 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem66 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem66 {}

/// M_MEM[67] Register
///
/// memory that stores M
///
/// Address offset: 0x030c
#[repr(transparent)]
pub struct MMem67 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem67 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem67 {}

/// M_MEM[68] Register
///
/// memory that stores M
///
/// Address offset: 0x0310
#[repr(transparent)]
pub struct MMem68 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem68 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem68 {}

/// M_MEM[69] Register
///
/// memory that stores M
///
/// Address offset: 0x0314
#[repr(transparent)]
pub struct MMem69 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem69 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem69 {}

/// M_MEM[70] Register
///
/// memory that stores M
///
/// Address offset: 0x0318
#[repr(transparent)]
pub struct MMem70 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem70 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem70 {}

/// M_MEM[71] Register
///
/// memory that stores M
///
/// Address offset: 0x031c
#[repr(transparent)]
pub struct MMem71 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem71 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem71 {}

/// M_MEM[72] Register
///
/// memory that stores M
///
/// Address offset: 0x0320
#[repr(transparent)]
pub struct MMem72 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem72 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem72 {}

/// M_MEM[73] Register
///
/// memory that stores M
///
/// Address offset: 0x0324
#[repr(transparent)]
pub struct MMem73 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem73 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem73 {}

/// M_MEM[74] Register
///
/// memory that stores M
///
/// Address offset: 0x0328
#[repr(transparent)]
pub struct MMem74 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem74 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem74 {}

/// M_MEM[75] Register
///
/// memory that stores M
///
/// Address offset: 0x032c
#[repr(transparent)]
pub struct MMem75 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem75 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem75 {}

/// M_MEM[76] Register
///
/// memory that stores M
///
/// Address offset: 0x0330
#[repr(transparent)]
pub struct MMem76 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem76 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem76 {}

/// M_MEM[77] Register
///
/// memory that stores M
///
/// Address offset: 0x0334
#[repr(transparent)]
pub struct MMem77 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem77 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem77 {}

/// M_MEM[78] Register
///
/// memory that stores M
///
/// Address offset: 0x0338
#[repr(transparent)]
pub struct MMem78 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem78 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem78 {}

/// M_MEM[79] Register
///
/// memory that stores M
///
/// Address offset: 0x033c
#[repr(transparent)]
pub struct MMem79 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem79 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem79 {}

/// M_MEM[80] Register
///
/// memory that stores M
///
/// Address offset: 0x0340
#[repr(transparent)]
pub struct MMem80 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem80 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem80 {}

/// M_MEM[81] Register
///
/// memory that stores M
///
/// Address offset: 0x0344
#[repr(transparent)]
pub struct MMem81 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem81 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem81 {}

/// M_MEM[82] Register
///
/// memory that stores M
///
/// Address offset: 0x0348
#[repr(transparent)]
pub struct MMem82 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem82 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem82 {}

/// M_MEM[83] Register
///
/// memory that stores M
///
/// Address offset: 0x034c
#[repr(transparent)]
pub struct MMem83 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem83 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem83 {}

/// M_MEM[84] Register
///
/// memory that stores M
///
/// Address offset: 0x0350
#[repr(transparent)]
pub struct MMem84 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem84 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem84 {}

/// M_MEM[85] Register
///
/// memory that stores M
///
/// Address offset: 0x0354
#[repr(transparent)]
pub struct MMem85 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem85 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem85 {}

/// M_MEM[86] Register
///
/// memory that stores M
///
/// Address offset: 0x0358
#[repr(transparent)]
pub struct MMem86 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem86 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem86 {}

/// M_MEM[87] Register
///
/// memory that stores M
///
/// Address offset: 0x035c
#[repr(transparent)]
pub struct MMem87 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem87 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem87 {}

/// M_MEM[88] Register
///
/// memory that stores M
///
/// Address offset: 0x0360
#[repr(transparent)]
pub struct MMem88 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem88 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem88 {}

/// M_MEM[89] Register
///
/// memory that stores M
///
/// Address offset: 0x0364
#[repr(transparent)]
pub struct MMem89 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem89 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem89 {}

/// M_MEM[90] Register
///
/// memory that stores M
///
/// Address offset: 0x0368
#[repr(transparent)]
pub struct MMem90 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem90 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem90 {}

/// M_MEM[91] Register
///
/// memory that stores M
///
/// Address offset: 0x036c
#[repr(transparent)]
pub struct MMem91 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem91 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem91 {}

/// M_MEM[92] Register
///
/// memory that stores M
///
/// Address offset: 0x0370
#[repr(transparent)]
pub struct MMem92 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem92 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem92 {}

/// M_MEM[93] Register
///
/// memory that stores M
///
/// Address offset: 0x0374
#[repr(transparent)]
pub struct MMem93 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem93 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem93 {}

/// M_MEM[94] Register
///
/// memory that stores M
///
/// Address offset: 0x0378
#[repr(transparent)]
pub struct MMem94 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem94 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem94 {}

/// M_MEM[95] Register
///
/// memory that stores M
///
/// Address offset: 0x037c
#[repr(transparent)]
pub struct MMem95 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem95 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem95 {}

/// M_MEM[96] Register
///
/// memory that stores M
///
/// Address offset: 0x0380
#[repr(transparent)]
pub struct MMem96 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem96 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem96 {}

/// M_MEM[97] Register
///
/// memory that stores M
///
/// Address offset: 0x0384
#[repr(transparent)]
pub struct MMem97 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem97 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem97 {}

/// M_MEM[98] Register
///
/// memory that stores M
///
/// Address offset: 0x0388
#[repr(transparent)]
pub struct MMem98 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem98 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem98 {}

/// M_MEM[99] Register
///
/// memory that stores M
///
/// Address offset: 0x038c
#[repr(transparent)]
pub struct MMem99 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem99 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem99 {}

/// M_MEM[100] Register
///
/// memory that stores M
///
/// Address offset: 0x0390
#[repr(transparent)]
pub struct MMem100 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem100 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem100 {}

/// M_MEM[101] Register
///
/// memory that stores M
///
/// Address offset: 0x0394
#[repr(transparent)]
pub struct MMem101 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem101 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem101 {}

/// M_MEM[102] Register
///
/// memory that stores M
///
/// Address offset: 0x0398
#[repr(transparent)]
pub struct MMem102 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem102 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem102 {}

/// M_MEM[103] Register
///
/// memory that stores M
///
/// Address offset: 0x039c
#[repr(transparent)]
pub struct MMem103 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem103 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem103 {}

/// M_MEM[104] Register
///
/// memory that stores M
///
/// Address offset: 0x03a0
#[repr(transparent)]
pub struct MMem104 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem104 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem104 {}

/// M_MEM[105] Register
///
/// memory that stores M
///
/// Address offset: 0x03a4
#[repr(transparent)]
pub struct MMem105 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem105 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem105 {}

/// M_MEM[106] Register
///
/// memory that stores M
///
/// Address offset: 0x03a8
#[repr(transparent)]
pub struct MMem106 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem106 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem106 {}

/// M_MEM[107] Register
///
/// memory that stores M
///
/// Address offset: 0x03ac
#[repr(transparent)]
pub struct MMem107 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem107 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem107 {}

/// M_MEM[108] Register
///
/// memory that stores M
///
/// Address offset: 0x03b0
#[repr(transparent)]
pub struct MMem108 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem108 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem108 {}

/// M_MEM[109] Register
///
/// memory that stores M
///
/// Address offset: 0x03b4
#[repr(transparent)]
pub struct MMem109 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem109 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem109 {}

/// M_MEM[110] Register
///
/// memory that stores M
///
/// Address offset: 0x03b8
#[repr(transparent)]
pub struct MMem110 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem110 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem110 {}

/// M_MEM[111] Register
///
/// memory that stores M
///
/// Address offset: 0x03bc
#[repr(transparent)]
pub struct MMem111 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem111 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem111 {}

/// M_MEM[112] Register
///
/// memory that stores M
///
/// Address offset: 0x03c0
#[repr(transparent)]
pub struct MMem112 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem112 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem112 {}

/// M_MEM[113] Register
///
/// memory that stores M
///
/// Address offset: 0x03c4
#[repr(transparent)]
pub struct MMem113 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem113 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem113 {}

/// M_MEM[114] Register
///
/// memory that stores M
///
/// Address offset: 0x03c8
#[repr(transparent)]
pub struct MMem114 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem114 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem114 {}

/// M_MEM[115] Register
///
/// memory that stores M
///
/// Address offset: 0x03cc
#[repr(transparent)]
pub struct MMem115 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem115 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem115 {}

/// M_MEM[116] Register
///
/// memory that stores M
///
/// Address offset: 0x03d0
#[repr(transparent)]
pub struct MMem116 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem116 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem116 {}

/// M_MEM[117] Register
///
/// memory that stores M
///
/// Address offset: 0x03d4
#[repr(transparent)]
pub struct MMem117 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem117 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem117 {}

/// M_MEM[118] Register
///
/// memory that stores M
///
/// Address offset: 0x03d8
#[repr(transparent)]
pub struct MMem118 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem118 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem118 {}

/// M_MEM[119] Register
///
/// memory that stores M
///
/// Address offset: 0x03dc
#[repr(transparent)]
pub struct MMem119 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem119 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem119 {}

/// M_MEM[120] Register
///
/// memory that stores M
///
/// Address offset: 0x03e0
#[repr(transparent)]
pub struct MMem120 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem120 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem120 {}

/// M_MEM[121] Register
///
/// memory that stores M
///
/// Address offset: 0x03e4
#[repr(transparent)]
pub struct MMem121 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem121 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem121 {}

/// M_MEM[122] Register
///
/// memory that stores M
///
/// Address offset: 0x03e8
#[repr(transparent)]
pub struct MMem122 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem122 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem122 {}

/// M_MEM[123] Register
///
/// memory that stores M
///
/// Address offset: 0x03ec
#[repr(transparent)]
pub struct MMem123 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem123 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem123 {}

/// M_MEM[124] Register
///
/// memory that stores M
///
/// Address offset: 0x03f0
#[repr(transparent)]
pub struct MMem124 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem124 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem124 {}

/// M_MEM[125] Register
///
/// memory that stores M
///
/// Address offset: 0x03f4
#[repr(transparent)]
pub struct MMem125 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem125 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem125 {}

/// M_MEM[126] Register
///
/// memory that stores M
///
/// Address offset: 0x03f8
#[repr(transparent)]
pub struct MMem126 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem126 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem126 {}

/// M_MEM[127] Register
///
/// memory that stores M
///
/// Address offset: 0x03fc
#[repr(transparent)]
pub struct MMem127 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem127 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem127 {}

/// RB_MEM[0] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0400
#[repr(transparent)]
pub struct RbMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem0 {}

/// RB_MEM[1] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0404
#[repr(transparent)]
pub struct RbMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem1 {}

/// RB_MEM[2] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0408
#[repr(transparent)]
pub struct RbMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem2 {}

/// RB_MEM[3] Register
///
/// memory that stores Rb
///
/// Address offset: 0x040c
#[repr(transparent)]
pub struct RbMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem3 {}

/// RB_MEM[4] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0410
#[repr(transparent)]
pub struct RbMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem4 {}

/// RB_MEM[5] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0414
#[repr(transparent)]
pub struct RbMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem5 {}

/// RB_MEM[6] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0418
#[repr(transparent)]
pub struct RbMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem6 {}

/// RB_MEM[7] Register
///
/// memory that stores Rb
///
/// Address offset: 0x041c
#[repr(transparent)]
pub struct RbMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem7 {}

/// RB_MEM[8] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0420
#[repr(transparent)]
pub struct RbMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem8 {}

/// RB_MEM[9] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0424
#[repr(transparent)]
pub struct RbMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem9 {}

/// RB_MEM[10] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0428
#[repr(transparent)]
pub struct RbMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem10 {}

/// RB_MEM[11] Register
///
/// memory that stores Rb
///
/// Address offset: 0x042c
#[repr(transparent)]
pub struct RbMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem11 {}

/// RB_MEM[12] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0430
#[repr(transparent)]
pub struct RbMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem12 {}

/// RB_MEM[13] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0434
#[repr(transparent)]
pub struct RbMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem13 {}

/// RB_MEM[14] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0438
#[repr(transparent)]
pub struct RbMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem14 {}

/// RB_MEM[15] Register
///
/// memory that stores Rb
///
/// Address offset: 0x043c
#[repr(transparent)]
pub struct RbMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem15 {}

/// RB_MEM[16] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0440
#[repr(transparent)]
pub struct RbMem16 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem16 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem16 {}

/// RB_MEM[17] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0444
#[repr(transparent)]
pub struct RbMem17 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem17 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem17 {}

/// RB_MEM[18] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0448
#[repr(transparent)]
pub struct RbMem18 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem18 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem18 {}

/// RB_MEM[19] Register
///
/// memory that stores Rb
///
/// Address offset: 0x044c
#[repr(transparent)]
pub struct RbMem19 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem19 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem19 {}

/// RB_MEM[20] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0450
#[repr(transparent)]
pub struct RbMem20 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem20 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem20 {}

/// RB_MEM[21] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0454
#[repr(transparent)]
pub struct RbMem21 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem21 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem21 {}

/// RB_MEM[22] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0458
#[repr(transparent)]
pub struct RbMem22 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem22 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem22 {}

/// RB_MEM[23] Register
///
/// memory that stores Rb
///
/// Address offset: 0x045c
#[repr(transparent)]
pub struct RbMem23 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem23 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem23 {}

/// RB_MEM[24] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0460
#[repr(transparent)]
pub struct RbMem24 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem24 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem24 {}

/// RB_MEM[25] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0464
#[repr(transparent)]
pub struct RbMem25 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem25 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem25 {}

/// RB_MEM[26] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0468
#[repr(transparent)]
pub struct RbMem26 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem26 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem26 {}

/// RB_MEM[27] Register
///
/// memory that stores Rb
///
/// Address offset: 0x046c
#[repr(transparent)]
pub struct RbMem27 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem27 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem27 {}

/// RB_MEM[28] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0470
#[repr(transparent)]
pub struct RbMem28 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem28 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem28 {}

/// RB_MEM[29] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0474
#[repr(transparent)]
pub struct RbMem29 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem29 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem29 {}

/// RB_MEM[30] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0478
#[repr(transparent)]
pub struct RbMem30 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem30 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem30 {}

/// RB_MEM[31] Register
///
/// memory that stores Rb
///
/// Address offset: 0x047c
#[repr(transparent)]
pub struct RbMem31 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem31 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem31 {}

/// RB_MEM[32] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0480
#[repr(transparent)]
pub struct RbMem32 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem32 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem32 {}

/// RB_MEM[33] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0484
#[repr(transparent)]
pub struct RbMem33 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem33 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem33 {}

/// RB_MEM[34] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0488
#[repr(transparent)]
pub struct RbMem34 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem34 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem34 {}

/// RB_MEM[35] Register
///
/// memory that stores Rb
///
/// Address offset: 0x048c
#[repr(transparent)]
pub struct RbMem35 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem35 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem35 {}

/// RB_MEM[36] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0490
#[repr(transparent)]
pub struct RbMem36 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem36 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem36 {}

/// RB_MEM[37] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0494
#[repr(transparent)]
pub struct RbMem37 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem37 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem37 {}

/// RB_MEM[38] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0498
#[repr(transparent)]
pub struct RbMem38 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem38 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem38 {}

/// RB_MEM[39] Register
///
/// memory that stores Rb
///
/// Address offset: 0x049c
#[repr(transparent)]
pub struct RbMem39 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem39 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem39 {}

/// RB_MEM[40] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04a0
#[repr(transparent)]
pub struct RbMem40 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem40 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem40 {}

/// RB_MEM[41] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04a4
#[repr(transparent)]
pub struct RbMem41 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem41 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem41 {}

/// RB_MEM[42] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04a8
#[repr(transparent)]
pub struct RbMem42 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem42 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem42 {}

/// RB_MEM[43] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04ac
#[repr(transparent)]
pub struct RbMem43 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem43 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem43 {}

/// RB_MEM[44] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04b0
#[repr(transparent)]
pub struct RbMem44 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem44 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem44 {}

/// RB_MEM[45] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04b4
#[repr(transparent)]
pub struct RbMem45 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem45 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem45 {}

/// RB_MEM[46] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04b8
#[repr(transparent)]
pub struct RbMem46 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem46 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem46 {}

/// RB_MEM[47] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04bc
#[repr(transparent)]
pub struct RbMem47 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem47 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem47 {}

/// RB_MEM[48] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04c0
#[repr(transparent)]
pub struct RbMem48 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem48 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem48 {}

/// RB_MEM[49] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04c4
#[repr(transparent)]
pub struct RbMem49 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem49 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem49 {}

/// RB_MEM[50] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04c8
#[repr(transparent)]
pub struct RbMem50 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem50 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem50 {}

/// RB_MEM[51] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04cc
#[repr(transparent)]
pub struct RbMem51 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem51 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem51 {}

/// RB_MEM[52] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04d0
#[repr(transparent)]
pub struct RbMem52 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem52 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem52 {}

/// RB_MEM[53] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04d4
#[repr(transparent)]
pub struct RbMem53 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem53 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem53 {}

/// RB_MEM[54] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04d8
#[repr(transparent)]
pub struct RbMem54 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem54 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem54 {}

/// RB_MEM[55] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04dc
#[repr(transparent)]
pub struct RbMem55 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem55 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem55 {}

/// RB_MEM[56] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04e0
#[repr(transparent)]
pub struct RbMem56 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem56 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem56 {}

/// RB_MEM[57] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04e4
#[repr(transparent)]
pub struct RbMem57 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem57 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem57 {}

/// RB_MEM[58] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04e8
#[repr(transparent)]
pub struct RbMem58 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem58 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem58 {}

/// RB_MEM[59] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04ec
#[repr(transparent)]
pub struct RbMem59 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem59 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem59 {}

/// RB_MEM[60] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04f0
#[repr(transparent)]
pub struct RbMem60 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem60 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem60 {}

/// RB_MEM[61] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04f4
#[repr(transparent)]
pub struct RbMem61 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem61 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem61 {}

/// RB_MEM[62] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04f8
#[repr(transparent)]
pub struct RbMem62 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem62 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem62 {}

/// RB_MEM[63] Register
///
/// memory that stores Rb
///
/// Address offset: 0x04fc
#[repr(transparent)]
pub struct RbMem63 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem63 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem63 {}

/// RB_MEM[64] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0500
#[repr(transparent)]
pub struct RbMem64 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem64 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem64 {}

/// RB_MEM[65] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0504
#[repr(transparent)]
pub struct RbMem65 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem65 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem65 {}

/// RB_MEM[66] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0508
#[repr(transparent)]
pub struct RbMem66 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem66 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem66 {}

/// RB_MEM[67] Register
///
/// memory that stores Rb
///
/// Address offset: 0x050c
#[repr(transparent)]
pub struct RbMem67 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem67 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem67 {}

/// RB_MEM[68] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0510
#[repr(transparent)]
pub struct RbMem68 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem68 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem68 {}

/// RB_MEM[69] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0514
#[repr(transparent)]
pub struct RbMem69 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem69 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem69 {}

/// RB_MEM[70] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0518
#[repr(transparent)]
pub struct RbMem70 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem70 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem70 {}

/// RB_MEM[71] Register
///
/// memory that stores Rb
///
/// Address offset: 0x051c
#[repr(transparent)]
pub struct RbMem71 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem71 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem71 {}

/// RB_MEM[72] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0520
#[repr(transparent)]
pub struct RbMem72 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem72 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem72 {}

/// RB_MEM[73] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0524
#[repr(transparent)]
pub struct RbMem73 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem73 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem73 {}

/// RB_MEM[74] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0528
#[repr(transparent)]
pub struct RbMem74 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem74 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem74 {}

/// RB_MEM[75] Register
///
/// memory that stores Rb
///
/// Address offset: 0x052c
#[repr(transparent)]
pub struct RbMem75 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem75 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem75 {}

/// RB_MEM[76] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0530
#[repr(transparent)]
pub struct RbMem76 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem76 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem76 {}

/// RB_MEM[77] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0534
#[repr(transparent)]
pub struct RbMem77 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem77 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem77 {}

/// RB_MEM[78] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0538
#[repr(transparent)]
pub struct RbMem78 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem78 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem78 {}

/// RB_MEM[79] Register
///
/// memory that stores Rb
///
/// Address offset: 0x053c
#[repr(transparent)]
pub struct RbMem79 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem79 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem79 {}

/// RB_MEM[80] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0540
#[repr(transparent)]
pub struct RbMem80 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem80 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem80 {}

/// RB_MEM[81] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0544
#[repr(transparent)]
pub struct RbMem81 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem81 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem81 {}

/// RB_MEM[82] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0548
#[repr(transparent)]
pub struct RbMem82 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem82 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem82 {}

/// RB_MEM[83] Register
///
/// memory that stores Rb
///
/// Address offset: 0x054c
#[repr(transparent)]
pub struct RbMem83 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem83 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem83 {}

/// RB_MEM[84] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0550
#[repr(transparent)]
pub struct RbMem84 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem84 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem84 {}

/// RB_MEM[85] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0554
#[repr(transparent)]
pub struct RbMem85 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem85 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem85 {}

/// RB_MEM[86] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0558
#[repr(transparent)]
pub struct RbMem86 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem86 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem86 {}

/// RB_MEM[87] Register
///
/// memory that stores Rb
///
/// Address offset: 0x055c
#[repr(transparent)]
pub struct RbMem87 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem87 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem87 {}

/// RB_MEM[88] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0560
#[repr(transparent)]
pub struct RbMem88 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem88 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem88 {}

/// RB_MEM[89] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0564
#[repr(transparent)]
pub struct RbMem89 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem89 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem89 {}

/// RB_MEM[90] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0568
#[repr(transparent)]
pub struct RbMem90 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem90 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem90 {}

/// RB_MEM[91] Register
///
/// memory that stores Rb
///
/// Address offset: 0x056c
#[repr(transparent)]
pub struct RbMem91 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem91 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem91 {}

/// RB_MEM[92] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0570
#[repr(transparent)]
pub struct RbMem92 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem92 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem92 {}

/// RB_MEM[93] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0574
#[repr(transparent)]
pub struct RbMem93 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem93 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem93 {}

/// RB_MEM[94] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0578
#[repr(transparent)]
pub struct RbMem94 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem94 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem94 {}

/// RB_MEM[95] Register
///
/// memory that stores Rb
///
/// Address offset: 0x057c
#[repr(transparent)]
pub struct RbMem95 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem95 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem95 {}

/// RB_MEM[96] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0580
#[repr(transparent)]
pub struct RbMem96 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem96 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem96 {}

/// RB_MEM[97] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0584
#[repr(transparent)]
pub struct RbMem97 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem97 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem97 {}

/// RB_MEM[98] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0588
#[repr(transparent)]
pub struct RbMem98 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem98 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem98 {}

/// RB_MEM[99] Register
///
/// memory that stores Rb
///
/// Address offset: 0x058c
#[repr(transparent)]
pub struct RbMem99 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem99 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem99 {}

/// RB_MEM[100] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0590
#[repr(transparent)]
pub struct RbMem100 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem100 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem100 {}

/// RB_MEM[101] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0594
#[repr(transparent)]
pub struct RbMem101 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem101 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem101 {}

/// RB_MEM[102] Register
///
/// memory that stores Rb
///
/// Address offset: 0x0598
#[repr(transparent)]
pub struct RbMem102 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem102 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem102 {}

/// RB_MEM[103] Register
///
/// memory that stores Rb
///
/// Address offset: 0x059c
#[repr(transparent)]
pub struct RbMem103 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem103 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem103 {}

/// RB_MEM[104] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05a0
#[repr(transparent)]
pub struct RbMem104 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem104 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem104 {}

/// RB_MEM[105] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05a4
#[repr(transparent)]
pub struct RbMem105 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem105 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem105 {}

/// RB_MEM[106] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05a8
#[repr(transparent)]
pub struct RbMem106 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem106 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem106 {}

/// RB_MEM[107] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05ac
#[repr(transparent)]
pub struct RbMem107 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem107 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem107 {}

/// RB_MEM[108] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05b0
#[repr(transparent)]
pub struct RbMem108 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem108 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem108 {}

/// RB_MEM[109] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05b4
#[repr(transparent)]
pub struct RbMem109 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem109 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem109 {}

/// RB_MEM[110] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05b8
#[repr(transparent)]
pub struct RbMem110 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem110 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem110 {}

/// RB_MEM[111] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05bc
#[repr(transparent)]
pub struct RbMem111 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem111 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem111 {}

/// RB_MEM[112] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05c0
#[repr(transparent)]
pub struct RbMem112 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem112 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem112 {}

/// RB_MEM[113] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05c4
#[repr(transparent)]
pub struct RbMem113 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem113 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem113 {}

/// RB_MEM[114] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05c8
#[repr(transparent)]
pub struct RbMem114 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem114 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem114 {}

/// RB_MEM[115] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05cc
#[repr(transparent)]
pub struct RbMem115 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem115 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem115 {}

/// RB_MEM[116] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05d0
#[repr(transparent)]
pub struct RbMem116 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem116 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem116 {}

/// RB_MEM[117] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05d4
#[repr(transparent)]
pub struct RbMem117 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem117 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem117 {}

/// RB_MEM[118] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05d8
#[repr(transparent)]
pub struct RbMem118 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem118 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem118 {}

/// RB_MEM[119] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05dc
#[repr(transparent)]
pub struct RbMem119 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem119 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem119 {}

/// RB_MEM[120] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05e0
#[repr(transparent)]
pub struct RbMem120 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem120 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem120 {}

/// RB_MEM[121] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05e4
#[repr(transparent)]
pub struct RbMem121 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem121 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem121 {}

/// RB_MEM[122] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05e8
#[repr(transparent)]
pub struct RbMem122 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem122 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem122 {}

/// RB_MEM[123] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05ec
#[repr(transparent)]
pub struct RbMem123 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem123 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem123 {}

/// RB_MEM[124] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05f0
#[repr(transparent)]
pub struct RbMem124 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem124 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem124 {}

/// RB_MEM[125] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05f4
#[repr(transparent)]
pub struct RbMem125 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem125 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem125 {}

/// RB_MEM[126] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05f8
#[repr(transparent)]
pub struct RbMem126 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem126 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem126 {}

/// RB_MEM[127] Register
///
/// memory that stores Rb
///
/// Address offset: 0x05fc
#[repr(transparent)]
pub struct RbMem127 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RbMem127 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RbMem127 {}

/// BOX_MEM[0] Register
///
/// memory that stores BOX
///
/// Address offset: 0x0600
#[repr(transparent)]
pub struct BoxMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem0 {}

/// BOX_MEM[1] Register
///
/// memory that stores BOX
///
/// Address offset: 0x0604
#[repr(transparent)]
pub struct BoxMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem1 {}

/// BOX_MEM[2] Register
///
/// memory that stores BOX
///
/// Address offset: 0x0608
#[repr(transparent)]
pub struct BoxMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem2 {}

/// BOX_MEM[3] Register
///
/// memory that stores BOX
///
/// Address offset: 0x060c
#[repr(transparent)]
pub struct BoxMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem3 {}

/// BOX_MEM[4] Register
///
/// memory that stores BOX
///
/// Address offset: 0x0610
#[repr(transparent)]
pub struct BoxMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem4 {}

/// BOX_MEM[5] Register
///
/// memory that stores BOX
///
/// Address offset: 0x0614
#[repr(transparent)]
pub struct BoxMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem5 {}

/// BOX_MEM[6] Register
///
/// memory that stores BOX
///
/// Address offset: 0x0618
#[repr(transparent)]
pub struct BoxMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem6 {}

/// BOX_MEM[7] Register
///
/// memory that stores BOX
///
/// Address offset: 0x061c
#[repr(transparent)]
pub struct BoxMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem7 {}

/// BOX_MEM[8] Register
///
/// memory that stores BOX
///
/// Address offset: 0x0620
#[repr(transparent)]
pub struct BoxMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem8 {}

/// BOX_MEM[9] Register
///
/// memory that stores BOX
///
/// Address offset: 0x0624
#[repr(transparent)]
pub struct BoxMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem9 {}

/// BOX_MEM[10] Register
///
/// memory that stores BOX
///
/// Address offset: 0x0628
#[repr(transparent)]
pub struct BoxMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem10 {}

/// BOX_MEM[11] Register
///
/// memory that stores BOX
///
/// Address offset: 0x062c
#[repr(transparent)]
pub struct BoxMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BoxMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BoxMem11 {}

/// IV_MEM[0] Register
///
/// memory that stores IV
///
/// Address offset: 0x0630
#[repr(transparent)]
pub struct IvMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IvMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IvMem0 {}

/// IV_MEM[1] Register
///
/// memory that stores IV
///
/// Address offset: 0x0634
#[repr(transparent)]
pub struct IvMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IvMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IvMem1 {}

/// IV_MEM[2] Register
///
/// memory that stores IV
///
/// Address offset: 0x0638
#[repr(transparent)]
pub struct IvMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IvMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IvMem2 {}

/// IV_MEM[3] Register
///
/// memory that stores IV
///
/// Address offset: 0x063c
#[repr(transparent)]
pub struct IvMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IvMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IvMem3 {}

/// X_MEM[0] Register
///
/// memory that stores X
///
/// Address offset: 0x0800
#[repr(transparent)]
pub struct XMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem0 {}

/// X_MEM[1] Register
///
/// memory that stores X
///
/// Address offset: 0x0804
#[repr(transparent)]
pub struct XMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem1 {}

/// X_MEM[2] Register
///
/// memory that stores X
///
/// Address offset: 0x0808
#[repr(transparent)]
pub struct XMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem2 {}

/// X_MEM[3] Register
///
/// memory that stores X
///
/// Address offset: 0x080c
#[repr(transparent)]
pub struct XMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem3 {}

/// X_MEM[4] Register
///
/// memory that stores X
///
/// Address offset: 0x0810
#[repr(transparent)]
pub struct XMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem4 {}

/// X_MEM[5] Register
///
/// memory that stores X
///
/// Address offset: 0x0814
#[repr(transparent)]
pub struct XMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem5 {}

/// X_MEM[6] Register
///
/// memory that stores X
///
/// Address offset: 0x0818
#[repr(transparent)]
pub struct XMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem6 {}

/// X_MEM[7] Register
///
/// memory that stores X
///
/// Address offset: 0x081c
#[repr(transparent)]
pub struct XMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem7 {}

/// X_MEM[8] Register
///
/// memory that stores X
///
/// Address offset: 0x0820
#[repr(transparent)]
pub struct XMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem8 {}

/// X_MEM[9] Register
///
/// memory that stores X
///
/// Address offset: 0x0824
#[repr(transparent)]
pub struct XMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem9 {}

/// X_MEM[10] Register
///
/// memory that stores X
///
/// Address offset: 0x0828
#[repr(transparent)]
pub struct XMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem10 {}

/// X_MEM[11] Register
///
/// memory that stores X
///
/// Address offset: 0x082c
#[repr(transparent)]
pub struct XMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem11 {}

/// X_MEM[12] Register
///
/// memory that stores X
///
/// Address offset: 0x0830
#[repr(transparent)]
pub struct XMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem12 {}

/// X_MEM[13] Register
///
/// memory that stores X
///
/// Address offset: 0x0834
#[repr(transparent)]
pub struct XMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem13 {}

/// X_MEM[14] Register
///
/// memory that stores X
///
/// Address offset: 0x0838
#[repr(transparent)]
pub struct XMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem14 {}

/// X_MEM[15] Register
///
/// memory that stores X
///
/// Address offset: 0x083c
#[repr(transparent)]
pub struct XMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem15 {}

/// X_MEM[16] Register
///
/// memory that stores X
///
/// Address offset: 0x0840
#[repr(transparent)]
pub struct XMem16 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem16 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem16 {}

/// X_MEM[17] Register
///
/// memory that stores X
///
/// Address offset: 0x0844
#[repr(transparent)]
pub struct XMem17 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem17 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem17 {}

/// X_MEM[18] Register
///
/// memory that stores X
///
/// Address offset: 0x0848
#[repr(transparent)]
pub struct XMem18 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem18 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem18 {}

/// X_MEM[19] Register
///
/// memory that stores X
///
/// Address offset: 0x084c
#[repr(transparent)]
pub struct XMem19 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem19 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem19 {}

/// X_MEM[20] Register
///
/// memory that stores X
///
/// Address offset: 0x0850
#[repr(transparent)]
pub struct XMem20 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem20 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem20 {}

/// X_MEM[21] Register
///
/// memory that stores X
///
/// Address offset: 0x0854
#[repr(transparent)]
pub struct XMem21 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem21 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem21 {}

/// X_MEM[22] Register
///
/// memory that stores X
///
/// Address offset: 0x0858
#[repr(transparent)]
pub struct XMem22 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem22 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem22 {}

/// X_MEM[23] Register
///
/// memory that stores X
///
/// Address offset: 0x085c
#[repr(transparent)]
pub struct XMem23 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem23 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem23 {}

/// X_MEM[24] Register
///
/// memory that stores X
///
/// Address offset: 0x0860
#[repr(transparent)]
pub struct XMem24 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem24 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem24 {}

/// X_MEM[25] Register
///
/// memory that stores X
///
/// Address offset: 0x0864
#[repr(transparent)]
pub struct XMem25 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem25 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem25 {}

/// X_MEM[26] Register
///
/// memory that stores X
///
/// Address offset: 0x0868
#[repr(transparent)]
pub struct XMem26 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem26 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem26 {}

/// X_MEM[27] Register
///
/// memory that stores X
///
/// Address offset: 0x086c
#[repr(transparent)]
pub struct XMem27 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem27 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem27 {}

/// X_MEM[28] Register
///
/// memory that stores X
///
/// Address offset: 0x0870
#[repr(transparent)]
pub struct XMem28 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem28 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem28 {}

/// X_MEM[29] Register
///
/// memory that stores X
///
/// Address offset: 0x0874
#[repr(transparent)]
pub struct XMem29 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem29 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem29 {}

/// X_MEM[30] Register
///
/// memory that stores X
///
/// Address offset: 0x0878
#[repr(transparent)]
pub struct XMem30 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem30 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem30 {}

/// X_MEM[31] Register
///
/// memory that stores X
///
/// Address offset: 0x087c
#[repr(transparent)]
pub struct XMem31 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem31 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem31 {}

/// X_MEM[32] Register
///
/// memory that stores X
///
/// Address offset: 0x0880
#[repr(transparent)]
pub struct XMem32 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem32 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem32 {}

/// X_MEM[33] Register
///
/// memory that stores X
///
/// Address offset: 0x0884
#[repr(transparent)]
pub struct XMem33 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem33 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem33 {}

/// X_MEM[34] Register
///
/// memory that stores X
///
/// Address offset: 0x0888
#[repr(transparent)]
pub struct XMem34 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem34 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem34 {}

/// X_MEM[35] Register
///
/// memory that stores X
///
/// Address offset: 0x088c
#[repr(transparent)]
pub struct XMem35 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem35 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem35 {}

/// X_MEM[36] Register
///
/// memory that stores X
///
/// Address offset: 0x0890
#[repr(transparent)]
pub struct XMem36 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem36 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem36 {}

/// X_MEM[37] Register
///
/// memory that stores X
///
/// Address offset: 0x0894
#[repr(transparent)]
pub struct XMem37 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem37 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem37 {}

/// X_MEM[38] Register
///
/// memory that stores X
///
/// Address offset: 0x0898
#[repr(transparent)]
pub struct XMem38 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem38 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem38 {}

/// X_MEM[39] Register
///
/// memory that stores X
///
/// Address offset: 0x089c
#[repr(transparent)]
pub struct XMem39 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem39 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem39 {}

/// X_MEM[40] Register
///
/// memory that stores X
///
/// Address offset: 0x08a0
#[repr(transparent)]
pub struct XMem40 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem40 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem40 {}

/// X_MEM[41] Register
///
/// memory that stores X
///
/// Address offset: 0x08a4
#[repr(transparent)]
pub struct XMem41 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem41 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem41 {}

/// X_MEM[42] Register
///
/// memory that stores X
///
/// Address offset: 0x08a8
#[repr(transparent)]
pub struct XMem42 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem42 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem42 {}

/// X_MEM[43] Register
///
/// memory that stores X
///
/// Address offset: 0x08ac
#[repr(transparent)]
pub struct XMem43 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem43 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem43 {}

/// X_MEM[44] Register
///
/// memory that stores X
///
/// Address offset: 0x08b0
#[repr(transparent)]
pub struct XMem44 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem44 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem44 {}

/// X_MEM[45] Register
///
/// memory that stores X
///
/// Address offset: 0x08b4
#[repr(transparent)]
pub struct XMem45 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem45 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem45 {}

/// X_MEM[46] Register
///
/// memory that stores X
///
/// Address offset: 0x08b8
#[repr(transparent)]
pub struct XMem46 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem46 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem46 {}

/// X_MEM[47] Register
///
/// memory that stores X
///
/// Address offset: 0x08bc
#[repr(transparent)]
pub struct XMem47 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem47 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem47 {}

/// X_MEM[48] Register
///
/// memory that stores X
///
/// Address offset: 0x08c0
#[repr(transparent)]
pub struct XMem48 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem48 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem48 {}

/// X_MEM[49] Register
///
/// memory that stores X
///
/// Address offset: 0x08c4
#[repr(transparent)]
pub struct XMem49 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem49 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem49 {}

/// X_MEM[50] Register
///
/// memory that stores X
///
/// Address offset: 0x08c8
#[repr(transparent)]
pub struct XMem50 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem50 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem50 {}

/// X_MEM[51] Register
///
/// memory that stores X
///
/// Address offset: 0x08cc
#[repr(transparent)]
pub struct XMem51 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem51 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem51 {}

/// X_MEM[52] Register
///
/// memory that stores X
///
/// Address offset: 0x08d0
#[repr(transparent)]
pub struct XMem52 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem52 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem52 {}

/// X_MEM[53] Register
///
/// memory that stores X
///
/// Address offset: 0x08d4
#[repr(transparent)]
pub struct XMem53 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem53 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem53 {}

/// X_MEM[54] Register
///
/// memory that stores X
///
/// Address offset: 0x08d8
#[repr(transparent)]
pub struct XMem54 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem54 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem54 {}

/// X_MEM[55] Register
///
/// memory that stores X
///
/// Address offset: 0x08dc
#[repr(transparent)]
pub struct XMem55 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem55 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem55 {}

/// X_MEM[56] Register
///
/// memory that stores X
///
/// Address offset: 0x08e0
#[repr(transparent)]
pub struct XMem56 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem56 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem56 {}

/// X_MEM[57] Register
///
/// memory that stores X
///
/// Address offset: 0x08e4
#[repr(transparent)]
pub struct XMem57 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem57 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem57 {}

/// X_MEM[58] Register
///
/// memory that stores X
///
/// Address offset: 0x08e8
#[repr(transparent)]
pub struct XMem58 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem58 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem58 {}

/// X_MEM[59] Register
///
/// memory that stores X
///
/// Address offset: 0x08ec
#[repr(transparent)]
pub struct XMem59 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem59 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem59 {}

/// X_MEM[60] Register
///
/// memory that stores X
///
/// Address offset: 0x08f0
#[repr(transparent)]
pub struct XMem60 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem60 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem60 {}

/// X_MEM[61] Register
///
/// memory that stores X
///
/// Address offset: 0x08f4
#[repr(transparent)]
pub struct XMem61 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem61 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem61 {}

/// X_MEM[62] Register
///
/// memory that stores X
///
/// Address offset: 0x08f8
#[repr(transparent)]
pub struct XMem62 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem62 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem62 {}

/// X_MEM[63] Register
///
/// memory that stores X
///
/// Address offset: 0x08fc
#[repr(transparent)]
pub struct XMem63 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem63 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem63 {}

/// X_MEM[64] Register
///
/// memory that stores X
///
/// Address offset: 0x0900
#[repr(transparent)]
pub struct XMem64 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem64 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem64 {}

/// X_MEM[65] Register
///
/// memory that stores X
///
/// Address offset: 0x0904
#[repr(transparent)]
pub struct XMem65 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem65 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem65 {}

/// X_MEM[66] Register
///
/// memory that stores X
///
/// Address offset: 0x0908
#[repr(transparent)]
pub struct XMem66 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem66 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem66 {}

/// X_MEM[67] Register
///
/// memory that stores X
///
/// Address offset: 0x090c
#[repr(transparent)]
pub struct XMem67 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem67 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem67 {}

/// X_MEM[68] Register
///
/// memory that stores X
///
/// Address offset: 0x0910
#[repr(transparent)]
pub struct XMem68 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem68 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem68 {}

/// X_MEM[69] Register
///
/// memory that stores X
///
/// Address offset: 0x0914
#[repr(transparent)]
pub struct XMem69 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem69 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem69 {}

/// X_MEM[70] Register
///
/// memory that stores X
///
/// Address offset: 0x0918
#[repr(transparent)]
pub struct XMem70 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem70 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem70 {}

/// X_MEM[71] Register
///
/// memory that stores X
///
/// Address offset: 0x091c
#[repr(transparent)]
pub struct XMem71 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem71 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem71 {}

/// X_MEM[72] Register
///
/// memory that stores X
///
/// Address offset: 0x0920
#[repr(transparent)]
pub struct XMem72 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem72 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem72 {}

/// X_MEM[73] Register
///
/// memory that stores X
///
/// Address offset: 0x0924
#[repr(transparent)]
pub struct XMem73 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem73 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem73 {}

/// X_MEM[74] Register
///
/// memory that stores X
///
/// Address offset: 0x0928
#[repr(transparent)]
pub struct XMem74 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem74 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem74 {}

/// X_MEM[75] Register
///
/// memory that stores X
///
/// Address offset: 0x092c
#[repr(transparent)]
pub struct XMem75 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem75 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem75 {}

/// X_MEM[76] Register
///
/// memory that stores X
///
/// Address offset: 0x0930
#[repr(transparent)]
pub struct XMem76 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem76 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem76 {}

/// X_MEM[77] Register
///
/// memory that stores X
///
/// Address offset: 0x0934
#[repr(transparent)]
pub struct XMem77 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem77 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem77 {}

/// X_MEM[78] Register
///
/// memory that stores X
///
/// Address offset: 0x0938
#[repr(transparent)]
pub struct XMem78 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem78 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem78 {}

/// X_MEM[79] Register
///
/// memory that stores X
///
/// Address offset: 0x093c
#[repr(transparent)]
pub struct XMem79 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem79 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem79 {}

/// X_MEM[80] Register
///
/// memory that stores X
///
/// Address offset: 0x0940
#[repr(transparent)]
pub struct XMem80 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem80 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem80 {}

/// X_MEM[81] Register
///
/// memory that stores X
///
/// Address offset: 0x0944
#[repr(transparent)]
pub struct XMem81 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem81 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem81 {}

/// X_MEM[82] Register
///
/// memory that stores X
///
/// Address offset: 0x0948
#[repr(transparent)]
pub struct XMem82 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem82 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem82 {}

/// X_MEM[83] Register
///
/// memory that stores X
///
/// Address offset: 0x094c
#[repr(transparent)]
pub struct XMem83 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem83 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem83 {}

/// X_MEM[84] Register
///
/// memory that stores X
///
/// Address offset: 0x0950
#[repr(transparent)]
pub struct XMem84 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem84 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem84 {}

/// X_MEM[85] Register
///
/// memory that stores X
///
/// Address offset: 0x0954
#[repr(transparent)]
pub struct XMem85 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem85 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem85 {}

/// X_MEM[86] Register
///
/// memory that stores X
///
/// Address offset: 0x0958
#[repr(transparent)]
pub struct XMem86 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem86 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem86 {}

/// X_MEM[87] Register
///
/// memory that stores X
///
/// Address offset: 0x095c
#[repr(transparent)]
pub struct XMem87 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem87 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem87 {}

/// X_MEM[88] Register
///
/// memory that stores X
///
/// Address offset: 0x0960
#[repr(transparent)]
pub struct XMem88 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem88 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem88 {}

/// X_MEM[89] Register
///
/// memory that stores X
///
/// Address offset: 0x0964
#[repr(transparent)]
pub struct XMem89 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem89 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem89 {}

/// X_MEM[90] Register
///
/// memory that stores X
///
/// Address offset: 0x0968
#[repr(transparent)]
pub struct XMem90 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem90 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem90 {}

/// X_MEM[91] Register
///
/// memory that stores X
///
/// Address offset: 0x096c
#[repr(transparent)]
pub struct XMem91 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem91 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem91 {}

/// X_MEM[92] Register
///
/// memory that stores X
///
/// Address offset: 0x0970
#[repr(transparent)]
pub struct XMem92 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem92 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem92 {}

/// X_MEM[93] Register
///
/// memory that stores X
///
/// Address offset: 0x0974
#[repr(transparent)]
pub struct XMem93 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem93 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem93 {}

/// X_MEM[94] Register
///
/// memory that stores X
///
/// Address offset: 0x0978
#[repr(transparent)]
pub struct XMem94 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem94 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem94 {}

/// X_MEM[95] Register
///
/// memory that stores X
///
/// Address offset: 0x097c
#[repr(transparent)]
pub struct XMem95 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem95 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem95 {}

/// X_MEM[96] Register
///
/// memory that stores X
///
/// Address offset: 0x0980
#[repr(transparent)]
pub struct XMem96 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem96 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem96 {}

/// X_MEM[97] Register
///
/// memory that stores X
///
/// Address offset: 0x0984
#[repr(transparent)]
pub struct XMem97 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem97 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem97 {}

/// X_MEM[98] Register
///
/// memory that stores X
///
/// Address offset: 0x0988
#[repr(transparent)]
pub struct XMem98 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem98 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem98 {}

/// X_MEM[99] Register
///
/// memory that stores X
///
/// Address offset: 0x098c
#[repr(transparent)]
pub struct XMem99 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem99 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem99 {}

/// X_MEM[100] Register
///
/// memory that stores X
///
/// Address offset: 0x0990
#[repr(transparent)]
pub struct XMem100 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem100 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem100 {}

/// X_MEM[101] Register
///
/// memory that stores X
///
/// Address offset: 0x0994
#[repr(transparent)]
pub struct XMem101 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem101 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem101 {}

/// X_MEM[102] Register
///
/// memory that stores X
///
/// Address offset: 0x0998
#[repr(transparent)]
pub struct XMem102 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem102 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem102 {}

/// X_MEM[103] Register
///
/// memory that stores X
///
/// Address offset: 0x099c
#[repr(transparent)]
pub struct XMem103 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem103 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem103 {}

/// X_MEM[104] Register
///
/// memory that stores X
///
/// Address offset: 0x09a0
#[repr(transparent)]
pub struct XMem104 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem104 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem104 {}

/// X_MEM[105] Register
///
/// memory that stores X
///
/// Address offset: 0x09a4
#[repr(transparent)]
pub struct XMem105 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem105 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem105 {}

/// X_MEM[106] Register
///
/// memory that stores X
///
/// Address offset: 0x09a8
#[repr(transparent)]
pub struct XMem106 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem106 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem106 {}

/// X_MEM[107] Register
///
/// memory that stores X
///
/// Address offset: 0x09ac
#[repr(transparent)]
pub struct XMem107 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem107 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem107 {}

/// X_MEM[108] Register
///
/// memory that stores X
///
/// Address offset: 0x09b0
#[repr(transparent)]
pub struct XMem108 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem108 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem108 {}

/// X_MEM[109] Register
///
/// memory that stores X
///
/// Address offset: 0x09b4
#[repr(transparent)]
pub struct XMem109 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem109 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem109 {}

/// X_MEM[110] Register
///
/// memory that stores X
///
/// Address offset: 0x09b8
#[repr(transparent)]
pub struct XMem110 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem110 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem110 {}

/// X_MEM[111] Register
///
/// memory that stores X
///
/// Address offset: 0x09bc
#[repr(transparent)]
pub struct XMem111 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem111 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem111 {}

/// X_MEM[112] Register
///
/// memory that stores X
///
/// Address offset: 0x09c0
#[repr(transparent)]
pub struct XMem112 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem112 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem112 {}

/// X_MEM[113] Register
///
/// memory that stores X
///
/// Address offset: 0x09c4
#[repr(transparent)]
pub struct XMem113 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem113 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem113 {}

/// X_MEM[114] Register
///
/// memory that stores X
///
/// Address offset: 0x09c8
#[repr(transparent)]
pub struct XMem114 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem114 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem114 {}

/// X_MEM[115] Register
///
/// memory that stores X
///
/// Address offset: 0x09cc
#[repr(transparent)]
pub struct XMem115 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem115 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem115 {}

/// X_MEM[116] Register
///
/// memory that stores X
///
/// Address offset: 0x09d0
#[repr(transparent)]
pub struct XMem116 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem116 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem116 {}

/// X_MEM[117] Register
///
/// memory that stores X
///
/// Address offset: 0x09d4
#[repr(transparent)]
pub struct XMem117 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem117 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem117 {}

/// X_MEM[118] Register
///
/// memory that stores X
///
/// Address offset: 0x09d8
#[repr(transparent)]
pub struct XMem118 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem118 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem118 {}

/// X_MEM[119] Register
///
/// memory that stores X
///
/// Address offset: 0x09dc
#[repr(transparent)]
pub struct XMem119 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem119 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem119 {}

/// X_MEM[120] Register
///
/// memory that stores X
///
/// Address offset: 0x09e0
#[repr(transparent)]
pub struct XMem120 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem120 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem120 {}

/// X_MEM[121] Register
///
/// memory that stores X
///
/// Address offset: 0x09e4
#[repr(transparent)]
pub struct XMem121 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem121 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem121 {}

/// X_MEM[122] Register
///
/// memory that stores X
///
/// Address offset: 0x09e8
#[repr(transparent)]
pub struct XMem122 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem122 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem122 {}

/// X_MEM[123] Register
///
/// memory that stores X
///
/// Address offset: 0x09ec
#[repr(transparent)]
pub struct XMem123 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem123 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem123 {}

/// X_MEM[124] Register
///
/// memory that stores X
///
/// Address offset: 0x09f0
#[repr(transparent)]
pub struct XMem124 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem124 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem124 {}

/// X_MEM[125] Register
///
/// memory that stores X
///
/// Address offset: 0x09f4
#[repr(transparent)]
pub struct XMem125 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem125 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem125 {}

/// X_MEM[126] Register
///
/// memory that stores X
///
/// Address offset: 0x09f8
#[repr(transparent)]
pub struct XMem126 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem126 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem126 {}

/// X_MEM[127] Register
///
/// memory that stores X
///
/// Address offset: 0x09fc
#[repr(transparent)]
pub struct XMem127 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem127 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem127 {}

/// Z_MEM[0] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a00
#[repr(transparent)]
pub struct ZMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem0 {}

/// Z_MEM[1] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a04
#[repr(transparent)]
pub struct ZMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem1 {}

/// Z_MEM[2] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a08
#[repr(transparent)]
pub struct ZMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem2 {}

/// Z_MEM[3] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a0c
#[repr(transparent)]
pub struct ZMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem3 {}

/// Z_MEM[4] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a10
#[repr(transparent)]
pub struct ZMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem4 {}

/// Z_MEM[5] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a14
#[repr(transparent)]
pub struct ZMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem5 {}

/// Z_MEM[6] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a18
#[repr(transparent)]
pub struct ZMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem6 {}

/// Z_MEM[7] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a1c
#[repr(transparent)]
pub struct ZMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem7 {}

/// Z_MEM[8] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a20
#[repr(transparent)]
pub struct ZMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem8 {}

/// Z_MEM[9] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a24
#[repr(transparent)]
pub struct ZMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem9 {}

/// Z_MEM[10] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a28
#[repr(transparent)]
pub struct ZMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem10 {}

/// Z_MEM[11] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a2c
#[repr(transparent)]
pub struct ZMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem11 {}

/// Z_MEM[12] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a30
#[repr(transparent)]
pub struct ZMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem12 {}

/// Z_MEM[13] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a34
#[repr(transparent)]
pub struct ZMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem13 {}

/// Z_MEM[14] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a38
#[repr(transparent)]
pub struct ZMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem14 {}

/// Z_MEM[15] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a3c
#[repr(transparent)]
pub struct ZMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem15 {}

/// Z_MEM[16] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a40
#[repr(transparent)]
pub struct ZMem16 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem16 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem16 {}

/// Z_MEM[17] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a44
#[repr(transparent)]
pub struct ZMem17 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem17 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem17 {}

/// Z_MEM[18] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a48
#[repr(transparent)]
pub struct ZMem18 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem18 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem18 {}

/// Z_MEM[19] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a4c
#[repr(transparent)]
pub struct ZMem19 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem19 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem19 {}

/// Z_MEM[20] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a50
#[repr(transparent)]
pub struct ZMem20 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem20 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem20 {}

/// Z_MEM[21] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a54
#[repr(transparent)]
pub struct ZMem21 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem21 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem21 {}

/// Z_MEM[22] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a58
#[repr(transparent)]
pub struct ZMem22 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem22 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem22 {}

/// Z_MEM[23] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a5c
#[repr(transparent)]
pub struct ZMem23 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem23 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem23 {}

/// Z_MEM[24] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a60
#[repr(transparent)]
pub struct ZMem24 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem24 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem24 {}

/// Z_MEM[25] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a64
#[repr(transparent)]
pub struct ZMem25 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem25 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem25 {}

/// Z_MEM[26] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a68
#[repr(transparent)]
pub struct ZMem26 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem26 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem26 {}

/// Z_MEM[27] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a6c
#[repr(transparent)]
pub struct ZMem27 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem27 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem27 {}

/// Z_MEM[28] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a70
#[repr(transparent)]
pub struct ZMem28 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem28 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem28 {}

/// Z_MEM[29] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a74
#[repr(transparent)]
pub struct ZMem29 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem29 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem29 {}

/// Z_MEM[30] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a78
#[repr(transparent)]
pub struct ZMem30 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem30 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem30 {}

/// Z_MEM[31] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a7c
#[repr(transparent)]
pub struct ZMem31 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem31 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem31 {}

/// Z_MEM[32] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a80
#[repr(transparent)]
pub struct ZMem32 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem32 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem32 {}

/// Z_MEM[33] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a84
#[repr(transparent)]
pub struct ZMem33 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem33 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem33 {}

/// Z_MEM[34] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a88
#[repr(transparent)]
pub struct ZMem34 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem34 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem34 {}

/// Z_MEM[35] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a8c
#[repr(transparent)]
pub struct ZMem35 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem35 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem35 {}

/// Z_MEM[36] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a90
#[repr(transparent)]
pub struct ZMem36 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem36 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem36 {}

/// Z_MEM[37] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a94
#[repr(transparent)]
pub struct ZMem37 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem37 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem37 {}

/// Z_MEM[38] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a98
#[repr(transparent)]
pub struct ZMem38 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem38 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem38 {}

/// Z_MEM[39] Register
///
/// memory that stores Z
///
/// Address offset: 0x0a9c
#[repr(transparent)]
pub struct ZMem39 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem39 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem39 {}

/// Z_MEM[40] Register
///
/// memory that stores Z
///
/// Address offset: 0x0aa0
#[repr(transparent)]
pub struct ZMem40 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem40 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem40 {}

/// Z_MEM[41] Register
///
/// memory that stores Z
///
/// Address offset: 0x0aa4
#[repr(transparent)]
pub struct ZMem41 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem41 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem41 {}

/// Z_MEM[42] Register
///
/// memory that stores Z
///
/// Address offset: 0x0aa8
#[repr(transparent)]
pub struct ZMem42 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem42 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem42 {}

/// Z_MEM[43] Register
///
/// memory that stores Z
///
/// Address offset: 0x0aac
#[repr(transparent)]
pub struct ZMem43 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem43 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem43 {}

/// Z_MEM[44] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ab0
#[repr(transparent)]
pub struct ZMem44 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem44 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem44 {}

/// Z_MEM[45] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ab4
#[repr(transparent)]
pub struct ZMem45 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem45 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem45 {}

/// Z_MEM[46] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ab8
#[repr(transparent)]
pub struct ZMem46 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem46 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem46 {}

/// Z_MEM[47] Register
///
/// memory that stores Z
///
/// Address offset: 0x0abc
#[repr(transparent)]
pub struct ZMem47 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem47 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem47 {}

/// Z_MEM[48] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ac0
#[repr(transparent)]
pub struct ZMem48 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem48 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem48 {}

/// Z_MEM[49] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ac4
#[repr(transparent)]
pub struct ZMem49 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem49 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem49 {}

/// Z_MEM[50] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ac8
#[repr(transparent)]
pub struct ZMem50 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem50 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem50 {}

/// Z_MEM[51] Register
///
/// memory that stores Z
///
/// Address offset: 0x0acc
#[repr(transparent)]
pub struct ZMem51 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem51 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem51 {}

/// Z_MEM[52] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ad0
#[repr(transparent)]
pub struct ZMem52 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem52 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem52 {}

/// Z_MEM[53] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ad4
#[repr(transparent)]
pub struct ZMem53 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem53 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem53 {}

/// Z_MEM[54] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ad8
#[repr(transparent)]
pub struct ZMem54 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem54 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem54 {}

/// Z_MEM[55] Register
///
/// memory that stores Z
///
/// Address offset: 0x0adc
#[repr(transparent)]
pub struct ZMem55 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem55 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem55 {}

/// Z_MEM[56] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ae0
#[repr(transparent)]
pub struct ZMem56 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem56 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem56 {}

/// Z_MEM[57] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ae4
#[repr(transparent)]
pub struct ZMem57 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem57 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem57 {}

/// Z_MEM[58] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ae8
#[repr(transparent)]
pub struct ZMem58 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem58 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem58 {}

/// Z_MEM[59] Register
///
/// memory that stores Z
///
/// Address offset: 0x0aec
#[repr(transparent)]
pub struct ZMem59 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem59 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem59 {}

/// Z_MEM[60] Register
///
/// memory that stores Z
///
/// Address offset: 0x0af0
#[repr(transparent)]
pub struct ZMem60 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem60 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem60 {}

/// Z_MEM[61] Register
///
/// memory that stores Z
///
/// Address offset: 0x0af4
#[repr(transparent)]
pub struct ZMem61 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem61 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem61 {}

/// Z_MEM[62] Register
///
/// memory that stores Z
///
/// Address offset: 0x0af8
#[repr(transparent)]
pub struct ZMem62 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem62 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem62 {}

/// Z_MEM[63] Register
///
/// memory that stores Z
///
/// Address offset: 0x0afc
#[repr(transparent)]
pub struct ZMem63 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem63 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem63 {}

/// Z_MEM[64] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b00
#[repr(transparent)]
pub struct ZMem64 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem64 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem64 {}

/// Z_MEM[65] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b04
#[repr(transparent)]
pub struct ZMem65 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem65 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem65 {}

/// Z_MEM[66] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b08
#[repr(transparent)]
pub struct ZMem66 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem66 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem66 {}

/// Z_MEM[67] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b0c
#[repr(transparent)]
pub struct ZMem67 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem67 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem67 {}

/// Z_MEM[68] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b10
#[repr(transparent)]
pub struct ZMem68 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem68 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem68 {}

/// Z_MEM[69] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b14
#[repr(transparent)]
pub struct ZMem69 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem69 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem69 {}

/// Z_MEM[70] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b18
#[repr(transparent)]
pub struct ZMem70 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem70 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem70 {}

/// Z_MEM[71] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b1c
#[repr(transparent)]
pub struct ZMem71 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem71 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem71 {}

/// Z_MEM[72] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b20
#[repr(transparent)]
pub struct ZMem72 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem72 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem72 {}

/// Z_MEM[73] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b24
#[repr(transparent)]
pub struct ZMem73 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem73 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem73 {}

/// Z_MEM[74] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b28
#[repr(transparent)]
pub struct ZMem74 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem74 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem74 {}

/// Z_MEM[75] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b2c
#[repr(transparent)]
pub struct ZMem75 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem75 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem75 {}

/// Z_MEM[76] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b30
#[repr(transparent)]
pub struct ZMem76 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem76 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem76 {}

/// Z_MEM[77] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b34
#[repr(transparent)]
pub struct ZMem77 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem77 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem77 {}

/// Z_MEM[78] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b38
#[repr(transparent)]
pub struct ZMem78 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem78 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem78 {}

/// Z_MEM[79] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b3c
#[repr(transparent)]
pub struct ZMem79 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem79 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem79 {}

/// Z_MEM[80] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b40
#[repr(transparent)]
pub struct ZMem80 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem80 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem80 {}

/// Z_MEM[81] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b44
#[repr(transparent)]
pub struct ZMem81 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem81 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem81 {}

/// Z_MEM[82] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b48
#[repr(transparent)]
pub struct ZMem82 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem82 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem82 {}

/// Z_MEM[83] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b4c
#[repr(transparent)]
pub struct ZMem83 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem83 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem83 {}

/// Z_MEM[84] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b50
#[repr(transparent)]
pub struct ZMem84 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem84 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem84 {}

/// Z_MEM[85] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b54
#[repr(transparent)]
pub struct ZMem85 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem85 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem85 {}

/// Z_MEM[86] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b58
#[repr(transparent)]
pub struct ZMem86 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem86 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem86 {}

/// Z_MEM[87] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b5c
#[repr(transparent)]
pub struct ZMem87 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem87 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem87 {}

/// Z_MEM[88] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b60
#[repr(transparent)]
pub struct ZMem88 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem88 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem88 {}

/// Z_MEM[89] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b64
#[repr(transparent)]
pub struct ZMem89 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem89 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem89 {}

/// Z_MEM[90] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b68
#[repr(transparent)]
pub struct ZMem90 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem90 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem90 {}

/// Z_MEM[91] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b6c
#[repr(transparent)]
pub struct ZMem91 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem91 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem91 {}

/// Z_MEM[92] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b70
#[repr(transparent)]
pub struct ZMem92 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem92 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem92 {}

/// Z_MEM[93] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b74
#[repr(transparent)]
pub struct ZMem93 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem93 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem93 {}

/// Z_MEM[94] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b78
#[repr(transparent)]
pub struct ZMem94 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem94 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem94 {}

/// Z_MEM[95] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b7c
#[repr(transparent)]
pub struct ZMem95 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem95 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem95 {}

/// Z_MEM[96] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b80
#[repr(transparent)]
pub struct ZMem96 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem96 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem96 {}

/// Z_MEM[97] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b84
#[repr(transparent)]
pub struct ZMem97 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem97 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem97 {}

/// Z_MEM[98] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b88
#[repr(transparent)]
pub struct ZMem98 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem98 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem98 {}

/// Z_MEM[99] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b8c
#[repr(transparent)]
pub struct ZMem99 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem99 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem99 {}

/// Z_MEM[100] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b90
#[repr(transparent)]
pub struct ZMem100 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem100 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem100 {}

/// Z_MEM[101] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b94
#[repr(transparent)]
pub struct ZMem101 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem101 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem101 {}

/// Z_MEM[102] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b98
#[repr(transparent)]
pub struct ZMem102 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem102 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem102 {}

/// Z_MEM[103] Register
///
/// memory that stores Z
///
/// Address offset: 0x0b9c
#[repr(transparent)]
pub struct ZMem103 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem103 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem103 {}

/// Z_MEM[104] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ba0
#[repr(transparent)]
pub struct ZMem104 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem104 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem104 {}

/// Z_MEM[105] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ba4
#[repr(transparent)]
pub struct ZMem105 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem105 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem105 {}

/// Z_MEM[106] Register
///
/// memory that stores Z
///
/// Address offset: 0x0ba8
#[repr(transparent)]
pub struct ZMem106 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem106 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem106 {}

/// Z_MEM[107] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bac
#[repr(transparent)]
pub struct ZMem107 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem107 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem107 {}

/// Z_MEM[108] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bb0
#[repr(transparent)]
pub struct ZMem108 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem108 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem108 {}

/// Z_MEM[109] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bb4
#[repr(transparent)]
pub struct ZMem109 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem109 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem109 {}

/// Z_MEM[110] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bb8
#[repr(transparent)]
pub struct ZMem110 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem110 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem110 {}

/// Z_MEM[111] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bbc
#[repr(transparent)]
pub struct ZMem111 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem111 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem111 {}

/// Z_MEM[112] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bc0
#[repr(transparent)]
pub struct ZMem112 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem112 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem112 {}

/// Z_MEM[113] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bc4
#[repr(transparent)]
pub struct ZMem113 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem113 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem113 {}

/// Z_MEM[114] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bc8
#[repr(transparent)]
pub struct ZMem114 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem114 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem114 {}

/// Z_MEM[115] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bcc
#[repr(transparent)]
pub struct ZMem115 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem115 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem115 {}

/// Z_MEM[116] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bd0
#[repr(transparent)]
pub struct ZMem116 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem116 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem116 {}

/// Z_MEM[117] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bd4
#[repr(transparent)]
pub struct ZMem117 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem117 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem117 {}

/// Z_MEM[118] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bd8
#[repr(transparent)]
pub struct ZMem118 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem118 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem118 {}

/// Z_MEM[119] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bdc
#[repr(transparent)]
pub struct ZMem119 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem119 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem119 {}

/// Z_MEM[120] Register
///
/// memory that stores Z
///
/// Address offset: 0x0be0
#[repr(transparent)]
pub struct ZMem120 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem120 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem120 {}

/// Z_MEM[121] Register
///
/// memory that stores Z
///
/// Address offset: 0x0be4
#[repr(transparent)]
pub struct ZMem121 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem121 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem121 {}

/// Z_MEM[122] Register
///
/// memory that stores Z
///
/// Address offset: 0x0be8
#[repr(transparent)]
pub struct ZMem122 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem122 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem122 {}

/// Z_MEM[123] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bec
#[repr(transparent)]
pub struct ZMem123 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem123 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem123 {}

/// Z_MEM[124] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bf0
#[repr(transparent)]
pub struct ZMem124 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem124 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem124 {}

/// Z_MEM[125] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bf4
#[repr(transparent)]
pub struct ZMem125 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem125 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem125 {}

/// Z_MEM[126] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bf8
#[repr(transparent)]
pub struct ZMem126 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem126 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem126 {}

/// Z_MEM[127] Register
///
/// memory that stores Z
///
/// Address offset: 0x0bfc
#[repr(transparent)]
pub struct ZMem127 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem127 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem127 {}

/// SET_START Register
///
/// DS start control register
///
/// Address offset: 0x0e00
///
/// ## Fields
///
/// - **SET_START** `[0]` (WO)
///
///   set this bit to start DS operation.
#[repr(transparent)]
pub struct SetStart {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetStart {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetStart {
    // ======== Field Constants ========

    /// **SET_START**
    ///
    /// set this bit to start DS operation.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_START_OFFSET: u32 = 0;
    pub const SET_START_MASK: u32 = 0x00000001;
    pub const SET_START: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_START** (WO)
    ///
    /// set this bit to start DS operation.
    #[inline(always)]
    pub fn trigger_set_start(&self) {
        self.value.write_bits(Self::SET_START_MASK);
    }
}

/// SET_START value type for batch read/modify/write operations
///
/// Use `setstart.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setstart.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetStartVal {
    bits: u32,
}

impl SetStartVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_START** `[0:0]`
    ///
    /// set this bit to start DS operation.
    #[inline(always)]
    pub const fn with_set_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetStart::SET_START_MASK
            } else {
                self.bits & !SetStart::SET_START_MASK
            },
        }
    }
}

impl Default for SetStartVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetStart {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetStartVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_CONTINUE Register
///
/// DS continue control register
///
/// Address offset: 0x0e04
///
/// ## Fields
///
/// - **SET_CONTINUE** `[0]` (WO)
///
///   set this bit to continue DS operation.
#[repr(transparent)]
pub struct SetContinue {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetContinue {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetContinue {
    // ======== Field Constants ========

    /// **SET_CONTINUE**
    ///
    /// set this bit to continue DS operation.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_CONTINUE_OFFSET: u32 = 0;
    pub const SET_CONTINUE_MASK: u32 = 0x00000001;
    pub const SET_CONTINUE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_CONTINUE** (WO)
    ///
    /// set this bit to continue DS operation.
    #[inline(always)]
    pub fn trigger_set_continue(&self) {
        self.value.write_bits(Self::SET_CONTINUE_MASK);
    }
}

/// SET_CONTINUE value type for batch read/modify/write operations
///
/// Use `setcontinue.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setcontinue.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetContinueVal {
    bits: u32,
}

impl SetContinueVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_CONTINUE** `[0:0]`
    ///
    /// set this bit to continue DS operation.
    #[inline(always)]
    pub const fn with_set_continue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetContinue::SET_CONTINUE_MASK
            } else {
                self.bits & !SetContinue::SET_CONTINUE_MASK
            },
        }
    }
}

impl Default for SetContinueVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetContinue {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetContinueVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_FINISH Register
///
/// DS finish control register
///
/// Address offset: 0x0e08
///
/// ## Fields
///
/// - **SET_FINISH** `[0]` (WO)
///
///   Set this bit to finish DS process.
#[repr(transparent)]
pub struct SetFinish {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetFinish {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetFinish {
    // ======== Field Constants ========

    /// **SET_FINISH**
    ///
    /// Set this bit to finish DS process.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_FINISH_OFFSET: u32 = 0;
    pub const SET_FINISH_MASK: u32 = 0x00000001;
    pub const SET_FINISH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_FINISH** (WO)
    ///
    /// Set this bit to finish DS process.
    #[inline(always)]
    pub fn trigger_set_finish(&self) {
        self.value.write_bits(Self::SET_FINISH_MASK);
    }
}

/// SET_FINISH value type for batch read/modify/write operations
///
/// Use `setfinish.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setfinish.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetFinishVal {
    bits: u32,
}

impl SetFinishVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_FINISH** `[0:0]`
    ///
    /// Set this bit to finish DS process.
    #[inline(always)]
    pub const fn with_set_finish(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetFinish::SET_FINISH_MASK
            } else {
                self.bits & !SetFinish::SET_FINISH_MASK
            },
        }
    }
}

impl Default for SetFinishVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetFinish {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetFinishVal) {
        self.value.write_bits(val.bits())
    }
}

/// QUERY_BUSY Register
///
/// DS query busy register
///
/// Address offset: 0x0e0c
///
/// ## Fields
///
/// - **QUERY_BUSY** `[0]` (RO)
///
///   digital signature state. 1'b0: idle, 1'b1: busy
#[repr(transparent)]
pub struct QueryBusy {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for QueryBusy {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QueryBusy {
    // ======== Field Constants ========

    /// **QUERY_BUSY**
    ///
    /// digital signature state. 1'b0: idle, 1'b1: busy
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const QUERY_BUSY_OFFSET: u32 = 0;
    pub const QUERY_BUSY_MASK: u32 = 0x00000001;
    pub const QUERY_BUSY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **QUERY_BUSY** `[0]` (RO)
    ///
    /// digital signature state. 1'b0: idle, 1'b1: busy
    #[inline(always)]
    pub fn query_busy(&self) -> bool {
        (self.value.read_bits() & Self::QUERY_BUSY_MASK) != 0
    }
}

/// QUERY_BUSY value type for batch read/modify/write operations
///
/// Use `querybusy.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `querybusy.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QueryBusyVal {
    bits: u32,
}

impl QueryBusyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **QUERY_BUSY** `[0:0]`
    ///
    /// digital signature state. 1'b0: idle, 1'b1: busy
    #[inline(always)]
    pub const fn query_busy(&self) -> bool {
        (self.bits & QueryBusy::QUERY_BUSY_MASK) != 0
    }
}

impl Default for QueryBusyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl QueryBusy {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QueryBusyVal {
        QueryBusyVal::from_bits(self.value.read_bits())
    }
}

/// QUERY_KEY_WRONG Register
///
/// DS query key-wrong counter register
///
/// Address offset: 0x0e10
///
/// ## Fields
///
/// - **QUERY_KEY_WRONG** `[3:0]` (RO)
///
///   digital signature key wrong counter
#[repr(transparent)]
pub struct QueryKeyWrong {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for QueryKeyWrong {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QueryKeyWrong {
    // ======== Field Constants ========

    /// **QUERY_KEY_WRONG**
    ///
    /// digital signature key wrong counter
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const QUERY_KEY_WRONG_OFFSET: u32 = 0;
    pub const QUERY_KEY_WRONG_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **QUERY_KEY_WRONG** `[3:0]` (RO)
    ///
    /// digital signature key wrong counter
    #[inline(always)]
    pub fn query_key_wrong(&self) -> u32 {
        (self.value.read_bits() & Self::QUERY_KEY_WRONG_MASK) >> Self::QUERY_KEY_WRONG_OFFSET
    }
}

/// QUERY_KEY_WRONG value type for batch read/modify/write operations
///
/// Use `querykeywrong.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `querykeywrong.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QueryKeyWrongVal {
    bits: u32,
}

impl QueryKeyWrongVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **QUERY_KEY_WRONG** `[3:0]`
    ///
    /// digital signature key wrong counter
    #[inline(always)]
    pub const fn query_key_wrong(&self) -> u32 {
        (self.bits & QueryKeyWrong::QUERY_KEY_WRONG_MASK) >> QueryKeyWrong::QUERY_KEY_WRONG_OFFSET
    }
}

impl Default for QueryKeyWrongVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl QueryKeyWrong {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QueryKeyWrongVal {
        QueryKeyWrongVal::from_bits(self.value.read_bits())
    }
}

/// QUERY_CHECK Register
///
/// DS query check result register
///
/// Address offset: 0x0e14
///
/// ## Fields
///
/// - **PADDING_BAD** `[1]` (RO)
///
///   padding checkout result. 1'b0: a good padding, 1'b1: a bad padding
///
/// - **MD_ERROR** `[0]` (RO)
///
///   MD checkout result. 1'b0: MD check pass, 1'b1: MD check fail
#[repr(transparent)]
pub struct QueryCheck {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for QueryCheck {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QueryCheck {
    // ======== Field Constants ========

    /// **PADDING_BAD**
    ///
    /// padding checkout result. 1'b0: a good padding, 1'b1: a bad padding
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PADDING_BAD_OFFSET: u32 = 1;
    pub const PADDING_BAD_MASK: u32 = 0x00000002;
    pub const PADDING_BAD: u32 = 1 << 1;

    /// **MD_ERROR**
    ///
    /// MD checkout result. 1'b0: MD check pass, 1'b1: MD check fail
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MD_ERROR_OFFSET: u32 = 0;
    pub const MD_ERROR_MASK: u32 = 0x00000001;
    pub const MD_ERROR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PADDING_BAD** `[1]` (RO)
    ///
    /// padding checkout result. 1'b0: a good padding, 1'b1: a bad padding
    #[inline(always)]
    pub fn padding_bad(&self) -> bool {
        (self.value.read_bits() & Self::PADDING_BAD_MASK) != 0
    }

    /// **MD_ERROR** `[0]` (RO)
    ///
    /// MD checkout result. 1'b0: MD check pass, 1'b1: MD check fail
    #[inline(always)]
    pub fn md_error(&self) -> bool {
        (self.value.read_bits() & Self::MD_ERROR_MASK) != 0
    }
}

/// QUERY_CHECK value type for batch read/modify/write operations
///
/// Use `querycheck.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `querycheck.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QueryCheckVal {
    bits: u32,
}

impl QueryCheckVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PADDING_BAD** `[1:1]`
    ///
    /// padding checkout result. 1'b0: a good padding, 1'b1: a bad padding
    #[inline(always)]
    pub const fn padding_bad(&self) -> bool {
        (self.bits & QueryCheck::PADDING_BAD_MASK) != 0
    }

    /// Read **MD_ERROR** `[0:0]`
    ///
    /// MD checkout result. 1'b0: MD check pass, 1'b1: MD check fail
    #[inline(always)]
    pub const fn md_error(&self) -> bool {
        (self.bits & QueryCheck::MD_ERROR_MASK) != 0
    }
}

impl Default for QueryCheckVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl QueryCheck {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QueryCheckVal {
        QueryCheckVal::from_bits(self.value.read_bits())
    }
}

/// DATE Register
///
/// DS version control register
///
/// Address offset: 0x0e20
/// Reset value: 0x20200618
///
/// ## Fields
///
/// - **DATE** `[29:0]` (RW)
///
///   ds version information
#[repr(transparent)]
pub struct Date {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Date {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Date {
    // ======== Field Constants ========

    /// **DATE**
    ///
    /// ds version information
    ///
    /// Bits `[29:0]`, mask `0x3fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x3fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[29:0]` (RW)
    ///
    /// ds version information
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[29:0]` (RW)
    ///
    /// ds version information
    #[inline(always)]
    pub fn write_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATE_MASK) | ((value << Self::DATE_OFFSET) & Self::DATE_MASK)
        });
    }
}

/// DATE value type for batch read/modify/write operations
///
/// Use `date.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `date.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DateVal {
    bits: u32,
}

impl DateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x20200618 }
    }

    /// Read **DATE** `[29:0]`
    ///
    /// ds version information
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[29:0]`
    ///
    /// ds version information
    #[inline(always)]
    pub const fn with_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::DATE_MASK) | ((value << Date::DATE_OFFSET) & Date::DATE_MASK),
        }
    }
}

impl Default for DateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Date {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DateVal {
        DateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DateVal) -> DateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// memory that stores Y
    pub y_mem0: YMem0,
    /// 0x0004
    /// memory that stores Y
    pub y_mem1: YMem1,
    /// 0x0008
    /// memory that stores Y
    pub y_mem2: YMem2,
    /// 0x000c
    /// memory that stores Y
    pub y_mem3: YMem3,
    /// 0x0010
    /// memory that stores Y
    pub y_mem4: YMem4,
    /// 0x0014
    /// memory that stores Y
    pub y_mem5: YMem5,
    /// 0x0018
    /// memory that stores Y
    pub y_mem6: YMem6,
    /// 0x001c
    /// memory that stores Y
    pub y_mem7: YMem7,
    /// 0x0020
    /// memory that stores Y
    pub y_mem8: YMem8,
    /// 0x0024
    /// memory that stores Y
    pub y_mem9: YMem9,
    /// 0x0028
    /// memory that stores Y
    pub y_mem10: YMem10,
    /// 0x002c
    /// memory that stores Y
    pub y_mem11: YMem11,
    /// 0x0030
    /// memory that stores Y
    pub y_mem12: YMem12,
    /// 0x0034
    /// memory that stores Y
    pub y_mem13: YMem13,
    /// 0x0038
    /// memory that stores Y
    pub y_mem14: YMem14,
    /// 0x003c
    /// memory that stores Y
    pub y_mem15: YMem15,
    /// 0x0040
    /// memory that stores Y
    pub y_mem16: YMem16,
    /// 0x0044
    /// memory that stores Y
    pub y_mem17: YMem17,
    /// 0x0048
    /// memory that stores Y
    pub y_mem18: YMem18,
    /// 0x004c
    /// memory that stores Y
    pub y_mem19: YMem19,
    /// 0x0050
    /// memory that stores Y
    pub y_mem20: YMem20,
    /// 0x0054
    /// memory that stores Y
    pub y_mem21: YMem21,
    /// 0x0058
    /// memory that stores Y
    pub y_mem22: YMem22,
    /// 0x005c
    /// memory that stores Y
    pub y_mem23: YMem23,
    /// 0x0060
    /// memory that stores Y
    pub y_mem24: YMem24,
    /// 0x0064
    /// memory that stores Y
    pub y_mem25: YMem25,
    /// 0x0068
    /// memory that stores Y
    pub y_mem26: YMem26,
    /// 0x006c
    /// memory that stores Y
    pub y_mem27: YMem27,
    /// 0x0070
    /// memory that stores Y
    pub y_mem28: YMem28,
    /// 0x0074
    /// memory that stores Y
    pub y_mem29: YMem29,
    /// 0x0078
    /// memory that stores Y
    pub y_mem30: YMem30,
    /// 0x007c
    /// memory that stores Y
    pub y_mem31: YMem31,
    /// 0x0080
    /// memory that stores Y
    pub y_mem32: YMem32,
    /// 0x0084
    /// memory that stores Y
    pub y_mem33: YMem33,
    /// 0x0088
    /// memory that stores Y
    pub y_mem34: YMem34,
    /// 0x008c
    /// memory that stores Y
    pub y_mem35: YMem35,
    /// 0x0090
    /// memory that stores Y
    pub y_mem36: YMem36,
    /// 0x0094
    /// memory that stores Y
    pub y_mem37: YMem37,
    /// 0x0098
    /// memory that stores Y
    pub y_mem38: YMem38,
    /// 0x009c
    /// memory that stores Y
    pub y_mem39: YMem39,
    /// 0x00a0
    /// memory that stores Y
    pub y_mem40: YMem40,
    /// 0x00a4
    /// memory that stores Y
    pub y_mem41: YMem41,
    /// 0x00a8
    /// memory that stores Y
    pub y_mem42: YMem42,
    /// 0x00ac
    /// memory that stores Y
    pub y_mem43: YMem43,
    /// 0x00b0
    /// memory that stores Y
    pub y_mem44: YMem44,
    /// 0x00b4
    /// memory that stores Y
    pub y_mem45: YMem45,
    /// 0x00b8
    /// memory that stores Y
    pub y_mem46: YMem46,
    /// 0x00bc
    /// memory that stores Y
    pub y_mem47: YMem47,
    /// 0x00c0
    /// memory that stores Y
    pub y_mem48: YMem48,
    /// 0x00c4
    /// memory that stores Y
    pub y_mem49: YMem49,
    /// 0x00c8
    /// memory that stores Y
    pub y_mem50: YMem50,
    /// 0x00cc
    /// memory that stores Y
    pub y_mem51: YMem51,
    /// 0x00d0
    /// memory that stores Y
    pub y_mem52: YMem52,
    /// 0x00d4
    /// memory that stores Y
    pub y_mem53: YMem53,
    /// 0x00d8
    /// memory that stores Y
    pub y_mem54: YMem54,
    /// 0x00dc
    /// memory that stores Y
    pub y_mem55: YMem55,
    /// 0x00e0
    /// memory that stores Y
    pub y_mem56: YMem56,
    /// 0x00e4
    /// memory that stores Y
    pub y_mem57: YMem57,
    /// 0x00e8
    /// memory that stores Y
    pub y_mem58: YMem58,
    /// 0x00ec
    /// memory that stores Y
    pub y_mem59: YMem59,
    /// 0x00f0
    /// memory that stores Y
    pub y_mem60: YMem60,
    /// 0x00f4
    /// memory that stores Y
    pub y_mem61: YMem61,
    /// 0x00f8
    /// memory that stores Y
    pub y_mem62: YMem62,
    /// 0x00fc
    /// memory that stores Y
    pub y_mem63: YMem63,
    /// 0x0100
    /// memory that stores Y
    pub y_mem64: YMem64,
    /// 0x0104
    /// memory that stores Y
    pub y_mem65: YMem65,
    /// 0x0108
    /// memory that stores Y
    pub y_mem66: YMem66,
    /// 0x010c
    /// memory that stores Y
    pub y_mem67: YMem67,
    /// 0x0110
    /// memory that stores Y
    pub y_mem68: YMem68,
    /// 0x0114
    /// memory that stores Y
    pub y_mem69: YMem69,
    /// 0x0118
    /// memory that stores Y
    pub y_mem70: YMem70,
    /// 0x011c
    /// memory that stores Y
    pub y_mem71: YMem71,
    /// 0x0120
    /// memory that stores Y
    pub y_mem72: YMem72,
    /// 0x0124
    /// memory that stores Y
    pub y_mem73: YMem73,
    /// 0x0128
    /// memory that stores Y
    pub y_mem74: YMem74,
    /// 0x012c
    /// memory that stores Y
    pub y_mem75: YMem75,
    /// 0x0130
    /// memory that stores Y
    pub y_mem76: YMem76,
    /// 0x0134
    /// memory that stores Y
    pub y_mem77: YMem77,
    /// 0x0138
    /// memory that stores Y
    pub y_mem78: YMem78,
    /// 0x013c
    /// memory that stores Y
    pub y_mem79: YMem79,
    /// 0x0140
    /// memory that stores Y
    pub y_mem80: YMem80,
    /// 0x0144
    /// memory that stores Y
    pub y_mem81: YMem81,
    /// 0x0148
    /// memory that stores Y
    pub y_mem82: YMem82,
    /// 0x014c
    /// memory that stores Y
    pub y_mem83: YMem83,
    /// 0x0150
    /// memory that stores Y
    pub y_mem84: YMem84,
    /// 0x0154
    /// memory that stores Y
    pub y_mem85: YMem85,
    /// 0x0158
    /// memory that stores Y
    pub y_mem86: YMem86,
    /// 0x015c
    /// memory that stores Y
    pub y_mem87: YMem87,
    /// 0x0160
    /// memory that stores Y
    pub y_mem88: YMem88,
    /// 0x0164
    /// memory that stores Y
    pub y_mem89: YMem89,
    /// 0x0168
    /// memory that stores Y
    pub y_mem90: YMem90,
    /// 0x016c
    /// memory that stores Y
    pub y_mem91: YMem91,
    /// 0x0170
    /// memory that stores Y
    pub y_mem92: YMem92,
    /// 0x0174
    /// memory that stores Y
    pub y_mem93: YMem93,
    /// 0x0178
    /// memory that stores Y
    pub y_mem94: YMem94,
    /// 0x017c
    /// memory that stores Y
    pub y_mem95: YMem95,
    /// 0x0180
    /// memory that stores Y
    pub y_mem96: YMem96,
    /// 0x0184
    /// memory that stores Y
    pub y_mem97: YMem97,
    /// 0x0188
    /// memory that stores Y
    pub y_mem98: YMem98,
    /// 0x018c
    /// memory that stores Y
    pub y_mem99: YMem99,
    /// 0x0190
    /// memory that stores Y
    pub y_mem100: YMem100,
    /// 0x0194
    /// memory that stores Y
    pub y_mem101: YMem101,
    /// 0x0198
    /// memory that stores Y
    pub y_mem102: YMem102,
    /// 0x019c
    /// memory that stores Y
    pub y_mem103: YMem103,
    /// 0x01a0
    /// memory that stores Y
    pub y_mem104: YMem104,
    /// 0x01a4
    /// memory that stores Y
    pub y_mem105: YMem105,
    /// 0x01a8
    /// memory that stores Y
    pub y_mem106: YMem106,
    /// 0x01ac
    /// memory that stores Y
    pub y_mem107: YMem107,
    /// 0x01b0
    /// memory that stores Y
    pub y_mem108: YMem108,
    /// 0x01b4
    /// memory that stores Y
    pub y_mem109: YMem109,
    /// 0x01b8
    /// memory that stores Y
    pub y_mem110: YMem110,
    /// 0x01bc
    /// memory that stores Y
    pub y_mem111: YMem111,
    /// 0x01c0
    /// memory that stores Y
    pub y_mem112: YMem112,
    /// 0x01c4
    /// memory that stores Y
    pub y_mem113: YMem113,
    /// 0x01c8
    /// memory that stores Y
    pub y_mem114: YMem114,
    /// 0x01cc
    /// memory that stores Y
    pub y_mem115: YMem115,
    /// 0x01d0
    /// memory that stores Y
    pub y_mem116: YMem116,
    /// 0x01d4
    /// memory that stores Y
    pub y_mem117: YMem117,
    /// 0x01d8
    /// memory that stores Y
    pub y_mem118: YMem118,
    /// 0x01dc
    /// memory that stores Y
    pub y_mem119: YMem119,
    /// 0x01e0
    /// memory that stores Y
    pub y_mem120: YMem120,
    /// 0x01e4
    /// memory that stores Y
    pub y_mem121: YMem121,
    /// 0x01e8
    /// memory that stores Y
    pub y_mem122: YMem122,
    /// 0x01ec
    /// memory that stores Y
    pub y_mem123: YMem123,
    /// 0x01f0
    /// memory that stores Y
    pub y_mem124: YMem124,
    /// 0x01f4
    /// memory that stores Y
    pub y_mem125: YMem125,
    /// 0x01f8
    /// memory that stores Y
    pub y_mem126: YMem126,
    /// 0x01fc
    /// memory that stores Y
    pub y_mem127: YMem127,
    /// 0x0200
    /// memory that stores M
    pub m_mem0: MMem0,
    /// 0x0204
    /// memory that stores M
    pub m_mem1: MMem1,
    /// 0x0208
    /// memory that stores M
    pub m_mem2: MMem2,
    /// 0x020c
    /// memory that stores M
    pub m_mem3: MMem3,
    /// 0x0210
    /// memory that stores M
    pub m_mem4: MMem4,
    /// 0x0214
    /// memory that stores M
    pub m_mem5: MMem5,
    /// 0x0218
    /// memory that stores M
    pub m_mem6: MMem6,
    /// 0x021c
    /// memory that stores M
    pub m_mem7: MMem7,
    /// 0x0220
    /// memory that stores M
    pub m_mem8: MMem8,
    /// 0x0224
    /// memory that stores M
    pub m_mem9: MMem9,
    /// 0x0228
    /// memory that stores M
    pub m_mem10: MMem10,
    /// 0x022c
    /// memory that stores M
    pub m_mem11: MMem11,
    /// 0x0230
    /// memory that stores M
    pub m_mem12: MMem12,
    /// 0x0234
    /// memory that stores M
    pub m_mem13: MMem13,
    /// 0x0238
    /// memory that stores M
    pub m_mem14: MMem14,
    /// 0x023c
    /// memory that stores M
    pub m_mem15: MMem15,
    /// 0x0240
    /// memory that stores M
    pub m_mem16: MMem16,
    /// 0x0244
    /// memory that stores M
    pub m_mem17: MMem17,
    /// 0x0248
    /// memory that stores M
    pub m_mem18: MMem18,
    /// 0x024c
    /// memory that stores M
    pub m_mem19: MMem19,
    /// 0x0250
    /// memory that stores M
    pub m_mem20: MMem20,
    /// 0x0254
    /// memory that stores M
    pub m_mem21: MMem21,
    /// 0x0258
    /// memory that stores M
    pub m_mem22: MMem22,
    /// 0x025c
    /// memory that stores M
    pub m_mem23: MMem23,
    /// 0x0260
    /// memory that stores M
    pub m_mem24: MMem24,
    /// 0x0264
    /// memory that stores M
    pub m_mem25: MMem25,
    /// 0x0268
    /// memory that stores M
    pub m_mem26: MMem26,
    /// 0x026c
    /// memory that stores M
    pub m_mem27: MMem27,
    /// 0x0270
    /// memory that stores M
    pub m_mem28: MMem28,
    /// 0x0274
    /// memory that stores M
    pub m_mem29: MMem29,
    /// 0x0278
    /// memory that stores M
    pub m_mem30: MMem30,
    /// 0x027c
    /// memory that stores M
    pub m_mem31: MMem31,
    /// 0x0280
    /// memory that stores M
    pub m_mem32: MMem32,
    /// 0x0284
    /// memory that stores M
    pub m_mem33: MMem33,
    /// 0x0288
    /// memory that stores M
    pub m_mem34: MMem34,
    /// 0x028c
    /// memory that stores M
    pub m_mem35: MMem35,
    /// 0x0290
    /// memory that stores M
    pub m_mem36: MMem36,
    /// 0x0294
    /// memory that stores M
    pub m_mem37: MMem37,
    /// 0x0298
    /// memory that stores M
    pub m_mem38: MMem38,
    /// 0x029c
    /// memory that stores M
    pub m_mem39: MMem39,
    /// 0x02a0
    /// memory that stores M
    pub m_mem40: MMem40,
    /// 0x02a4
    /// memory that stores M
    pub m_mem41: MMem41,
    /// 0x02a8
    /// memory that stores M
    pub m_mem42: MMem42,
    /// 0x02ac
    /// memory that stores M
    pub m_mem43: MMem43,
    /// 0x02b0
    /// memory that stores M
    pub m_mem44: MMem44,
    /// 0x02b4
    /// memory that stores M
    pub m_mem45: MMem45,
    /// 0x02b8
    /// memory that stores M
    pub m_mem46: MMem46,
    /// 0x02bc
    /// memory that stores M
    pub m_mem47: MMem47,
    /// 0x02c0
    /// memory that stores M
    pub m_mem48: MMem48,
    /// 0x02c4
    /// memory that stores M
    pub m_mem49: MMem49,
    /// 0x02c8
    /// memory that stores M
    pub m_mem50: MMem50,
    /// 0x02cc
    /// memory that stores M
    pub m_mem51: MMem51,
    /// 0x02d0
    /// memory that stores M
    pub m_mem52: MMem52,
    /// 0x02d4
    /// memory that stores M
    pub m_mem53: MMem53,
    /// 0x02d8
    /// memory that stores M
    pub m_mem54: MMem54,
    /// 0x02dc
    /// memory that stores M
    pub m_mem55: MMem55,
    /// 0x02e0
    /// memory that stores M
    pub m_mem56: MMem56,
    /// 0x02e4
    /// memory that stores M
    pub m_mem57: MMem57,
    /// 0x02e8
    /// memory that stores M
    pub m_mem58: MMem58,
    /// 0x02ec
    /// memory that stores M
    pub m_mem59: MMem59,
    /// 0x02f0
    /// memory that stores M
    pub m_mem60: MMem60,
    /// 0x02f4
    /// memory that stores M
    pub m_mem61: MMem61,
    /// 0x02f8
    /// memory that stores M
    pub m_mem62: MMem62,
    /// 0x02fc
    /// memory that stores M
    pub m_mem63: MMem63,
    /// 0x0300
    /// memory that stores M
    pub m_mem64: MMem64,
    /// 0x0304
    /// memory that stores M
    pub m_mem65: MMem65,
    /// 0x0308
    /// memory that stores M
    pub m_mem66: MMem66,
    /// 0x030c
    /// memory that stores M
    pub m_mem67: MMem67,
    /// 0x0310
    /// memory that stores M
    pub m_mem68: MMem68,
    /// 0x0314
    /// memory that stores M
    pub m_mem69: MMem69,
    /// 0x0318
    /// memory that stores M
    pub m_mem70: MMem70,
    /// 0x031c
    /// memory that stores M
    pub m_mem71: MMem71,
    /// 0x0320
    /// memory that stores M
    pub m_mem72: MMem72,
    /// 0x0324
    /// memory that stores M
    pub m_mem73: MMem73,
    /// 0x0328
    /// memory that stores M
    pub m_mem74: MMem74,
    /// 0x032c
    /// memory that stores M
    pub m_mem75: MMem75,
    /// 0x0330
    /// memory that stores M
    pub m_mem76: MMem76,
    /// 0x0334
    /// memory that stores M
    pub m_mem77: MMem77,
    /// 0x0338
    /// memory that stores M
    pub m_mem78: MMem78,
    /// 0x033c
    /// memory that stores M
    pub m_mem79: MMem79,
    /// 0x0340
    /// memory that stores M
    pub m_mem80: MMem80,
    /// 0x0344
    /// memory that stores M
    pub m_mem81: MMem81,
    /// 0x0348
    /// memory that stores M
    pub m_mem82: MMem82,
    /// 0x034c
    /// memory that stores M
    pub m_mem83: MMem83,
    /// 0x0350
    /// memory that stores M
    pub m_mem84: MMem84,
    /// 0x0354
    /// memory that stores M
    pub m_mem85: MMem85,
    /// 0x0358
    /// memory that stores M
    pub m_mem86: MMem86,
    /// 0x035c
    /// memory that stores M
    pub m_mem87: MMem87,
    /// 0x0360
    /// memory that stores M
    pub m_mem88: MMem88,
    /// 0x0364
    /// memory that stores M
    pub m_mem89: MMem89,
    /// 0x0368
    /// memory that stores M
    pub m_mem90: MMem90,
    /// 0x036c
    /// memory that stores M
    pub m_mem91: MMem91,
    /// 0x0370
    /// memory that stores M
    pub m_mem92: MMem92,
    /// 0x0374
    /// memory that stores M
    pub m_mem93: MMem93,
    /// 0x0378
    /// memory that stores M
    pub m_mem94: MMem94,
    /// 0x037c
    /// memory that stores M
    pub m_mem95: MMem95,
    /// 0x0380
    /// memory that stores M
    pub m_mem96: MMem96,
    /// 0x0384
    /// memory that stores M
    pub m_mem97: MMem97,
    /// 0x0388
    /// memory that stores M
    pub m_mem98: MMem98,
    /// 0x038c
    /// memory that stores M
    pub m_mem99: MMem99,
    /// 0x0390
    /// memory that stores M
    pub m_mem100: MMem100,
    /// 0x0394
    /// memory that stores M
    pub m_mem101: MMem101,
    /// 0x0398
    /// memory that stores M
    pub m_mem102: MMem102,
    /// 0x039c
    /// memory that stores M
    pub m_mem103: MMem103,
    /// 0x03a0
    /// memory that stores M
    pub m_mem104: MMem104,
    /// 0x03a4
    /// memory that stores M
    pub m_mem105: MMem105,
    /// 0x03a8
    /// memory that stores M
    pub m_mem106: MMem106,
    /// 0x03ac
    /// memory that stores M
    pub m_mem107: MMem107,
    /// 0x03b0
    /// memory that stores M
    pub m_mem108: MMem108,
    /// 0x03b4
    /// memory that stores M
    pub m_mem109: MMem109,
    /// 0x03b8
    /// memory that stores M
    pub m_mem110: MMem110,
    /// 0x03bc
    /// memory that stores M
    pub m_mem111: MMem111,
    /// 0x03c0
    /// memory that stores M
    pub m_mem112: MMem112,
    /// 0x03c4
    /// memory that stores M
    pub m_mem113: MMem113,
    /// 0x03c8
    /// memory that stores M
    pub m_mem114: MMem114,
    /// 0x03cc
    /// memory that stores M
    pub m_mem115: MMem115,
    /// 0x03d0
    /// memory that stores M
    pub m_mem116: MMem116,
    /// 0x03d4
    /// memory that stores M
    pub m_mem117: MMem117,
    /// 0x03d8
    /// memory that stores M
    pub m_mem118: MMem118,
    /// 0x03dc
    /// memory that stores M
    pub m_mem119: MMem119,
    /// 0x03e0
    /// memory that stores M
    pub m_mem120: MMem120,
    /// 0x03e4
    /// memory that stores M
    pub m_mem121: MMem121,
    /// 0x03e8
    /// memory that stores M
    pub m_mem122: MMem122,
    /// 0x03ec
    /// memory that stores M
    pub m_mem123: MMem123,
    /// 0x03f0
    /// memory that stores M
    pub m_mem124: MMem124,
    /// 0x03f4
    /// memory that stores M
    pub m_mem125: MMem125,
    /// 0x03f8
    /// memory that stores M
    pub m_mem126: MMem126,
    /// 0x03fc
    /// memory that stores M
    pub m_mem127: MMem127,
    /// 0x0400
    /// memory that stores Rb
    pub rb_mem0: RbMem0,
    /// 0x0404
    /// memory that stores Rb
    pub rb_mem1: RbMem1,
    /// 0x0408
    /// memory that stores Rb
    pub rb_mem2: RbMem2,
    /// 0x040c
    /// memory that stores Rb
    pub rb_mem3: RbMem3,
    /// 0x0410
    /// memory that stores Rb
    pub rb_mem4: RbMem4,
    /// 0x0414
    /// memory that stores Rb
    pub rb_mem5: RbMem5,
    /// 0x0418
    /// memory that stores Rb
    pub rb_mem6: RbMem6,
    /// 0x041c
    /// memory that stores Rb
    pub rb_mem7: RbMem7,
    /// 0x0420
    /// memory that stores Rb
    pub rb_mem8: RbMem8,
    /// 0x0424
    /// memory that stores Rb
    pub rb_mem9: RbMem9,
    /// 0x0428
    /// memory that stores Rb
    pub rb_mem10: RbMem10,
    /// 0x042c
    /// memory that stores Rb
    pub rb_mem11: RbMem11,
    /// 0x0430
    /// memory that stores Rb
    pub rb_mem12: RbMem12,
    /// 0x0434
    /// memory that stores Rb
    pub rb_mem13: RbMem13,
    /// 0x0438
    /// memory that stores Rb
    pub rb_mem14: RbMem14,
    /// 0x043c
    /// memory that stores Rb
    pub rb_mem15: RbMem15,
    /// 0x0440
    /// memory that stores Rb
    pub rb_mem16: RbMem16,
    /// 0x0444
    /// memory that stores Rb
    pub rb_mem17: RbMem17,
    /// 0x0448
    /// memory that stores Rb
    pub rb_mem18: RbMem18,
    /// 0x044c
    /// memory that stores Rb
    pub rb_mem19: RbMem19,
    /// 0x0450
    /// memory that stores Rb
    pub rb_mem20: RbMem20,
    /// 0x0454
    /// memory that stores Rb
    pub rb_mem21: RbMem21,
    /// 0x0458
    /// memory that stores Rb
    pub rb_mem22: RbMem22,
    /// 0x045c
    /// memory that stores Rb
    pub rb_mem23: RbMem23,
    /// 0x0460
    /// memory that stores Rb
    pub rb_mem24: RbMem24,
    /// 0x0464
    /// memory that stores Rb
    pub rb_mem25: RbMem25,
    /// 0x0468
    /// memory that stores Rb
    pub rb_mem26: RbMem26,
    /// 0x046c
    /// memory that stores Rb
    pub rb_mem27: RbMem27,
    /// 0x0470
    /// memory that stores Rb
    pub rb_mem28: RbMem28,
    /// 0x0474
    /// memory that stores Rb
    pub rb_mem29: RbMem29,
    /// 0x0478
    /// memory that stores Rb
    pub rb_mem30: RbMem30,
    /// 0x047c
    /// memory that stores Rb
    pub rb_mem31: RbMem31,
    /// 0x0480
    /// memory that stores Rb
    pub rb_mem32: RbMem32,
    /// 0x0484
    /// memory that stores Rb
    pub rb_mem33: RbMem33,
    /// 0x0488
    /// memory that stores Rb
    pub rb_mem34: RbMem34,
    /// 0x048c
    /// memory that stores Rb
    pub rb_mem35: RbMem35,
    /// 0x0490
    /// memory that stores Rb
    pub rb_mem36: RbMem36,
    /// 0x0494
    /// memory that stores Rb
    pub rb_mem37: RbMem37,
    /// 0x0498
    /// memory that stores Rb
    pub rb_mem38: RbMem38,
    /// 0x049c
    /// memory that stores Rb
    pub rb_mem39: RbMem39,
    /// 0x04a0
    /// memory that stores Rb
    pub rb_mem40: RbMem40,
    /// 0x04a4
    /// memory that stores Rb
    pub rb_mem41: RbMem41,
    /// 0x04a8
    /// memory that stores Rb
    pub rb_mem42: RbMem42,
    /// 0x04ac
    /// memory that stores Rb
    pub rb_mem43: RbMem43,
    /// 0x04b0
    /// memory that stores Rb
    pub rb_mem44: RbMem44,
    /// 0x04b4
    /// memory that stores Rb
    pub rb_mem45: RbMem45,
    /// 0x04b8
    /// memory that stores Rb
    pub rb_mem46: RbMem46,
    /// 0x04bc
    /// memory that stores Rb
    pub rb_mem47: RbMem47,
    /// 0x04c0
    /// memory that stores Rb
    pub rb_mem48: RbMem48,
    /// 0x04c4
    /// memory that stores Rb
    pub rb_mem49: RbMem49,
    /// 0x04c8
    /// memory that stores Rb
    pub rb_mem50: RbMem50,
    /// 0x04cc
    /// memory that stores Rb
    pub rb_mem51: RbMem51,
    /// 0x04d0
    /// memory that stores Rb
    pub rb_mem52: RbMem52,
    /// 0x04d4
    /// memory that stores Rb
    pub rb_mem53: RbMem53,
    /// 0x04d8
    /// memory that stores Rb
    pub rb_mem54: RbMem54,
    /// 0x04dc
    /// memory that stores Rb
    pub rb_mem55: RbMem55,
    /// 0x04e0
    /// memory that stores Rb
    pub rb_mem56: RbMem56,
    /// 0x04e4
    /// memory that stores Rb
    pub rb_mem57: RbMem57,
    /// 0x04e8
    /// memory that stores Rb
    pub rb_mem58: RbMem58,
    /// 0x04ec
    /// memory that stores Rb
    pub rb_mem59: RbMem59,
    /// 0x04f0
    /// memory that stores Rb
    pub rb_mem60: RbMem60,
    /// 0x04f4
    /// memory that stores Rb
    pub rb_mem61: RbMem61,
    /// 0x04f8
    /// memory that stores Rb
    pub rb_mem62: RbMem62,
    /// 0x04fc
    /// memory that stores Rb
    pub rb_mem63: RbMem63,
    /// 0x0500
    /// memory that stores Rb
    pub rb_mem64: RbMem64,
    /// 0x0504
    /// memory that stores Rb
    pub rb_mem65: RbMem65,
    /// 0x0508
    /// memory that stores Rb
    pub rb_mem66: RbMem66,
    /// 0x050c
    /// memory that stores Rb
    pub rb_mem67: RbMem67,
    /// 0x0510
    /// memory that stores Rb
    pub rb_mem68: RbMem68,
    /// 0x0514
    /// memory that stores Rb
    pub rb_mem69: RbMem69,
    /// 0x0518
    /// memory that stores Rb
    pub rb_mem70: RbMem70,
    /// 0x051c
    /// memory that stores Rb
    pub rb_mem71: RbMem71,
    /// 0x0520
    /// memory that stores Rb
    pub rb_mem72: RbMem72,
    /// 0x0524
    /// memory that stores Rb
    pub rb_mem73: RbMem73,
    /// 0x0528
    /// memory that stores Rb
    pub rb_mem74: RbMem74,
    /// 0x052c
    /// memory that stores Rb
    pub rb_mem75: RbMem75,
    /// 0x0530
    /// memory that stores Rb
    pub rb_mem76: RbMem76,
    /// 0x0534
    /// memory that stores Rb
    pub rb_mem77: RbMem77,
    /// 0x0538
    /// memory that stores Rb
    pub rb_mem78: RbMem78,
    /// 0x053c
    /// memory that stores Rb
    pub rb_mem79: RbMem79,
    /// 0x0540
    /// memory that stores Rb
    pub rb_mem80: RbMem80,
    /// 0x0544
    /// memory that stores Rb
    pub rb_mem81: RbMem81,
    /// 0x0548
    /// memory that stores Rb
    pub rb_mem82: RbMem82,
    /// 0x054c
    /// memory that stores Rb
    pub rb_mem83: RbMem83,
    /// 0x0550
    /// memory that stores Rb
    pub rb_mem84: RbMem84,
    /// 0x0554
    /// memory that stores Rb
    pub rb_mem85: RbMem85,
    /// 0x0558
    /// memory that stores Rb
    pub rb_mem86: RbMem86,
    /// 0x055c
    /// memory that stores Rb
    pub rb_mem87: RbMem87,
    /// 0x0560
    /// memory that stores Rb
    pub rb_mem88: RbMem88,
    /// 0x0564
    /// memory that stores Rb
    pub rb_mem89: RbMem89,
    /// 0x0568
    /// memory that stores Rb
    pub rb_mem90: RbMem90,
    /// 0x056c
    /// memory that stores Rb
    pub rb_mem91: RbMem91,
    /// 0x0570
    /// memory that stores Rb
    pub rb_mem92: RbMem92,
    /// 0x0574
    /// memory that stores Rb
    pub rb_mem93: RbMem93,
    /// 0x0578
    /// memory that stores Rb
    pub rb_mem94: RbMem94,
    /// 0x057c
    /// memory that stores Rb
    pub rb_mem95: RbMem95,
    /// 0x0580
    /// memory that stores Rb
    pub rb_mem96: RbMem96,
    /// 0x0584
    /// memory that stores Rb
    pub rb_mem97: RbMem97,
    /// 0x0588
    /// memory that stores Rb
    pub rb_mem98: RbMem98,
    /// 0x058c
    /// memory that stores Rb
    pub rb_mem99: RbMem99,
    /// 0x0590
    /// memory that stores Rb
    pub rb_mem100: RbMem100,
    /// 0x0594
    /// memory that stores Rb
    pub rb_mem101: RbMem101,
    /// 0x0598
    /// memory that stores Rb
    pub rb_mem102: RbMem102,
    /// 0x059c
    /// memory that stores Rb
    pub rb_mem103: RbMem103,
    /// 0x05a0
    /// memory that stores Rb
    pub rb_mem104: RbMem104,
    /// 0x05a4
    /// memory that stores Rb
    pub rb_mem105: RbMem105,
    /// 0x05a8
    /// memory that stores Rb
    pub rb_mem106: RbMem106,
    /// 0x05ac
    /// memory that stores Rb
    pub rb_mem107: RbMem107,
    /// 0x05b0
    /// memory that stores Rb
    pub rb_mem108: RbMem108,
    /// 0x05b4
    /// memory that stores Rb
    pub rb_mem109: RbMem109,
    /// 0x05b8
    /// memory that stores Rb
    pub rb_mem110: RbMem110,
    /// 0x05bc
    /// memory that stores Rb
    pub rb_mem111: RbMem111,
    /// 0x05c0
    /// memory that stores Rb
    pub rb_mem112: RbMem112,
    /// 0x05c4
    /// memory that stores Rb
    pub rb_mem113: RbMem113,
    /// 0x05c8
    /// memory that stores Rb
    pub rb_mem114: RbMem114,
    /// 0x05cc
    /// memory that stores Rb
    pub rb_mem115: RbMem115,
    /// 0x05d0
    /// memory that stores Rb
    pub rb_mem116: RbMem116,
    /// 0x05d4
    /// memory that stores Rb
    pub rb_mem117: RbMem117,
    /// 0x05d8
    /// memory that stores Rb
    pub rb_mem118: RbMem118,
    /// 0x05dc
    /// memory that stores Rb
    pub rb_mem119: RbMem119,
    /// 0x05e0
    /// memory that stores Rb
    pub rb_mem120: RbMem120,
    /// 0x05e4
    /// memory that stores Rb
    pub rb_mem121: RbMem121,
    /// 0x05e8
    /// memory that stores Rb
    pub rb_mem122: RbMem122,
    /// 0x05ec
    /// memory that stores Rb
    pub rb_mem123: RbMem123,
    /// 0x05f0
    /// memory that stores Rb
    pub rb_mem124: RbMem124,
    /// 0x05f4
    /// memory that stores Rb
    pub rb_mem125: RbMem125,
    /// 0x05f8
    /// memory that stores Rb
    pub rb_mem126: RbMem126,
    /// 0x05fc
    /// memory that stores Rb
    pub rb_mem127: RbMem127,
    /// 0x0600
    /// memory that stores BOX
    pub box_mem0: BoxMem0,
    /// 0x0604
    /// memory that stores BOX
    pub box_mem1: BoxMem1,
    /// 0x0608
    /// memory that stores BOX
    pub box_mem2: BoxMem2,
    /// 0x060c
    /// memory that stores BOX
    pub box_mem3: BoxMem3,
    /// 0x0610
    /// memory that stores BOX
    pub box_mem4: BoxMem4,
    /// 0x0614
    /// memory that stores BOX
    pub box_mem5: BoxMem5,
    /// 0x0618
    /// memory that stores BOX
    pub box_mem6: BoxMem6,
    /// 0x061c
    /// memory that stores BOX
    pub box_mem7: BoxMem7,
    /// 0x0620
    /// memory that stores BOX
    pub box_mem8: BoxMem8,
    /// 0x0624
    /// memory that stores BOX
    pub box_mem9: BoxMem9,
    /// 0x0628
    /// memory that stores BOX
    pub box_mem10: BoxMem10,
    /// 0x062c
    /// memory that stores BOX
    pub box_mem11: BoxMem11,
    /// 0x0630
    /// memory that stores IV
    pub iv_mem0: IvMem0,
    /// 0x0634
    /// memory that stores IV
    pub iv_mem1: IvMem1,
    /// 0x0638
    /// memory that stores IV
    pub iv_mem2: IvMem2,
    /// 0x063c
    /// memory that stores IV
    pub iv_mem3: IvMem3,
    _reserved0: [u8; 448],
    /// 0x0800
    /// memory that stores X
    pub x_mem0: XMem0,
    /// 0x0804
    /// memory that stores X
    pub x_mem1: XMem1,
    /// 0x0808
    /// memory that stores X
    pub x_mem2: XMem2,
    /// 0x080c
    /// memory that stores X
    pub x_mem3: XMem3,
    /// 0x0810
    /// memory that stores X
    pub x_mem4: XMem4,
    /// 0x0814
    /// memory that stores X
    pub x_mem5: XMem5,
    /// 0x0818
    /// memory that stores X
    pub x_mem6: XMem6,
    /// 0x081c
    /// memory that stores X
    pub x_mem7: XMem7,
    /// 0x0820
    /// memory that stores X
    pub x_mem8: XMem8,
    /// 0x0824
    /// memory that stores X
    pub x_mem9: XMem9,
    /// 0x0828
    /// memory that stores X
    pub x_mem10: XMem10,
    /// 0x082c
    /// memory that stores X
    pub x_mem11: XMem11,
    /// 0x0830
    /// memory that stores X
    pub x_mem12: XMem12,
    /// 0x0834
    /// memory that stores X
    pub x_mem13: XMem13,
    /// 0x0838
    /// memory that stores X
    pub x_mem14: XMem14,
    /// 0x083c
    /// memory that stores X
    pub x_mem15: XMem15,
    /// 0x0840
    /// memory that stores X
    pub x_mem16: XMem16,
    /// 0x0844
    /// memory that stores X
    pub x_mem17: XMem17,
    /// 0x0848
    /// memory that stores X
    pub x_mem18: XMem18,
    /// 0x084c
    /// memory that stores X
    pub x_mem19: XMem19,
    /// 0x0850
    /// memory that stores X
    pub x_mem20: XMem20,
    /// 0x0854
    /// memory that stores X
    pub x_mem21: XMem21,
    /// 0x0858
    /// memory that stores X
    pub x_mem22: XMem22,
    /// 0x085c
    /// memory that stores X
    pub x_mem23: XMem23,
    /// 0x0860
    /// memory that stores X
    pub x_mem24: XMem24,
    /// 0x0864
    /// memory that stores X
    pub x_mem25: XMem25,
    /// 0x0868
    /// memory that stores X
    pub x_mem26: XMem26,
    /// 0x086c
    /// memory that stores X
    pub x_mem27: XMem27,
    /// 0x0870
    /// memory that stores X
    pub x_mem28: XMem28,
    /// 0x0874
    /// memory that stores X
    pub x_mem29: XMem29,
    /// 0x0878
    /// memory that stores X
    pub x_mem30: XMem30,
    /// 0x087c
    /// memory that stores X
    pub x_mem31: XMem31,
    /// 0x0880
    /// memory that stores X
    pub x_mem32: XMem32,
    /// 0x0884
    /// memory that stores X
    pub x_mem33: XMem33,
    /// 0x0888
    /// memory that stores X
    pub x_mem34: XMem34,
    /// 0x088c
    /// memory that stores X
    pub x_mem35: XMem35,
    /// 0x0890
    /// memory that stores X
    pub x_mem36: XMem36,
    /// 0x0894
    /// memory that stores X
    pub x_mem37: XMem37,
    /// 0x0898
    /// memory that stores X
    pub x_mem38: XMem38,
    /// 0x089c
    /// memory that stores X
    pub x_mem39: XMem39,
    /// 0x08a0
    /// memory that stores X
    pub x_mem40: XMem40,
    /// 0x08a4
    /// memory that stores X
    pub x_mem41: XMem41,
    /// 0x08a8
    /// memory that stores X
    pub x_mem42: XMem42,
    /// 0x08ac
    /// memory that stores X
    pub x_mem43: XMem43,
    /// 0x08b0
    /// memory that stores X
    pub x_mem44: XMem44,
    /// 0x08b4
    /// memory that stores X
    pub x_mem45: XMem45,
    /// 0x08b8
    /// memory that stores X
    pub x_mem46: XMem46,
    /// 0x08bc
    /// memory that stores X
    pub x_mem47: XMem47,
    /// 0x08c0
    /// memory that stores X
    pub x_mem48: XMem48,
    /// 0x08c4
    /// memory that stores X
    pub x_mem49: XMem49,
    /// 0x08c8
    /// memory that stores X
    pub x_mem50: XMem50,
    /// 0x08cc
    /// memory that stores X
    pub x_mem51: XMem51,
    /// 0x08d0
    /// memory that stores X
    pub x_mem52: XMem52,
    /// 0x08d4
    /// memory that stores X
    pub x_mem53: XMem53,
    /// 0x08d8
    /// memory that stores X
    pub x_mem54: XMem54,
    /// 0x08dc
    /// memory that stores X
    pub x_mem55: XMem55,
    /// 0x08e0
    /// memory that stores X
    pub x_mem56: XMem56,
    /// 0x08e4
    /// memory that stores X
    pub x_mem57: XMem57,
    /// 0x08e8
    /// memory that stores X
    pub x_mem58: XMem58,
    /// 0x08ec
    /// memory that stores X
    pub x_mem59: XMem59,
    /// 0x08f0
    /// memory that stores X
    pub x_mem60: XMem60,
    /// 0x08f4
    /// memory that stores X
    pub x_mem61: XMem61,
    /// 0x08f8
    /// memory that stores X
    pub x_mem62: XMem62,
    /// 0x08fc
    /// memory that stores X
    pub x_mem63: XMem63,
    /// 0x0900
    /// memory that stores X
    pub x_mem64: XMem64,
    /// 0x0904
    /// memory that stores X
    pub x_mem65: XMem65,
    /// 0x0908
    /// memory that stores X
    pub x_mem66: XMem66,
    /// 0x090c
    /// memory that stores X
    pub x_mem67: XMem67,
    /// 0x0910
    /// memory that stores X
    pub x_mem68: XMem68,
    /// 0x0914
    /// memory that stores X
    pub x_mem69: XMem69,
    /// 0x0918
    /// memory that stores X
    pub x_mem70: XMem70,
    /// 0x091c
    /// memory that stores X
    pub x_mem71: XMem71,
    /// 0x0920
    /// memory that stores X
    pub x_mem72: XMem72,
    /// 0x0924
    /// memory that stores X
    pub x_mem73: XMem73,
    /// 0x0928
    /// memory that stores X
    pub x_mem74: XMem74,
    /// 0x092c
    /// memory that stores X
    pub x_mem75: XMem75,
    /// 0x0930
    /// memory that stores X
    pub x_mem76: XMem76,
    /// 0x0934
    /// memory that stores X
    pub x_mem77: XMem77,
    /// 0x0938
    /// memory that stores X
    pub x_mem78: XMem78,
    /// 0x093c
    /// memory that stores X
    pub x_mem79: XMem79,
    /// 0x0940
    /// memory that stores X
    pub x_mem80: XMem80,
    /// 0x0944
    /// memory that stores X
    pub x_mem81: XMem81,
    /// 0x0948
    /// memory that stores X
    pub x_mem82: XMem82,
    /// 0x094c
    /// memory that stores X
    pub x_mem83: XMem83,
    /// 0x0950
    /// memory that stores X
    pub x_mem84: XMem84,
    /// 0x0954
    /// memory that stores X
    pub x_mem85: XMem85,
    /// 0x0958
    /// memory that stores X
    pub x_mem86: XMem86,
    /// 0x095c
    /// memory that stores X
    pub x_mem87: XMem87,
    /// 0x0960
    /// memory that stores X
    pub x_mem88: XMem88,
    /// 0x0964
    /// memory that stores X
    pub x_mem89: XMem89,
    /// 0x0968
    /// memory that stores X
    pub x_mem90: XMem90,
    /// 0x096c
    /// memory that stores X
    pub x_mem91: XMem91,
    /// 0x0970
    /// memory that stores X
    pub x_mem92: XMem92,
    /// 0x0974
    /// memory that stores X
    pub x_mem93: XMem93,
    /// 0x0978
    /// memory that stores X
    pub x_mem94: XMem94,
    /// 0x097c
    /// memory that stores X
    pub x_mem95: XMem95,
    /// 0x0980
    /// memory that stores X
    pub x_mem96: XMem96,
    /// 0x0984
    /// memory that stores X
    pub x_mem97: XMem97,
    /// 0x0988
    /// memory that stores X
    pub x_mem98: XMem98,
    /// 0x098c
    /// memory that stores X
    pub x_mem99: XMem99,
    /// 0x0990
    /// memory that stores X
    pub x_mem100: XMem100,
    /// 0x0994
    /// memory that stores X
    pub x_mem101: XMem101,
    /// 0x0998
    /// memory that stores X
    pub x_mem102: XMem102,
    /// 0x099c
    /// memory that stores X
    pub x_mem103: XMem103,
    /// 0x09a0
    /// memory that stores X
    pub x_mem104: XMem104,
    /// 0x09a4
    /// memory that stores X
    pub x_mem105: XMem105,
    /// 0x09a8
    /// memory that stores X
    pub x_mem106: XMem106,
    /// 0x09ac
    /// memory that stores X
    pub x_mem107: XMem107,
    /// 0x09b0
    /// memory that stores X
    pub x_mem108: XMem108,
    /// 0x09b4
    /// memory that stores X
    pub x_mem109: XMem109,
    /// 0x09b8
    /// memory that stores X
    pub x_mem110: XMem110,
    /// 0x09bc
    /// memory that stores X
    pub x_mem111: XMem111,
    /// 0x09c0
    /// memory that stores X
    pub x_mem112: XMem112,
    /// 0x09c4
    /// memory that stores X
    pub x_mem113: XMem113,
    /// 0x09c8
    /// memory that stores X
    pub x_mem114: XMem114,
    /// 0x09cc
    /// memory that stores X
    pub x_mem115: XMem115,
    /// 0x09d0
    /// memory that stores X
    pub x_mem116: XMem116,
    /// 0x09d4
    /// memory that stores X
    pub x_mem117: XMem117,
    /// 0x09d8
    /// memory that stores X
    pub x_mem118: XMem118,
    /// 0x09dc
    /// memory that stores X
    pub x_mem119: XMem119,
    /// 0x09e0
    /// memory that stores X
    pub x_mem120: XMem120,
    /// 0x09e4
    /// memory that stores X
    pub x_mem121: XMem121,
    /// 0x09e8
    /// memory that stores X
    pub x_mem122: XMem122,
    /// 0x09ec
    /// memory that stores X
    pub x_mem123: XMem123,
    /// 0x09f0
    /// memory that stores X
    pub x_mem124: XMem124,
    /// 0x09f4
    /// memory that stores X
    pub x_mem125: XMem125,
    /// 0x09f8
    /// memory that stores X
    pub x_mem126: XMem126,
    /// 0x09fc
    /// memory that stores X
    pub x_mem127: XMem127,
    /// 0x0a00
    /// memory that stores Z
    pub z_mem0: ZMem0,
    /// 0x0a04
    /// memory that stores Z
    pub z_mem1: ZMem1,
    /// 0x0a08
    /// memory that stores Z
    pub z_mem2: ZMem2,
    /// 0x0a0c
    /// memory that stores Z
    pub z_mem3: ZMem3,
    /// 0x0a10
    /// memory that stores Z
    pub z_mem4: ZMem4,
    /// 0x0a14
    /// memory that stores Z
    pub z_mem5: ZMem5,
    /// 0x0a18
    /// memory that stores Z
    pub z_mem6: ZMem6,
    /// 0x0a1c
    /// memory that stores Z
    pub z_mem7: ZMem7,
    /// 0x0a20
    /// memory that stores Z
    pub z_mem8: ZMem8,
    /// 0x0a24
    /// memory that stores Z
    pub z_mem9: ZMem9,
    /// 0x0a28
    /// memory that stores Z
    pub z_mem10: ZMem10,
    /// 0x0a2c
    /// memory that stores Z
    pub z_mem11: ZMem11,
    /// 0x0a30
    /// memory that stores Z
    pub z_mem12: ZMem12,
    /// 0x0a34
    /// memory that stores Z
    pub z_mem13: ZMem13,
    /// 0x0a38
    /// memory that stores Z
    pub z_mem14: ZMem14,
    /// 0x0a3c
    /// memory that stores Z
    pub z_mem15: ZMem15,
    /// 0x0a40
    /// memory that stores Z
    pub z_mem16: ZMem16,
    /// 0x0a44
    /// memory that stores Z
    pub z_mem17: ZMem17,
    /// 0x0a48
    /// memory that stores Z
    pub z_mem18: ZMem18,
    /// 0x0a4c
    /// memory that stores Z
    pub z_mem19: ZMem19,
    /// 0x0a50
    /// memory that stores Z
    pub z_mem20: ZMem20,
    /// 0x0a54
    /// memory that stores Z
    pub z_mem21: ZMem21,
    /// 0x0a58
    /// memory that stores Z
    pub z_mem22: ZMem22,
    /// 0x0a5c
    /// memory that stores Z
    pub z_mem23: ZMem23,
    /// 0x0a60
    /// memory that stores Z
    pub z_mem24: ZMem24,
    /// 0x0a64
    /// memory that stores Z
    pub z_mem25: ZMem25,
    /// 0x0a68
    /// memory that stores Z
    pub z_mem26: ZMem26,
    /// 0x0a6c
    /// memory that stores Z
    pub z_mem27: ZMem27,
    /// 0x0a70
    /// memory that stores Z
    pub z_mem28: ZMem28,
    /// 0x0a74
    /// memory that stores Z
    pub z_mem29: ZMem29,
    /// 0x0a78
    /// memory that stores Z
    pub z_mem30: ZMem30,
    /// 0x0a7c
    /// memory that stores Z
    pub z_mem31: ZMem31,
    /// 0x0a80
    /// memory that stores Z
    pub z_mem32: ZMem32,
    /// 0x0a84
    /// memory that stores Z
    pub z_mem33: ZMem33,
    /// 0x0a88
    /// memory that stores Z
    pub z_mem34: ZMem34,
    /// 0x0a8c
    /// memory that stores Z
    pub z_mem35: ZMem35,
    /// 0x0a90
    /// memory that stores Z
    pub z_mem36: ZMem36,
    /// 0x0a94
    /// memory that stores Z
    pub z_mem37: ZMem37,
    /// 0x0a98
    /// memory that stores Z
    pub z_mem38: ZMem38,
    /// 0x0a9c
    /// memory that stores Z
    pub z_mem39: ZMem39,
    /// 0x0aa0
    /// memory that stores Z
    pub z_mem40: ZMem40,
    /// 0x0aa4
    /// memory that stores Z
    pub z_mem41: ZMem41,
    /// 0x0aa8
    /// memory that stores Z
    pub z_mem42: ZMem42,
    /// 0x0aac
    /// memory that stores Z
    pub z_mem43: ZMem43,
    /// 0x0ab0
    /// memory that stores Z
    pub z_mem44: ZMem44,
    /// 0x0ab4
    /// memory that stores Z
    pub z_mem45: ZMem45,
    /// 0x0ab8
    /// memory that stores Z
    pub z_mem46: ZMem46,
    /// 0x0abc
    /// memory that stores Z
    pub z_mem47: ZMem47,
    /// 0x0ac0
    /// memory that stores Z
    pub z_mem48: ZMem48,
    /// 0x0ac4
    /// memory that stores Z
    pub z_mem49: ZMem49,
    /// 0x0ac8
    /// memory that stores Z
    pub z_mem50: ZMem50,
    /// 0x0acc
    /// memory that stores Z
    pub z_mem51: ZMem51,
    /// 0x0ad0
    /// memory that stores Z
    pub z_mem52: ZMem52,
    /// 0x0ad4
    /// memory that stores Z
    pub z_mem53: ZMem53,
    /// 0x0ad8
    /// memory that stores Z
    pub z_mem54: ZMem54,
    /// 0x0adc
    /// memory that stores Z
    pub z_mem55: ZMem55,
    /// 0x0ae0
    /// memory that stores Z
    pub z_mem56: ZMem56,
    /// 0x0ae4
    /// memory that stores Z
    pub z_mem57: ZMem57,
    /// 0x0ae8
    /// memory that stores Z
    pub z_mem58: ZMem58,
    /// 0x0aec
    /// memory that stores Z
    pub z_mem59: ZMem59,
    /// 0x0af0
    /// memory that stores Z
    pub z_mem60: ZMem60,
    /// 0x0af4
    /// memory that stores Z
    pub z_mem61: ZMem61,
    /// 0x0af8
    /// memory that stores Z
    pub z_mem62: ZMem62,
    /// 0x0afc
    /// memory that stores Z
    pub z_mem63: ZMem63,
    /// 0x0b00
    /// memory that stores Z
    pub z_mem64: ZMem64,
    /// 0x0b04
    /// memory that stores Z
    pub z_mem65: ZMem65,
    /// 0x0b08
    /// memory that stores Z
    pub z_mem66: ZMem66,
    /// 0x0b0c
    /// memory that stores Z
    pub z_mem67: ZMem67,
    /// 0x0b10
    /// memory that stores Z
    pub z_mem68: ZMem68,
    /// 0x0b14
    /// memory that stores Z
    pub z_mem69: ZMem69,
    /// 0x0b18
    /// memory that stores Z
    pub z_mem70: ZMem70,
    /// 0x0b1c
    /// memory that stores Z
    pub z_mem71: ZMem71,
    /// 0x0b20
    /// memory that stores Z
    pub z_mem72: ZMem72,
    /// 0x0b24
    /// memory that stores Z
    pub z_mem73: ZMem73,
    /// 0x0b28
    /// memory that stores Z
    pub z_mem74: ZMem74,
    /// 0x0b2c
    /// memory that stores Z
    pub z_mem75: ZMem75,
    /// 0x0b30
    /// memory that stores Z
    pub z_mem76: ZMem76,
    /// 0x0b34
    /// memory that stores Z
    pub z_mem77: ZMem77,
    /// 0x0b38
    /// memory that stores Z
    pub z_mem78: ZMem78,
    /// 0x0b3c
    /// memory that stores Z
    pub z_mem79: ZMem79,
    /// 0x0b40
    /// memory that stores Z
    pub z_mem80: ZMem80,
    /// 0x0b44
    /// memory that stores Z
    pub z_mem81: ZMem81,
    /// 0x0b48
    /// memory that stores Z
    pub z_mem82: ZMem82,
    /// 0x0b4c
    /// memory that stores Z
    pub z_mem83: ZMem83,
    /// 0x0b50
    /// memory that stores Z
    pub z_mem84: ZMem84,
    /// 0x0b54
    /// memory that stores Z
    pub z_mem85: ZMem85,
    /// 0x0b58
    /// memory that stores Z
    pub z_mem86: ZMem86,
    /// 0x0b5c
    /// memory that stores Z
    pub z_mem87: ZMem87,
    /// 0x0b60
    /// memory that stores Z
    pub z_mem88: ZMem88,
    /// 0x0b64
    /// memory that stores Z
    pub z_mem89: ZMem89,
    /// 0x0b68
    /// memory that stores Z
    pub z_mem90: ZMem90,
    /// 0x0b6c
    /// memory that stores Z
    pub z_mem91: ZMem91,
    /// 0x0b70
    /// memory that stores Z
    pub z_mem92: ZMem92,
    /// 0x0b74
    /// memory that stores Z
    pub z_mem93: ZMem93,
    /// 0x0b78
    /// memory that stores Z
    pub z_mem94: ZMem94,
    /// 0x0b7c
    /// memory that stores Z
    pub z_mem95: ZMem95,
    /// 0x0b80
    /// memory that stores Z
    pub z_mem96: ZMem96,
    /// 0x0b84
    /// memory that stores Z
    pub z_mem97: ZMem97,
    /// 0x0b88
    /// memory that stores Z
    pub z_mem98: ZMem98,
    /// 0x0b8c
    /// memory that stores Z
    pub z_mem99: ZMem99,
    /// 0x0b90
    /// memory that stores Z
    pub z_mem100: ZMem100,
    /// 0x0b94
    /// memory that stores Z
    pub z_mem101: ZMem101,
    /// 0x0b98
    /// memory that stores Z
    pub z_mem102: ZMem102,
    /// 0x0b9c
    /// memory that stores Z
    pub z_mem103: ZMem103,
    /// 0x0ba0
    /// memory that stores Z
    pub z_mem104: ZMem104,
    /// 0x0ba4
    /// memory that stores Z
    pub z_mem105: ZMem105,
    /// 0x0ba8
    /// memory that stores Z
    pub z_mem106: ZMem106,
    /// 0x0bac
    /// memory that stores Z
    pub z_mem107: ZMem107,
    /// 0x0bb0
    /// memory that stores Z
    pub z_mem108: ZMem108,
    /// 0x0bb4
    /// memory that stores Z
    pub z_mem109: ZMem109,
    /// 0x0bb8
    /// memory that stores Z
    pub z_mem110: ZMem110,
    /// 0x0bbc
    /// memory that stores Z
    pub z_mem111: ZMem111,
    /// 0x0bc0
    /// memory that stores Z
    pub z_mem112: ZMem112,
    /// 0x0bc4
    /// memory that stores Z
    pub z_mem113: ZMem113,
    /// 0x0bc8
    /// memory that stores Z
    pub z_mem114: ZMem114,
    /// 0x0bcc
    /// memory that stores Z
    pub z_mem115: ZMem115,
    /// 0x0bd0
    /// memory that stores Z
    pub z_mem116: ZMem116,
    /// 0x0bd4
    /// memory that stores Z
    pub z_mem117: ZMem117,
    /// 0x0bd8
    /// memory that stores Z
    pub z_mem118: ZMem118,
    /// 0x0bdc
    /// memory that stores Z
    pub z_mem119: ZMem119,
    /// 0x0be0
    /// memory that stores Z
    pub z_mem120: ZMem120,
    /// 0x0be4
    /// memory that stores Z
    pub z_mem121: ZMem121,
    /// 0x0be8
    /// memory that stores Z
    pub z_mem122: ZMem122,
    /// 0x0bec
    /// memory that stores Z
    pub z_mem123: ZMem123,
    /// 0x0bf0
    /// memory that stores Z
    pub z_mem124: ZMem124,
    /// 0x0bf4
    /// memory that stores Z
    pub z_mem125: ZMem125,
    /// 0x0bf8
    /// memory that stores Z
    pub z_mem126: ZMem126,
    /// 0x0bfc
    /// memory that stores Z
    pub z_mem127: ZMem127,
    _reserved1: [u8; 512],
    /// 0x0e00
    /// DS start control register
    pub set_start: SetStart,
    /// 0x0e04
    /// DS continue control register
    pub set_continue: SetContinue,
    /// 0x0e08
    /// DS finish control register
    pub set_finish: SetFinish,
    /// 0x0e0c
    /// DS query busy register
    pub query_busy: QueryBusy,
    /// 0x0e10
    /// DS query key-wrong counter register
    pub query_key_wrong: QueryKeyWrong,
    /// 0x0e14
    /// DS query check result register
    pub query_check: QueryCheck,
    _reserved2: [u8; 8],
    /// 0x0e20
    /// DS version control register
    pub date: Date,
}

//! Fields specific for NOR flash

/// `pageSize` field a serial NOR-specific FCB
pub struct PageSize([u8; 4]);
/// `sectorSize` field a serial NOR-specific FCB
pub struct SectorSize([u8; 4]);

impl PageSize {
    pub fn new(page_size: u32) -> Self {
        PageSize(page_size.to_le_bytes())
    }
}

as_ref_bytes_newtype!(PageSize);

impl SectorSize {
    pub fn new(sector_size: u32) -> Self {
        SectorSize(sector_size.to_le_bytes())
    }
}

as_ref_bytes_newtype!(SectorSize);

/// `ipCmdSerialClkFreq` field for serial NOR-specific FCB
///
/// Chip specific value, not used by ROM
#[repr(u8)]
pub enum SerialClockFrequency {
    /// No change, keep current serial clock unchanged
    NoChange = 0,
    MHz30 = 1,
    MHz50 = 2,
    MHz60 = 3,
    MHz75 = 4,
    MHz80 = 5,
    MHz100 = 6,
    #[cfg(any(feature = "imxrt1061", feature = "imxrt1062", feature = "imxrt1064"))]
    MHz120 = 7,
    #[cfg(feature = "imxrt1011")]
    MHz133 = 7,
    #[cfg(any(feature = "imxrt1061", feature = "imxrt1062", feature = "imxrt1064"))]
    MHz133 = 8,
    #[cfg(any(feature = "imxrt1061", feature = "imxrt1062", feature = "imxrt1064"))]
    MHz166 = 9,
}

/// The fields specific for defining a serial NOR FCB
pub struct ConfigurationBlock {
    pub page_size: PageSize,
    pub sector_size: SectorSize,
    pub ip_cmd_serial_clk_freq: SerialClockFrequency,
}

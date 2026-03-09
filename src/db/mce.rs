//! MCA (Machine Check Architecture) bank type and error code tables.
//!
//! AMD SMCA and Intel MCA bank layouts, plus MCA error code classification.

/// AMD SMCA bank type name by bank index.
/// Covers Zen 3/4/5 layout (family 0x19, 0x1A).
pub fn amd_smca_bank_name(bank: u8) -> &'static str {
    match bank {
        0 => "Load-Store",
        1 => "Instruction Fetch",
        2 | 3 => "L2 Cache",
        4 => "Decode",
        5 => "Execution",
        6 | 7 => "Floating Point",
        8..=11 => "L3 Cache",
        12 | 13 => "Coherent Slave",
        14 => "Platform Interface",
        15..=19 => "MCA Extension",
        20 | 21 => "Unified Memory Controller",
        22 | 23 => "UMC Extension",
        24 | 25 => "Parameter Block",
        26 | 27 => "PSP",
        28 | 29 => "SMU",
        30 | 31 => "NBIO/PCIe",
        _ => "Unknown",
    }
}

/// Intel MCA bank type name by bank index.
/// Common layout across Core/Xeon (Skylake through Raptor Lake / Sapphire Rapids).
/// Bank assignments vary by microarchitecture; these are the standard mappings.
pub fn intel_mca_bank_name(bank: u8) -> &'static str {
    match bank {
        0 => "DCU",       // Data Cache Unit
        1 => "IFU",       // Instruction Fetch Unit
        2 => "DTLB",      // Data TLB
        3 => "MLC",       // Mid-Level Cache (L2)
        4 => "PCU",       // Power Control Unit
        5 => "UPI/QPI",   // Intel Ultra Path / QuickPath Interconnect
        6 => "IIO",       // Integrated I/O
        7 => "M2M",       // Mesh to Memory
        8 | 9 => "M2M",   // Additional M2M banks (multi-socket)
        10 | 11 => "CHA", // Caching/Home Agent
        12 => "IMC",      // Integrated Memory Controller
        13..=15 => "IMC", // Additional IMC channels
        16..=19 => "CHA", // Additional CHA banks
        _ => "Bank",
    }
}

/// Classify MCA_STATUS ErrorCode[15:0] into a human-readable error type.
/// The error code encoding is shared between AMD and Intel (IA-32 MCA standard).
///
/// - `0000 0000 0001 TTLL` → TLB error
/// - `0000 0001 RRRR TTLL` → Memory/Cache error
/// - `0000 1PPT RRRR TTLL` → Bus/Interconnect error
pub fn mca_error_type(error_code: u16) -> &'static str {
    if error_code == 0 {
        return "No Error";
    }
    // Bus error: bit 11 set (0000 1PPT RRRR TTLL)
    if (error_code & 0x0800) != 0 {
        return "Bus/Interconnect Error";
    }
    // Memory/Cache error: bits [11:8] = 0001 (0000 0001 RRRR TTLL)
    if (error_code & 0x0F00) == 0x0100 {
        return "Memory/Cache Error";
    }
    // TLB error: bits [11:4] = 0000 0001 (0000 0000 0001 TTLL)
    if (error_code & 0x0FF0) == 0x0010 {
        return "TLB Error";
    }
    "Internal Error"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amd_smca_bank_names() {
        assert_eq!(amd_smca_bank_name(0), "Load-Store");
        assert_eq!(amd_smca_bank_name(1), "Instruction Fetch");
        assert_eq!(amd_smca_bank_name(2), "L2 Cache");
        assert_eq!(amd_smca_bank_name(5), "Execution");
        assert_eq!(amd_smca_bank_name(8), "L3 Cache");
        assert_eq!(amd_smca_bank_name(20), "Unified Memory Controller");
        assert_eq!(amd_smca_bank_name(30), "NBIO/PCIe");
        assert_eq!(amd_smca_bank_name(99), "Unknown");
    }

    #[test]
    fn test_intel_mca_bank_names() {
        assert_eq!(intel_mca_bank_name(0), "DCU");
        assert_eq!(intel_mca_bank_name(1), "IFU");
        assert_eq!(intel_mca_bank_name(2), "DTLB");
        assert_eq!(intel_mca_bank_name(3), "MLC");
        assert_eq!(intel_mca_bank_name(4), "PCU");
        assert_eq!(intel_mca_bank_name(5), "UPI/QPI");
        assert_eq!(intel_mca_bank_name(6), "IIO");
        assert_eq!(intel_mca_bank_name(7), "M2M");
        assert_eq!(intel_mca_bank_name(12), "IMC");
        assert_eq!(intel_mca_bank_name(99), "Bank");
    }

    #[test]
    fn test_mca_error_codes() {
        assert_eq!(mca_error_type(0), "No Error");
        assert_eq!(mca_error_type(0x0010), "TLB Error");
        assert_eq!(mca_error_type(0x0110), "Memory/Cache Error");
        assert_eq!(mca_error_type(0x0800), "Bus/Interconnect Error");
        assert_eq!(mca_error_type(0xFF00), "Bus/Interconnect Error"); // bit 11 set
        assert_eq!(mca_error_type(0x0001), "Internal Error"); // no standard pattern
    }
}

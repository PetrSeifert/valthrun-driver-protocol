use bitflags::bitflags;

use crate::utils;

pub type ProcessId = u32;

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DriverFeature : u64 {
        const MemoryRead                = 0x01;
        const MemoryWrite               = 0x02;

        const InputKeyboard             = 0x04;
        const InputMouse                = 0x08;

        const ProcessProtectionKernel   = 0x10;
        const ProcessProtectionZenith   = 0x20;

        const Metrics                   = 0x40;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessFilter {
    None,
    Id { id: ProcessId },
    ImageBaseName { name: *const u8, name_length: usize },
}

impl Default for ProcessFilter {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessModuleInfo {
    pub base_dll_name: [u8; 0x100],
    pub base_address: u64,
    pub module_size: u64,
}

impl ProcessModuleInfo {
    pub fn get_base_dll_name(&self) -> Option<&str> {
        utils::fixed_buffer_to_str(&self.base_dll_name)
    }

    pub fn set_base_dll_name(&mut self, value: &str) -> bool {
        utils::str_to_fixed_buffer(&mut self.base_dll_name, value)
    }
}

impl Default for ProcessModuleInfo {
    fn default() -> Self {
        Self {
            base_dll_name: [0; 0x100],
            base_address: 0,
            module_size: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryAccessResult {
    Success,
    PartialSuccess { bytes_copied: usize },

    ProcessUnknown,
}

impl Default for MemoryAccessResult {
    fn default() -> Self {
        MemoryAccessResult::ProcessUnknown
    }
}

use crate::display_control::DDCControl;
use anyhow::{anyhow, Result};

pub struct DDCControlMacos();

extern "C" {
    fn ddcWriteInputSelect(screen_idx: isize, input: u16) -> bool;
    fn ddcReadInputSelect(screen_idx: isize) -> isize;
    fn getDisplayCount() -> isize;
}

impl DDCControl for DDCControlMacos {
    fn get_display_range() -> std::ops::Range<isize> {
        unsafe { 0..getDisplayCount() }
    }

    fn ddc_read_input_select(screen_idx: isize) -> Result<u16> {
        let source = unsafe { ddcReadInputSelect(screen_idx) };
        if source > 0 {
            info!(
                "Monitor '{:?}' is currently set to 0x{:x}",
                screen_idx, source
            );
            Ok(source as u16)
        } else {
            error!("Failed to get current input for monitor '{:?}'", screen_idx);
            Err(anyhow!("DDC error"))
        }
    }

    fn ddc_write_input_select(screen_idx: isize, source: u16) -> Result<()> {
        let result = unsafe { ddcWriteInputSelect(screen_idx, source) };
        if result {
            info!("Monitor '{:?}' set to 0x{:x}", screen_idx, source);
            Ok(())
        } else {
            error!("Failed to set monitor '{:?}' to 0x{:x}", screen_idx, source);
            Err(anyhow!("DDC error"))
        }
    }
}

use utility::Global;
use super::buffers::BootBuffer;
use super::drivers::GlobalFacade;
use super::printers::ScrollPrinter;
use super::drivers::vga_text_driver::VgaTextDriver;

pub static SYSTEM_DISPLAY: Global<VgaTextDriver> = Global::new("SYSTEM_DISPLAY");
pub static SYSTEM_PRINTER: Global<ScrollPrinter<BootBuffer<GlobalFacade>>> = Global::new("SYSTEM_PRINTER");

pub fn initialize() {
	SYSTEM_DISPLAY.set(unsafe { VgaTextDriver::new() });
	SYSTEM_PRINTER.set(ScrollPrinter::new(BootBuffer::new(GlobalFacade {})));
}

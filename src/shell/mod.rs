use crate::io::vga::vga_screen::VgaScreen;

pub const SHELL_PROMPT: &str = "$> ";
const MAX_COMMANDS_IN_HISTORY: usize = 20;

pub struct Shell {
    vga_screen: VgaScreen,
}

extern crate gameman;
extern crate env_logger;

use gameman::emu::Emulator;

fn main() {
    match env_logger::init() {
        Ok(v) => println!("Logger started: {:?}", v),
        Err(e) => println!("Failed to start logger: {:?}", e),
    }

    let mut emulator = Emulator::new();
    emulator.load_bios();
    emulator.load_rom("roms/Tetris (World).gb");
//    emulator.load_rom("roms/individual/03-op sp,hl.gb");
    emulator.run();
}

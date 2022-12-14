use clap::Parser;

mod core;
mod emulator;

use crate::core::rom::Rom;
use crate::emulator::emulator::Emulator;

fn main() {
    let args = Args::parse();
    let rom: Rom = Rom::new(&args.rom_file_path).unwrap();
    let emulator = Emulator::new(rom);
}

// TODO: Convert to some config file in the future
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to ROM file
    #[clap(short, long)]
    rom_file_path: String,

    /// Scale factor for display
    #[clap(short, long, default_value_t = 20)]
    scale: u32,

    /// Refresh Rate
    #[clap(short, long, default_value_t = 60)]
    fps: u32,

    /// CPU frequency
    #[clap(short, long, default_value_t = 500)]
    cpu_freq: u32,

    /// Timer frequency
    #[clap(short, long, default_value_t = 60)]
    timer_freq: u32,
}

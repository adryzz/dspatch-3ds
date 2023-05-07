#![feature(fs_try_exists)]

mod haxxstation;

use std::fs::read;
use std::path::Path;

use ctru::prelude::*;
use ctru::services::romfs::RomFS;

const ROMFS_PATH0: &str = "romfs:/ds-download-station.nds";
const ROMFS_PATH1: &str =
    "romfs:/xxxx - DS Download Station - Volume 1 (Kiosk WiFi Demo Cart) (U)(Independent).nds";

const SDMC_PATH0: &str = "sdmc:/3ds/dspatch-3ds/ds-download-station.nds";
const SDMC_PATH1: &str = "sdmc:/3ds/dspatch-3ds/xxxx - DS Download Station - Volume 1 (Kiosk WiFi Demo Cart) (U)(Independent).nds";

const OUT_PATH0: &str = "sdmc:/3ds/dspatch-3ds/haxxstation.nds";
const OUT_PATH1: &str = "sdmc:/roms/nds/haxxstation.nds";

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());

    // init RomFS
    let _romfs = RomFS::new().unwrap();

    println!("== DS Download Station Patcher v1.0 3DS Edition ==\n");
    println!("Exploit by Gericom, shutterbug2000 and Apache Thunder");
    println!("Ported by Lena to Rust and the 3DS\n");

    let mut download_station_bytes: Option<Vec<u8>> = None;

    // Check presence of the DS Download Station file first in the RomFS and then in the SD Card.
    let romfs_path0 = Path::new(ROMFS_PATH0);
    let romfs_path1 = Path::new(ROMFS_PATH1);
    let sdmc_path0 = Path::new(SDMC_PATH0);
    let sdmc_path1 = Path::new(SDMC_PATH1);

    if romfs_path0.is_file() {
        match read(romfs_path0) {
            Ok(bytes) => {
                download_station_bytes = Some(bytes);
            }
            Err(e) => eprintln!("{}", e),
        }
    } else if romfs_path1.is_file() {
        match read(romfs_path1) {
            Ok(bytes) => {
                download_station_bytes = Some(bytes);
            }
            Err(e) => eprintln!("{}", e),
        }
    } else if sdmc_path0.is_file() {
        match read(sdmc_path0) {
            Ok(bytes) => {
                download_station_bytes = Some(bytes);
            }
            Err(e) => eprintln!("{}", e),
        }
    } else if sdmc_path1.is_file() {
        match read(sdmc_path1) {
            Ok(bytes) => {
                download_station_bytes = Some(bytes);
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    for d in std::fs::read_dir("").unwrap() {
        println!("{:?}", d);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    if download_station_bytes.is_none() {
        println!("Could not find the DS Download Station file to patch!");
        println!(
            "Ensure it's either baked in the RomFS or it exists in either of the following paths:\n{}\n{}",
            SDMC_PATH0, SDMC_PATH1
        )
    }

    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}

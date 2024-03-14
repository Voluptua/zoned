use bincode::serialize;
use std::io::Write as _;
use std::{fs, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::cli::InputOutput;

#[derive(Serialize, Deserialize, Debug, Default)]
#[repr(C)]
pub struct ZoneHeaderData {
    map_type: u8,
    map_move: u8,
    area_data_id: u16,
    map_matrix_id: u16,
    text_id: u16,

    bgm_spring: u32,
    bgm_summer: u32,
    bgm_autumn: u32,
    bgm_winter: u32,

    script_id: u16,
    town_map_group: u16,

    hex_1c: u16,
    hex_1e: u16,
    hex_20: u16,

    bg_sys_type: u16,
    unused_field: u16,
    unique_sequence_library_id: u16,

    unknown_flags: u32,

    fly_x: u32,
    fly_y: u32,
    fly_z: u32,
}

impl ZoneHeaderData {
    pub fn get_header_from_bytes(input_path: PathBuf) -> Result<Self, std::io::Error> {
        let mut zoneheader = ZoneHeaderData::default();
        let mut f = std::fs::File::open(input_path)?;

        unsafe {
            let buf: &mut [u8] = std::slice::from_raw_parts_mut(
                &mut zoneheader as *mut ZoneHeaderData as *mut u8,
                std::mem::size_of::<ZoneHeaderData>(),
            );

            f.read_exact(buf)?;

            Ok(zoneheader)
        }
    }

    pub fn store_deserialized_into_json(&self, io: InputOutput) -> Result<(), std::io::Error> {
        let json_text = serde_json::to_string_pretty(self)?;
        let mut file_name = String::new();

        if io.output.is_none() {
            file_name = format!(
                "./{}.json",
                io.input.iter().last().unwrap().to_str().unwrap()
            );
        } else {
            file_name = io.output.unwrap();
        }
        fs::write(file_name, json_text)?;
        Ok(())
    }

    pub fn serialize_struct_and_store_into_bin(io: InputOutput) -> Result<(), std::io::Error> {
        let input_path = io.input;

        let json = fs::read_to_string(input_path)?;
        let header = serde_json::from_str::<ZoneHeaderData>(json.as_str())?;

        let bin = serialize(&header).unwrap();

        if io.output.is_none() {
            let mut file = std::fs::File::create("./0")?;
            let slice = unsafe {
                std::slice::from_raw_parts(
                    bin.as_ptr() as *const u8,
                    std::mem::size_of::<ZoneHeaderData>(),
                )
            };
            file.write_all(slice).unwrap();
        } else if io.output.is_some() {
            // Cannot panic because it is_some(); haha funny
            let path = io.output.unwrap();
            let mut file = std::fs::File::create(path)?;
            let slice = unsafe {
                std::slice::from_raw_parts(
                    bin.as_ptr() as *const u8,
                    std::mem::size_of::<ZoneHeaderData>(),
                )
            };
            file.write_all(slice).unwrap();
        }
        Ok(())
    }
}

use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::cli::InputOutput;

#[derive(Serialize, Debug, Default)]
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
        //if !Path::new(&file_name).is_dir() {
        fs::write(file_name, json_text)?;
        //}
        Ok(())
    }
}

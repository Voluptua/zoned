use zoned::*;

fn main() -> std::io::Result<()> {
    let cli = cli::ZonedCli::do_parse();
    let mode = cli.mode;

    if let cli::Mode::Convert(c) = mode {
        let zone_data_filetype = c.file;
        if let cli::ZoneDataFile::ZoneHeader(io) = zone_data_filetype {
            let zone_header = zone_data::ZoneHeaderData::get_header_from_bytes(io.input.clone())?;
            if zone_header.store_deserialized_into_json(io).is_ok() {
                println!("Converted Zone-Header successfully!");
            }
        } else if let cli::ZoneDataFile::ZoneEntities(_io) = zone_data_filetype {
            println!("SORRY: You can't convert ZoneEntities yet...");
        }
    } else if let cli::Mode::Compile = mode {
        println!("SORRY: You can't compile ZoneData yet...");
    }
    Ok(())
}

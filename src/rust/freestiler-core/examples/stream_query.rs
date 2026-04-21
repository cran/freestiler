use freestiler_core::engine::{ProgressReporter, TileConfig};
use freestiler_core::pmtiles_writer::TileFormat;

struct StdoutReporter;

impl ProgressReporter for StdoutReporter {
    fn report(&self, msg: &str) {
        println!("{msg}");
    }
}

fn parse_tile_format(value: &str) -> Result<TileFormat, String> {
    match value {
        "mlt" => Ok(TileFormat::Mlt),
        "mvt" => Ok(TileFormat::Mvt),
        other => Err(format!("Unsupported tile format: {other}")),
    }
}

fn parse_optional_u8(value: &str) -> Result<Option<u8>, String> {
    if value.eq_ignore_ascii_case("none") {
        Ok(None)
    } else {
        value
            .parse::<u8>()
            .map(Some)
            .map_err(|e| format!("Invalid zoom value '{value}': {e}"))
    }
}

fn parse_optional_f64(value: &str) -> Result<Option<f64>, String> {
    if value.eq_ignore_ascii_case("none") {
        Ok(None)
    } else {
        value
            .parse::<f64>()
            .map(Some)
            .map_err(|e| format!("Invalid numeric value '{value}': {e}"))
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 10 {
        return Err(
            "Usage: cargo run --example stream_query --features duckdb -- \
             <db_path> <output_path> <layer_name> <tile_format> \
             <min_zoom> <max_zoom> <base_zoom|none> <drop_rate|none> <sql>"
                .to_string(),
        );
    }

    let config = TileConfig {
        tile_format: parse_tile_format(&args[4])?,
        min_zoom: args[5]
            .parse::<u8>()
            .map_err(|e| format!("Invalid min_zoom '{}': {e}", args[5]))?,
        max_zoom: args[6]
            .parse::<u8>()
            .map_err(|e| format!("Invalid max_zoom '{}': {e}", args[6]))?,
        base_zoom: parse_optional_u8(&args[7])?,
        simplification: true,
        drop_rate: parse_optional_f64(&args[8])?,
        cluster_distance: None,
        cluster_maxzoom: None,
        coalesce: false,
    };

    let reporter = StdoutReporter;
    let row_count = freestiler_core::streaming::generate_pmtiles_from_duckdb_query(
        Some(&args[1]),
        &args[9],
        &args[2],
        &args[3],
        &config,
        &reporter,
    )?;

    println!("streamed_rows={row_count}");
    Ok(())
}

pub fn print_usage() {
    tracing::info!("Usage:");
    tracing::info!(
        "  diplo_sim run --turns N --seed S (--map <map_file> | --map-id <map_id>) --players <n> --out <match.json> [--replay-out <replay.bin>]"
    );
    tracing::info!("  diplo_sim replay --replay <replay.bin> --out <match.json>");
    tracing::info!("  diplo_sim validate-map --map <map_file>");
    tracing::info!("  diplo_sim validate-orders --map <map_file> --orders <orders_file>");
    tracing::info!("  diplo_sim list-maps --out <maps.json>");
    tracing::info!("  diplo_sim map-info --map-id <id> --out <map_info.json>");
}

pub fn get_arg(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

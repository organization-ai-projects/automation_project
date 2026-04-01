mod config;
mod diagnostics;
mod graph;
mod io;
mod layout;
mod render;

#[cfg(test)]
mod tests;

use std::{env, process};

use crate::config::RenderConfig;
use crate::diagnostics::GraphvizorError;
use crate::io::JsonCodec;
use crate::layout::{LayeredDag, LayoutEngine, SimpleForce};
use crate::render::{HtmlRenderer, SvgRenderer};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(2);
    }

    let result = match args[1].as_str() {
        "render" => cmd_render(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(2);
        }
    };

    match result {
        Ok(msg) => {
            println!("{msg}");
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}

fn cmd_render(args: &[String]) -> Result<String, GraphvizorError> {
    let config = RenderConfig::parse(args)?;
    let graph = JsonCodec::load(&config.input_path)?;

    let positions = match config.layout.as_str() {
        "layered" => LayeredDag.compute(&graph),
        "force" => SimpleForce::default().compute(&graph),
        other => return Err(GraphvizorError::UnknownLayout(other.to_string())),
    };

    let output = if config.output_path.extension().and_then(|e| e.to_str()) == Some("html") {
        HtmlRenderer::render(&graph, &positions)
    } else {
        SvgRenderer::render(&graph, &positions)
    };

    std::fs::write(&config.output_path, &output)?;

    Ok(format!(
        "Rendered {} nodes, {} edges -> {}",
        graph.nodes.len(),
        graph.edges.len(),
        config.output_path.display()
    ))
}

fn print_usage() {
    println!("graphvizor_backend - deterministic graph visualizer");
    println!();
    println!("Commands:");
    println!("  graphvizor_backend render --in <graph.json> --out <out.svg> --layout <name>");
    println!();
    println!("Layouts: layered (default), force");
}

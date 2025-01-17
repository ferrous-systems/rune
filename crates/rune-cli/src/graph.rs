//! Generate a DOT graph using a poor man's graph generator.

use std::{fs::File, io::Write, path::PathBuf};
use anyhow::{Context, Error};
use hotg_rune_syntax::hir::{HirId, NameTable, Node, Rune, Slot, Type};
use codespan_reporting::term::termcolor::ColorChoice;
use indexmap::IndexMap;

use crate::inspect::Metadata;

const WASM_MAGIC_BYTES: &[u8; 4] = b"\0asm";

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Graph {
    /// Where to write the generated file (stdout by default).
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// A Rune or Runefile to graph.
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Graph {
    pub fn execute(self, color: ColorChoice) -> Result<(), Error> {
        let rune = self.load_rune(color).context("unable to load the input")?;

        let mut writer = self.writer()?;
        generate_graph(&mut *writer, &rune)?;
        writer.flush()?;

        Ok(())
    }

    fn load_rune(&self, color: ColorChoice) -> Result<Rune, Error> {
        let bytes = std::fs::read(&self.input).with_context(|| {
            format!("Unable to read \"{}\"", self.input.display())
        })?;

        if bytes.starts_with(WASM_MAGIC_BYTES) {
            // It's a compiled Rune
            Metadata::from_wasm_binary(&bytes)
                .take_rune()
                .context("Unable to load the Rune metadata from the input")
        } else {
            // Try to analyse it as a Runefile
            crate::build::analyze(&self.input, color)
        }
    }

    fn writer(&self) -> Result<Box<dyn Write>, Error> {
        match &self.output {
            Some(path) => {
                let f = File::create(path).with_context(|| {
                    format!("Unable to open \"{}\" for writing", path.display())
                })?;

                Ok(Box::new(f))
            },
            None => Ok(Box::new(std::io::stdout())),
        }
    }
}

fn generate_graph(w: &mut dyn Write, rune: &Rune) -> Result<(), Error> {
    writeln!(w, "digraph {{")?;
    writeln!(w, "  rankdir=TD;")?;
    writeln!(w, "  node [shape=plaintext];")?;

    declare_nodes(w, &rune.stages, &rune.names)?;
    declare_edges(w, &rune)?;

    writeln!(w, "}}")?;

    Ok(())
}

fn declare_edges(w: &mut dyn Write, rune: &Rune) -> Result<(), Error> {
    for (&id, node) in &rune.stages {
        declare_input_edges(w, rune, id, &node.input_slots)?;
    }

    Ok(())
}

fn declare_input_edges(
    w: &mut dyn Write,
    rune: &Rune,
    id: HirId,
    input_slots: &[HirId],
) -> Result<(), Error> {
    for (i, slot_id) in input_slots.iter().enumerate() {
        let Slot {
            element_type,
            input_node,
            ..
        } = &rune.slots[slot_id];

        let input = &rune.stages[input_node];
        let index = input
            .output_slots
            .iter()
            .position(|s| s == slot_id)
            .unwrap();

        write!(
            w,
            "  node_{}:output_{}:s -> node_{}:input_{}:n",
            input_node, index, id, i,
        )?;

        if let Some(type_name) = type_name(element_type, &rune.types) {
            write!(w, " [label=\"{}\"]", type_name)?;
        }
        writeln!(w, ";")?;
    }

    Ok(())
}

fn type_name(type_id: &HirId, types: &IndexMap<HirId, Type>) -> Option<String> {
    match types.get(type_id)? {
        Type::Primitive(p) => Some(p.rust_name().to_string()),
        Type::Buffer {
            underlying_type,
            dimensions,
        } => {
            let underlying_type = type_name(underlying_type, types)?;

            // as a special case, let's avoid writing a trailing "[1]" if it can
            // be simplified
            if dimensions == &[1] {
                Some(underlying_type)
            } else {
                Some(format!("{}{:?}", underlying_type, dimensions))
            }
        },
        _ => None,
    }
}

fn declare_nodes(
    w: &mut dyn Write,
    stages: &IndexMap<HirId, Node>,
    names: &NameTable,
) -> Result<(), Error> {
    for (&id, node) in stages {
        let name = names.get_name(id).with_context(|| {
            format!("Unable to get the name for node {}", id)
        })?;

        let colour = match &node.stage {
            hotg_rune_syntax::hir::Stage::Source(_) => "lightgreen",
            hotg_rune_syntax::hir::Stage::Model(_) => "violet",
            hotg_rune_syntax::hir::Stage::ProcBlock(_) => "tan1",
            hotg_rune_syntax::hir::Stage::Sink(_) => "indianred1",
        };

        write!(
            w,
            r#"  node_{} [fillcolor={}, style="filled",label="#,
            id, colour,
        )?;
        format_node_label(w, name, node)?;
        writeln!(w, "];")?;
    }

    Ok(())
}

fn format_node_label(
    w: &mut dyn Write,
    name: &str,
    node: &Node,
) -> Result<(), Error> {
    writeln!(w, "<")?;
    writeln!(
        w,
        r#"    <table border="0" cellborder="0" cellspacing="5">"#
    )?;

    if !node.input_slots.is_empty() {
        write!(
            w,
            r#"      <tr><td><table cellborder="1" cellspacing="0"><tr>"#
        )?;
        for i in 0..node.input_slots.len() {
            write!(w, "<td port=\"input_{}\">{}</td>", i, i)?;
        }
        writeln!(w, "</tr></table></td></tr>")?;
    }

    let qualifier = match &node.stage {
        hotg_rune_syntax::hir::Stage::Source(s) => s.kind.to_string(),
        hotg_rune_syntax::hir::Stage::Sink(s) => s.kind.to_string(),
        hotg_rune_syntax::hir::Stage::Model(m) => {
            m.model_file.display().to_string()
        },
        hotg_rune_syntax::hir::Stage::ProcBlock(p) => p.path.to_string(),
    };

    writeln!(w, "      <tr><td>{}: {}</td></tr>", name, qualifier)?;

    if !node.output_slots.is_empty() {
        write!(
            w,
            r#"      <tr><td><table cellborder="1" cellspacing="0"><tr>"#
        )?;
        for i in 0..node.output_slots.len() {
            write!(w, "<td port=\"output_{}\">{}</td>", i, i)?;
        }
        writeln!(w, "</tr></table></td></tr>")?;
    }

    writeln!(w, "    </table>")?;
    write!(w, "  >")?;

    Ok(())
}

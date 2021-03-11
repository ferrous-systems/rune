//! The *High-level Internal Representation*.

use std::{collections::HashMap, path::PathBuf};
use codespan::Span;
use crate::ast::{ArgumentValue, Path};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rune {
    pub base_image: Option<Path>,
    pub sinks: HashMap<HirId, Sink>,
    pub sources: HashMap<HirId, Source>,
    pub models: HashMap<HirId, Model>,
    pub types: HashMap<HirId, Type>,
    pub pipelines: HashMap<HirId, Pipeline>,
    pub proc_blocks: HashMap<HirId, ProcBlock>,
    pub names: NameTable,
    pub spans: HashMap<HirId, Span>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HirId(u32);

impl HirId {
    pub const ERROR: HirId = HirId(0);

    pub fn is_error(self) -> bool { self == HirId::ERROR }

    pub(crate) fn next(self) -> Self { HirId(self.0 + 1) }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sink {
    Serial,
}

/// A table mapping names to [`HirId`]s.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NameTable {
    name_to_id: HashMap<String, HirId>,
    id_to_name: HashMap<HirId, String>,
}

impl NameTable {
    pub fn register(&mut self, name: &str, id: HirId) {
        if self.name_to_id.contains_key(name)
            || self.id_to_name.contains_key(&id)
        {
            unimplemented!("How do we want to signal duplicate names?");
        }

        self.name_to_id.insert(name.to_string(), id);
        self.id_to_name.insert(id, name.to_string());
    }

    pub fn get_name(&self, id: HirId) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }

    pub fn get_id(&self, name: &str) -> Option<HirId> {
        self.name_to_id.get(name).copied()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub input: HirId,
    pub output: HirId,
    pub model_file: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Primitive(Primitive),
    /// The concrete type isn't yet known.
    Unknown,
    /// A multidimensional array of data.
    Buffer {
        underlying_type: HirId,
        dimensions: Vec<usize>,
    },
    /// This can be *any* type.
    Any,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Primitive {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    String,
}

impl Primitive {
    pub fn rust_name(self) -> &'static str {
        match self {
            Primitive::U8 => "u8",
            Primitive::I8 => "i8",
            Primitive::U16 => "u16",
            Primitive::I16 => "i16",
            Primitive::U32 => "u32",
            Primitive::I32 => "i32",
            Primitive::U64 => "u64",
            Primitive::I64 => "i64",
            Primitive::F32 => "f32",
            Primitive::F64 => "f64",
            Primitive::String => "&'static str",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    pub kind: SourceKind,
    pub output_type: HirId,
    pub parameters: HashMap<String, ArgumentValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceKind {
    Random,
    Accelerometer,
    Sound,
    Image,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pipeline {
    /// A linked list representing a pipeline.
    ///
    /// Note: We use a linked list to make sure it is impossible to create an
    /// illogical pipeline (e.g. with a sink in the middle) and so you can
    /// later include some sort of "merge" node for joining two
    /// sub-pipelines.
    pub last_step: PipelineNode,
}

impl Pipeline {
    /// Iterate over each step in the pipeline.
    pub fn iter(&self) -> impl Iterator<Item = &PipelineNode> + '_ {
        let mut current_node = Some(&self.last_step);
        let mut nodes = Vec::new();

        while let Some(node) = current_node.take() {
            nodes.push(node);
            current_node = node.previous();
        }

        nodes.into_iter().rev()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PipelineNode {
    Source {
        source: HirId,
        output_type: HirId,
    },
    Model {
        model: HirId,
        previous: Box<PipelineNode>,
        output_type: HirId,
    },
    ProcBlock {
        proc_block: HirId,
        previous: Box<PipelineNode>,
        output_type: HirId,
    },
    Sink {
        sink: HirId,
        previous: Box<PipelineNode>,
    },
}

impl PipelineNode {
    pub fn previous(&self) -> Option<&PipelineNode> {
        match self {
            PipelineNode::Source { .. } => None,
            PipelineNode::Model { previous, .. } => Some(&**previous),
            PipelineNode::ProcBlock { previous, .. } => Some(&**previous),
            PipelineNode::Sink { previous, .. } => Some(&**previous),
        }
    }

    pub fn id(&self) -> HirId {
        match self {
            PipelineNode::Source { source: id, .. }
            | PipelineNode::Model { model: id, .. }
            | PipelineNode::ProcBlock { proc_block: id, .. }
            | PipelineNode::Sink { sink: id, .. } => *id,
        }
    }

    pub fn output_type(&self) -> Option<HirId> {
        match self {
            PipelineNode::Source { output_type, .. }
            | PipelineNode::Model { output_type, .. }
            | PipelineNode::ProcBlock { output_type, .. } => Some(*output_type),
            PipelineNode::Sink { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcBlock {
    pub input: HirId,
    pub output: HirId,
    pub path: Path,
    pub parameters: HashMap<String, ArgumentValue>,
}

impl ProcBlock {
    pub fn name(&self) -> &str {
        let full_name = self.path.sub_path.as_ref().unwrap_or(&self.path.base);

        let start_of_name = full_name.rfind("/").map(|ix| ix + 1).unwrap_or(0);

        &full_name[start_of_name..]
    }
}

use crate::{
    Diagnostics,
    ast::{
        Argument, ArgumentValue, CapabilityInstruction, Ident, Instruction,
        ModelInstruction, OutInstruction, ProcBlockInstruction, RunInstruction,
        Runefile,
    },
    hir::{
        HirId, Model, Pipeline, PipelineNode, Primitive, ProcBlock, Rune, Sink,
        Source, SourceKind, Type,
    },
};
use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::{collections::HashMap, path::PathBuf};

pub fn analyse<FileId: Copy>(
    file_id: FileId,
    runefile: &Runefile,
    diags: &mut Diagnostics<FileId>,
) -> Rune {
    let mut analyser = Analyser::new(file_id, diags);

    analyser.load_runefile(runefile);
    analyser.infer_types();

    analyser.rune
}

#[derive(Debug)]
struct Analyser<'diag, FileId> {
    diags: &'diag mut Diagnostics<FileId>,
    file_id: FileId,
    rune: Rune,
    ids: HirIds,
    builtins: Builtins,
}

impl<'diag, FileId: Copy> Analyser<'diag, FileId> {
    fn new(file_id: FileId, diags: &'diag mut Diagnostics<FileId>) -> Self {
        let mut rune = Rune::default();

        let mut ids = HirIds::new();
        let builtins = Builtins::new(&mut ids);
        builtins.copy_into(&mut rune);

        Analyser {
            diags,
            file_id,
            rune,
            ids,
            builtins,
        }
    }

    /// Report an error to the user.
    fn error(&mut self, msg: impl Into<String>, span: Span) {
        let diag = Diagnostic::error()
            .with_message(msg)
            .with_labels(vec![Label::primary(self.file_id, span)]);
        self.diags.push(diag);
    }

    /// Report a warning to the user.
    fn warn(&mut self, msg: impl Into<String>, span: Span) {
        let diag = Diagnostic::warning()
            .with_message(msg)
            .with_labels(vec![Label::primary(self.file_id, span)]);
        self.diags.push(diag);
    }

    fn load_runefile(&mut self, runefile: &Runefile) {
        let mut instructions = runefile.instructions.iter();

        match instructions.next() {
            Some(first_instruction) => {
                if self.load_from(first_instruction).is_err() {
                    // The first instruction was dodgy but we want to process it
                    // anyway.
                    self.load_instruction(first_instruction);
                }
            },
            None => {
                self.error(
                    "A Runefile must contain at least a FROM instruction",
                    runefile.span,
                );
            },
        }

        for instruction in instructions {
            self.load_instruction(instruction);
        }
    }

    fn load_from(&mut self, instruction: &Instruction) -> Result<(), ()> {
        match instruction {
            Instruction::From(f) => {
                self.rune.base_image = Some(f.image.clone());
                Ok(())
            },
            other => {
                self.error(
                    "Runefiles should start with a FROM instruction",
                    other.span(),
                );

                Err(())
            },
        }
    }

    fn load_instruction(&mut self, instruction: &Instruction) -> HirId {
        match instruction {
            Instruction::From(f) => {
                self.error(
                    "A FROM instruction can only be at the top of a Runefile",
                    f.span,
                );
                HirId::ERROR
            },
            Instruction::Model(m) => self.load_model(m),
            Instruction::Capability(c) => self.load_capability(c),
            Instruction::Run(r) => self.load_run(r),
            Instruction::ProcBlock(p) => self.load_proc_block(p),
            Instruction::Out(out) => self.load_out(out),
        }
    }

    fn load_model(&mut self, model: &ModelInstruction) -> HirId {
        let hir = Model {
            input: self.interpret_type(&model.input_type),
            output: self.interpret_type(&model.output_type),
            model_file: PathBuf::from(&model.file),
        };
        let id = self.ids.next();
        self.rune.spans.insert(id, model.span);
        self.rune.models.insert(id, hir);
        self.rune.names.register(&model.name.value, id);
        id
    }

    fn load_capability(&mut self, capability: &CapabilityInstruction) -> HirId {
        let kind = match capability.kind.value.as_str() {
            "RAND" => SourceKind::Random,
            "ACCEL" => SourceKind::Accelerometer,
            "SOUND" => SourceKind::Sound,
            "IMAGE" => SourceKind::Image,
            _ => {
                self.error(
                    "This isn't one of the builtin capabilities",
                    capability.kind.span,
                );
                return HirId::ERROR;
            },
        };

        let id = self.ids.next();
        self.rune.spans.insert(id, capability.span);
        let output_type = self.interpret_type(&capability.output_type);
        self.rune.sources.insert(
            id,
            Source {
                kind,
                output_type,
                parameters: args_to_parameters(&capability.parameters),
            },
        );
        self.rune.names.register(&capability.name.value, id);

        id
    }

    fn interpret_type(&mut self, ty: &crate::ast::Type) -> HirId {
        match &ty.kind {
            crate::ast::TypeKind::Inferred => self.builtins.unknown_type,
            crate::ast::TypeKind::Named(name) => {
                let underlying_type = self.primitive_type(name);

                if underlying_type == self.builtins.string {
                    return underlying_type;
                }

                // All non-string types are passed around as an array, so a
                // plain `T` gets turned into a `[T; 1]`.
                self.intern_type(Type::Buffer {
                    underlying_type,
                    dimensions: vec![1],
                })
            },
            crate::ast::TypeKind::Buffer {
                type_name,
                dimensions,
            } => {
                let underlying_type = self.primitive_type(type_name);
                self.intern_type(Type::Buffer {
                    underlying_type,
                    dimensions: dimensions.clone(),
                })
            },
        }
    }

    /// Add a type to the rune, returning its [`HirId`].
    ///
    /// Adding the same type multiple times is guaranteed to return the same
    /// [`HirId`].
    fn intern_type(&mut self, ty: Type) -> HirId {
        match self.rune.types.iter().find(|(_, t)| **t == ty) {
            Some((id, _)) => *id,
            None => {
                // new buffer type
                let id = self.ids.next();
                self.rune.types.insert(id, ty);
                id
            },
        }
    }

    fn primitive_type(&mut self, ident: &Ident) -> HirId {
        match ident.value.as_str() {
            "u8" | "U8" => self.builtins.u8,
            "i8" | "I8" => self.builtins.i8,
            "u16" | "U16" => self.builtins.u16,
            "i16" | "I16" => self.builtins.i16,
            "u32" | "U32" => self.builtins.u32,
            "i32" | "I32" => self.builtins.i32,
            "u64" | "U64" => self.builtins.u64,
            "i64" | "I64" => self.builtins.i64,
            "f32" | "F32" => self.builtins.f32,
            "f64" | "F64" => self.builtins.f64,
            "utf8" | "UTF8" => self.builtins.string,
            _ => {
                self.warn("Unknown type", ident.span);
                self.builtins.unknown_type
            },
        }
    }

    fn get_named(&mut self, name: &Ident) -> HirId {
        match self.rune.names.get_id(&name.value) {
            Some(id) => id,
            None => {
                self.error("Unknown name", name.span);
                HirId::ERROR
            },
        }
    }

    fn load_run(&mut self, run: &RunInstruction) -> HirId {
        let (first, rest) = match run.steps.as_slice() {
            [f, r @ ..] => (f, r),
            [] => {
                self.error("A RUN instruction can't be empty", run.span);
                return HirId::ERROR;
            },
        };

        let source = self.get_named(first);

        let mut pipeline_node = match self.rune.sources.get(&source) {
            Some(s) => PipelineNode::Source {
                source,
                output_type: s.output_type,
            },
            None => {
                self.error(
                    "RUN instructions must start with a CAPABILITY",
                    first.span,
                );
                return HirId::ERROR;
            },
        };

        for step in rest {
            let id = self.get_named(step);
            if id.is_error() {
                // it's a dodgy name, we may as well bail.
                return HirId::ERROR;
            }

            if let Some(model) = self.rune.models.get(&id) {
                pipeline_node = PipelineNode::Model {
                    model: id,
                    previous: Box::new(pipeline_node),
                    output_type: model.output,
                };
            } else if let Some(proc_block) = self.rune.proc_blocks.get(&id) {
                pipeline_node = PipelineNode::ProcBlock {
                    proc_block: id,
                    previous: Box::new(pipeline_node),
                    output_type: proc_block.output,
                };
            } else if self.rune.sinks.contains_key(&id) {
                pipeline_node = PipelineNode::Sink {
                    sink: id,
                    previous: Box::new(pipeline_node),
                };
            } else {
                self.error("Unknown pipeline node type", step.span);
                return HirId::ERROR;
            }
        }

        let pipeline = Pipeline {
            last_step: pipeline_node,
        };
        let id = self.ids.next();
        self.rune.spans.insert(id, run.span);
        self.rune.pipelines.insert(id, pipeline);

        id
    }

    fn load_proc_block(&mut self, proc_block: &ProcBlockInstruction) -> HirId {
        let id = self.ids.next();
        self.rune.names.register(&proc_block.name.value, id);
        self.rune.spans.insert(id, proc_block.span);

        let pb = ProcBlock {
            input: self.interpret_type(&proc_block.input_type),
            output: self.interpret_type(&proc_block.output_type),
            path: proc_block.path.clone(),
            parameters: args_to_parameters(&proc_block.params),
        };
        self.rune.proc_blocks.insert(id, pb);

        id
    }

    fn load_out(&mut self, out: &OutInstruction) -> HirId {
        match out.out_type.value.as_str() {
            "SERIAL" | "serial" => {
                let id = self.ids.next();
                self.rune.spans.insert(id, out.span);
                self.rune.sinks.insert(id, Sink::Serial);
                self.rune.names.register("serial", id);

                id
            },
            _ => {
                self.error("Unknown OUT type", out.out_type.span);

                HirId::ERROR
            },
        }
    }

    fn infer_types(&mut self) {
        // TODO: Go through each pipeline and try to figure out what the
        // input/output type at each stage should be.
        //
        // This will be a bit like a fixed-point iteration, where you keep
        // running inference in a loop until you've either inferred all the
        // types or are unable to make any more progress.
        //
        // For now, let's just emit a warning.

        let unknown = self.builtins.unknown_type;

        let msg = "Unable to infer the input or output type.";

        warn_on_unknown_type(
            &mut self.diags,
            &self.rune.spans,
            self.file_id,
            &self.rune.models,
            msg,
            |m| m.input == unknown || m.output == unknown,
        );
        warn_on_unknown_type(
            &mut self.diags,
            &self.rune.spans,
            self.file_id,
            &self.rune.proc_blocks,
            msg,
            |p| p.input == unknown || p.output == unknown,
        );
        warn_on_unknown_type(
            &mut self.diags,
            &self.rune.spans,
            self.file_id,
            &self.rune.sources,
            msg,
            |s| s.output_type == unknown,
        );
    }
}

fn args_to_parameters(
    parameters: &[Argument],
) -> HashMap<String, ArgumentValue> {
    parameters
        .iter()
        .map(|arg| {
            let key = arg.name.value.replace("-", "_");
            (key, arg.value.clone())
        })
        .collect()
}

fn warn_on_unknown_type<'a, I, T, F, FileId>(
    diags: &mut Diagnostics<FileId>,
    spans: &HashMap<HirId, Span>,
    file_id: FileId,
    items: I,
    msg: &str,
    mut filter: F,
) where
    I: IntoIterator<Item = (&'a HirId, &'a T)> + 'a,
    T: 'a,
    F: FnMut(&T) -> bool,
    FileId: Copy,
{
    for (id, value) in items {
        if filter(value) {
            let mut diag =
                Diagnostic::warning().with_message(msg).with_notes(vec![
                    String::from(
                        "See <https://github.com/hotg-ai/rune/issues/33>",
                    ),
                ]);

            if let Some(span) = spans.get(id) {
                diag = diag.with_labels(vec![Label::primary(file_id, *span)]);
            }

            diags.push(diag);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct HirIds {
    last_id: HirId,
}

impl HirIds {
    fn new() -> Self {
        HirIds {
            last_id: HirId::ERROR,
        }
    }

    fn next(&mut self) -> HirId {
        let id = self.last_id.next();
        self.last_id = id;
        id
    }
}

#[derive(Copy, Clone, Debug)]
struct Builtins {
    unknown_type: HirId,
    u8: HirId,
    i8: HirId,
    u16: HirId,
    i16: HirId,
    u32: HirId,
    i32: HirId,
    u64: HirId,
    i64: HirId,
    f32: HirId,
    f64: HirId,
    string: HirId,
}

impl Builtins {
    fn new(ids: &mut HirIds) -> Self {
        Builtins {
            unknown_type: ids.next(),
            u8: ids.next(),
            i8: ids.next(),
            u16: ids.next(),
            i16: ids.next(),
            u32: ids.next(),
            i32: ids.next(),
            u64: ids.next(),
            i64: ids.next(),
            f32: ids.next(),
            f64: ids.next(),
            string: ids.next(),
        }
    }

    fn copy_into(&self, rune: &mut Rune) {
        let Builtins {
            unknown_type,
            u8,
            i8,
            u16,
            i16,
            u32,
            i32,
            u64,
            i64,
            f32,
            f64,
            string,
        } = *self;

        rune.types.insert(unknown_type, Type::Unknown);
        rune.types.insert(u8, Type::Primitive(Primitive::U8));
        rune.types.insert(i8, Type::Primitive(Primitive::I8));
        rune.types.insert(u16, Type::Primitive(Primitive::U16));
        rune.types.insert(i16, Type::Primitive(Primitive::I16));
        rune.types.insert(u32, Type::Primitive(Primitive::U32));
        rune.types.insert(i16, Type::Primitive(Primitive::I16));
        rune.types.insert(i32, Type::Primitive(Primitive::I32));
        rune.types.insert(u64, Type::Primitive(Primitive::U64));
        rune.types.insert(i64, Type::Primitive(Primitive::I64));
        rune.types.insert(f32, Type::Primitive(Primitive::F32));
        rune.types.insert(f64, Type::Primitive(Primitive::F64));
        rune.types
            .insert(string, Type::Primitive(Primitive::String));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::ast::{Argument, Ident, Literal, Path};
    use codespan::Span;

    fn setup_analyser(diags: &mut Diagnostics<()>) -> Analyser<'_, ()> {
        Analyser::new((), diags)
    }

    fn setup(src: &str) -> ((), Runefile) {
        let runefile = match crate::parse(src) {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };

        ((), runefile)
    }

    #[test]
    fn empty_runefile_is_error() {
        let (id, runefile) = setup("");
        let mut diags = Diagnostics::new();

        let got = analyse(id, &runefile, &mut diags);

        assert!(diags.has_errors());
        assert!(got.base_image.is_none());
    }

    #[test]
    fn runefiles_must_start_with_a_from_line() {
        let (id, runefile) = setup("OUT serial");
        let mut diags = Diagnostics::new();

        let got = analyse(id, &runefile, &mut diags);

        assert!(diags.has_errors());
        assert!(got.base_image.is_none());
    }

    #[test]
    fn valid_base_image() {
        let (id, runefile) = setup("FROM runicos/base@1.0");
        let mut diags = Diagnostics::new();

        let got = analyse(id, &runefile, &mut diags);

        assert!(!diags.has_errors());
        assert_eq!(
            got.base_image,
            Some(Path::new(
                "runicos/base",
                None,
                "1.0".to_string(),
                Span::new(5, 21)
            ))
        );
    }

    #[test]
    fn unknown_sink_type() {
        let (id, runefile) = setup("FROM runicos/base\nOUT asdf");
        let mut diags = Diagnostics::new();

        let got = analyse(id, &runefile, &mut diags);

        assert!(diags.has_errors());
        assert!(got.sinks.is_empty());
    }

    #[test]
    fn output_serial() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        let out = OutInstruction {
            out_type: Ident::dangling("serial"),
            span: Span::new(0, 0),
        };

        let id = analyser.load_out(&out);

        assert!(!analyser.diags.has_errors());
        assert_eq!(analyser.rune.sinks.len(), 1);
        assert_eq!(analyser.rune.sinks.get(&id), Some(&Sink::Serial));
        assert_eq!(analyser.rune.names.get_name(id), Some("serial"));
    }

    #[test]
    fn add_model_to_rune() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        let model = ModelInstruction {
            name: Ident::dangling("sine"),
            file: String::from("./sine.tflite"),
            input_type: crate::ast::Type::inferred_dangling(),
            output_type: crate::ast::Type::inferred_dangling(),
            parameters: Vec::new(),
            span: Span::new(0, 0),
        };

        let id = analyser.load_model(&model);

        assert!(!analyser.diags.has_errors());
        assert!(!id.is_error());
        assert_eq!(analyser.rune.names.get_name(id), Some("sine"));
        assert!(analyser.rune.models.contains_key(&id));
    }

    #[test]
    fn add_rand_capability_to_rune() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        // CAPABILITY<_,I32> RAND rand --n 1
        let capability = CapabilityInstruction {
            kind: Ident::dangling("RAND"),
            name: Ident::new("rand", Span::new(0, 0)),
            parameters: vec![Argument::literal(
                Ident::new("n", Span::new(0, 0)),
                Literal::new(1, Span::new(0, 0)),
                Span::new(0, 0),
            )]
            .into_iter()
            .collect(),
            output_type: crate::ast::Type::named_dangling("I32"),
            span: Span::new(0, 0),
        };

        let id = analyser.load_capability(&capability);

        assert!(!analyser.diags.has_errors());
        assert!(!id.is_error());
        assert_eq!(analyser.rune.names.get_name(id), Some("rand"));
        let i32_by_1_type = analyser.intern_type(Type::Buffer {
            underlying_type: analyser.builtins.i32,
            dimensions: vec![1],
        });
        let should_be = Source {
            kind: SourceKind::Random,
            output_type: i32_by_1_type,
            parameters: args_to_parameters(&capability.parameters),
        };
        let source = &analyser.rune.sources[&id];
        assert_eq!(source, &should_be);
    }

    #[test]
    fn kebab_case_arguments_are_converted_to_snake_case() {
        let args: Vec<Argument> = vec![
            "--oneword 1".parse().unwrap(),
            "--kebab-case 1".parse().unwrap(),
            "--snake_case 1".parse().unwrap(),
        ];
        let should_be: HashSet<_> = vec!["oneword", "kebab_case", "snake_case"]
            .into_iter()
            .collect();

        let got = args_to_parameters(&args);

        let argument_names: HashSet<_> =
            got.keys().map(|s| s.as_str()).collect();
        assert_eq!(argument_names, should_be);
    }

    #[test]
    fn known_capabilities() {
        let inputs = vec![
            ("RAND", SourceKind::Random),
            ("ACCEL", SourceKind::Accelerometer),
            ("SOUND", SourceKind::Sound),
            ("IMAGE", SourceKind::Image),
        ];

        for (src, should_be) in inputs {
            let capability = CapabilityInstruction {
                kind: Ident::dangling(src),
                name: Ident::new("foo", Span::new(0, 0)),
                parameters: Default::default(),
                output_type: crate::ast::Type::named_dangling("I32"),
                span: Span::new(0, 0),
            };
            let mut diags = Diagnostics::new();
            let mut analyser = setup_analyser(&mut diags);

            let id = analyser.load_capability(&capability);

            assert!(analyser.diags.is_empty(), "{:?}", analyser.diags);
            let got = &analyser.rune.sources[&id];
            assert_eq!(got.kind, should_be);
        }
    }

    #[test]
    fn load_primitive_type() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        let ident = Ident::dangling("u32");

        let id = analyser.primitive_type(&ident);

        assert!(!id.is_error());
        assert_eq!(id, analyser.builtins.u32);
        assert!(analyser.rune.types.get(&id).is_some());
    }

    #[test]
    fn load_buffer_type() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        let ty = crate::ast::Type {
            kind: crate::ast::TypeKind::Buffer {
                type_name: Ident::dangling("U32"),
                dimensions: vec![1, 2],
            },
            span: Span::new(0, 0),
        };

        let id = analyser.interpret_type(&ty);

        assert!(!id.is_error());
        let got_type = &analyser.rune.types[&id];
        assert_eq!(
            got_type,
            &Type::Buffer {
                underlying_type: analyser.builtins.u32,
                dimensions: vec![1, 2],
            }
        );
        assert!(analyser.rune.types.get(&id).is_some());
    }

    #[test]
    fn all_types_are_memoised() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        let ty = crate::ast::Type {
            kind: crate::ast::TypeKind::Buffer {
                type_name: Ident::dangling("U32"),
                dimensions: vec![1, 2],
            },
            span: Span::new(0, 0),
        };

        let first = analyser.interpret_type(&ty);
        let second = analyser.interpret_type(&ty);
        let third = analyser.interpret_type(&ty);

        assert_eq!(first, second);
        assert_eq!(second, third);
    }

    #[test]
    fn different_buffer_sizes_have_different_types() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        let type_1 = crate::ast::Type {
            kind: crate::ast::TypeKind::Buffer {
                type_name: Ident::dangling("U32"),
                dimensions: vec![1, 2],
            },
            span: Span::new(0, 0),
        };
        let type_2 = crate::ast::Type {
            kind: crate::ast::TypeKind::Buffer {
                type_name: Ident::dangling("U32"),
                dimensions: vec![1],
            },
            span: Span::new(0, 0),
        };

        let first = analyser.interpret_type(&type_1);
        let second = analyser.interpret_type(&type_2);

        assert_ne!(first, second);
    }

    #[test]
    fn strings_dont_get_wrapped_in_a_buffer() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        let ty = crate::ast::Type {
            kind: crate::ast::TypeKind::Named(Ident::dangling("UTF8")),
            span: Span::new(0, 0),
        };

        let got = analyser.interpret_type(&ty);

        assert_eq!(got, analyser.builtins.string);
        assert_eq!(
            analyser.rune.types[&got],
            Type::Primitive(Primitive::String),
        );
    }
}

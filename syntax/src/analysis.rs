use std::path::PathBuf;

use crate::{
    ast::{
        CapabilityInstruction, Instruction, ModelInstruction, OutInstruction,
        ProcBlockInstruction, RunInstruction, Runefile,
    },
    hir::{HirId, Model, Rune, Sink, Type},
    Diagnostics,
};
use codespan::FileId;
use codespan_reporting::diagnostic::{Diagnostic, Label};

type Diag = Diagnostic<FileId>;

pub fn analyse(
    file_id: FileId,
    runefile: &Runefile,
    diags: &mut Diagnostics,
) -> Rune {
    let mut analyser = Analyser::new(file_id, diags);

    analyser.load_runefile(runefile);

    analyser.rune
}

#[derive(Debug)]
struct Analyser<'diag> {
    diags: &'diag mut Diagnostics,
    file_id: FileId,
    rune: Rune,
    last_hir_id: HirId,
}

impl<'diag> Analyser<'diag> {
    fn new(file_id: FileId, diags: &'diag mut Diagnostics) -> Self {
        Analyser {
            diags,
            file_id,
            rune: Rune::default(),
            last_hir_id: HirId::ERROR,
        }
    }

    fn next_id(&mut self) -> HirId {
        let id = self.last_hir_id.next();
        self.last_hir_id = id;
        id
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
                let diag = Diagnostic::error()
                    .with_message(
                        "A Runefile must contain at least a FROM instruction",
                    )
                    .with_labels(vec![Label::primary(
                        self.file_id,
                        runefile.span,
                    )]);
                self.diags.push(diag);
            },
        }

        for instruction in instructions {
            self.load_instruction(instruction);
        }
    }

    fn load_from(&mut self, instruction: &Instruction) -> Result<(), ()> {
        match instruction {
            Instruction::From(f) => {
                self.rune.base_image = Some(f.image.value.clone());
                Ok(())
            },
            other => {
                let diag = Diag::error()
                    .with_message(
                        "Runefiles should start with a FROM instruction",
                    )
                    .with_labels(vec![Label::primary(
                        self.file_id,
                        other.span(),
                    )]);
                self.diags.push(diag);

                Err(())
            },
        }
    }

    fn load_instruction(&mut self, instruction: &Instruction) -> HirId {
        match instruction {
            Instruction::From(f) => {
                let diag = Diag::error()
                    .with_message(
                        "A FROM instruction can only be at the top of a Runefile",
                    )
                    .with_labels(vec![Label::primary(
                        self.file_id,
                        f.span,
                    )]);
                self.diags.push(diag);
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
            input: Type::Unknown,
            output: Type::Unknown,
            model_file: PathBuf::from(&model.file),
        };
        let id = self.next_id();
        self.rune.models.insert(id, hir);
        self.rune.names.register(&model.name.value, id);
        id
    }

    fn load_capability(
        &mut self,
        _capability: &CapabilityInstruction,
    ) -> HirId {
        todo!()
    }

    fn load_run(&mut self, _run: &RunInstruction) -> HirId { todo!() }

    fn load_proc_block(&mut self, _proc_block: &ProcBlockInstruction) -> HirId {
        todo!()
    }

    fn load_out(&mut self, out: &OutInstruction) -> HirId {
        match out.out_type.value.as_str() {
            "serial" => {
                let id = self.next_id();
                self.rune.sinks.insert(id, Sink::Serial);
                self.rune.names.register("serial", id);

                id
            },
            _ => {
                let diag = Diagnostic::error()
                    .with_message("Unknown sink type")
                    .with_labels(vec![Label::primary(
                        self.file_id,
                        out.out_type.span,
                    )]);
                self.diags.push(diag);

                HirId::ERROR
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use codespan::{Files, Span};

    use crate::ast::Ident;

    use super::*;

    fn setup_analyser(diags: &mut Diagnostics) -> Analyser<'_> {
        let mut files = Files::new();
        let id = files.add("", "");
        Analyser::new(id, diags)
    }

    fn setup(src: &str) -> (FileId, Runefile) {
        let mut files = Files::new();
        let id = files.add("", src.to_string());

        let runefile = match crate::parse(src) {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };

        (id, runefile)
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
        let (id, runefile) = setup("FROM runicos/base");
        let mut diags = Diagnostics::new();

        let got = analyse(id, &runefile, &mut diags);

        assert!(!diags.has_errors());
        assert_eq!(got.base_image, Some(String::from("runicos/base")));
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
            parameters: HashMap::new(),
            span: Span::new(0, 0),
        };

        let id = analyser.load_model(&model);

        assert!(!analyser.diags.has_errors());
        assert!(!id.is_error());
        assert_eq!(analyser.rune.names.get_name(id), Some("sine"));
        assert!(analyser.rune.models.contains_key(&id));
    }
}

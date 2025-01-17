use assert_cmd::Command;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

fn project_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();

    for ancestor in manifest_dir.ancestors() {
        if ancestor.join(".git").is_dir() {
            return ancestor.to_path_buf();
        }
    }

    unreachable!(
        "Unable to determine the project's root directory. Where is \".git/\"?"
    );
}

fn example_dir() -> PathBuf { project_root().join("examples") }

fn cache_dir() -> PathBuf {
    project_root().join("target").join(concat!(
        env!("CARGO_PKG_NAME"),
        "-",
        module_path!()
    ))
}

#[cfg(target_os = "linux")] // See https://github.com/hotg-ai/rune/issues/131
#[test]
fn person_detection() {
    let person_detection_dir = example_dir().join("person_detection");
    let runefile = person_detection_dir.join("Runefile");
    let build_dir = cache_dir().join("person_detection");
    let rune = build_dir.join("person_detection.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build")
        .arg(&runefile)
        .arg("--output")
        .arg(&rune)
        .unwrap();

    let image = person_detection_dir.join("image_grayscale.png");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run").arg(&rune).arg("--image").arg(image);

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("\"person_prob\""));
}

#[test]
fn build_all_examples() {
    let runefiles = WalkDir::new(example_dir())
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_name() == "Runefile"
                || entry.file_name() == "Runefile.yml"
        });

    let cache_dir = cache_dir().join("all-examples");

    for runefile in runefiles {
        let path = runefile.path();
        let name = path.parent().unwrap().file_name().unwrap();
        let cache_dir = cache_dir.join(name);

        let mut cmd = Command::cargo_bin("rune").unwrap();
        cmd.arg("build")
            .arg(path)
            .arg("--cache-dir")
            .arg(&cache_dir)
            .assert()
            .success();
    }
}

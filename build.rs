fn main() {
    // Auto-discover all .fbs files under actions/.
    // The root action.fbs must come last since it includes the others.
    let mut files: Vec<_> = glob::glob("actions/**/*.fbs")
        .expect("failed to glob actions/**/*.fbs")
        .filter_map(|e| e.ok())
        .collect();

    // Sort so dependencies (sub-schemas) come before the root action.fbs
    files.sort_by(|a, b| {
        let a_is_root = a.file_name().map_or(false, |n| n == "action.fbs");
        let b_is_root = b.file_name().map_or(false, |n| n == "action.fbs");
        a_is_root.cmp(&b_is_root)
    });

    flatbuffers_build::BuilderOptions::new_with_files(&files)
        .compile()
        .expect("failed to compile FlatBuffer schemas");
}

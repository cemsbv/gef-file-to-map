use divan::Bencher;

/// Run a benchmark for the test GEF file in the repo.
#[divan::bench(args = ["test.gef", "big-file.gef"], sample_count = 1000, threads)]
fn gef_file(bencher: Bencher, test: &str) {
    let file = match test {
        "test.gef" => include_str!("../tests/test.gef"),
        "big-file.gef" => include_str!("../tests/big-file.gef"),
        _ => panic!("Unrecognized input '{test}'"),
    };

    bencher.bench(|| {
        let result = gef_file_to_map::parse(divan::black_box(file));
        divan::black_box_drop(result);
    });
}

/// Run registered benchmarks.
fn main() {
    divan::main();
}

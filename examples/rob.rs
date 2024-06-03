extern crate robuild as rob;
use rob::*;
use std::process::Output;

// const THREADS: &str = "-Z threads=10";
const THREADS: &str = "";
const LIB_FLAGS: &str = "--crate-type=rlib";
const DEBUG_FLAGS: &str = "-g -C \"opt-level=0\"";

const BUILD: &str = "build";
const EXAMPLES: &str = "examples";

fn main() -> IoResult::<()> {
    go_rebuild_yourself!(?);

    Rob::mkdir(path!("examples", "build")).unwrap();

    let mut rob = Rob::new();
    build_rakivo_mm(&mut rob)?;
    test_rakivo_mm(&mut rob)?;

    Ok(())
}

fn build_file(rob: &mut Rob, out: &str, name: &str, flags: &str) {
    let build_dir = path!(EXAMPLES, BUILD);
    let link_with_mm_flags: &str = &format!("--extern mm={path}", path = format!("{build_dir}/libmm.rlib"));
    rob.append(&["rustc", DEBUG_FLAGS, flags, THREADS, link_with_mm_flags, "-o",
                 &format!("{build_dir}/{out}"),
                 &path!(EXAMPLES, "mm", &format!("{name}.rs"))]);
}

// Link to Rakivo's mm: https://github.com/rakivo/mm
fn build_rakivo_mm(rob: &mut Rob) -> IoResult::<Vec::<Output>> {
    build_file(rob, "libmm.rlib", "mm", LIB_FLAGS);
    build_file(rob, "load_from_binary", &path!("examples", "load_from_binary"), "");
    build_file(rob, "translate_masm", &path!("examples", "translate_masm"), "");
    rob.execute_all_sync()
}

fn test_rakivo_mm(rob: &mut Rob) -> IoResult::<()> {
    use std::fs::read_to_string;

    let build_dir = path!(EXAMPLES, BUILD);
    let output_path = format!("{build_dir}/fibm.out");
    let expected_path = path!(EXAMPLES, "mm", "load_from_binary.expected");

    rob.append(&[&format!("{build_dir}/translate_masm"),
                 &path!(EXAMPLES, "mm", "fib.masm"),
                 &format!("{build_dir}/fibm")])
       .append(&[&format!("{build_dir}/load_from_binary"),
                  &format!("{p1} > {p2}", p1 = &format!("{build_dir}/fibm"), p2 = &output_path)])
       .execute_all_sync()?;

    let output_string = read_to_string(&output_path)?;
    let expected_string = read_to_string(&expected_path)?;
    if output_string.trim() != expected_string.trim() {
        log!(PANIC, "Output of {output_path} doesn't equal to the expected one: {expected_path}");
    } else {
        log!(INFO, "TEST: `translate_masm`: OK");
    }

    Ok(())
}

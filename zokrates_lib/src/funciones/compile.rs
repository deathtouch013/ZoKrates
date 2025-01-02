use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::path::{Path, PathBuf};


use serde_json::{to_writer_pretty};
use typed_arena::Arena;









use zokrates_circom::{write_r1cs};


use zokrates_core::compile::{compile, CompileConfig, CompileError};
use zokrates_field::{Field};
use zokrates_fs_resolver::FileSystemResolver;




pub fn compilation<T: Field>(
    input_file: &str,
    output_file: &str,
    rc1s_path: &str,
    abi_path: &str,
    stdlib_path: &str
) -> Result<(), String> {
    println!("Compiling {}\n", input_file);
    let path = PathBuf::from(input_file);
    let bin_output_path = Path::new(&output_file);
    let r1cs_output_path = Path::new(rc1s_path);
    let abi_spec_path = Path::new(abi_path);


    log::debug!("Load entry point file {}", path.display());

    let file = File::open(path.clone())
        .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;

    let mut reader = BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).unwrap();

    let fmt_error = |e: &CompileError| {
        let file = e.file().canonicalize().unwrap();
        format!(
            "{}:{}",
            file.strip_prefix(std::env::current_dir().unwrap())
                .unwrap_or(file.as_path())
                .display(),
            e.value()
        )
    };

    match Path::new(stdlib_path).exists() {
        true => Ok(()),
        _ => Err(format!(
            "Invalid standard library source path: {}",
            stdlib_path
        )),
    }?;

    let config = CompileConfig::default().debug(false);
    let resolver = FileSystemResolver::with_stdlib_root(stdlib_path);

    log::debug!("Compile");

    let arena = Arena::new();

    let artifacts = compile::<T, _>(source, path.clone(), Some(&resolver), config, &arena)
        .map_err(|e| {
            format!(
                "Compilation failed:\n\n{}",
                e.0.iter().map(fmt_error).collect::<Vec<_>>().join("\n\n")
            )
        })?;

    let (program_flattened, abi) = artifacts.into_inner();

    // serialize flattened program and write to binary file
    log::debug!("Serialize program");
    let bin_output_file = File::create(bin_output_path)
        .map_err(|why| format!("Could not create {}: {}", bin_output_path.display(), why))?;

    let r1cs_output_file = File::create(r1cs_output_path)
        .map_err(|why| format!("Could not create {}: {}", r1cs_output_path.display(), why))?;

    let mut bin_writer = BufWriter::new(bin_output_file);
    let mut r1cs_writer = BufWriter::new(r1cs_output_file);

    let mut program_flattened = program_flattened.collect();

    // hide user path
    program_flattened.module_map = program_flattened
        .module_map
        .remap_prefix(path.parent().unwrap(), Path::new(""));
    program_flattened.module_map = program_flattened
        .module_map
        .remap_prefix(Path::new(stdlib_path), Path::new("STDLIB"));

    write_r1cs(&mut r1cs_writer, program_flattened.clone()).unwrap();

    match program_flattened.serialize(&mut bin_writer) {
        Ok(constraint_count) => {
            // serialize ABI spec and write to JSON file
            log::debug!("Serialize ABI");
            let abi_spec_file = File::create(abi_spec_path)
                .map_err(|why| format!("Could not create {}: {}", abi_spec_path.display(), why))?;

            let mut writer = BufWriter::new(abi_spec_file);
            to_writer_pretty(&mut writer, &abi)
                .map_err(|_| "Unable to write data to file.".to_string())?;

            println!("Compiled code written to '{}'", bin_output_path.display());

            println!("Number of constraints: {}", constraint_count);

            Ok(())
        }
        Err(e) => {
            // something wrong happened, clean up
            std::fs::remove_file(bin_output_path).unwrap();
            Err(e.to_string())
        }
    }
}
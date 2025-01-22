use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use rand_0_8::rngs::StdRng;
use rand_0_8::SeedableRng;

#[cfg(feature = "ark")]
use zokrates_ark::Ark;
use zokrates_ast::ir;
use zokrates_ast::ir::ProgEnum;
#[cfg(feature = "bellman")]
use zokrates_bellman::Bellman;
use zokrates_common::helpers::{BackendParameter, CurveParameter, Parameters, SchemeParameter};
use zokrates_field::Field;
#[cfg(any(feature = "bellman", feature = "ark"))]
use zokrates_proof_systems::{Backend, G16, GM17, Marlin, Scheme, TaggedProof};

pub fn generate_proof(
    input_file: &str,
    witness_path: &str,
    prooving_key_path: &str,
    prooving_json_path: &str,
    backend: &str,
    prooving_scheme: &str
) -> Result<(), String> {
    let program_path = Path::new(input_file);
    let program_file = File::open(program_path)
        .map_err(|why| format!("Could not open {}: {}", program_path.display(), why))?;

    let mut reader = BufReader::new(program_file);
    let prog = ProgEnum::deserialize(&mut reader)?;

    let curve_parameter = CurveParameter::try_from(prog.curve())?;

    let backend_parameter = BackendParameter::try_from(backend)?;
    let scheme_parameter =
        SchemeParameter::try_from(prooving_scheme)?;

    let parameters = Parameters(backend_parameter, curve_parameter, scheme_parameter);

    match parameters {
        #[cfg(feature = "bellman")]
        Parameters(BackendParameter::Bellman, _, SchemeParameter::G16) => match prog {
            ProgEnum::Bn128Program(p) =>
                generate_proof_aux::<_, _, G16, Bellman>(p, witness_path,prooving_key_path,prooving_json_path),
            ProgEnum::Bls12_381Program(p) => {
                generate_proof_aux::<_, _, G16, Bellman>(p, witness_path,prooving_key_path,prooving_json_path)
            }
            _ => unreachable!(),
        },
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, _, SchemeParameter::G16) => match prog {
            ProgEnum::Bn128Program(p) => generate_proof_aux::<_, _, G16, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            ProgEnum::Bls12_381Program(p) => generate_proof_aux::<_, _, G16, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            ProgEnum::Bls12_377Program(p) => generate_proof_aux::<_, _, G16, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            ProgEnum::Bw6_761Program(p) => generate_proof_aux::<_, _, G16, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            _ => unreachable!(),
        },
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, _, SchemeParameter::GM17) => match prog {
            ProgEnum::Bn128Program(p) => generate_proof_aux::<_, _, GM17, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            ProgEnum::Bls12_381Program(p) => generate_proof_aux::<_, _, GM17, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            ProgEnum::Bls12_377Program(p) => generate_proof_aux::<_, _, GM17, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            ProgEnum::Bw6_761Program(p) => generate_proof_aux::<_, _, GM17, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            _ => unreachable!(),
        },
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, _, SchemeParameter::MARLIN) => match prog {
            ProgEnum::Bn128Program(p) => generate_proof_aux::<_, _, Marlin, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            ProgEnum::Bls12_381Program(p) => {
                generate_proof_aux::<_, _, Marlin, Ark>(p, witness_path,prooving_key_path,prooving_json_path)
            }
            ProgEnum::Bls12_377Program(p) => {
                generate_proof_aux::<_, _, Marlin, Ark>(p, witness_path,prooving_key_path,prooving_json_path)
            }
            ProgEnum::Bw6_761Program(p) => generate_proof_aux::<_, _, Marlin, Ark>(p, witness_path,prooving_key_path,prooving_json_path),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_proof_aux<
    'a,
    T: Field,
    I: Iterator<Item = ir::Statement<'a, T>>,
    S: Scheme<T>,
    B: Backend<T, S>,
>(
    program: ir::ProgIterator<'a, T, I>,
    witness_file: &str,
    prooving_key_path: &str,
    prooving_json_path: &str
) -> Result<(), String> {
    let verbose = false;

    //println!("Generating proof...");

    // deserialize witness
    let witness_path = Path::new(witness_file);
    let witness_file = File::open(witness_path)
        .map_err(|why| format!("Could not open {}: {}", witness_path.display(), why))?;

    let witness_reader = BufReader::new(witness_file);

    let witness = ir::Witness::read(witness_reader)
        .map_err(|why| format!("Could not load witness: {:?}", why))?;

    let pk_path = Path::new(prooving_key_path);
    let proof_path = Path::new(prooving_json_path);

    let pk_file = File::open(pk_path)
        .map_err(|why| format!("Could not open {}: {}", pk_path.display(), why))?;

    let pk_reader = BufReader::new(pk_file);

    let mut rng = StdRng::from_entropy();

    let proof = B::generate_proof(program, witness, pk_reader, &mut rng);
    let mut proof_file = File::create(proof_path).unwrap();

    let proof =
        serde_json::to_string_pretty(&TaggedProof::<T, S>::new(proof.proof, proof.inputs)).unwrap();
    proof_file
        .write(proof.as_bytes())
        .map_err(|why| format!("Could not write to {}: {}", proof_path.display(), why))?;

    if verbose {
        println!("Proof:\n{}", proof);
    }

    //println!("Proof written to '{}'", proof_path.display());
    Ok(())
}
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
use zokrates_common::helpers::{BackendParameter, Parameters, SchemeParameter};
use zokrates_field::Field;
#[cfg(any(feature = "bellman", feature = "ark"))]
use zokrates_proof_systems::{NonUniversalBackend, NonUniversalScheme, TaggedVerificationKey};

pub fn setup(
    input_file: &str,
    backend: &str,
    proving_scheme: &str,
    proving_key_path: &str,
    verification_key_path: &str
) -> Result<(), String> {

    // read compiled program
    let path = Path::new(input_file);
    let file =
        File::open(path).map_err(|why| format!("Couldn't open {}: {}", path.display(), why))?;

    let mut reader = BufReader::new(file);
    let prog = ProgEnum::deserialize(&mut reader)?;

    let parameters = Parameters::try_from((
        backend,
        prog.curve(),
        proving_scheme
    ))?;

    match parameters {
        #[cfg(feature = "bellman")]
        Parameters(BackendParameter::Bellman, _, SchemeParameter::G16) => match prog {
            ProgEnum::Bn128Program(p) => {
                setup_aux::<_, _, zokrates_proof_systems::G16, Bellman>(p, proving_key_path, verification_key_path)
            }
            ProgEnum::Bls12_381Program(p) => {
                setup_aux::<_, _, zokrates_proof_systems::G16, Bellman>(p, proving_key_path, verification_key_path)
            }
            _ => unreachable!(),
        },
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, _, SchemeParameter::G16) => match prog {
            ProgEnum::Bn128Program(p) => setup_aux::<_, _, zokrates_proof_systems::G16, Ark>(p, proving_key_path, verification_key_path),
            ProgEnum::Bls12_381Program(p) => {
                setup_aux::<_, _, zokrates_proof_systems::G16, Ark>(p, proving_key_path, verification_key_path)
            }
            ProgEnum::Bls12_377Program(p) => {
                setup_aux::<_, _, zokrates_proof_systems::G16, Ark>(p, proving_key_path, verification_key_path)
            }
            ProgEnum::Bw6_761Program(p) => {
                setup_aux::<_, _, zokrates_proof_systems::G16, Ark>(p, proving_key_path, verification_key_path)
            }
            _ => unreachable!(),
        },
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, _, SchemeParameter::GM17) => match prog {
            ProgEnum::Bn128Program(p) => setup_aux::<_, _, zokrates_proof_systems::GM17, Ark>(p, proving_key_path, verification_key_path),
            ProgEnum::Bls12_381Program(p) => {
                setup_aux::<_, _, zokrates_proof_systems::GM17, Ark>(p, proving_key_path, verification_key_path)
            }
            ProgEnum::Bls12_377Program(p) => {
                setup_aux::<_, _, zokrates_proof_systems::GM17, Ark>(p, proving_key_path, verification_key_path)
            }
            ProgEnum::Bw6_761Program(p) => {
                setup_aux::<_, _, zokrates_proof_systems::GM17, Ark>(p, proving_key_path, verification_key_path)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn setup_aux <
    'a,
    T: Field,
    I: Iterator<Item = ir::Statement<'a, T>>,
    S: NonUniversalScheme<T>,
    B: NonUniversalBackend<T, S>,
>(
    program: ir::ProgIterator<'a, T, I>,
    proving_key_path: &str,
    verification_key_path: &str
) -> Result<(), String> {
    println!("Performing setup...");

    // get paths for proving and verification keys
    let pk_path = Path::new(proving_key_path);
    let vk_path = Path::new(verification_key_path);

    let mut rng = StdRng::from_entropy();

    // run setup phase
    let keypair = B::setup(program, &mut rng);

    // write verification key
    let mut vk_file = File::create(vk_path)
        .map_err(|why| format!("Could not create {}: {}", vk_path.display(), why))?;
    vk_file
        .write_all(
            serde_json::to_string_pretty(&TaggedVerificationKey::<T, S>::new(keypair.vk))
                .unwrap()
                .as_bytes(),
        )
        .map_err(|why| format!("Could not write to {}: {}", vk_path.display(), why))?;

    println!("Verification key written to '{}'", vk_path.display());

    // write proving key
    let mut pk_file = File::create(pk_path)
        .map_err(|why| format!("Could not create {}: {}", pk_path.display(), why))?;
    pk_file
        .write_all(keypair.pk.as_ref())
        .map_err(|why| format!("Could not write to {}: {}", pk_path.display(), why))?;

    println!("Proving key written to '{}'", pk_path.display());
    println!("Setup completed");

    Ok(())
}

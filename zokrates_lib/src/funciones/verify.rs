use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[cfg(feature = "ark")]
use zokrates_ark::Ark;
#[cfg(feature = "bellman")]
use zokrates_bellman::Bellman;
use zokrates_common::helpers::{BackendParameter, CurveParameter, Parameters, SchemeParameter};
use zokrates_field::{Bls12_377Field, Bls12_381Field, Bn128Field, Bw6_761Field, Field};
#[cfg(any(feature = "bellman", feature = "ark"))]
use zokrates_proof_systems::{Backend, G16, GM17, Marlin, Scheme};
use zokrates_proof_systems::Proof;

pub fn verify(
    verification_key_path: &str,
    prooving_json_path: &str,
    backend: &str
) -> Result<(), String> {

    println!("Comenzando Verificacion");

    let vk_path = Path::new(verification_key_path);
    let vk_file = File::open(vk_path)
        .map_err(|why| format!("Could not open {}: {}", vk_path.display(), why))?;

    // deserialize vk to JSON
    let vk_reader = BufReader::new(vk_file);
    let vk: serde_json::Value = serde_json::from_reader(vk_reader)
        .map_err(|why| format!("Could not deserialize verification key: {}", why))?;

    let proof_path = Path::new(prooving_json_path);
    let proof_file = File::open(proof_path)
        .map_err(|why| format!("Could not open {}: {}", proof_path.display(), why))?;

    // deserialize proof to JSON
    let proof_reader = BufReader::new(proof_file);
    let proof: serde_json::Value = serde_json::from_reader(proof_reader)
        .map_err(|why| format!("Could not deserialize proof: {}", why))?;

    println!("Leyendo ficheros");
    // extract curve and scheme parameters from both
    let proof_curve = proof
        .get("curve")
        .ok_or_else(|| "Field `curve` not found in proof".to_string())?
        .as_str()
        .ok_or_else(|| "`curve` should be a string".to_string())?;
    let proof_scheme = proof
        .get("scheme")
        .ok_or_else(|| "Field `scheme` not found in proof".to_string())?
        .as_str()
        .ok_or_else(|| "`scheme` should be a string".to_string())?;
    let vk_curve = vk
        .get("curve")
        .ok_or_else(|| "Field `curve` not found in verification key".to_string())?
        .as_str()
        .ok_or_else(|| "`curve` should be a string".to_string())?;
    let vk_scheme = vk
        .get("scheme")
        .ok_or_else(|| "Field `scheme` not found in verification key".to_string())?
        .as_str()
        .ok_or_else(|| "`scheme` should be a string".to_string())?;

    if proof_curve != vk_curve {
        return Err(format!(
            "Expected the curve of the proof and the verification key to be equal, found {} != {}",
            proof_curve, vk_curve
        ));
    }

    if proof_scheme != vk_scheme {
        return Err(format!(
            "Expected the scheme of the proof and the verification key to be equal, found {} != {}",
            proof_scheme, vk_scheme
        ));
    }

    let scheme = vk_scheme;
    let curve = vk_curve;

    // determine parameters based on that
    let parameters =
        Parameters::try_from((backend, curve, scheme))?;

    match parameters {
        #[cfg(feature = "bellman")]
        Parameters(BackendParameter::Bellman, CurveParameter::Bn128, SchemeParameter::G16) => {
            verify_aux::<Bn128Field, G16, Bellman>(vk, proof)
        }
        #[cfg(feature = "bellman")]
        Parameters(BackendParameter::Bellman, CurveParameter::Bls12_381, SchemeParameter::G16) => {
            verify_aux::<Bls12_381Field, G16, Bellman>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bn128, SchemeParameter::G16) => {
            verify_aux::<Bn128Field, G16, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_381, SchemeParameter::G16) => {
            verify_aux::<Bls12_381Field, G16, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_377, SchemeParameter::G16) => {
            verify_aux::<Bls12_377Field, G16, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bw6_761, SchemeParameter::G16) => {
            verify_aux::<Bw6_761Field, G16, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bn128, SchemeParameter::GM17) => {
            verify_aux::<Bn128Field, GM17, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_381, SchemeParameter::GM17) => {
            verify_aux::<Bls12_381Field, GM17, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_377, SchemeParameter::GM17) => {
            verify_aux::<Bls12_377Field, GM17, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bw6_761, SchemeParameter::GM17) => {
            verify_aux::<Bw6_761Field, GM17, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bn128, SchemeParameter::MARLIN) => {
            verify_aux::<Bn128Field, Marlin, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_381, SchemeParameter::MARLIN) => {
            verify_aux::<Bls12_381Field, Marlin, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_377, SchemeParameter::MARLIN) => {
            verify_aux::<Bls12_377Field, Marlin, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bw6_761, SchemeParameter::MARLIN) => {
            verify_aux::<Bw6_761Field, Marlin, Ark>(vk, proof)
        }
        _ => unreachable!(),
    }
}

fn verify_aux<T: Field, S: Scheme<T>, B: Backend<T, S>>(
    vk: serde_json::Value,
    proof: serde_json::Value,
) -> Result<(), String> {
    // convert the JSON vk and proof to the correct types
    let vk = serde_json::from_value(vk)
        .map_err(|why| format!("Could not deserialize verification key: {}", why))?;
    let proof: Proof<T, S> = serde_json::from_value(proof)
        .map_err(|why| format!("Could not deserialize proof: {}", why))?;

    println!("Performing verification...");
    println!(
        "{}",
        match B::verify(vk, proof) {
            true => "PASSED",
            false => "FAILED",
        }
    );

    Ok(())
}
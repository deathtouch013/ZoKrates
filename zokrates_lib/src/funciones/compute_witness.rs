use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use zokrates_abi::{Encode, Inputs};
use zokrates_ast::ir;
use zokrates_ast::ir::ProgEnum;
use zokrates_ast::typed::{ConcreteSignature, ConcreteType};
use zokrates_ast::typed::types::GTupleType;
use zokrates_circom::write_witness;
use zokrates_field::Field;

pub fn compute_witness(
    input_file: &str,
    witness_file: &str,
    out_wtns_file: &str,
    argumentos: Vec<&str>
) -> Result<(), String> {
    // read compiled program
    let path = Path::new(input_file);
    let file =
        File::open(path).map_err(|why| format!("Could not open {}: {}", path.display(), why))?;

    let mut reader = BufReader::new(file);

    match ProgEnum::deserialize(&mut reader)? {
        ProgEnum::Bn128Program(p) => compute_witness_aux(p, witness_file, out_wtns_file, argumentos),
        ProgEnum::Bls12_377Program(p) => compute_witness_aux(p, witness_file, out_wtns_file, argumentos),
        ProgEnum::Bls12_381Program(p) => compute_witness_aux(p, witness_file, out_wtns_file, argumentos),
        ProgEnum::Bw6_761Program(p) => compute_witness_aux(p, witness_file, out_wtns_file, argumentos),
        ProgEnum::PallasProgram(p) => compute_witness_aux(p, witness_file, out_wtns_file, argumentos),
        ProgEnum::VestaProgram(p) => compute_witness_aux(p, witness_file, out_wtns_file, argumentos),
    }
}

fn compute_witness_aux<'a, T: Field, I: Iterator<Item = ir::Statement<'a, T>>>(
    ir_prog: ir::ProgIterator<'a, T, I>,
    witness_file: &str,
    out_wtns_file: &str,
    argumentos: Vec<&str>
) -> Result<(), String> {

    let verbose = false;

    //println!("Computing witness...");

    let signature = ConcreteSignature::new()
        .inputs(vec![ConcreteType::FieldElement; ir_prog.arguments.len()])
        .output(ConcreteType::Tuple(GTupleType::new(
            vec![ConcreteType::FieldElement; ir_prog.return_count],
        )));


    /*let arguments = argumentos;
    let a = arguments
        .map(|a| {
            a.iter()
                .map(|x| T::try_from_dec_str(x).map_err(|_| x.to_string()))
                .collect::<Result<Vec<_>, _>>()
        })
        .unwrap_or_else(|| Ok(vec![]))
        .map(Inputs::Raw);*/

    let mut result_vec = Vec::new();

    for arg in argumentos {
        match T::try_from_dec_str(arg) {
            Ok(value) => result_vec.push(value),
            Err(_) => {
                // Manejar el error si es necesario
                println!("Error al convertir el argumento {}", arg);
                result_vec = vec![];
                break;
            }
        }
    }

    let arguments = Inputs::Raw(result_vec);

    let interpreter = zokrates_interpreter::Interpreter::default();
    let public_inputs = ir_prog.public_inputs();

    let witness = interpreter
        .execute_with_log_stream(
            &arguments.encode(),
            ir_prog.statements,
            &ir_prog.arguments,
            &ir_prog.solvers,
            &mut std::io::stdout(),
        )
        .map_err(|e| format!("Execution failed: {}", e))?;

    use zokrates_abi::Decode;

    let results_json_value: serde_json::Value =
        zokrates_abi::Value::decode(witness.return_values(), *signature.output).into_serde_json();

    if verbose {
        println!("\nWitness: \n{}\n", results_json_value);
    }

    // write witness to file
    let output_path = Path::new(witness_file);
    let output_file = File::create(output_path)
        .map_err(|why| format!("Could not create {}: {}", output_path.display(), why))?;

    let writer = BufWriter::new(output_file);

    witness
        .write(writer)
        .map_err(|why| format!("Could not save witness: {:?}", why))?;

    // write circom witness to file
    let wtns_path = Path::new(out_wtns_file);
    let wtns_file = File::create(wtns_path)
        .map_err(|why| format!("Could not create {}: {}", output_path.display(), why))?;

    let mut writer = BufWriter::new(wtns_file);

    write_witness(&mut writer, witness, public_inputs)
        .map_err(|why| format!("Could not save circom witness: {:?}", why))?;

    println!("Witness file written to '{}'", output_path.display());
    Ok(())
}
use std::fmt;
use std::str::FromStr;


pub enum Command {
    Compile,
    Setup,
    ComputeWitness,
    GenerateProof,
    Verify,
}

impl FromStr for Command {

    type Err = String;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "compile"  => Ok(Command::Compile),
            "setup"  => Ok(Command::Setup),
            "compute-witness"  => Ok(Command::ComputeWitness),
            "generate-proof" => Ok(Command::GenerateProof),
            "verify" => Ok(Command::Verify),
            _      => Err(format!("Comando desconocido: {}", input)),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Command::Compile => "compile",
            Command::Setup => "setup",
            Command::ComputeWitness => "compute-witness",
            Command::GenerateProof => "generate-proof",
            Command::Verify => "Verify",
        };
        write!(f, "{}", s)
    }
}
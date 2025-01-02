use std::string::String;

use zokrates_common::constants;
use zokrates_field::Bn128Field;

use funciones::command::Command;
use crate::funciones::compile::compilation;
use crate::funciones::compute_witness::compute_witness;
use crate::funciones::generate_proof::generate_proof;
use crate::funciones::setup::setup;
use crate::funciones::verify::verify;


pub mod funciones;

pub fn ejecutar_zokrates(
    command: Command,
    input_file: &str,
    output_file: &str,
    argumentos: Vec<&str>,
    stdlib_path: &str
) -> String {
    match command {
        Command::Compile => {
            //TODO añadir field T como posible parametro para poder seleccionar como compilarlo
            let result = compilation::<Bn128Field>(input_file, output_file,"out.r1cs","abi.json",stdlib_path);
            match result {
                Ok(()) => String::from("Compile completado"),
                Err(e) => e.to_string(),
            }
        },
        Command::Setup => {
            let result = setup(input_file, constants::ARK, constants::G16,"proving.key","verification.key");
            match result {
                Ok(()) => String::from("Setup completado"),
                Err(e) => e.to_string(),
            }
        },
        Command::ComputeWitness => {
            let result = compute_witness(input_file,argumentos);
            match result {
                Ok(()) => String::from("ComputeWitness completado"),
                Err(e) => e.to_string(),
            }
        },
        Command::GenerateProof => {
            let result = generate_proof(input_file, "witness", "proving.key", "proof.json", constants::ARK, constants::G16);
            match result {
                Ok(()) => String::from("GenerateProof completado"),
                Err(e) => e.to_string(),
            }
        },
        Command::Verify => {
            let result = verify("verification.key", input_file, constants::ARK);
            match result {
                Ok(()) => String::from("Verify completado"),
                Err(e) => e.to_string(),
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::funciones::command::Command;
    use crate::ejecutar_zokrates;
    #[test]
    fn llamada_modulo() {


        //Es necesario cambiar el directorio para que escriba bien el abi y el r1cs, sino hay que indicarle el path para que los escriba en el path absoluto
        let _ = std::env::set_current_dir("/home/ivan/tfm/prueba_zokrates/compiles");

        let resultado_compilacion = ejecutar_zokrates(Command::Compile, "MerkleTreeCommitment.zok", "MerkleTreeCommitment", vec![],"/home/ivan/tfm/ZoKrates/zokrates_stdlib/stdlib");
        println!("------\t\t\t{:?}\t\t\t------", resultado_compilacion);

        let resultado_setup = ejecutar_zokrates(Command::Setup, "MerkleTreeCommitment", "", vec![],"");
        println!("------\t\t\t{:?}\t\t\t------", resultado_setup);

        let argumentos = vec!["10874558257364215411585792936079464004088800861433801238921804623288757929722", "13762358941768060070215400923872154315006674136045207949897221747335757266424", "16105191279656109253283103316616030658748842110341659597772514765816651818802", "0", "887525050574869521205990955787716651163918155412797083726597746922154651110", "11050979768535823918351860532261257640072880689938297246333979254422816301271", "8288731795619312171523842516157094509384652055000294762778343591764565093361", "14192372706728730262214646371619318843204289811732587862356038721176256807525", "11983198567403984051002948413771339140689169369117609381907901297285492131164", "16599766575779795195729212475904391163108316947290243782794558589702567108707", "20918184019111500814196096401449180003253579405224235981966127464564761082267", "15829511081209404397577575097317678835635646685878202442616197376204840360061", "9402186187262750674047647290099660463259914612728196813920227416994949476076", "8243671134951889089257678411057335736741791746315306846234082991322408172610", "21508867748397084465220216237713075060258082107027640546264217857815064595119", "3624297512770083993419845819211375331492550006139367946605089419454664589685", "11316258149442062774693465605895787326541341904732487124871518722117565425681", "10637197348159513850929689621921827989639546645444390273184234326946781076498", "12981099420153462223692035909374294775575070631180341811796228095170312359698", "7844671023818699159194868358967633237009524741595680909229795142580481255916", "7525429295520227748714687749531906346931526855502012585494473710207283202080", "19028180442761522130161482463153549347260180697374226797101488662593401080767"];
        let resultado_compute_withness = ejecutar_zokrates(Command::ComputeWitness, "MerkleTreeCommitment", "", argumentos, "");
        println!("------\t\t\t{:?}\t\t\t------", resultado_compute_withness);

        let resultado_generate_proof = ejecutar_zokrates(Command::GenerateProof, "MerkleTreeCommitment", "", vec![], "");
        println!("------\t\t\t{:?}\t\t\t------", resultado_generate_proof);

        let resultado_verify = ejecutar_zokrates(Command::Verify, "proof.json", "", vec![], "");
        println!("------\t\t\t{:?}\t\t\t------", resultado_verify);


    }

    /*
    #[test]
    fn llamada_modulo() {


        //Es necesario cambiar el directorio para que escriba bien el abi y el r1cs, sino hay que indicarle el path para que los escriba en el path absoluto
        let _ = std::env::set_current_dir("C:\\Users\\ivan\\Desktop\\sharedDebian\\prueba_zokrates");

        // Compilacion

        let result = compilation::<Bn128Field>("MerkleTreeCommitment.zok", "MerkleTreeCommitment","out.r1cs","abi.json","C:\\Users\\ivan\\Desktop\\sharedDebian\\ZoKrates\\zokrates_stdlib\\stdlib");
        println!("{:?}", result);

        // Set up

        let _ = setup("MerkleTreeCommitment", constants::ARK, constants::G16,"proving.key","verification.key");

        // Compute-witness

        let vec = vec!["3074700584","3297633031","3586955801","596102331","2510301539","3541234568","4192264471","261357471","3694357574","3209696115","2744578102","3444621100","3748548492","507865226","520184419","489593409","4274748281","1987463201","838268950","3895429365","394107150","2819274376","3720704477","2356102804","2","124397511","2656751146","1028127197","780395775","1856538411","1008867574","1821554843","1442014560","1551731367","3990524871","1287917505","3067797748","173343371","2369310072","4076470279","713754573","44761127","3767745481","456920052","4258932622","3113558012","2143645027","2542720859","956652685","192581672","463918470","3039789661","2897768309","23264053","1630770507","2203458128","2618280509","1919659818","855723775","1228099537","1576734779","2732467002","1563540401","3853004647","1480635276","1570800242","2946367298","2400415195","30270881","2958269891","2974649601","95341755","162131955","3458954803","3129621389","3660766988","1059436805","1164907141","1682791105","2804186849","2736952205","1536416055","1661868864","2437401428","25615811","598517623","1000019315","3826150235","2120092690","2491448803","1556518222","2832927532","2989494499","3771074425","18599028","1246886353","757211706","2081219313","2585321247","344997021","261034445","1503059972","2586456332","105003828","1134462869","1568973916","900247223","2680031666","3005138058","4070577543","2966166433","397131196","4017998637","609859258","3824102339","3880320535","1208493104","780848012","2747072069","1424675035","3080066725","2781632314","2525761659","1482129315","964149825","137221024","1602659599","29188632","112002440","174147161","2426679788","4227565212","1583043394","3320064619","3128866329","3246823852","1626001575","2026402776","3790187636","1125066562","1866574701","4142449049","3529206398","4057857133","2594476076","3257770671","1126241521","3164731293","2654330723","418790464","954852372","1680176805","3589928573","1131353510","4075607393","2397264447","832811729","2604816875","1933262996","4080691625","2553500063","3363759428","2530306752","3316374645","965951524","3763480610","1379792578","3771605124","1667105125"];
        let _ = compute_witness("MerkleTreeCommitment",vec);

        // Generate-proof
        let _ = generate_proof("MerkleTreeCommitment", "witness", "proving.key", "proof.json", constants::ARK, constants::G16);

        //Verify

        let _ = verify("verification.key", "proof.json", constants::ARK);
    }

     */
}

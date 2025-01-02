#!/bin/bash

set -e

# Lista de módulos en orden de dependencias
modules=("zokrates_common" "zokrates_field" "zokrates_fs_resolver" "zokrates_parser" "zokrates_pest_ast" "zokrates_embed" "zokrates_ast" "zokrates_proof_systems" "zokrates_profiler" "zokrates_abi" "zokrates_analysis" "zokrates_interpreter" "zokrates_ark" "zokrates_codegen" "zokrates_core" "zokrates_circom" "zokrates_bellperson" "zokrates_bellman" "zokrates_cli" "zokrates_lib")

for module in "${modules[@]}"; do
    echo "Publicando $module..."
    cd $module
    cargo publish --registry kellnr
    cd ..
done

echo "Todos los módulos se publicaron correctamente."

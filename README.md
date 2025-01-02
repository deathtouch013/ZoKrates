
<img src="zokrates_logo.svg" width="100%" height="180">

# ZoKrates

[![Join the chat at https://gitter.im/ZoKrates/Lobby](https://badges.gitter.im/ZoKrates/Lobby.svg)](https://gitter.im/ZoKrates/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![CircleCI develop](https://img.shields.io/circleci/project/github/Zokrates/ZoKrates/develop.svg?label=develop)](https://circleci.com/gh/Zokrates/ZoKrates/tree/develop)

ZoKrates is a toolbox for zkSNARKs on Ethereum.

_This is a proof-of-concept implementation. It has not been tested for production._

## Subir ZoKrates a un resgistry

Para poder subir los módulos de Zokrates a un registro, tanto privado como público, fue necesario modificar todas las dependencias.

### Configuración de un registro privado

Para utilizar un registro privado en el que subir los módulos de Zokrates, he desplegado un servidor `kellnr`. Con el siguiente docker-compose se puede desplegar un `kellnr` facilmente:

```yaml
services:
  registry:
    image: "ghcr.io/kellnr/kellnr:5.2.6"
    ports:
      - "8000:8000"
    volumes:
      - kellnr-persistant-test:/opt/kdata
    environment:
      KELLNR_SETUP__ADMIN_PWD: "<password>" 
      KELLNR_SETUP__ADMIN_TOKEN: "<admintoken>"
      KELLNR_REGISTRY__MAX_CRATE_SIZE: "500"
      KELLNR_ORIGIN__HOSTNAME: "<hostname>"
      KELLNR_ORIGIN__PORT: "8000"
      KELLNR_LOCAL__PORT: "8000"


volumes:
  kellnr-persistant-test:
```

Al emplear un registro privado, he modificado todas las dependencias de las librerías para que apunten a mi registro. Como no es público, le indiqué a `cargo` dónde se encuentra mi registro modificando el fichero`~/.cargo/config.toml` añadiendo la siguiente configuración:

```toml
[registries.kellnr]

index = "sparse+http://<URL>/api/v1/crates/"
credential-provider = ["cargo:token"]
token = "<token>"
```

### Compilación de los módulos y subida al registro

Para poder compilar los módulos de ZoKrates es necesario descargarse dos repositorios antes para subirlos previamente al registro. Estos repositorios son:

- [phase2](https://github.com/Zokrates/phase2) (commit 971123223b9cb8c628e885120b120f1ddb413553)
- [marlin](https://github.com/arkworks-rs/marlin) (commit 026b73c20638f4f86cbae0946045934c865d5a30)

En el repositorio de `marlin` es necesario modificar una línea para que compile correctamente. En el fichero `lib.rs` de la carpeta `src` es necesario eliminar de la línea 15 `, const_err`. Es necesario hacer un commit con este cambio para poder subirlo al registro o usar el flag `--allow-dirty`.

Una vez descargados los repositorios, se pueden subir al registro ejecutando los siguientes comandos en los repositorios correspondientes:

```bash
cd phase2
cargo publish --package phase2 --registry kellnr

cd ../marlin
cargo publish --package ark-marlin --registry kellnr
```

El siguiente paso es compilar los módulos de ZoKrates y subirlos al registro. Para ello, es necesario clonar el repositorio de ZoKrates y ejecutar el siguiente script, el cual publica los módulos en el orden correspondiente en el registro:

```bash
./publish.sh
```

## Getting Started

Load the ZoKrates Plugin on [Remix](https://remix.ethereum.org) to write your first SNARK program!

Alternatively, you can install the ZoKrates CLI:

```bash
curl -LSfs get.zokrat.es | sh
```

Have a look at the [documentation](https://zokrates.github.io/) for more information about using ZoKrates.
[Get started](https://zokrates.github.io/gettingstarted.html), then try a [tutorial](https://zokrates.github.io/examples/rng_tutorial.html)!

## Getting Help

If you run into problems, ZoKrates has a [Gitter](https://gitter.im/ZoKrates/Lobby) room.

## License

ZoKrates is released under the GNU Lesser General Public License v3.

## Contributing

We happily welcome contributions. You can either pick an existing issue or reach out on [Gitter](https://gitter.im/ZoKrates/Lobby).

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in the work shall be licensed as above, without any additional terms or conditions.

### Git Hooks

You can enable zokrates git hooks locally by running:

```sh
git config core.hooksPath .githooks
```

### `{js,json,ts}` formatting

We enforce strict formatting of `.{js,json,ts}` files in CI. This check is not included in the git hooks. If you modify such a file, you can ensure its formatting is correct by running:

```
npm i -g prettier
prettier --write "./**/*.{js,ts,json}" --ignore-path .gitignore
```

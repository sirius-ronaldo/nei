#!/usr/bin/env bash

set -euo pipefail

project_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$project_root"

command -v cargo >/dev/null || { echo "erro: cargo não encontrado" >&2; exit 1; }
command -v dpkg-deb >/dev/null || { echo "erro: dpkg-deb não encontrado" >&2; exit 1; }

package_name=nei
package_version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
package_architecture=${DEB_HOST_ARCH:-}

if [[ -z "$package_architecture" ]]; then
    package_architecture=$(dpkg --print-architecture 2>/dev/null || true)
fi

if [[ -z "$package_architecture" ]]; then
    echo "erro: não foi possível determinar a arquitetura Debian" >&2
    exit 1
fi

if [[ -z "$package_version" ]]; then
    echo "erro: versão não encontrada em Cargo.toml" >&2
    exit 1
fi

if [[ "${NEI_SKIP_BUILD:-0}" == "1" ]]; then
    echo "Usando o binário de release existente (NEI_SKIP_BUILD=1)..."
else
    echo "Compilando nei $package_version ($package_architecture)..."
    cargo build --release
fi

binary_path="target/release/$package_name"
if [[ ! -x "$binary_path" ]]; then
    echo "erro: binário não encontrado em $binary_path" >&2
    exit 1
fi

package_root="target/debian/${package_name}_${package_version}_${package_architecture}"
package_file="target/${package_name}_${package_version}_${package_architecture}.deb"

rm -rf "$package_root"
mkdir -p "$package_root/usr/bin" \
    "$package_root/usr/share/doc/$package_name" \
    "$package_root/usr/share/man/man1" \
    "$package_root/DEBIAN"

install -m 0755 "$binary_path" "$package_root/usr/bin/$package_name"
install -m 0644 README.md "$package_root/usr/share/doc/$package_name/README.md"
install -m 0644 LICENSE "$package_root/usr/share/doc/$package_name/LICENSE"
install -m 0644 packaging/nei.1 "$package_root/usr/share/man/man1/nei.1"

sed \
    -e "s/@VERSION@/$package_version/g" \
    -e "s/@ARCHITECTURE@/$package_architecture/g" \
    packaging/control \
    > "$package_root/DEBIAN/control"

dpkg-deb --build --root-owner-group "$package_root" "$package_file" >/dev/null
echo "Pacote gerado: $package_file"

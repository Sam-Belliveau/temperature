# Maintainer: Your Name <sam.belliveau@gmail.com>
pkgname=smooth-temperature-git
pkgver=0.1.0
pkgrel=1
epoch=
pkgdesc="read hwmon sensors and smooth out the value"
arch=("x86_64")
url="https://github.com/Sam-Belliveau/temperature"
license=('GPL')
groups=()
depends=()
makedepends=()
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("https://github.com/Sam-Belliveau/temperature/archive/master.tar.gz")
noextract=()
b2sums=('f9aa7408af57e772a7553e9d06ccd4b9fad61cd2033ca8c03f6e997d648af168be07989b5ec3845a3c7f3951408fe05eb6dd4e69dcd52d972f7c850373cee4a8')
validpgpkeys=()

prepare() {
	return 0
}

build() {
	cargo build --release --locked --all-features --target-dir=target
}

check() {
	cargo test --release --locked --target-dir=target
}

package() {
	install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin"
}

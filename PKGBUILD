# Maintainer: SamBelliveau <sam.belliveau@gmail.com>
pkgname=smooth-temperature-git
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

build() {
    return 0
}

package() {
    cargo install \
         --root="$pkgdir" \
         --git https://github.com/Sam-Belliveau/temperature \
         --branch main \
         smooth-temperature
}

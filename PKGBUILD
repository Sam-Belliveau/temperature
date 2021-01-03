# Maintainer: Sam Belliveau <sam.belliveau@gmail.com>
pkgname=smooth-temperature-bin
pkgver=0.2.0
pkgrel=1
pkgdesc="a utility for smoothly reading hwmon sensor files and smoothly interpolating their values without high cpu or filesystem usage"
url="https://github.com/Sam-Belliveau/temperature"
license=("GPL")
arch=("x86_64")
provides=("smooth-temperature")
options=("strip")
source=("https://github.com/Sam-Belliveau/temperature/releases/download/v$pkgver/smooth-temperature-$pkgver-x86_64.tar.gz")
sha256sums=("a26d8b3623bbc83e6f887d60258c2402b8e01d7a08cb6fc0777c7a0a47192e69")

package() {
    install -Dm755 smooth-temperature -t "$pkgdir/usr/bin/"
}

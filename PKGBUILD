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
sha256sums=("08d7029dea428a79ec18d3290abb3d103aa46504e94286cdfbc75a03886ea945")

package() {
    install -Dm755 smooth-temperature -t "$pkgdir/usr/bin/"
}

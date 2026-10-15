#!/usr/bin/env bash
set -euo pipefail

# Determine package version:
# 1. From VERSION environment variable
# 2. Or from GITHUB_REF_NAME (e.g. tag v0.2.0 -> 0.2.0)
# 3. Or from root Cargo.toml
if [[ -n "${VERSION:-}" ]]; then
    PKG_VERSION="${VERSION#v}"
elif [[ -n "${GITHUB_REF_NAME:-}" ]]; then
    PKG_VERSION="${GITHUB_REF_NAME#v}"
else
    PKG_VERSION=$(grep -m1 '^version = ' Cargo.toml | cut -d'"' -f2 || echo "0.1.0")
fi

VERSION="${PKG_VERSION}"
ARCH="amd64"
PKG_NAME="redragon-m709-rgb"
PKG_FULL="${PKG_NAME}_${VERSION}_${ARCH}"
STAGING="target/deb-staging"
DEB_FILE="${PKG_FULL}.deb"

echo "==> Building release binaries with Cargo..."
cargo build --release

echo "==> Cleaning staging directory: ${STAGING}..."
rm -rf "${STAGING}"
mkdir -p "${STAGING}/DEBIAN"
chmod 755 "${STAGING}/DEBIAN"
mkdir -p "${STAGING}/usr/bin"
mkdir -p "${STAGING}/usr/share/applications"
mkdir -p "${STAGING}/usr/share/icons/hicolor/scalable/apps"
mkdir -p "${STAGING}/etc/udev/rules.d"
mkdir -p "${STAGING}/usr/share/doc/${PKG_NAME}"

echo "==> Installing binaries and assets to staging..."
install -m 755 target/release/m709-gui "${STAGING}/usr/bin/m709-gui"
install -m 755 target/release/m709 "${STAGING}/usr/bin/m709"
ln -sf m709 "${STAGING}/usr/bin/m709-cli"

install -m 644 packaging/com.redragon.mouse.m709.desktop "${STAGING}/usr/share/applications/"
install -m 644 packaging/redragon-m709.svg "${STAGING}/usr/share/icons/hicolor/scalable/apps/"
install -m 644 99-redragon-m709.rules "${STAGING}/etc/udev/rules.d/"
install -m 644 LICENSE "${STAGING}/usr/share/doc/${PKG_NAME}/copyright"
install -m 644 README.md "${STAGING}/usr/share/doc/${PKG_NAME}/README"

# Calculate installed size in KB
INSTALLED_SIZE=$(du -sk "${STAGING}/usr" | awk '{print $1}')

echo "==> Writing DEBIAN/control..."
cat <<EOF > "${STAGING}/DEBIAN/control"
Package: ${PKG_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Installed-Size: ${INSTALLED_SIZE}
Maintainer: Jefferson <jefferson.admsistemas@gmail.com>
Homepage: https://github.com/computocell/redragon-m709-rgb
Depends: libgtk-4-1 (>= 4.6.0), libusb-1.0-0 (>= 1.0.20), libc6 (>= 2.34)
Description: Linux driver & GTK4 desktop control panel for Redragon M709 RGB mouse
 Native Rust GTK4 application to configure RGB lighting effects, custom colors,
 DPI stages, USB polling rates, and physical button mappings for the Redragon
 M709 (Ranger RGB) gaming mouse on Linux. Includes both a modern GTK4 GUI
 (m709-gui) and a fast CLI utility (m709).
EOF

echo "==> Writing DEBIAN/postinst..."
cat <<'EOF' > "${STAGING}/DEBIAN/postinst"
#!/bin/sh
set -e

if command -v udevadm >/dev/null 2>&1; then
    udevadm control --reload-rules || true
    udevadm trigger --subsystem-match=usb || true
fi

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications || true
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
fi

exit 0
EOF
chmod 755 "${STAGING}/DEBIAN/postinst"

echo "==> Writing DEBIAN/postrm..."
cat <<'EOF' > "${STAGING}/DEBIAN/postrm"
#!/bin/sh
set -e

if command -v udevadm >/dev/null 2>&1; then
    udevadm control --reload-rules || true
    udevadm trigger --subsystem-match=usb || true
fi

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications || true
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
fi

exit 0
EOF
chmod 755 "${STAGING}/DEBIAN/postrm"

echo "==> Packaging .deb with dpkg-deb..."
dpkg-deb --build --root-owner-group "${STAGING}" "${DEB_FILE}"

echo ""
echo "✅ Package created successfully: ${DEB_FILE}"
ls -lh "${DEB_FILE}"
dpkg-deb --info "${DEB_FILE}"

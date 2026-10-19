#!/bin/bash
set -euo pipefail

format=${1:?expected deb, rpm, appimage or flatpak}
version=$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml", "rb"))["package"]["version"])')
mkdir -p dist
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
sh packaging/stage.sh "$work/root/usr"
desktop-file-validate "$work/root/usr/share/applications/com.garandplg.WarInTunnels.desktop"
"$work/root/usr/bin/war-in-tunnels" --version | grep -Fx "war_in_tunnels $version"
test -s "$work/root/usr/lib/war-in-tunnels/soundtracks/default/calm.ogg"

case "$format" in
  deb)
    mkdir -p "$work/root/DEBIAN"
    cat > "$work/root/DEBIAN/control" <<EOF
Package: war-in-tunnels
Version: $version
Architecture: amd64
Maintainer: GarandPLG
Depends: libasound2, libc6 (>= 2.36), libgcc-s1
Section: games
Priority: optional
Description: Underground strategy game in your terminal
EOF
    output="$PWD/dist/war-in-tunnels_${version}_amd64.deb"
    dpkg-deb --root-owner-group --build "$work/root" "$output"
    dpkg-deb --info "$output"
    dpkg-deb --extract "$output" "$work/extracted"
    "$work/extracted/usr/bin/war-in-tunnels" --version
    ;;
  rpm)
    rpmbuild -bb packaging/war-in-tunnels.spec \
      --define "_topdir $work/rpmbuild" \
      --define "package_version $version" \
      --define "staging $work/root" \
      --define '_build_id_links none'
    cp "$work"/rpmbuild/RPMS/x86_64/*.rpm dist/
    rpm -qp --requires dist/*.rpm
    mkdir "$work/extracted"
    rpm2cpio dist/*.rpm | (cd "$work/extracted" && cpio -idm --quiet)
    "$work/extracted/usr/bin/war-in-tunnels" --version
    ;;
  appimage)
    appdir="$work/WarInTunnels.AppDir"
    mv "$work/root" "$appdir"
    cp packaging/com.garandplg.WarInTunnels.desktop "$appdir/"
    cp packaging/com.garandplg.WarInTunnels.svg "$appdir/"
    mkdir -p "$appdir/usr/lib/bundled"
    # Bundle ALSA; glibc and the ELF loader come from the host.
    cp -L /usr/lib/x86_64-linux-gnu/libasound.so.2 "$appdir/usr/lib/bundled/"
    cat > "$appdir/AppRun" <<'EOF'
#!/bin/sh
set -eu
appdir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
export LD_LIBRARY_PATH="$appdir/usr/lib/bundled${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
exec "$appdir/usr/bin/war-in-tunnels" "$@"
EOF
    chmod +x "$appdir/AppRun"
    curl -fL --retry 3 https://github.com/AppImage/appimagetool/releases/download/1.9.1/appimagetool-x86_64.AppImage -o "$work/appimagetool"
    # Extract without executing the runtime: works without FUSE and with host binfmt handlers.
    7z x "$work/appimagetool" "-o$work/appimagetool-dir" > /dev/null
    output="$PWD/dist/war-in-tunnels-${version}-x86_64.AppImage"
    ARCH=x86_64 "$work/appimagetool-dir/AppRun" --no-appstream "$appdir" "$output"
    7z x "$output" "-o$work/appimage-check" > /dev/null
    "$work/appimage-check/AppRun" --version
    ;;
  flatpak)
    appid=com.garandplg.WarInTunnels
    # Package the Debian-built binary against a newer glibc runtime; no nested sandbox is needed.
    flatpak --user remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
    flatpak --user install -y --noninteractive --no-related flathub org.freedesktop.Platform//25.08
    flatpak build-init "$work/flatpak" "$appid" org.freedesktop.Platform org.freedesktop.Platform 25.08
    cp -a "$work/root/usr/." "$work/flatpak/files/"
    install -Dm755 /usr/lib/x86_64-linux-gnu/libasound.so.2 "$work/flatpak/files/lib/libasound.so.2"
    flatpak build-finish --command=war-in-tunnels --socket=pulseaudio --device=all "$work/flatpak"
    # The icon is repository-owned; its validator cannot create a sandbox in standard Docker.
    flatpak build-export --disable-sandbox "$work/repo" "$work/flatpak" test
    output="$PWD/dist/war-in-tunnels-${version}-x86_64.flatpak"
    flatpak build-bundle --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo "$work/repo" "$output" "$appid" test
    flatpak --user install -y --noninteractive --no-related "$output"
    location=$(flatpak --user info --show-location "$appid")
    test -s "$location/files/lib/war-in-tunnels/soundtracks/default/calm.ogg"
    "$location/files/bin/war-in-tunnels" --version
    ;;
  *)
    echo "Unknown package format: $format" >&2
    exit 1
    ;;
esac

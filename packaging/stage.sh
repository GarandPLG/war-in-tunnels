#!/bin/sh
set -eu

prefix=${1:?installation prefix required}
binary=${2:-target/release/war_in_tunnels}
install -Dm755 "$binary" "$prefix/lib/war-in-tunnels/war-in-tunnels"
mkdir -p "$prefix/bin"
cat > "$prefix/bin/war-in-tunnels" <<'EOF'
#!/bin/sh
set -eu
bindir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
exec "$bindir/../lib/war-in-tunnels/war-in-tunnels" "$@"
EOF
chmod +x "$prefix/bin/war-in-tunnels"
cp -r soundtracks "$prefix/lib/war-in-tunnels/"
install -Dm644 LICENSE "$prefix/share/licenses/war-in-tunnels/LICENSE"
install -Dm644 packaging/com.garandplg.WarInTunnels.desktop "$prefix/share/applications/com.garandplg.WarInTunnels.desktop"
install -Dm644 packaging/com.garandplg.WarInTunnels.svg "$prefix/share/icons/hicolor/scalable/apps/com.garandplg.WarInTunnels.svg"

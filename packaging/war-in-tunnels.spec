Name: war-in-tunnels
Version: %{package_version}
Release: 1
Summary: Underground strategy game in your terminal
License: MIT
URL: https://gitea.garandplg.com/GarandPLG/war-in-tunnels
Requires: alsa-lib

%description
A terminal-based underground strategy game.

%install
mkdir -p %{buildroot}
cp -a %{staging}/. %{buildroot}/

%files
/usr/bin/war-in-tunnels
/usr/lib/war-in-tunnels
/usr/share/applications/com.garandplg.WarInTunnels.desktop
/usr/share/icons/hicolor/scalable/apps/com.garandplg.WarInTunnels.svg
%license /usr/share/licenses/war-in-tunnels/LICENSE

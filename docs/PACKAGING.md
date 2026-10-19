# Testowe paczki Linux

Workflow `.gitea/workflows/packages.yml` uruchamia się po pushu i dla pull requestów. Nie tworzy tagów, wydań ani wpisów w AUR/Flathub. Paczki są niesygnowanymi artefaktami CI przechowywanymi przez 7 dni. Używa `upload-artifact@v3` dla zgodności z Gitea.

Obsługiwana architektura to x86_64. Są to formaty dystrybucji Linux, nie kompilacje Windows/macOS.

| Format | Budowanie i kontrola |
|---|---|
| `.deb` | Rust 1.99.1 w Debian Bookworm, `dpkg-deb`, odczyt metadanych i uruchomienie rozpakowanej binarki z `--version` |
| `.rpm` | Ta sama binarka, `rpmbuild` i `packaging/war-in-tunnels.spec`, automatyczne zależności ELF, ekstrakcja i `--version` |
| AppImage | AppDir z binarką, muzyką i libasound.so.2; appimagetool 1.9.1; ekstrakcja narzędzia i paczki przez 7z, uruchomienie AppRun z `--version` bez FUSE |
| AUR | `packaging/aur/PKGBUILD`, lokalne archiwum bieżącego checkoutu z SHA256, `makepkg` jako zwykły użytkownik w Arch Linux; kompilacja, testy, instalacja paczki i `--version`; eksport .SRCINFO |
| Flatpak | Binarka z Debiana pakowana poleceniami `flatpak build-init`, `build-finish`, `build-export`, `build-bundle`; runtime org.freedesktop.Platform 25.08; instalacja paczki w kontenerze, kontrola soundtracku i `--version` zainstalowanej binarki bez sandboxa |

`packaging/stage.sh` instaluje binarkę i soundtracki w `PREFIX/lib/war-in-tunnels`, launcher w `PREFIX/bin`, licencję, plik desktop z `Terminal=true` i ikonę SVG. Nie zmienia katalogu roboczego, więc opcja `--log` nadal zapisuje log w katalogu wybranym przez użytkownika. Audio najpierw szuka soundtracków obok rzeczywistej binarki, a następnie w bieżącym katalogu dla lokalnego Cargo. Test audio sprawdza działanie poza repozytorium.

W kontenerze Debian z zależnościami workflow można wykonać:

```sh
cargo build --release --locked
bash packaging/build.sh deb
bash packaging/build.sh rpm
bash packaging/build.sh appimage
bash packaging/build.sh flatpak
```

Lokalnie na hoście projektu polecenia Cargo uruchamiaj przez `nix develop -c`; binarka zbudowana przez Nix może zależeć od `/nix/store`, dlatego do paczek dystrybucyjnych używaj kompilacji kontenerowej. Skrypty oczekują bieżącego katalogu repozytorium i zapisują wyniki do `dist/`.

Przepis AUR jest testowy: nie wskazuje publicznego tarballa wydania. Workflow dostarcza lokalny tarball oraz zastępuje `SKIP` rzeczywistym SHA256. Wersja `pkgver` musi odpowiadać Cargo.toml; CI to sprawdza. Artefakt AUR zawiera paczkę, przepis, .SRCINFO i pasujące źródła.

Flatpak jest testem pakowania istniejącej binarki, nie kompilacją wewnątrz SDK Flatpak. Bookworm dostarcza glibc 2.36, runtime 25.08 jest nowszy. Pakowana jest biblioteka ALSA; uprawnienia obejmują PulseAudio i urządzenia dla bezpośredniego ALSA. `build-export --disable-sandbox` wyłącza wyłącznie sandbox walidatora własnej ikony z repozytorium, nie walidację ani sandbox gotowej aplikacji. Tworzenie paczki nie potrzebuje privileged ani zmiany polityki Dockera. Instalacja w Dockerze może zgłaszać bwrap dla hooków integracji pulpitu mimo poprawnego zainstalowania paczki; sprawdzamy kod wyjścia, pliki i binarkę. Pełnego uruchomienia TUI i audio w sandboxie te kontrole nie zastępują. Runtime nie jest zawarty w pliku .flatpak; użytkownik musi go zainstalować z Flathub. CI używa --no-related, aby nie pobierać opcjonalnych dodatków GPU i tłumaczeń.

Paczki Debian, RPM i AppImage mają bazę ABI Bookworm (glibc >= 2.36); nie gwarantują działania na starszych dystrybucjach. AppImage zawiera ALSA, ale korzysta z konfiguracji audio hosta. Testy nie potwierdzają odsłuchu ani otwierania terminala z menu desktop. Appimagetool jest przypięty do wersji; jego domyślny runtime jest pobierany z upstream podczas pakowania. Rust i Cargo mają cache oddzielny od zwykłego workflow Rust; dla cache wymagany jest działający serwer runnera. Arch i Flatpak pobierają własne zależności bez trwałego cache.

Dokumentacja narzędzi: [AppImage](https://github.com/AppImage/appimagetool/tree/1.9.1), [Flatpak](https://docs.flatpak.org/en/latest/flatpak-command-reference.html), [makepkg](https://man.archlinux.org/man/makepkg.8).

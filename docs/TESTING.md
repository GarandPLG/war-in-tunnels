# Testy

Uruchomienie całego zestawu z katalogu głównego:

```sh
nix develop -c cargo test --all-targets --locked
```

Testy korzystają z wbudowanego mechanizmu Rust, Clap i `ratatui::backend::TestBackend`. Nie dodano zależności. Nie wymagają terminala interaktywnego ani urządzenia audio. Test audio używa pliku `soundtracks/default/calm.ogg`, więc zasoby muszą być dostępne w repozytorium.

## Zakres

`tests/tunnel_stress.rs` obejmuje 17 testów: odległość od bazy, propagacja podpór w czterech kierunkach, alternatywne podpory, śmiertelna kaskada i gruz, koszt oraz blokada zdalnego budowania, auto-podpory w ostatnich bezpiecznych miejscach, brak środków, zachowanie opcji przy konwersji zadań, szybsze ponowne kopanie, zakładka Support, widok stresu i boczny panel zadań, wybór Moving bez kopania oraz położenie panelu poza środkiem planszy. Regresje obejmują także pełny czas budowy i rozbiórki, zatrzymanie pracującego górnika, pauzę bez budowniczego, brak dwóch równoległych prac jednego górnika, zwykły kamień w widoku stresu oraz t dostępne dopiero po Space.

| Plik | Liczba | Zachowania |
|---|---:|---|
| `tests/cli.rs` | 5 | Domyślne ustawienia, granice wszystkich opcji liczbowych, XP (także NaN i infinity), błędne enumy, flagi audio, uruchomienie binarki z help/version i błędnym argumentem |
| `tests/app.rs` | 7 | Nawigacja menu, wybór ekranów, wyjście, ignorowanie zwolnienia klawisza, komendy audio i zamknięty kanał, panel, zoom, ruch, renderowanie wszystkich widoków bez mutacji stanu, wejście przed inicjalizacją |
| `tests/board.rs` | 7 | Rozmieszczenie baz/złóż/jednostek, granice kursora i kamery, zatwierdzanie/anulowanie ścieżki, cofanie z powtórzonym polem, duży viewport, resize i zoom |
| `tests/tasks.rs` | 7 | Przyrost i limit kopania, reset etapu, pusta ścieżka, kolejność zadań, timer MovingTask, całkowity postęp, zachowanie liczby etapów przy konwersji, formatowanie postępu aktywnych i oczekujących zadań |
| `tests/audio.rs` | 2 | Zapętlanie rzeczywistego pliku OGG po jego końcu i ładowanie soundtracków obok binarki poza katalogiem repozytorium |
| `tests/resources.rs` | 7 | Stan z CLI, alias starting-iron, pasywne Wood, neutralne złoża, brak kopalń na starcie, budowa wyłącznie na Ore, wyczerpanie złoża i przychód właściciela, ochrona przepełnień i bieżące zasoby w TUI |
| `tests/context_keybindings.rs` | 6 | Kontekst mapy i trasy, anulowanie przez Esc, blokada budowania na nieprawidłowym polu, b i odrzucenie starych skrótów, globalne wyjście i czytelność legendy przy 120 kolumnach i kolory tekstu obu okien |
| `tests/construction.rs` | 16 | Uprawnienia budowniczego i delegacja, sąsiedztwo i brak zdalnej budowy, właściciel, reguła kamienia i wyjątek Ore, ochrona zajętych/wyczerpanych/wrogich złóż, atomowa opłata, 10 sekund budowy, pauza i wznowienie, brak przedwczesnego wydobycia/wejścia, procenty i anulowanie Delete w TUI, jednokrotny zwrot i zachowanie Ore, ruch do/z Mine i bonus wydobycia, ochrona rezerwy |
| `tests/recruitment.rs` | 10 | Wyłącznie własna Base, atomowy koszt każdej pozycji i brak obciążenia przy błędzie, 10 sekund rekrutacji, oczekiwanie przy 100%, brak nadpisania jednostki, blokada kolejnych pozycji do wyjścia poprzedniego rekruta, wznowienie po ruchu, niezależne bazy i właściciele, r/Enter, ignorowanie Esc w obu panelach i kontekstowa legenda |
| `tests/digging.rs` | 18 | Wykonywanie kopania i ruchu, kolejki, 3 sekundy na krok, wejście po wykopaniu, blokady i walidacja, kolory i procenty w TUI, podział trasy Moving/Digging, ochrona struktur, zmiana rodzaju zadania, panel Tab z postępem etapu i całego zadania |

Łącznie: 107 testów. Wszystkie testy znajdują się w `tests/` i sprawdzają publiczne interfejsy biblioteki lub binarkę. Funkcja `load_audio` jest eksportowana przez `app::threads`, dzięki czemu test korzysta z tego samego dekodera co odtwarzanie.

Wybrane zestawy i pojedynczy przypadek:

```sh
nix develop -c cargo test --test cli
nix develop -c cargo test --test app
nix develop -c cargo test --test board
nix develop -c cargo test --test tasks
nix develop -c cargo test --test audio
nix develop -c cargo test --test digging
nix develop -c cargo test --test board large_viewport_and_zoom_keep_camera_inside_map
```

Kontrole uzupełniające:

```sh
nix develop -c cargo check --all-targets --locked
nix develop -c cargo fmt --all -- --check
nix develop -c cargo clippy --all-targets --locked
```

`shellHook` środowiska Nix wyświetla dostępne polecenia; nie kompiluje automatycznie programu.

Testy `tests/task_cancellation.rs` (4) sprawdzają usuwanie ostatniego zadania i całej kolejki pod Tab, zachowanie wykonanych uszkodzeń, przerwany ruch bez teleportacji i zachowania timera, ochronę przeciwnika i rysowania trasy. Dodatkowa regresja sprawdza kolory tras, znikanie po wykonaniu i odwołaniu, brak zmian flag modelu oraz ukrywanie tras przeciwnika.

## CI i profile kompilacji

Jeden workflow `Project tests` w `.gitea/workflows/tests.yml` uruchamia się po pushu i dla pull requestów. Zależności `needs` ustalają kolejność `check → nix → linux → aur`; kolejny job czeka na sukces poprzedniego, a błąd pomija zależne joby. Zastępuje wcześniejsze osobne pliki `rust.yml`, `nix.yml` i `packages.yml`. Kolejność dotyczy jobów jednego przebiegu, nie serializuje różnych pushów.

Job `linux` sprawdza testowe paczki deb, rpm, AppImage i Flatpak, a następujący po nim `aur` sprawdza paczkę Arch, bez publikowania wydań. Zasady budowania, artefakty i ograniczenia opisano w [PACKAGING.md](PACKAGING.md).

Job `nix` instaluje Nix w `node:22-bookworm` i wykonuje `nix build .#default --no-update-lock-file --no-link --print-out-paths --print-build-logs`. Atrybut `.#default` wywołuje `pkgs.callPackage ./default.nix {}` z nixpkgs przypiętym w `flake.lock`; nie buduje wariantu `.#develop` opartego na Fenix. Budowa obejmuje standardową fazę testów `buildRustPackage` i włączony w pakiecie `versionCheckHook`. Nie wymaga devShell ani uruchamiania gry. Instalator potrzebuje sudo; KVM jest wyłączone. Workflow nie konfiguruje trwałego cache magazynu Nix, więc nowy job może ponownie pobierać zależności.

Job `check` wykonuje kontrole Rust. Workflow wymaga włączonych Actions w repozytorium Gitea i runnera Linux x86_64 o etykiecie `ubuntu-latest`, obsługującego akcje Node. Nazwy akcji zawierają pełne adresy GitHub. Konfiguracja nie rejestruje runnera ani nie włącza Actions na serwerze; zobacz [instrukcję Gitea](https://docs.gitea.com/usage/actions/quickstart/).

Job `check` określa obraz zadania jawnie jako `node:22-bookworm`. Kontener działa jako root i instaluje pakiety potrzebne do budowania: build-essential, pkg-config i libasound2-dev oraz curl, ca-certificates i zstd. Rust 1.99.1 z rustfmt i Clippy jest instalowany przez rustup. Ten job nie instaluje Nix i nie wykonuje jego shellHook.

Wersję Rust w CI określa `RUST_VERSION` w workflow, niezależnie od lokalnego `flake.lock`. Formatowanie, kompilacja, testy i Clippy są oddzielnymi krokami Cargo. Polecenia kompilujące projekt mają `--locked`; Clippy nie używa `-D warnings` z powodu istniejących ostrzeżeń. `actions/cache@v4` zachowuje osobno toolchain oraz rejestr Cargo, repozytoria git i `target`. Klucze uwzględniają wersję Rust, a dla Cargo także manifest, lockfile i kod; klucze częściowe pozwalają ponownie wykorzystać wcześniejsze kompilacje. Działanie cache wymaga dostępnego serwera cache runnera. Pierwszy przebieg wypełnia cache; błędy pobierania cache mogą spowodować ponowną instalację i kompilację.

Release ma `overflow-checks = true`. Ten sam zestaw 107 testów można wykonać z kontrolą przepełnień w profilu produkcyjnym:

```sh
nix develop -c cargo test --all-targets --release --locked
```

Manifest określa minimalną wspieraną wersję Rust jako 1.88. Potwierdzono `cargo check --all-targets --locked` na Rust 1.88.0 dostarczonym tymczasowo przez Fenix (`toolchainOf`, kanał `1.88.0`) wewnątrz `nix develop`. Domyślny toolchain pozostaje zarządzany przez Fenix i `flake.lock`; nie dodano drugiego pliku przypinającego toolchain. Linty z `Cargo.toml` zabraniają `unsafe` we własnym kodzie oraz ignorowania wyników `must_use`; `unused_unit` jest dozwolony, by zachować wymagane jawne `-> ()`.

## Ograniczenia

Zestaw weryfikuje wykonywanie zadań przez ticki potyczki i podstawowe zasoby, ale nie pełną rozgrywkę, balans ekonomii ani walkę. Nie potwierdza odsłuchu, zużycia CPU, obsługi urządzeń, pakowania Nix ani wszystkich układów małych terminali. Testy renderowania są kontrolą podstawową, bez utrwalania pełnych zrzutów ekranów. Transport artefaktów i odczyt cache wymagają przebiegu na Gitea.

# Testy

Uruchomienie całego zestawu z katalogu głównego:

```sh
nix develop -c cargo test --all-targets --locked
```

Testy korzystają z wbudowanego mechanizmu Rust, Clap i `ratatui::backend::TestBackend`. Nie dodano zależności. Nie wymagają terminala interaktywnego ani urządzenia audio. Test audio używa pliku `soundtracks/default/calm.ogg`, więc zasoby muszą być dostępne w repozytorium.

## Zakres

| Plik | Liczba | Zachowania |
|---|---:|---|
| `tests/cli.rs` | 5 | Domyślne ustawienia, granice wszystkich opcji liczbowych, XP (także NaN i infinity), błędne enumy, flagi audio, uruchomienie binarki z help/version i błędnym argumentem |
| `tests/app.rs` | 7 | Nawigacja menu, wybór ekranów, wyjście, ignorowanie zwolnienia klawisza, komendy audio i zamknięty kanał, panel, zoom, ruch, renderowanie wszystkich widoków bez mutacji stanu, wejście przed inicjalizacją |
| `tests/board.rs` | 7 | Rozmieszczenie baz/złóż/jednostek, granice kursora i kamery, zatwierdzanie/anulowanie ścieżki, cofanie z powtórzonym polem, duży viewport, resize i zoom |
| `tests/tasks.rs` | 7 | Przyrost i limit kopania, reset etapu, pusta ścieżka, kolejność zadań, timer MovingTask, całkowity postęp, zachowanie liczby etapów przy konwersji, formatowanie postępu aktywnych i oczekujących zadań |
| `tests/audio.rs` | 1 | Zapętlanie rzeczywistego pliku OGG po jego końcu |
| `tests/digging.rs` | 18 | Wykonywanie kopania i ruchu, kolejki, 3 sekundy na krok, wejście po wykopaniu, blokady i walidacja, kolory i procenty w TUI, podział trasy Moving/Digging, ochrona struktur, zmiana rodzaju zadania, panel Tab z postępem etapu i całego zadania |

Łącznie: 45 testów. Wszystkie testy znajdują się w `tests/` i sprawdzają publiczne interfejsy biblioteki lub binarkę. Funkcja `load_audio` jest eksportowana przez `app::threads`, dzięki czemu test korzysta z tego samego dekodera co odtwarzanie.

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

`shellHook` środowiska Nix dodatkowo wykonuje `cargo run --locked -- -h`. Wynik pomocy nie jest wynikiem testów; właściwy raport zawiera podsumowania `test result`.

## CI i profile kompilacji

Workflow `.gitea/workflows/rust.yml` uruchamia kontrole po pushu i dla pull requestów. Wymaga włączonych Actions w repozytorium Gitea i runnera Linux x86_64 o etykiecie `ubuntu-latest`, obsługującego akcje Node oraz instalację Nix. Nazwy akcji zawierają pełne adresy GitHub. Konfiguracja nie rejestruje runnera ani nie włącza Actions na serwerze; zobacz [instrukcję Gitea](https://docs.gitea.com/usage/actions/quickstart/).

Etykieta `ubuntu-latest` służy do wyboru runnera i nie gwarantuje obrazu Ubuntu: w dostarczonym logu runner 0.6.1 mapował ją na `node:16-bullseye`. Workflow określa teraz obraz zadania jawnie jako `node:22-bookworm`. Kontener działa jako root; przed instalacją Nix wykonuje `apt-get update` i instaluje `sudo`, wymagane przez `cachix/install-nix-action`. `enable_kvm: false` wyłącza zbędne próby konfiguracji KVM. Zmiana nie wymaga przełączania runnera na tryb hosta ani nadawania kontenerowi trybu privileged.

CI używa `nix develop --no-update-lock-file -c`, więc wersję środowiska określa `flake.lock`. Formatowanie, kompilacja, testy i Clippy są oddzielnymi krokami. Polecenia Cargo kompilujące projekt mają `--locked`; workflow nie zmienia zależności. Clippy nie używa `-D warnings` z powodu istniejących ostrzeżeń.

Release ma `overflow-checks = true`. Ten sam zestaw 45 testów można wykonać z kontrolą przepełnień w profilu produkcyjnym:

```sh
nix develop -c cargo test --all-targets --release --locked
```

Manifest określa minimalną wspieraną wersję Rust jako 1.88. Potwierdzono `cargo check --all-targets --locked` na Rust 1.88.0 dostarczonym tymczasowo przez Fenix (`toolchainOf`, kanał `1.88.0`) wewnątrz `nix develop`. Domyślny toolchain pozostaje zarządzany przez Fenix i `flake.lock`; nie dodano drugiego pliku przypinającego toolchain. Linty z `Cargo.toml` zabraniają `unsafe` we własnym kodzie oraz ignorowania wyników `must_use`; `unused_unit` jest dozwolony, by zachować wymagane jawne `-> ()`.

## Ograniczenia

Zestaw weryfikuje wykonywanie zadań przez ticki potyczki, ale nie pełną rozgrywkę, ekonomię ani walkę. Nie potwierdza odsłuchu, zużycia CPU, obsługi urządzeń, pakowania Nix ani wszystkich układów małych terminali. Testy renderowania są kontrolą podstawową, bez utrwalania pełnych zrzutów ekranów. Po błędzie instalacji Nix na runnerze zweryfikowano lokalnie poprawioną instalację w `node:22-bookworm` oraz `nix eval`; cały workflow wymaga ponownego przebiegu na Gitea.

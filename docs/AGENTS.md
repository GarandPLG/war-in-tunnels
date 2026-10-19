# Praca nad War in Tunnels

Instrukcje i notatki dla agentów przechowujemy w `docs/`. Ten plik jest punktem wejścia do kontekstu projektu; ponieważ znajduje się w `docs/`, agent rozpoczynający pracę w katalogu głównym powinien otworzyć go jawnie.

## Przed zmianą

1. Przeczytaj [mapę kodu](CODE_MAP.md) i [dziennik prac](PROGRESS.md), następnie aktualny kod zmienianej funkcji i jej wywołania (`rg`). Dokumentacja jest migawką, kod jest źródłem prawdy.
2. Sprawdź `git status --short`. Nie nadpisuj zastanych zmian użytkownika. Przy rozpoczęciu tej analizy zmodyfikowany był `flake.lock`.
3. Ustal zasięg: stan, obsługa wejścia, renderowanie, CLI, eksporty `mod.rs`, testy i konfiguracja. Zmieniaj tylko elementy potrzebne do zadania.
4. Aktualizuj `docs/PROGRESS.md`: cel, konkretne zmiany, wykonane kontrole i ograniczenia. Przy zmianie architektury aktualizuj też mapę.

## Narzędzia i weryfikacja

Nie wolno zakończyć zadania, jeżeli kompilator Rust zgłasza błędy uniemożliwiające kompilację. Przed zakończeniem uruchom `nix develop -c cargo check --all-targets`; jeśli wystąpią błędy kompilacji, usuń je i powtórz kontrolę aż do uzyskania sukcesu.

Wszystkie narzędzia Rust uruchamiaj przez `nix develop -c` z katalogu repozytorium:

```sh
nix develop -c cargo test --all-targets --locked
nix develop -c cargo fmt --all -- --check
nix develop -c cargo clippy --all-targets --locked
nix develop -c cargo run -- --help
nix develop -c cargo run -- --disable-audio --view skirmish
```

Ostatnie polecenie wymaga interaktywnego terminala. `shellHook` sam wykonuje `cargo run --locked -- -h`, więc wejście do środowiska także kompiluje program i drukuje pomoc. Przy kontrolach używaj `--locked`, aby nie aktualizować `Cargo.lock`. Formatowanie stosuj wyłącznie do uzgodnionych zmian; nie używaj automatycznego `clippy --fix` do porządkowania całego repozytorium.

Stan bazowy po konfiguracji lintów: 45 testów i formatowanie przechodzą; Clippy zgłasza 50 ostrzeżeń dla biblioteki. `clippy::unused_unit` jest dozwolony ze względu na wymagane jawne `-> ()`. Zakres i polecenia opisano w [TESTING.md](TESTING.md). Nie traktuj tego jako potwierdzenia działania pełnej rozgrywki. Nowa nietrywialna logika wymaga małego testu regresji. Nie dodawaj zależności ani warstw abstrakcji bez potrzeby.

`Cargo.toml` wymaga Rust 1.88 lub nowszego, zabrania `unsafe` we własnym kodzie i ignorowania wyników `must_use`, a w release włącza kontrolę przepełnień. CI Gitea w `.gitea/workflows/rust.yml` sprawdza formatowanie, kompilację, testy debug/release i Clippy bez Nix, przez Rust 1.99.1 instalowany rustup; lokalnie nadal używaj Nix. Oddzielny `.gitea/workflows/nix.yml` buduje `.#default` z `default.nix`, z testami pakietu i kontrolą instalacji. Nie dodawaj `-D warnings` przed uporządkowaniem istniejących ostrzeżeń.

Audio wymaga ALSA i urządzenia odtwarzania; do pracy nad TUI używaj `--disable-audio`. `--mute` nadal uruchamia wątek audio. Ścieżki zasobów i pakowanie sprawdzaj osobno przy zmianach audio/Nix. Rozróżniaj lokalny build Cargo, pakiet `.#develop` i domyślny pakiet Nix.

## Konwencje kodu

- Formatowanie określa `rustfmt.toml`: styl Rust 2024, wcięcia po 2 spacje, szerokość docelowa 100 znaków i zakończenia LF. Stosuj `nix develop -c cargo fmt --all`; kontroluj przez `nix develop -c cargo fmt --all -- --check`.
- Jawne adnotacje typów są OBOWIĄZKOWE w nowym i zmienianym kodzie: przy zmiennych i stałych, polach struktur, parametrach funkcji i domknięć oraz typach zwracanych (także `-> ()` dla funkcji bez wyniku). Nie polegaj na inferencji typu tam, gdzie można podać typ jawnie. Dotyczy to również testów. Jest to zasada jawnego typowania; rzutowania wartości (`as`, `From`, `TryFrom`) stosuj tylko wtedy, gdy potrzebna jest rzeczywista konwersja.
- Rust 2024, jedna biblioteka i jeden binarny punkt wejścia; bez workspace.
- Wykorzystuj istniejące `App`, `GameStates`, `BoardState`, akcje i widgety.
- Współrzędne planszy to `(row, col)`, indeksowanie `cells[row][col]`; rozmiar mapy i rozmiar widocznego obszaru to różne wartości.
- `CellWidget` przechowuje także dane gry. Zmiana modelu komórki dotyka planszy, jednostek, struktur i panelu bocznego.
- Lista skrótów w widokach oraz tabela `KEYBINDINGS` muszą odpowiadać obsłudze akcji.
- `advance_turn` jest wywoływane przez tick potyczki. Zachowaj jedną aktualizację na jednostkę na tick, aktualność jej współrzędnych, kolejność zadań i ochronę zajętych pól. Regresje sprawdzaj przez `tests/digging.rs`.

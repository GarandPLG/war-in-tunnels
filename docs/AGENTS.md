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
nix develop -c cargo test --all-targets
nix develop -c cargo fmt --all -- --check
nix develop -c cargo clippy --all-targets
nix develop -c cargo run -- --help
nix develop -c cargo run -- --disable-audio --view skirmish
```

Ostatnie polecenie wymaga interaktywnego terminala. `shellHook` sam wykonuje `cargo run -- -h`, więc wejście do środowiska także kompiluje program i drukuje pomoc. Formatowanie stosuj wyłącznie do uzgodnionych zmian; nie używaj automatycznego `clippy --fix` do porządkowania całego repozytorium.

Stan bazowy po dodaniu zestawu testów: 23 testy i formatowanie przechodzą; Clippy zgłasza 51 ostrzeżeń dla biblioteki. Zakres i polecenia opisano w [TESTING.md](TESTING.md). Nie traktuj tego jako potwierdzenia działania pełnej rozgrywki. Nowa nietrywialna logika wymaga małego testu regresji. Nie dodawaj zależności ani warstw abstrakcji bez potrzeby.

Audio wymaga ALSA i urządzenia odtwarzania; do pracy nad TUI używaj `--disable-audio`. `--mute` nadal uruchamia wątek audio. Ścieżki zasobów i pakowanie sprawdzaj osobno przy zmianach audio/Nix. Rozróżniaj lokalny build Cargo, pakiet `.#develop` i domyślny pakiet Nix.

## Konwencje kodu

- Rust 2024, jedna biblioteka i jeden binarny punkt wejścia; bez workspace.
- Wykorzystuj istniejące `App`, `GameStates`, `BoardState`, akcje i widgety.
- Współrzędne planszy to `(row, col)`, indeksowanie `cells[row][col]`; rozmiar mapy i rozmiar widocznego obszaru to różne wartości.
- `CellWidget` przechowuje także dane gry. Zmiana modelu komórki dotyka planszy, jednostek, struktur i panelu bocznego.
- Lista skrótów w widokach oraz tabela `KEYBINDINGS` muszą odpowiadać obsłudze akcji.
- Nie włączaj `advance_turn` bez sprawdzenia opisanych w mapie problemów symulacji.

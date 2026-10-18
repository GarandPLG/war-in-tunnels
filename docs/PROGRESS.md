# Dziennik prac

## 2026-10-18 — przeniesienie testu audio

Przeniesiono `audio_repeats_after_end_of_file` z `src/app/threads/audio.rs` do `tests/audio.rs`, bez zmiany jego asercji. Udostępniono `load_audio` przez `app::threads`, aby test integracyjny nadal sprawdzał produkcyjną funkcję dekodowania. Zaktualizowano `docs/TESTING.md`.

Weryfikacja przez `nix develop -c`: `cargo check --all-targets`, `cargo test --all-targets` i `cargo fmt --all -- --check` zakończone sukcesem. Zestaw nadal liczy 23 testy. Nie powtarzano odsłuchu ani pomiarów CPU; zachowanie odtwarzania nie było zmieniane.

## 2026-10-18 — zestaw testów projektu

Dodano 22 testy integracyjne w `tests/cli.rs`, `tests/app.rs`, `tests/board.rs` i `tests/tasks.rs`. Razem z wcześniejszym testem audio zestaw liczy 23 przypadki. Zakres i polecenia zapisano w `docs/TESTING.md`, a mapę i instrukcje agentów zaktualizowano.

Testy obejmują walidację CLI i uruchomienie binarki bez TUI, menu i klawisze potyczki, przesyłanie komend audio, renderowanie ekranów przez TestBackend, generowanie planszy, granice kursora i kamery, zaznaczanie/cofanie/anulowanie ścieżki, kolejki i postęp zadań.

Najpierw uruchomiono testy na istniejącym kodzie. Wykryły trzy błędy, następnie poprawione w ich wspólnych implementacjach:

- `BoardState::new`: odejmowanie liczby widocznych wierszy od wysokości mapy powodowało panic przy dużym terminalu; użyto odejmowania nasycającego.
- `Offset`: konstruktor i settery pozwalały przekroczyć limit, także po resize; wartość jest teraz ograniczana do maksimum.
- `DiggingTask::progress`: obliczony postęp był odrzucany; jest teraz zapisywany z ograniczeniem do 0–100.

Nie włączano `advance_turn` ani nie zmieniano zasad symulacji. Zastane zmiany zależności w `Cargo.toml` i `Cargo.lock` oraz wcześniejsze zmiany audio pozostawiono; nie dodano zależności testowych.

Weryfikacja przez `nix develop -c`: `cargo check --all-targets`, `cargo test --all-targets` (23 zaliczone, 0 błędów), `cargo fmt --all -- --check` i `cargo clippy --all-targets` zakończone sukcesem. Clippy raportuje 51 ostrzeżeń dla biblioteki; nowe pliki testów nie zgłaszają ostrzeżeń. Nie sprawdzano urządzenia audio, pełnej symulacji ani pakowania Nix.

## 2026-10-18 — usunięcie aktywnej pętli audio

Przyczyna obciążenia: `handle_audio` bez oczekiwania wykonywał `rx.try_iter()` i `player.empty()`, także podczas normalnego odtwarzania bez komend.

Zmiany w `src/app/threads/audio.rs`:

- Zapętlanie przeniesiono do `Decoder::new_looped`; nie trzeba sprawdzać końca utworu ani ponownie kopiować danych z cache przy każdym powtórzeniu.
- Odbiór komend blokuje wątek przez `for cmd in rx`; zamknięcie kanału kończy wątek. Zachowano obsługę mute, głośności, zmiany części i soundtracku oraz dotychczasowy poziom wzmocnienia.
- Dodano test regresji odczytujący cały `calm.ogg` i kolejne 1024 próbki z zapętlonego źródła. Nie wymaga urządzenia audio.
- Zaktualizowano `docs/CODE_MAP.md`. Bez zmian zależności, CLI, zasobów i konfiguracji Nix.

Weryfikacja przez `nix develop -c`: `cargo check --all-targets`, `cargo test --all-targets` (1 test zaliczony), `cargo fmt --all -- --check` i `cargo clippy --all-targets` zakończone sukcesem. Clippy nadal raportuje 54 ostrzeżenia dla biblioteki.

Nie wykonano pomiaru CPU przed/po na urządzeniu docelowym ani odsłuchu i interaktywnego sprawdzenia komend. Usunięcie aktywnego odpytywania wynika z przepływu kodu; konkretnego procentowego spadku CPU nie zmierzono.

## 2026-10-18 — obowiązek poprawnej kompilacji

Dopisano do `docs/AGENTS.md` zakaz kończenia zadania z błędami kompilacji Rust oraz obowiązek wykonania `nix develop -c cargo check --all-targets` i usunięcia błędów przed zakończeniem. Zmiana dotyczy wyłącznie dokumentacji.

Weryfikacja: `nix develop -c cargo check --all-targets` zakończone sukcesem. Nie powtarzano testów działania gry przy zmianie samej dokumentacji.

## 2026-10-18 — przygotowanie kontekstu repozytorium

Cel: rozpoznać architekturę i przepływy oraz przygotować trwały kontekst do kolejnych zadań.

Wykonano:

- Przejrzano źródła Rust, manifest, konfigurację Nix, README, strukturę zasobów i szkiców UI.
- Przygotowano `docs/CODE_MAP.md`: odpowiedzialności modułów, diagramy uruchomienia i renderowania, przepływ klawiszy i zadań, audio, pakowanie oraz miejsca rozszerzeń.
- Przygotowano `docs/AGENTS.md`: punkt wejścia dla agentów, zasady pracy, polecenia i ograniczenia weryfikacji.
- Oddzielono działające ścieżki od placeholderów i niepodłączonej symulacji. Zapisano ryzyka w kodzie kopania do sprawdzenia przed jego aktywacją.
- Nie zmieniano źródeł gry ani konfiguracji. Zastana modyfikacja `flake.lock` należy do wcześniejszego stanu roboczego.

Weryfikacja przez środowisko Nix:

| Polecenie | Wynik |
|---|---|
| `nix develop -c cargo test --all-targets` | Sukces; oba targety wykonują 0 testów |
| `nix develop -c cargo fmt --all -- --check` | Sukces |
| `nix develop -c cargo clippy --all-targets` | Sukces z 54 ostrzeżeniami dla biblioteki, raportowanymi też dla targetu testowego |
| `cargo run -- -h` wykonywane automatycznie przez shellHook | Kompilacja i pomoc CLI działają |

Nie wykonano: interaktywnej rozgrywki, odtwarzania audio, budowy pakietów Nix ani napraw znalezionych problemów. Brak testów oznacza, że pomyślna kompilacja nie potwierdza poprawności mechanik. Kolejne zmiany należy dopisywać jako osobne wpisy z ich walidacją.

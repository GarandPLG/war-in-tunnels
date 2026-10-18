# Testy

Uruchomienie całego zestawu z katalogu głównego:

```sh
nix develop -c cargo test --all-targets
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
nix develop -c cargo check --all-targets
nix develop -c cargo fmt --all -- --check
nix develop -c cargo clippy --all-targets
```

`shellHook` środowiska Nix dodatkowo wykonuje `cargo run -- -h`. Wynik pomocy nie jest wynikiem testów; właściwy raport zawiera podsumowania `test result`.

## Ograniczenia

Zestaw weryfikuje wykonywanie zadań przez ticki potyczki, ale nie pełną rozgrywkę, ekonomię ani walkę. Nie potwierdza odsłuchu, zużycia CPU, obsługi urządzeń, pakowania Nix ani wszystkich układów małych terminali. Testy renderowania są kontrolą podstawową, bez utrwalania pełnych zrzutów ekranów. Brak konfiguracji CI; polecenia należy wykonywać lokalnie przed zakończeniem zmian.

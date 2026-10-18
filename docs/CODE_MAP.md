# Mapa kodu i przepływu

Migawka: 2026-10-18. Projekt `war_in_tunnels` 0.1.0 to prototyp terminalowego RTS. Działają menu, prezentacja planszy, nawigacja, zaznaczanie ścieżki i obsługa audio; pełna symulacja nie jest podłączona do zegara.

## Moduły i odpowiedzialność

Wszystkie ścieżki poniżej są względem katalogu głównego repozytorium.

| Obszar | Pliki | Odpowiedzialność |
|---|---|---|
| Start | `src/main.rs`, `src/lib.rs` | CLI, kanały, wątki, terminal, eksporty biblioteki |
| Parametry | `src/cli.rs`, `src/logs.rs` | Clap, walidacja zakresów, opcjonalny logger |
| Pętla aplikacji | `src/app/app.rs` | `App`, inicjalizacja stanów przy pierwszym rysowaniu, wejście, resize, update |
| Zbiór stanów | `src/app/state.rs`, `src/app/states/` | `GameStates`, menu, potyczka, ustawienia, perki, umiejętności |
| Routing widoku | `src/app/view.rs`, `src/app/views/` | `View`, implementacja `Widget for &mut App`, layouty i ekrany |
| Wejście | `src/app/keybind.rs`, `src/app/keybindings/` | Routing według widoku, `KEYBINDINGS`, `Action`, `Group`, mapowanie zdarzeń |
| Plansza | `src/app/states/skirmish_states/board.rs` | Tworzenie mapy, kamera, zoom, focus, zaznaczenia, zlecanie i szkic wykonywania zadań |
| Współrzędne | `focused_cell.rs`, `offset.rs`, `marked_cells.rs`, `zoom_level.rs` w `skirmish_states/` | Granice kursora i przewijania, ścieżka zaznaczenia, rozmiar komórek |
| Obiekty gry | `skirmish_states/structures/`, `units/`, `tasks/` | Traity `Structure`, `Unit`, `Task` i delegujące enumy `Structures`, `Units`, `Tasks` |
| Widgety | `src/app/widgets/` | Plansza, komórka, panel boczny i legenda klawiszy |
| Pomocniczy layout | `src/app/helpers/` | Obszar komórek, tytuły bloków i tekst opcji menu |
| Wątki | `src/app/threads/{events,tick,audio}.rs` | Crossterm, zegar 33 ms, Rodio i komendy audio |
| Środowisko | `Cargo.toml`, `flake.nix`, `default.nix`, `flake.lock` | Zależności, toolchain Fenix, ALSA, pakowanie przez Naersk lub buildRustPackage |
| Zasoby | `soundtracks/default/`, `ui_draft/` | Dwa pliki OGG i tekstowe szkice ekranów |

Ścieżki skrócone `skirmish_states/` oznaczają `src/app/states/skirmish_states/`. Nie ma osobnego silnika gry, bazy danych ani warstwy sieciowej. Zależności Cargo: Ratatui/Crossterm, Clap, Rodio, log/simplelog i once_cell.

## Uruchomienie i pętla

```mermaid
flowchart TD
    CLI[Cli::parse / get_args] --> MAIN[main]
    MAIN --> INPUT[Wątek handle_ct_events]
    MAIN --> TICK[Wątek handle_tick_event: 33 ms]
    MAIN --> AUDIO[Wątek handle_audio, jeśli włączony]
    MAIN --> INIT[ratatui::init / App::new]
    INIT --> LOOP[App::run]
    INPUT -->|kanały key i resize| LOOP
    TICK -->|kanał tick| LOOP
    LOOP --> DRAW[terminal.draw / App::draw]
    DRAW --> STATES[GameStates::new przy pierwszym rozmiarze okna]
    LOOP --> KEYS[handle_keybindings]
    KEYS -->|AudioCmd| AUDIO
    LOOP --> UPDATE[App::update / SkirmishState::tick_update]
    LOOP --> SLEEP[sleep 10 ms]
    SLEEP --> LOOP
    LOOP -->|exit lub błąd| RESTORE[ratatui::restore]
```

Kanały to standardowe `std::sync::mpsc`. Stan gry mutuje główny wątek. Każdy obrót pętli najpierw rysuje, następnie próbuje odebrać po jednym zdarzeniu z każdego kanału. Aktualizacja potyczki odbywa się niezależnie od wybranego ekranu. `tick_update` obecnie tylko zwiększa `turn_counter`; `self.board.advance_turn()` jest zakomentowane.

`GameStates::new` tworzy wszystkie stany jednocześnie. `SettingsState` przechowuje parametry z CLI; samo istnienie parametrów ekonomii, XP i trybu gry nie oznacza zaimplementowanych mechanik. `View` zawiera MainMenu, Skirmish, PerkDecks i SkillsConfig. Dwa ostatnie używają `default_view` z napisem „Work in progress”; ustawienia nie mają osobnego wariantu `View`.

## Renderowanie i dane

```mermaid
flowchart LR
    APP[App + GameStates] --> ROUTE[View::render]
    ROUTE --> MENU[main_menu_view]
    ROUTE --> SK[skirmish_view i layouty]
    ROUTE --> WIP[default_view]
    SK --> BOARD[BoardWidget]
    SK --> SIDE[SidePanelWidget]
    SK --> HELP[KeybindingsWidget]
    BOARD --> CELL[CellWidget]
    CELL --> STRUCT[Structures / Structure]
    CELL --> UNIT[Option Units / Unit]
    UNIT --> TASK[VecDeque Tasks / Task]
```

`BoardState.cells: Vec<Vec<CellWidget>>` łączy model i prezentację: komórka zawiera strukturę, opcjonalną jednostkę, pozycję i flagi UI. `BoardWidget` wybiera widoczne komórki przez offsety i layout; `CellWidget` odpowiada za ich tekst i styl. Panel boczny prezentuje dane zaznaczenia. Zoom wpływa na wielkość komórek i liczbę widocznych wierszy/kolumn; resize i przełączanie panelu zmieniają granice offsetów.

Mapa powstaje deterministycznie: kamień, baza gracza w `(map_height / 2, 2)`, baza przeciwnika w `(map_height / 2, map_width - 3)`, sąsiadujące złoża i po jednym górniku w bazie. CLI domyślnie wybiera mapę 42 × 11; dopuszczalne zakresy to szerokość 36–108 i wysokość 11–39. Dostępne struktury to Base, Ore, Stone, Tunnel; jedyna jednostka to Miner, jedyne zadanie to Digging.

## Klawisze i zlecanie kopania

1. `handle_ct_events` odbiera zdarzenie Crossterm i przekazuje je kanałem do `App`.
2. `handle_keybindings` wybiera obsługę menu, potyczki albo domyślną.
3. `event_to_action` porównuje kod, rodzaj i modyfikatory z `KEYBINDINGS`, następnie obsługuje znak wildcard.
4. Akcje wspólne realizują wyjście, powrót do menu i komendy audio.
5. W potyczce ruch zmienia focus i ewentualnie kamerę; zoom przelicza rozmiary; Tab przełącza panel.
6. Spacja przełącza zaznaczanie. `start_marking_cells` zapamiętuje jednostkę z komórki początkowej; ruch dopisuje współrzędne do kolejki. Backspace cofa ścieżkę, Delete ją anuluje.
7. `end_marking_cells(false)` pomija pierwsze pole, tworzy `DiggingTask` z pozostałej ścieżki i siły kopania oraz przekazuje zadanie jednostce.

Projektowane wykonanie: `advance_turn` skanuje komórki, wyjmuje jednostkę przez `take()`, obsługuje pierwsze zadanie i przenosi jednostkę na cel, zastępując strukturę tunelem. Ten przepływ istnieje w kodzie, ale nie jest obecnie wywoływany przez tick.

## Audio i pakowanie

Główny wątek wysyła `AudioCmd`; osobny wątek obsługuje głośność, mute, część i soundtrack. Cache wykorzystuje `Lazy`, `RwLock`, `Arc` i bajty OGG. Calm ładuje `calm.ogg`; Buildup, Assault i Outro korzystają obecnie z `test.ogg`. Błędy urządzenia, odczytu i dekodowania mogą zakończyć wątek przez `expect`.

Rodio zapętla źródło przez `Decoder::new_looped`. Wątek sterujący blokuje się na odbiorniku (`for cmd in rx`), zamiast aktywnie sprawdzać komendy i koniec utworu. Zmiana części lub soundtracku dopisuje zapętlone źródło i pomija bieżące przez `skip_one`. Zamknięcie kanału kończy wątek. Test `audio_repeats_after_end_of_file` sprawdza przekroczenie końca rzeczywistego pliku OGG bez urządzenia audio.

`flake.nix` dostarcza devShell dla `x86_64-linux`, domyślny pakiet z `default.nix` i pakiet `.#develop` budowany przez Naersk. Obie ścieżki pakowania kopiują soundtracki do `$out/bin` i zmieniają nazwę binarki na `war-in-tunnels`. `default.nix` ma stały `cargoHash`; po zmianie zależności wymaga osobnej weryfikacji. Samo `cargo test` nie sprawdza pakowania.

## Punkty wymagające uwagi przed dalszą implementacją

To obserwacje statyczne, nie lista napraw wykonanych w tej sesji:

- **Symulacja wyłączona:** `SkirmishState::tick_update` nie wywołuje `advance_turn`.
- **Postęp zadania:** naprawiono zapisywanie przyrostu i ograniczenie do 0–100; zachowanie jest objęte testami w `tests/tasks.rs`. Nie oznacza to uruchomienia symulacji.
- **Ruch w szkicu symulacji:** `advance_turn` pobiera cel bez warunku osiągnięcia postępu, nie aktualizuje współrzędnych jednostki i nie usuwa zakończonego zadania z kolejki. Jednostka przeniesiona na późniejszą komórkę może zostać obsłużona ponownie w tym samym skanowaniu. Zapis celu może nadpisać inną jednostkę i strukturę. Przed aktywacją potrzebne są reguły kolizji i testy pojedynczej tury.
- **Zaznaczenie:** jednostka startowa jest zapamiętywana oddzielnie; przy uruchomieniu ruchu trzeba sprawdzić ważność jej pozycji i zachowanie dla pustej komórki oraz jednostki przeciwnika.
- **Audio:** usunięto aktywne odpytywanie; pomiar CPU na docelowym urządzeniu i odsłuch pozostają do wykonania. Test dekodera nie weryfikuje urządzenia audio.
- **Granice UI:** małe terminale, resize i trzy poziomy zoomu wymagają interaktywnego sprawdzenia. Indeksowanie planszy zakłada poprawne współrzędne.
- **Pokrycie:** 23 testy CLI, sterowania, renderowania, planszy, zadań i dekodowania audio; szczegóły w [TESTING.md](TESTING.md). Brak testów pełnej symulacji i konfiguracji CI. Clippy zgłasza 51 ostrzeżeń; nie uruchamiać automatycznych poprawek poza zakresem zadania.

## Gdzie zacząć konkretną zmianę

| Zadanie | Główne miejsca i powiązania |
|---|---|
| Nowa akcja | `keybindings/keybindings.rs` → handler widoku → lista akcji w `views/` |
| Mechanika kopania | `tasks/digging.rs` → `units/` → `board.rs::advance_turn` → `skirmish.rs::tick_update` |
| Nowy obiekt | Trait i enum w `structures/` lub `units/`, eksporty `mod.rs`, tworzenie mapy, komórka i panel |
| Kamera i zoom | `board.rs`, `focused_cell.rs`, `offset.rs`, `zoom_level.rs`, `helpers/cells_area.rs`, layout i widget planszy |
| Nowy ekran | `View`, `GameStates`, `states/`, `views/`, routing klawiszy i menu |
| Parametr CLI | `cli.rs`, walidacja, `GameStates::new`, faktyczny konsument i README |
| Audio lub dystrybucja | `threads/audio.rs`, `main.rs`, zasoby, `flake.nix`, `default.nix` |

# War in Tunnels

**War in Tunnels** – to terminalowa gra typu RTS (real‑time strategy) rozgrywająca się pod ziemią. Gra wykorzystuje bibliotekę **ratatui** do budowy interfejsu tekstowego oraz **rodio** do odtwarzania ścieżek dźwiękowych. Projekt jest skonfigurowany jako **Nix Flake**, co ułatwia zarządzanie zależnościami, budowanie wersji produkcyjnej oraz deweloperskiej.

## Funkcjonalności

- Tekstowy interfejs użytkownika z podziałem na menu, podgląd mapy i panel boczny.
- Różne tryby gry (np. Last‑Man‑Standing).
- Skalowanie mapy i poziom przybliżenia UI.
- Wsparcie dla kilku zestawów perków.
- Konfigurowalne zasoby startowe (drewno, żelazo) oraz limit jednostek.
- System punktów doświadczenia i limitu punktów umiejętności.
- Odtwarzanie ścieżek dźwiękowych (możliwość wyciszenia lub wyłączenia).

## Wymagania

- **Nix** (z obsługą Flake) – do budowania i uruchamiania projektu.
- **Rust ≥ 1.88**, edycja 2024 – używany wewnątrz środowiska Nix (fenix toolchain).
- Biblioteki systemowe: `alsa-lib` (dla obsługi dźwięku).

## Budowanie i uruchamianie

Projekt udostępnia dwa profile Flake:

- **default** – wersja produkcyjna (z optymalizacjami).
- **develop** – wersja deweloperska z dodatkowymi zależnościami i nie‑zoptymalizowanym kodem.

### Przykładowe polecenia

```bash
# Budowanie wersji produkcyjnej
nix build

# Uruchomienie wersji produkcyjnej
nix run

# Budowanie wersji deweloperskiej
nix build .#develop

# Uruchomienie wersji deweloperskiej
nix run .#develop
```

W środowisku deweloperskim dostępny jest także **shell** z pre‑konfigurowanymi zmiennymi:

```bash
nix develop
```

Po wejściu do powłoki pojawi się informacja o wersji oraz dostępne komendy.

## Opcje wiersza poleceń (CLI)

Gra korzysta z biblioteki **clap** – wszystkie opcje są dostępne jako flagi. Poniżej najważniejsze:

| Skrót | Flaga | Domyślna wartość | Opis |
|-------|-------|------------------|------|
| `-v` | `--view` | `MainMenu` | Określa początkowy widok UI. |
| `-u` | `--username` | `Player` | Nazwa gracza wyświetlana w interfejsie. |
| `-g` | `--game-mode` | `LastManStanding` | Tryb rozgrywki. |
| `-W` | `--map-width` | `42` | Szerokość generowanej mapy (36‑108). |
| `-H` | `--map-height` | `11` | Wysokość mapy (11‑39). |
| `-z` | `--zoom-level` | `Default` | Poziom przybliżenia UI. |
| `-p` | `--perk-deck` | `Silesian` | Zestaw perków. |
| `-w` | `--starting-wood` | `50` | Początkowa ilość drewna (1‑150). |
| `-i` | `--starting-metal` | `25` | Początkowa ilość metalu (1‑150); stara nazwa `--starting-iron` nadal działa jako alias. |
| `-L` | `--supply-limit` | `99` | Limit jednostek na mapie (49‑149). |
| `-x` | `--xp-modifier` | `1.0` | Mnożnik doświadczenia (0.5‑2.0). |
| `-P` | `--skill-points-limit` | `120` | Górny limit punktów umiejętności (90‑690). |
| `-S` | `--soundtrack` | `Default` | Wybór ścieżki dźwiękowej. |
| `-D` | `--disable-audio` | `false` | Wyłącza odtwarzanie dźwięku. |
| `-m` | `--mute` | `false` | Wycisza odtwarzany dźwięk. |
| `-l` | `--log` | `false` | Włącza logowanie do pliku. |

Aby wyświetlić pomoc, uruchom:

```bash
cargo run -- -h
```

## Struktura projektu

- `src/main.rs` – punkt wejścia, inicjalizuje terminal, wątki i główną pętlę aplikacji.
- `src/lib.rs` – eksportuje moduły aplikacji i CLI.
- `src/app/` – logika gry, stany, widoki i obsługa klawiszy.
- `src/cli.rs` – definicja opcji CLI.
- `Cargo.toml` – zależności Rust (ratatui, clap, log, simplelog, rodio, once_cell).
- `flake.nix` – opis flake, zależności Nix, profile build i develop.

## Kopanie

W potyczce gracz zaczyna z jednym górnikiem `Miner` w swojej bazie. Ustaw kursor na jednostce (z początkowej pozycji wystarczy strzałka w lewo), naciśnij spację, wytycz ścieżkę strzałkami i zatwierdź spacją. Backspace cofa ostatni krok, Delete lub Esc anuluje rysowanie.

Górnik wykonuje zadania kolejno. Kamień stopniowo traci durability; po wykopaniu zmienia się w tunel, do którego jednostka wchodzi w tym samym ticku. Pełna kratka kamienia zajmuje około 6 sekund. Procent pozostałej wytrzymałości widać na uszkodzonej kratce; Tab otwiera panel z dokładną wartością dla pola pod kursorem. Przejście do istniejącego tunelu lub niezajętej bazy trwa około 3 sekund na kratkę. Znacznik jednostki ma kolor właściciela: niebieski dla gracza, czerwony dla przeciwnika.

Podczas przechodzenia na kratce jednostki pojawia się procent ukończenia bieżącego kroku, np. `50%`. Wskaźnik resetuje się po przejściu i znika, gdy jednostka nie podróżuje. Jest widoczny na wszystkich poziomach zoomu, w kolorze właściciela jednostki.

Zatwierdzona ścieżka dzieli się automatycznie na zadania `Moving` (`→`, gotowe tunele i bazy) oraz `Digging` (`⛏`, kamień). Jednostka wykonuje je kolejno; po wykopaniu kamienia wejście do nowego tunelu pozostaje częścią kopania.

W kolejce pod Tab każde zadanie pokazuje `Step` (postęp bieżącej kratki) oraz `Total` (postęp całego zadania według liczby kratek, z uwzględnieniem częściowo wykonanego etapu). Zadania oczekujące mają oba wskaźniki na 0%.

Możesz zaznaczyć pracującego górnika ponownie: rysowanie kolejnego zadania zacznie się na końcu jego dotychczasowej kolejki. Podczas rysowania wybrana jednostka wstrzymuje pracę. Zajęte pole lub złoże rudy zatrzymuje wykonywanie ścieżki; ten typ zadania kopie tylko kamień.

## Zasoby i budowanie

Wood i Metal mają rzeczywisty stan w magazynie każdego gracza, inicjalizowany z `--starting-wood` i `--starting-metal`. Nagłówek pokazuje bieżące zasoby. Gracz otrzymuje 5 Wood co około 5 sekund symulacji; metal nie przyrasta bez kopalni. Złoża Ore są początkowo neutralne.

Ustaw kursor na neutralnym Ore i otwórz widok budowania przez `Ctrl+Space`. Jeśli na jednej z czterech sąsiednich kratek stoi własny Miner z uprawnieniem budowania i masz 20 Wood oraz 10 Metal, Enter rozpoczyna budowę Mine. Koszt jest pobierany od razu, a budowa trwa około 10 sekund. Panel pozostaje otwarty i pokazuje postęp; procent widać też na kratce. Po odejściu budowniczego budowa zatrzymuje się do jego powrotu. Nie można budować zdalnie na rudzie bez sąsiedniego własnego budowniczego.

Delete w panelu budowy anuluje trwającą budowę, przywraca pierwotne neutralne Ore i zwraca cały koszt. Ctrl+Space lub Esc jedynie zamyka panel — budowa nadal postępuje, jeżeli budowniczy jest obecny. Niedokończona kopalnia nie produkuje metalu i nie przyjmuje jednostek. Na początkowej mapie złoże obok bazy spełnia warunek sąsiedztwa; kopalnie nie pojawiają się automatycznie.

Mine wydobywa 1 Metal co około sekundę, aż do wyczerpania złoża. Możesz wysłać do własnej kopalni górnika zwykłą trasą zatwierdzaną spacją. Wejście trwa około 3 sekund jako Moving; górnik pozostaje na kratce kopalni i zwiększa wydobycie do 2 Metal/s. Po jego wyjściu bonus znika. W kopalni mieści się jedna jednostka; obcy górnicy nie mogą do niej wejść. Panel Tab pokazuje aktualne tempo wydobycia.

Wspólna reguła zwykłych budynków wymaga pustego kamienia sąsiadującego z tunelem lub własną bazą oraz sąsiedniego własnego budowniczego. Mine jest wyjątkiem: wymaga rudy zamiast kamienia i nie potrzebuje połączenia z tunelem/bazą. Innych typów budynków nie można jeszcze stawiać.

Legenda klawiszy zależy od trybu: na mapie pokazuje nawigację i dostępne zaznaczenie własnego górnika; podczas rysowania trasy — ruch, zatwierdzanie i anulowanie; w widoku budowania — rozpoczęcie lub anulowanie budowy; w rekrutacji — dodawanie górników do kolejki. Globalne skróty wyjścia i muzyki nadal działają. W zwykłym trybie Esc wraca do menu.

## Rekrutacja

Ustaw kursor na własnym budynku Base i naciśnij `Alt+Space` lub `r`. Jeśli terminal albo menedżer okien przechwytuje Alt+Space, użyj `r`. Enter dodaje jednego Minera do kolejki za **10 Wood i 15 Metal**, pobierane od razu. Każda rekrutacja trwa około **10 sekund**. Panel pokazuje liczbę zamówionych górników (włącznie z aktualnie rekrutowanym), procent bieżącej rekrutacji i powód oczekiwania. Esc, Alt+Space lub `r` zamyka panel bez kasowania kolejki. Liczbę i postęp widać też pod Tab.

Pierwszy górnik może zostać wyszkolony do 100%, nawet jeśli baza jest zajęta. Gotowy czeka, aż kratka będzie wolna, i pojawia się w bazie bez nadpisywania stojącej tam jednostki. Następna rekrutacja pozostaje na 0%, dopóki poprzedni zrekrutowany górnik nie opuści bazy. Wyprowadź go zwykłą trasą, aby wznowić kolejkę. Każda baza ma niezależną kolejkę; nowe jednostki otrzymują właściciela bazy i pełne uprawnienia Minera.

## Dźwięk

Ścieżki dźwiękowe znajdują się w folderze `soundtracks`. Biblioteka **rodio** odtwarza je w oddzielnym wątku. Dostępne flagi pozwalają wybrać konkretną ścieżkę, wyciszyć lub całkowicie wyłączyć dźwięk.

## Logowanie

Gdy flaga `--log` jest włączona, aplikacja zapisuje logi do pliku przy użyciu **simplelog**. Domyślnie logowanie jest wyłączone.

## Kontrole jakości

`Cargo.toml` zabrania `unsafe` we własnym kodzie i ignorowania wyników `must_use`, dopuszcza jawne `-> ()` oraz włącza kontrolę przepełnień także w release. Gitea Actions w `.gitea/workflows/rust.yml` sprawdza formatowanie, kompilację, testy debug/release i Clippy przez Nix, z zachowaniem plików lock. Uruchomienie CI wymaga włączonych Actions i runnera `ubuntu-latest`.

Polecenia lokalne i szczegóły środowiska opisano w [docs/TESTING.md](docs/TESTING.md).

## Licencja

Projekt jest udostępniany na licencji [MIT](LICENSE).
Copyright (c) 2026 GarandPLG.

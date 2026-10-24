# Praca nad War in Tunnels

Instrukcje i notatki dla agentów przechowujemy w `docs/`. Ten plik jest punktem wejścia do kontekstu projektu; ponieważ znajduje się w `docs/`, agent rozpoczynający pracę w katalogu głównym powinien otworzyć go jawnie.

## Przed zmianą

1. Przeczytaj [mapę kodu](CODE_MAP.md) i [dziennik prac](PROGRESS.md), następnie aktualny kod zmienianej funkcji i jej wywołania (`rg`). Dokumentacja jest migawką, kod jest źródłem prawdy.
2. Sprawdź `git status --short`. Nie nadpisuj zastanych zmian użytkownika. Przy rozpoczęciu tej analizy zmodyfikowany był `flake.lock`.
3. Ustal zasięg: stan, obsługa wejścia, renderowanie, CLI, eksporty `mod.rs`, testy i konfiguracja. Zmieniaj tylko elementy potrzebne do zadania.
4. Aktualizuj `docs/PROGRESS.md`: cel, konkretne zmiany, wykonane kontrole i ograniczenia. Przy zmianie architektury aktualizuj też mapę.

## Narzędzia i weryfikacja

Flake uruchamiany z repozytorium Git pomija pliki nieśledzone, nawet gdy Cargo widzi je lokalnie. Przy błędach Nix „file not found for module” sprawdź git status i git ls-files. Nowe źródła i testy można uwidocznić przez git add -N -- <pliki> (intent-to-add, bez dodawania treści do stagingu ani tworzenia commita). Sama zmiana src = ./. w default.nix nie przywróci plików pominiętych na wejściu flake.

Nie wolno zakończyć zadania, jeżeli kompilator Rust zgłasza błędy uniemożliwiające kompilację. Przed zakończeniem uruchom `nix develop -c cargo check --all-targets`; jeśli wystąpią błędy kompilacji, usuń je i powtórz kontrolę aż do uzyskania sukcesu.

Wszystkie narzędzia Rust uruchamiaj przez `nix develop -c` z katalogu repozytorium:

```sh
nix develop -c cargo test --all-targets --locked
nix develop -c cargo fmt --all -- --check
nix develop -c cargo clippy --all-targets --locked
nix develop -c cargo run -- --help
nix develop -c cargo run -- --disable-audio --view skirmish
```

Ostatnie polecenie wymaga interaktywnego terminala. `shellHook` drukuje dostępne polecenia, bez automatycznej kompilacji. Przy kontrolach używaj `--locked`, aby nie aktualizować `Cargo.lock`. Formatowanie stosuj wyłącznie do uzgodnionych zmian; nie używaj automatycznego `clippy --fix` do porządkowania całego repozytorium.

Aktualny zestaw obejmuje 106 testów. Clippy ma istniejące ostrzeżenia dla biblioteki. `clippy::unused_unit` jest dozwolony ze względu na wymagane jawne `-> ()`. Zakres i polecenia opisano w [TESTING.md](TESTING.md). Nie traktuj tego jako potwierdzenia działania pełnej rozgrywki. Nowa nietrywialna logika wymaga małego testu regresji. Nie dodawaj zależności ani warstw abstrakcji bez potrzeby.

`Cargo.toml` wymaga Rust 1.88 lub nowszego, zabrania `unsafe` we własnym kodzie i ignorowania wyników `must_use`, a w release włącza kontrolę przepełnień. CI Gitea to jeden workflow `Project tests` w `.gitea/workflows/tests.yml`, uruchamiany po pushu i dla pull requestów. Zależności `needs` ustalają kolejność `check → nix → linux → aur`; błąd joba pomija zależne joby. `check` sprawdza formatowanie, kompilację, testy debug/release i Clippy bez Nix, przez Rust 1.99.1 instalowany rustup; lokalnie nadal używaj Nix. Job `nix` buduje `.#default` z `default.nix`, z testami pakietu i kontrolą instalacji. Nie dodawaj `-D warnings` przed uporządkowaniem istniejących ostrzeżeń.

Audio wymaga ALSA i urządzenia odtwarzania; do pracy nad TUI używaj `--disable-audio`. `--mute` nadal uruchamia wątek audio. Ścieżki zasobów i pakowanie sprawdzaj osobno przy zmianach audio/Nix. Rozróżniaj lokalny build Cargo, pakiet `.#develop` i domyślny pakiet Nix.

Joby `linux` i `aur` w `.gitea/workflows/tests.yml` testują paczki Linux x86_64 bez Nix: deb, rpm, AppImage, Flatpak oraz przepis AUR budowany przez makepkg. Nie publikują wydań ani tagów. Przepisy, layout i ograniczenia opisano w [PACKAGING.md](PACKAGING.md); zmiany pakowania muszą zachować muzykę obok binarki i działanie poza katalogiem repozytorium.

## Konwencje kodu

Mechanikę tuneli sprawdzaj przez tests/tunnel_stress.rs. Pełna symulacja przechodzi przez SkirmishState::tick_update, który rozlicza Excavation, środki na automatyczne podpory i śmiertelne kaskady. Ręczne podpory zmieniaj przez build_support/remove_support; surowa flaga Tunnel::set_support jest API modelu i sama nie rozlicza zasobów ani zawaleń. Digging/Moving muszą zachować auto_supports przy konwersji; jawnie wybrany Moving (dig_if_blocked=false) nie może zacząć kopać. Render stresu nie może nadpisywać modelu komórki.

- Formatowanie określa `rustfmt.toml`: styl Rust 2024, wcięcia po 2 spacje, szerokość docelowa 100 znaków i zakończenia LF. Stosuj `nix develop -c cargo fmt --all`; kontroluj przez `nix develop -c cargo fmt --all -- --check`.
- Jawne adnotacje typów są OBOWIĄZKOWE w nowym i zmienianym kodzie: przy zmiennych i stałych, polach struktur, parametrach funkcji i domknięć oraz typach zwracanych (także `-> ()` dla funkcji bez wyniku). Nie polegaj na inferencji typu tam, gdzie można podać typ jawnie. Dotyczy to również testów. Jest to zasada jawnego typowania; rzutowania wartości (`as`, `From`, `TryFrom`) stosuj tylko wtedy, gdy potrzebna jest rzeczywista konwersja.
- Rust 2024, jedna biblioteka i jeden binarny punkt wejścia; bez workspace.
- Wykorzystuj istniejące `App`, `GameStates`, `BoardState`, akcje i widgety.
- Współrzędne planszy to `(row, col)`, indeksowanie `cells[row][col]`; rozmiar mapy i rozmiar widocznego obszaru to różne wartości.
- `CellWidget` przechowuje także dane gry. Zmiana modelu komórki dotyka planszy, jednostek, struktur i panelu bocznego.
- Lista skrótów w widokach oraz tabela `KEYBINDINGS` muszą odpowiadać obsłudze akcji.
- Konteksty mapy, trasy i budowania określa `available_skirmish_actions`. Globalne wyjście i muzyka działają niezależnie od legendy. b otwiera budowanie dla wskazanego pola; Enter wymaga poprawnego miejsca, sąsiedniego własnego budowniczego i środków. Delete anuluje własną budowę i zwraca koszt, b tylko zamyka panel.
- Bieżące zasoby są w SkirmishState, nie w ustawieniach CLI. Gameplay rozpoczyna budowę przez SkirmishState::start_mine_construction, aby objąć ją opłatą; BoardState::can_build_at/start_mine_construction obsługują samą walidację terenu i zmianę struktury. MineConstruction zachowuje oryginalne Ore i postępuje tylko przy sąsiednim własnym budowniczym. Mine zachowuje rezerwę Ore i kredytuje wyłącznie właściciela. Unit::get_can_build deleguje uprawnienia przez Units i Option. Ruch do gotowej własnej Mine korzysta ze Structures::can_enter i pozostaje Moving. Górnik na kratce kopalni podwaja wydobycie; nie twórz odrębnej kopii obsadzonej jednostki. Zmiany sprawdzaj przez tests/resources.rs, tests/context_keybindings.rs i tests/construction.rs.
- Rekrutację opłacaj przez SkirmishState::recruit_miner, nie bezpośrednie BaseBuilding::queue_miner. r otwiera panel tylko przy własnej bazie i poza innym trybem modalnym. Pierwsza rekrutacja może dojść do 100% na zajętej bazie, ale nie może nadpisać jednostki; kolejne stoją na 0% do wyjścia poprzedniego rekruta. BaseBuilding zachowuje licznik, postęp i waiting_for_departure; BoardState uruchamia spawn po ruchu istniejących jednostek. Sprawdzaj tests/recruitment.rs.
- Tick potyczki najpierw rozlicza 3-sekundowe prace podpór i uruchamia automat przed niebezpiecznym wykopem, następnie używa advance_turn_with_events i rozstrzyga zawalenia; advance_turn to sam krok jednostek. Zachowaj jedną aktualizację na jednostkę na tick, aktualność jej współrzędnych, kolejność zadań i ochronę zajętych pól. Regresje sprawdzaj przez `tests/digging.rs`.

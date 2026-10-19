# Dziennik prac

## 2026-10-19 — Node.js wymagany przez akcje joba AUR

Log Gitea wykazał brak `node` podczas uruchamiania checkout@v4 i jego kroku post. Arch base-devel nie zawiera Node.js, a runner 0.6.1 wywołuje akcje JavaScript przez `node` w kontenerze. Dodano `nodejs` do instalacji zależności przed checkoutem oraz kontrolę `node --version`. Node.js jest zależnością CI, nie paczki gry. Poprzednia próba lokalna weryfikowała makepkg, lecz nie uruchamiała akcji JavaScript; uzupełniono ten brak.

Weryfikacja w jednorazowym Arch base-devel: zainstalowano Node.js, `node --version` zwróciło v26.12.0; sprawdzono `node --check` rzeczywistych entrypointów checkout@v4 i upload-artifact@v3. Uruchomienie checkoutu doszło do oczekiwanego błędu brakującego inputu token (bez poświadczeń i klonowania repozytorium), potwierdzając poprawny start akcji. `nix develop -c cargo check --all-targets --locked` i `git diff --check` przeszły. Nie powtarzano budowania gry ani nie uruchamiano pełnego workflow na serwerze; transport artefaktów pozostaje do potwierdzenia na Gitea.

## 2026-10-19 — testowe paczki Linux poza Nix

Dodano `.gitea/workflows/packages.yml` dla push i pull request: job Debian kompiluje Rust 1.99.1 i tworzy deb, rpm, AppImage oraz Flatpak; job Arch kompiluje i testuje lokalny przepis AUR przez makepkg jako użytkownik builder. Workflow zapisuje artefakty przez 7 dni, nie tworzy wydań, tagów ani publikacji AUR/Flathub. Zależności i toolchain Cargo mają cache; Flatpak i Arch nie mają jeszcze trwałego cache. Obsługiwana architektura: x86_64.

Przepisy i wspólna instalacja są w `packaging/`, a zakres i ograniczenia w `docs/PACKAGING.md`. Pakowane są muzyka, licencja, launcher zachowujący katalog roboczy, plik desktop Terminal=true i ikona SVG. Flatpak pakuje binarkę z Debiana do runtime 25.08, bez kompilacji w SDK i bez zagnieżdżonego sandboxa. Przepis AUR dostaje archiwum checkoutu z rzeczywistym SHA256; nie jest jeszcze przepisem publikowanego wydania. Dodano `dist/` do .gitignore.

Audio szuka zasobów obok rzeczywistej binarki z fallbackiem do katalogu roboczego, aby instalowane paczki działały poza repozytorium bez zmiany miejsca zapisu logów. Dodano regresję uruchamianą w osobnym procesie z przeniesioną binarką testu i soundtrackami. Łącznie 46 testów integracyjnych.

Weryfikacja: cargo check --all-targets --locked, cargo test --all-targets --locked (46 testów), cargo fmt --all -- --check i cargo clippy --all-targets --locked przez nix develop przeszły; Clippy nadal ma 50 ostrzeżeń biblioteki. Przeszły git diff --check i bash -n. W jednorazowym Debianie rzeczywiście skompilowano grę, utworzono deb i rpm, rozpakowano i sprawdzono --version; zbudowano też AppImage, rozpakowano je i uruchomiono AppRun --version. Narzędzie i wynik AppImage rozpakowywane są przez 7z, bez FUSE i kolizji z binfmt hosta. Osobny Arch Linux wykonał makepkg, testy release i doctest, instalację paczki oraz --version. Flatpak został wyeksportowany, zapisany do pliku i zainstalowany w kontenerze; zweryfikowano soundtrack i --version zainstalowanej binarki. Wyłączono sandbox wyłącznie walidatora własnej ikony przy build-export; instalacja może zgłaszać ostrzeżenia bwrap dla hooków integracji pulpitu w standardowym Dockerze. Nie wyłączono sandboxa gotowej aplikacji. Instalacja runtime i aplikacji używa --no-related, aby nie pobierać opcjonalnych sterowników GPU ani tłumaczeń dla testu TUI.

Nie sprawdzono pełnej rozgrywki, odsłuchu, uruchamiania z menu pulpitu ani działania na starszych dystrybucjach; pełny workflow i transport artefaktów wymagają przebiegu na Gitea. Testowe kontenery miały repozytorium tylko do odczytu; nie instalowano paczek na hoście. Usunięto tymczasowy kontener war-packages-check i obraz war-in-tunnels-packaging-test:local.

## 2026-10-19 — oddzielny workflow budowania default.nix

Dodano `.gitea/workflows/nix.yml` dla push i pull request. Workflow w `node:22-bookworm` instaluje sudo i Nix (KVM wyłączone), następnie buduje `.#default` z przypiętym `flake.lock`, bez aktualizacji lockfile i bez devShell. Obejmuje testy buildRustPackage oraz versionCheckHook. Nie zmieniono workflow Rust ani konfiguracji runnera; nie dodano trwałego cache Nix.

Próbna budowa ujawniła nieaktualny cargoHash w `default.nix`; poprawiono go na hash rzeczywiście pobranych zależności. Ponowne `nix build .#default --no-update-lock-file --no-link --print-out-paths --print-build-logs` zakończyło się sukcesem. Lokalna kontrola `nix develop -c cargo check --all-targets --locked` również przeszła. Zaktualizowano mapę, instrukcje i opis CI. Zachowano zastaną zmianę użytkownika w `flake.nix`. Nowy workflow wymaga jeszcze uruchomienia na Gitea.

## 2026-10-19 — CI Rust bez Nix

Na życzenie użytkownika usunięto instalację Nix i wywołania `nix develop` z workflow. W kontenerze `node:22-bookworm` rustup instaluje Rust 1.99.1 z rustfmt i Clippy. Dodano zależności systemowe ALSA, pkg-config i kompilator C oraz dwa cache `actions/cache@v4`: toolchain i Cargo (registry, git, target). Klucze Cargo uwzględniają wersję Rust, Cargo.lock, manifest i źródła; częściowe trafienia pozwalają wykorzystać wcześniejsze artefakty. Zachowano wszystkie pięć kontroli i `--locked`. Lokalne środowisko Nix pozostaje bez zmian. Zaktualizowano mapę i instrukcje.

Weryfikacja: lokalne `nix develop -c cargo check --all-targets --locked`, `nix develop -c cargo fmt --all -- --check` i `git diff --check` zakończyły się sukcesem. Pełne wykonanie nowego workflow i skuteczność cache wymagają sprawdzenia na Gitea; lokalna kontrola nie potwierdza instalacji rustup ani sieciowego dostępu do cache serwera.

## 2026-10-19 — naprawa instalacji Nix w kontenerze Gitea Actions

Log runnera 0.6.1 wskazał `sudo: command not found` w `cachix/install-nix-action@v31`. Etykieta `ubuntu-latest` była mapowana na `node:16-bullseye`, który nie zawiera sudo. Lokalny test odtworzenia wykazał dodatkowo błąd 404 podczas instalacji sudo z repozytorium starego obrazu.

W `.gitea/workflows/rust.yml` wskazano jawnie kontener `node:22-bookworm`, dodano przed instalacją Nix krok `apt-get update` i `apt-get install -y --no-install-recommends sudo` oraz ustawiono `enable_kvm: false`. Nie zmieniano konfiguracji runnera na serwerze ani nie dodawano trybu privileged.

Weryfikacja: uruchomiono jednorazowy kontener `node:22-bookworm`, zainstalowano sudo i wykonano skrypt `install-nix.sh` z v31 z wyłączonym KVM. Instalacja zakończyła się sukcesem, `nix --version` zwróciło 2.35.3, a `nix eval --expr "1 + 1"` zwróciło 2. Instalator ostrzega o pracy jako root; w tym izolowanym kontenerze instalacja i próbne polecenia przeszły. Nix automatycznie użył sandbox-fallback z powodu niedostępności namespaces; nie zmieniano ustawienia sandbox w workflow.

`nix develop --no-update-lock-file -c cargo check --all-targets --locked` zakończone sukcesem. Nie powtarzano testów gry przy zmianie samego workflow. Zaktualizowano opis CI i mapę. Pełny przebieg Actions wymaga ponownego pushu i wykonania na serwerze; lokalny test obejmował instalację Nix i podstawowe wywołanie, nie cały workflow.

## 2026-10-19 — ujednolicenie licencji na MIT

Na polecenie użytkownika zastąpiono CC0 w `LICENSE` standardowym tekstem MIT z nagłówkiem `Copyright (c) 2026 GarandPLG`, zgodnym z nazwą autora i rokiem w historii Git. Dodano `license = "MIT"` w `Cargo.toml`, sekcję licencji w README i wpis w mapie. `default.nix` już deklarował MIT i nie wymagał edycji.

Zmiana dotyczy bieżącego stanu repozytorium i nie cofa uprawnień do wersji wcześniej udostępnionych na CC0. Nie wykonywano audytu praw do muzyki ani innych materiałów zewnętrznych. Nie zmieniano źródeł Rust ani zależności; testów działania gry nie powtarzano.

Weryfikacja: `nix develop --no-update-lock-file -c cargo check --all-targets --locked` zakończone sukcesem; sprawdzono zgodność deklaracji MIT w LICENSE, Cargo.toml i default.nix oraz `git diff --check`.

## 2026-10-19 — linty, minimalny Rust i CI

W `Cargo.toml` dodano `rust-version = "1.88"`, `unused_must_use = "deny"`, `unsafe_code = "forbid"`, `clippy::unused_unit = "allow"` oraz `overflow-checks = true` dla release. Minimalna wersja wynika z wymagań aktualnych zależności (m.in. Ratatui 0.30.2 i Darling 0.23) i składni kodu; potwierdzono kompilację wszystkich targetów na Rust 1.88.0 przez tymczasowy toolchain Fenix wewnątrz `nix develop`. Nie zmieniono domyślnego toolchaina ani plików lock.

Dodano `.gitea/workflows/rust.yml` dla pushów i pull requestów: checkout, instalacja Nix, kontrola formatowania, kompilacja, testy debug/release i Clippy. Kontrole używają `nix develop --no-update-lock-file`, polecenia Cargo kompilujące kod mają `--locked`. Również `shellHook` we `flake.nix` wykonuje teraz pomoc CLI z `--locked`. Nie włączano `-D warnings`, grup `pedantic` ani `restriction`.

Weryfikacja lokalna na domyślnym toolchainie Nix: formatowanie, `cargo check --all-targets --locked`, `cargo test --all-targets --locked` (45 zaliczonych), `cargo test --all-targets --release --locked` (45 zaliczonych) i `cargo clippy --all-targets --locked` zakończone sukcesem. Clippy nadal zgłasza 50 ostrzeżeń biblioteki; 4 ostrzeżenia o jawnych `-> ()` zniknęły zgodnie z nowym ustawieniem. Dodatkowa kontrola Rust 1.88.0: `cargo check --all-targets --locked` zakończone sukcesem.

Zaktualizowano instrukcje agentów, mapę i opis testów. Nie zmieniano mechaniki gry ani zależności. Workflow przygotowano i przejrzano lokalnie; nie był uruchamiany na serwerze. Wymaga włączonych Actions i runnera Linux x86_64 o etykiecie `ubuntu-latest`, obsługującego akcje Node i instalację Nix. Nie rejestrowano runnera ani nie zmieniano ustawień serwera Gitea.

## 2026-10-19 — formatowanie Rust z wcięciami po 2 spacje

Dodano `rustfmt.toml` z uzgodnionymi opcjami: edycja i styl 2024, `max_width = 100`, `hard_tabs = false`, `tab_spaces = 2`, zakończenia Unix, sortowanie importów i modułów oraz domyślne heurystyki krótkich konstrukcji. Zastosowano `nix develop -c cargo fmt --all` do źródeł i testów (56 zmienionych plików Rust). Zaktualizowano instrukcje agentów i mapę konfiguracji projektu.

Weryfikacja przez `nix develop -c`: `cargo fmt --all -- --check`, `cargo check --all-targets` i `cargo test --all-targets` zakończone sukcesem; 45 testów zaliczonych. Nie wprowadzano zmian mechanik ani zależności. Nie powtarzano Clippy ani ręcznej sesji TUI przy zmianie formatowania.

## 2026-10-18 — przekazanie kontekstu na kolejną sesję

Użytkownik zaakceptował aktualny stan. Nie ma ustalonego kolejnego zadania. Na start kolejnej sesji przeczytaj `docs/AGENTS.md`, `docs/CODE_MAP.md` i `docs/TESTING.md`; instrukcje w `docs/` mogą wymagać jawnego wskazania agentowi.

Stan końcowy: zoptymalizowane audio, wykonywanie kopania i ruchu w osobnych zadaniach, 3-sekundowy krok, kolor jednostki według właściciela, procent ruchu na kratce oraz postęp etapu i całości zadania pod Tab. Ostatni pełny zestaw: 45 testów zaliczonych, kompilacja i formatowanie poprawne. Ostrzeżenia Clippy: 50 biblioteki oraz 4 stylistyczne uwagi o wymaganym jawnym `-> ()` w testach.

Uwagi do dalszej pracy:

- `Total` jest postępem według liczby kratek, nie przewidywanym czasem zakończenia. Przy zmianie rodzaju zadania zachowuje liczbę wykonanych etapów, ale postęp bieżącego etapu jest resetowany.
- Zajęte pole lub ruda zatrzymuje jednostkę; brak omijania przeszkód. Rysowanie trasy pauzuje wybraną jednostkę. Potyczka nadal otrzymuje ticki także poza jej ekranem.
- Kolejka w panelu zajmuje dwa wiersze na zadanie. Długie kolejki w małym terminalu mogą zostać ucięte; przewijania panelu nie dodano.
- Przed wydaniem pozostają ręczne sprawdzenie TUI na różnych rozmiarach terminala, odsłuch i pomiar CPU audio oraz budowa pakietów Nix. Zmiany zależności z tej sesji mogą wymagać aktualizacji `cargoHash` w `default.nix`; nie było to weryfikowane.
- Zmiany pozostają niezatwierdzone w Git, w tym nowe `tasks/moving.rs` i `tests/digging.rs`. Przed commitem sprawdź pełne `git status`, aby uwzględnić także pliki nieśledzone. Nie wykonano commita ani publikacji.

W tym kroku zmieniono wyłącznie dokumentację; nie rozpoczynano nowych poprawek ani refaktoryzacji. Końcowe `nix develop -c cargo check --all-targets` zakończone sukcesem; testów nie powtarzano po zmianie samej dokumentacji.

## 2026-10-18 — postęp zadań w panelu i obowiązkowe adnotacje typów

Kolejka pod Tab pokazuje osobno postęp bieżącej kratki (`Step`) i całego zadania (`Total`). Całość jest liczona według początkowej liczby kratek: wykonane etapy plus ułamek bieżącego. Oba typy zadań przechowują `total_steps`, zachowywane także przy zmianie Moving/Digging wskutek zmiany terenu. Zadania oczekujące pokazują 0%, zakończone osiągają 100%; pusta trasa nie powoduje dzielenia przez zero.

Dodano do `docs/AGENTS.md` obowiązek jawnych adnotacji typów w nowym i zmienianym kodzie, włącznie z testami, parametrami domknięć i wynikami funkcji. Rozróżniono adnotacje typów od rzeczywistych konwersji wartości; nie wykonywano masowego przepisywania wcześniejszego kodu.

Dodano 4 testy: postęp całego kopania między etapami, zachowanie początkowej liczby etapów przy konwersji typu, formatowanie kopania i oczekującego ruchu oraz renderowanie panelu Tab z aktywnym ruchem (`Step: 50% | Total: 75%`) i kolejnym zadaniem na 0%. Zaktualizowano README i dokumentację.

Weryfikacja przez `nix develop -c`: `cargo check --all-targets`, `cargo test --all-targets` (45 zaliczonych), `cargo fmt --all -- --check` i `cargo clippy --all-targets` zakończone sukcesem. Clippy raportuje 50 dotychczasowych ostrzeżeń biblioteki i 4 ostrzeżenia `unused_unit` w testach za jawne `-> ()`, wymagane nową konwencją. Nie wykonywano ręcznej sesji terminala ani budowy pakietów Nix.

## 2026-10-18 — osobne zadanie Moving

Dodano `MovingTask` i wariant `Tasks::Moving`. Przeniesiono do niego licznik 3-sekundowego przejścia, procentowy postęp i reset dla każdej kratki. `DiggingTask` zawiera teraz tylko stan kopania. Eksporty i delegowanie wspólnego traitu `Task` obsługują oba warianty.

Zatwierdzona spacją ścieżka jest dzielona na odcinki `Moving` (tunele/bazy) i `Digging` (kamień), z zachowaniem kolejności. Kolejka pokazuje odpowiednio ikony `→` i `⛏`. Wykonywanie ruchu nie uszkadza struktur; wejście do świeżo wykopanego tunelu nadal następuje w ramach kopania. Gdy teren zmieni się od planowania lub jednostka wraca przez właśnie wykopane kratki, typ aktywnego zadania dopasowuje się do aktualnego celu bez utraty reszty trasy. TUI pobiera timer tylko z `Moving`.

Dostosowano test timera do `MovingTask`. Dodano regresje podziału mieszanej ścieżki i wykonania całej kolejki, zachowania durability podczas ruchu, powrotu przez świeżo wykopany tunel i zmiany typu zadania po zmianie terenu. Zaktualizowano README, mapę i instrukcje agentów.

Weryfikacja przez `nix develop -c`: `cargo check --all-targets`, `cargo test --all-targets` (41 zaliczonych), `cargo fmt --all -- --check` i `cargo clippy --all-targets` zakończone sukcesem. Clippy nadal zgłasza 50 ostrzeżeń biblioteki. Bez nowych zależności. Nie wykonywano ręcznej sesji TUI ani budowy pakietów Nix.

## 2026-10-18 — procentowy wskaźnik przejścia

Dodano na kratce poruszającej się jednostki procent ukończenia bieżącego kroku, w kolorze właściciela. Wskaźnik korzysta z istniejącego licznika ruchu, a czas przejścia i wyświetlane procenty mają wspólną stałą `MOVEMENT_TICKS`. Po wejściu na następną kratkę wskaźnik resetuje się; bez ruchu nie jest wyświetlany. W pomniejszonym widoku na czas ruchu zastępuje znak struktury, w pozostałych nie zasłania znacznika jednostki. Pauza ruchu zatrzymuje również postęp wskaźnika.

Dodano test rozdzielenia postępu kopania i ruchu oraz resetu po kroku. Test TUI sprawdza 50% w połowie przejścia na wszystkich poziomach zoomu, zachowanie koloru `M`, reset na kolejnym polu i zniknięcie wskaźnika po ukończeniu zadania. Zaktualizowano README i dokumentację.

Weryfikacja przez `nix develop -c`: `cargo check --all-targets`, `cargo test --all-targets` (37 zaliczonych), `cargo fmt --all -- --check` i `cargo clippy --all-targets` zakończone sukcesem. Clippy nadal zgłasza 50 ostrzeżeń biblioteki. Nie wykonywano ręcznych oględzin terminala; renderowanie sprawdzono buforem Ratatui.

## 2026-10-18 — czas podróży i kolor znacznika jednostki

Przejście do istniejącego tunelu lub pustej bazy trwa teraz 91 ticków, czyli około 3 sekund przy ticku 33 ms. Licznik ruchu znajduje się w zadaniu i resetuje po każdym wykonanym kroku. Zajęty cel oraz rysowanie trasy wstrzymują naliczanie. Wejście do świeżo wykopanego tunelu nadal odbywa się w ticku zniszczenia kamienia.

Znacznik jednostki w normalnym i powiększonym widoku jest osobnym `Span` w kolorze właściciela (`Players::get_color`), również na tunelu oraz przy zaznaczeniu lub oznaczeniu komórki. Dotychczasowy układ komórki i kolor jej obramowania pozostają zachowane.

Dostosowano testy ruchu, kolejek i blokad do nowego czasu. Dodano regresję natychmiastowego wejścia po kopaniu oraz test koloru znacznika dla obu graczy, dwóch poziomów zoomu i wszystkich kombinacji zaznaczenia/oznaczenia. Łącznie 35 testów przechodzi. Zaktualizowano README oraz dokumentację agentów.

Weryfikacja przez `nix develop -c`: `cargo check --all-targets`, `cargo test --all-targets`, `cargo fmt --all -- --check` i `cargo clippy --all-targets` zakończone sukcesem. Clippy nadal raportuje 50 ostrzeżeń biblioteki. Nie wykonywano ręcznego pomiaru czasu ani oględzin terminala; czas zweryfikowano tickami, kolory buforem Ratatui. W pomniejszonym widoku nadal wyświetlany jest wyłącznie znacznik struktury.

## 2026-10-18 — wykonywanie kopania przez jednostki

Podłączono `BoardState::advance_turn` do ticków potyczki. Zastąpiono szkic symulacji przetwarzaniem jednej aktualizacji każdej aktywnej jednostki na tick. Górnik stopniowo uszkadza kamień, po osiągnięciu zera tworzy tunel i przechodzi do niego. Ruch aktualizuje współrzędne jednostki, usuwa wykonany krok i po zakończeniu ścieżki usuwa zadanie. Kolejne zadania wykonują się w kolejności; tunele i niezajęte bazy można przechodzić bez kopania.

Kamień pokazuje pozostałą wytrzymałość procentowo na wszystkich poziomach zoomu, a istniejący panel boczny pod Tab odświeża dokładne durability. Użyto istniejącej siły kopania 0.55% maksymalnej wytrzymałości na tick, z zachowaniem części ułamkowej obrażeń (około 6 sekund na pełną kratkę).

Zaznaczenie spacją działa tylko dla własnego górnika. Podczas rysowania wybrany górnik czeka; nowa ścieżka przedłuża koniec jego kolejki. Pominięto powtarzanie tej samej kratki przy ruchu na granicy mapy. Zajęte pola i ruda blokują ruch, nie są nadpisywane. Błędne cele spoza mapy lub niesąsiadujące anulują bieżące zadanie bez utraty jednostki.

Dodano 10 testów w `tests/digging.rs`: rzeczywisty przepływ klawiszy i ticków, utrata durability i ukończenie kopania, ruch w obu kierunkach tylko raz na tick, kolejki i ich przedłużanie przez zaznaczenie, aktualność współrzędnych, zajęte pola, ochrona rudy, błędne/puste ścieżki, własność jednostek, pauza przy rysowaniu i renderowanie procentów oraz dokładnych wartości. Dostosowano test cofania ścieżki do wyboru rzeczywistego górnika.

Weryfikacja przez `nix develop -c`: `cargo check --all-targets`, `cargo test --all-targets` (33 zaliczone), `cargo fmt --all -- --check` i `cargo clippy --all-targets` zakończone sukcesem. Clippy zgłasza 50 ostrzeżeń dla biblioteki. Zaktualizowano README, mapę, instrukcje agentów i opis testów. Nie dodano zależności.

Nie wykonywano ręcznej sesji w terminalu, odsłuchu ani budowy pakietów Nix; renderowanie sprawdzono przez TestBackend. Omijanie przeszkód, wydobycie rudy, ekonomia i walka pozostają poza zakresem tej zmiany.

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

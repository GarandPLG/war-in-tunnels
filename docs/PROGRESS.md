# Dziennik prac

## 2026-10-26 — koordynaty w mapie stresu i fokus gotowej trasy

CellWidget::render_stress wyświetla koordynaty na obramowaniu, w kolorze mapy stresu lub aktywnego fokusu. Kamień zachowuje zwykły wygląd. BoardWidget nie nadaje flagi marked skupionemu polu gotowej kolejki poza trybem wytyczania zadania: obramowanie jest wtedy żółte, a podczas rysowania pozostaje magenta. Podświetlenie pozostałej trasy i flagi modelu pozostają zachowane.

Regresje sprawdzają koordynaty przy wszystkich zoomach oraz fokus gotowej trasy przed zaznaczeniem jednostki i po Space, zarówno w zwykłym widoku, jak i mapie stresu. cargo fmt, cargo check --all-targets --locked i wszystkie 107 testów przez nix develop przeszły. Nie wykonywano ręcznej kontroli wyglądu w terminalu.

## 2026-10-26 — spokojniejsze kolory kratek

CellWidget obniża kanały RGB zwykłych, niezaznaczonych kratek o około 25%, wyłącznie podczas renderowania. Kolory jednostek, postępu, fokusu, tras, paneli i mapy stresu pozostają czytelne w dotychczasowej palecie; model struktur nie zmienia kolorów. Zastosowano cargo fmt; cargo check --all-targets --locked i wszystkie 106 testów przez nix develop przeszły. Nie oceniano wyglądu ręcznie w terminalu.

## 2026-10-26 — jawne kolory RGB zamiast palety terminala

Zastąpiono wszystkie nazwane kolory Ratatui w src/ i tests/ przez Color::Rgb, również skróty Stylize takie jak .gray(), .cyan() i .yellow(). Przyjęto standardowe wartości ANSI/xterm: podstawowe kanały 128, jasne 255, Gray (192, 192, 192), DarkGray (128, 128, 128). Kolor rudy Indexed(217) zastąpiono jego odpowiednikiem xterm (255, 175, 175). Istniejący gradient RGB stresu pozostaje bez zmian. Jawne RGB uniezależnia te kolory od palety motywu terminala, więc wygląd może różnić się od wcześniejszych kolorów nazwanych. Zmieniono wyłącznie zapis kolorów i potrzebne importy, bez dodawania warstwy motywów.

Zastosowano cargo fmt; cargo check --all-targets --locked i cargo test --all-targets --locked przez nix develop przeszły. git diff --check przeszedł; wyszukiwanie potwierdziło brak Color:: innych niż Rgb i nazwanych skrótów kolorowania w źródłach oraz testach. Nie sprawdzano wyglądu w interaktywnym terminalu. Zachowano zastaną zmianę użytkownika w flake.lock.

## 2026-10-24 — wspólny workflow Gitea i kolejność jobów

Odnotowano zmianę użytkownika: `.gitea/workflows/tests.yml` (`Project tests`) zastępuje osobne workflowy `rust.yml`, `nix.yml` i `packages.yml`. Po pushu i dla pull requestów wykonuje joby w kolejności `check → nix → linux → aur`, wymuszonej przez `needs`. Błąd wcześniejszego joba pomija zależne joby. `check` wykonuje kontrole Rust, `nix` buduje `.#default`, `linux` pakuje deb/rpm/AppImage/Flatpak, a `aur` buduje i instaluje paczkę Arch. Artefakty pozostają testowe, bez publikowania wydań i tagów, z retencją 7 dni.

Zaktualizowano CODE_MAP, TESTING, PACKAGING i AGENTS, w tym opis współdzielonego cache toolchaina oraz osobnych cache Cargo. Nie zmieniano workflowu ani flake.nix. Dokumentację porównano z aktualnym yml; `git diff --check` i `nix develop -c cargo check --all-targets --locked` przeszły. Nie uruchamiano jobów Gitea.

## 2026-10-24 — czas podpór, kamień bez stresu i wybór zadania po Space

Budowa i usuwanie podpór trwają po 91 ticków pracy (3003 ms). Tunnel przechowuje SupportWork z właścicielem, rodzajem pracy i postępem; roof_support zmienia się dopiero przy ukończeniu. Budowa pobiera koszt raz przy rozpoczęciu, a podpora w trakcie rozbiórki nadal stabilizuje sieć. Kaskada następuje dopiero po zakończeniu usuwania. Postęp jest na kratce, w zakładce Support i pod Tab. Bez budowniczego timer pauzuje; jeden górnik nie wykonuje dwóch podpór naraz, a jego ruch i kopanie są wstrzymane na czas pracy.

Automatyczne podpory są teraz przygotowywane przed ruchem i kopaniem: prospective_tunnel_stress przewiduje stres przyszłego wykopu bez zmiany terenu. Przy ryzyku 100% automat rozpoczyna pracę na obecnym polu 80%, zatrzymuje górnika i dopiero po jej zakończeniu pozwala kopać dalej. Nie buduje zdalnie, nie nalicza opłaty ponownie i nadal nie marnuje podpory na końcu trasy. Brak zasobów zachowuje wcześniejsze ryzyko śmiertelnego zawalenia.

Stone w mapie stresu korzysta ze zwykłego renderowania bez procentu stresu i bez podświetlenia kolejki; fokus zachowuje dotychczasowy wygląd. Widoczny i dostępny skrót t został usunięty z samego fokusu na Minerze, a dodany do trybu zaznaczenia przez Space. Enter w bocznym panelu tylko wybiera zadanie i wraca do rysowania, nie zatwierdza kolejki. Jeśli zmiana na Moving unieważnia szkic przez kamień, configure_selected_task resetuje szkic do początku trasy, zachowując zaznaczenie jednostki.

Dodano 4 regresje i dostosowano testy poprzedniej natychmiastowej budowy: pełny czas i zachowanie podpory do końca rozbiórki, zatrzymanie kolejki budowniczego, pauza bez budowniczego i wznowienie, jedna praca na górnika, normalny kamień przy wszystkich zoomach i kombinacjach fokusu oraz t dopiero po Space i reset nieprawidłowego szkicu Moving. Wszystkie 106 testów przeszły w debug i release; cargo check --all-targets --locked i cargo clippy --all-targets --locked przez nix develop przeszły. Clippy nadal ma 47 wcześniejszych ostrzeżeń. Zastosowano cargo fmt; git diff --check przeszedł. Uzupełniono README, CODE_MAP, TESTING i AGENTS. Nie wykonywano ręcznej rozgrywki ani budowy paczek.

## 2026-10-24 — stres tuneli, podpory, zawalenia i wybór zadania

Zachowano zmianę użytkownika: domyślny stres Tunnel to 20%. Podpora jest ulepszeniem roof_support, oznaczanym przez Structure::get_level jako T, z ceną 10 Wood + 2 Metal. Dodano zakładkę Support w budowaniu: Enter natychmiast stawia podporę na tunelu przy własnym Minerze na polu lub obok; Delete usuwa ją bez zwrotu kosztu i natychmiast rozstrzyga kaskadę. Walidacja terenu, budowniczego i zasobów jest wspólna dla TUI i automatu, bez zdalnego budowania ani powtórnych opłat.

board_stress.rs przelicza najkrótsze połączenia w czterech kierunkach: bazy startują z 0, podpory z 20, każdy kolejny krok dodaje 20 do limitu 100. Gotowe Mine przepuszczają połączenie; odłączone tunele mają stres 100. Osiągnięcie 100 zamienia tunel w nieprzechodni CollapsedTunnel (#, 750 durability), usuwa znajdującą się tam jednostkę i ponawia przeliczenie do stabilizacji. Wykonywane i oczekujące zadania zasypanych jednostek giną razem z nimi; rysowana trasa ginącej jednostki również jest czyszczona. Gruz można wykopać ponownie; obrażenia bazują na 1000 HP kamienia, więc usuwa się go szybciej niż zwykły kamień. Tunel ma nadal 500 HP.

Tick korzysta z advance_turn_with_events i rekordów Excavation. Po wykopach, przed rozstrzygnięciem zawaleń i rekrutacją, automat opłaca podporę na poprzednim polu 80% wyłącznie wtedy, gdy kolejny wykop bez niej osiągnąłby 100%. Pierwsza podpora na prostej trasie jest cztery pola od bazy, kolejne co trzy; nie ma zbędnej podpory na końcu trasy. Ustawienie auto_supports jest domyślnie włączone i kopiowane do nowych zadań; konwersje Digging/Moving je zachowują. Brak zasobów nie zatrzymuje kopania — może zakończyć się śmiercią górnika. Low-level advance_turn pozostaje samym krokiem jednostek; pełne rozliczenie stresu i gospodarki jest w SkirmishState::tick_update.

Dodano boczny panel t, który zajmuje prawy obszar layoutu i nie przykrywa środka mapy. Strzałki wybierają Digging/Moving, Enter zaczyna rysowanie, t zamyka panel; Esc jest ignorowany. a przełącza automatyczne podpory dla nowych zadań Digging, także podczas rysowania. Jawnie wybrany Moving ogranicza trasę do pól przechodnich i nie zaczyna kopać, gdy droga później się zawali. Spacja pozostaje skrótem domyślnej trasy z kopaniem. Resize uwzględnia panel zadania. Skrót s przełącza widok stresu: same procenty, gradient zielony/czerwony, zachowany kolor fokusu, bez jednostek, oznaczeń tras i innych postępów. W małym zoomie 100 nie ma sufiksu %. Renderer nie zmienia modelu.

Dodano 13 testów w tests/tunnel_stress.rs: odległości, cztery kierunki podpór, alternatywne podparcie, kaskady i śmierć obu stron, walidacja i koszty, minimalne rozmieszczenie auto-podpór, brak środków/wyłączenie automatu, konwersje opcji, gruz i ponowne kopanie, zakładka Support, widok stresu we wszystkich zoomach, fokus, geometria bocznego panelu oraz Moving bez kopania. Starszy test mieszanej trasy ma teraz podporę w odizolowanym końcowym tunelu, aby nadal sprawdzał podział Moving/Digging bez niezamierzonego zawalenia testowej planszy.

Wszystkie 102 testy przeszły w debug i release; cargo check --all-targets --locked i cargo clippy --all-targets --locked przez nix develop przeszły. Pozostało 47 dotychczasowych ostrzeżeń Clippy. Zastosowano cargo fmt, git diff --check przeszedł. Uzupełniono README, CODE_MAP, TESTING i AGENTS. Nie wykonywano ręcznej rozgrywki, odsłuchu ani przebudowy paczek. Ceny są wstępne; szansa przeżycia i ratowanie jednostek nie są częścią tej implementacji.

## 2026-10-23 — zakładki budowania i rekrutacji

Dodano paski Tabs z wyróżnieniem LightMagenta oraz strzałki Left/Right przełączające opcje w aktywnym oknie bez ruchu mapy. SkirmishState przechowuje osobne indeksy i listy opcji (obecnie wyłącznie Mine oraz Miner); wybór zawija na końcach listy. Przy jednej pozycji strzałki zachowują ten sam wybór. Nie dodano budynków ani jednostek. Enter, Delete i zamykanie b/r zachowują dotychczasowe działanie; Esc pozostaje ignorowany. Dodanie przyszłych typów wymaga powiązania ich szczegółów i akcji z wyborem, zgodnie z CODE_MAP.

Dodano test strzałek w obu oknach, niezmienności planszy i kolejek oraz zamykania właściwym skrótem. Dostosowano test kolorów zaznaczonej zakładki i legendy. Wszystkie 89 testów oraz cargo check --all-targets --locked przez nix develop przeszły. Zastosowano cargo fmt; git diff --check przeszedł. Zaktualizowano README, CODE_MAP i TESTING. Nie sprawdzano ręcznie terminala.

## 2026-10-23 — procent rekrutacji na kratce bazy

CellWidget wyświetla cyjanowy procent rekrutacji na środku Base z niepustą kolejką. Pokazuje także 0% i gotowe 100% czekające na wolne miejsce; znika po zakończeniu kolejki. W najmniejszym zoomie 100 jest bez sufiksu %, aby zmieścić się w trzech znakach wnętrza. Wskaźnik ma pierwszeństwo przed procentem ruchu jednostki opuszczającej bazę; w pozostałych zoomach zachowano znacznik jednostki. Brak zmian symulacji i czasu rekrutacji.

Dodano regresję wszystkich trzech zoomów: brak procentu bez kolejki, 0%, 50%, cyjanowy kolor, znacznik Minera, oczekiwanie przy 100% oraz usunięcie wskaźnika po spawnie. Wszystkie 88 testów i cargo check --all-targets --locked przez nix develop przeszły. Zastosowano cargo fmt, git diff --check przeszedł; zaktualizowano README i CODE_MAP. Nie sprawdzano ręcznie terminala.

## 2026-10-23 — podświetlenie ścieżek zadań

BoardWidget zbiera pozostałe pola wszystkich zadań własnych jednostek do HashSet i podświetla widoczne kopie komórek istniejącymi kolorami LightMagenta oraz Magenta pod kursorem. Obsługuje ruch i kopanie, także dalsze pozycje kolejki. Flagi marked modelu i wytyczanie nowych tras nie są zmieniane. Pola znikają z podświetlenia po wykonaniu lub odwołaniu zadań; przeciwnik nie ujawnia tras. Wykorzystano istniejące renderowanie CellWidget bez nowego stanu gry.

Dodano test renderowania w tests/task_cancellation.rs: kolor trasy, skracanie po przejściu, znikanie po anulowaniu i zakończeniu, zachowanie flag modelu oraz brak kolorowania tras przeciwnika. Wszystkie 87 testów i cargo check --all-targets --locked przez nix develop przeszły. Zastosowano cargo fmt; git diff --check przeszedł. Uzupełniono README, CODE_MAP i TESTING. Nie sprawdzano ręcznie terminala.

## 2026-10-23 — odwoływanie zadań jednostek

Pod Tab, z fokusem na własnej jednostce z zadaniami, Backspace odwołuje ostatnie zadanie, a Delete całą kolejkę. Jeśli zostało jedno zadanie, Backspace odwołuje także wykonywane. Legenda dopasowuje opisy do kontekstu; podczas rysowania trasy klawisze zachowują cofanie/anulowanie trasy, a Delete w budowaniu nadal anuluje budowę. BoardState::cancel_focused_tasks waliduje właściciela, kolejkę i brak rysowania. Nie dodano usuwania środkowej pozycji, ponieważ dalsze zadania bazują na końcu poprzednich odcinków trasy.

Anulowanie usuwa timer zadania, zachowując położenie jednostki, wykopane tunele i uszkodzenia kamienia. Dodano 3 testy integracyjne obejmujące opóźnione i aktywne zadania, przerwany ruch, ochronę przeciwnika i trybu trasy oraz klawisze pod Tab. Wszystkie 86 testów oraz cargo check --all-targets --locked przez nix develop przeszły. Zastosowano cargo fmt i zaktualizowano dokumentację. Nie sprawdzano ręcznej rozgrywki.

## 2026-10-23 — zamykanie paneli wyłącznie skrótem otwierającym

Odświeżono kontekst po zmianach z drugiego komputera: aktualne skróty to b (budowanie), r (rekrutacja), + i - (głośność), a rustfmt używa 4 spacji. Zachowano te ustawienia. Usunięto Esc z akcji i legendy obu paneli. Obsługa modalna jawnie ignoruje Esc, aby nie uruchamiał również globalnego powrotu do menu. b zamyka budowanie, r zamyka rekrutację; kolejki i budowy pozostają zachowane. Esc poza tymi panelami zachowuje dotychczasowe działanie.

Dostosowano testy kontekstu, rekrutacji i budowania: sprawdzają ignorowanie Esc bez zmiany widoku, brak Esc w legendzie oraz zamykanie odpowiednim skrótem. Zaktualizowano README, CODE_MAP, TESTING i instrukcje agenta. Wszystkie 83 testy oraz cargo check --all-targets --locked przez nix develop przeszły; zastosowano cargo fmt. Nie sprawdzano ręcznej rozgrywki w terminalu.

## 2026-10-21 — spójne skróty głośności

Przyciszanie muzyki przeniesiono z `n` na `-`, zgodnie z parą `+`/`-`. Zaktualizowano wspólną tabelę skrótów (także legendę), istniejący test komend audio i README. Kompilacja wszystkich targetów, testy `tests/app.rs` i kontrola formatowania przez Nix przeszły. Nie sprawdzano odsłuchu.

## 2026-10-21 — skróty b/r i kolory okien budowy oraz rekrutacji

Zastąpiono Ctrl+Space przez `b`, a Alt+Space przez `r`; usunięto także obsługę NUL dla budowania. Oba klawisze otwierają i zamykają właściwy panel. Ponieważ `b` sterowało głośnością, zwiększanie głośności przeniesiono na `+`. Legenda korzysta ze wspólnej tabeli skrótów.

Okna używają kolorowanych linii Ratatui: żółte nazwy i koszty, cyjanowe postępy i produkcja, zielone dostępne działania, czerwone błędy oraz żółte komunikaty oczekiwania. Zachowano tekstowe opisy statusów. Zaktualizowano testy i bieżącą dokumentację; dodano regresję kolorów obu okien i sprawdzenie odrzucania starych skrótów.

Weryfikacja: kompilacja wszystkich targetów i 83 testy debug przez `nix develop -c` przeszły. Nie sprawdzano interaktywnego terminala ani odsłuchu. Zastanej zmiany `flake.lock` nie modyfikowano.

## 2026-10-19 — alternatywny skrót rekrutacji

Zgłoszono brak reakcji na Alt+Space. Testy przekazywanego zdarzenia Alt+Space działają; nie potwierdzono, jakie zdarzenie wysyła terminal użytkownika ani czy skrót przechwytuje menedżer okien. Dodano alternatywne `r` do tej samej akcji Recruitment i pokazano oba skróty w legendzie. Obowiązują te same ograniczenia własnej bazy i trybu; `r` także zamyka panel bez kasowania kolejki. README opisuje alternatywę.

Dodano regresję otwarcia, opłacenia rekrutacji i zamknięcia przez `r`, oraz sprawdzenie blokady poza bazą i podczas rysowania trasy. Przeszły wszystkie 82 testy, cargo check --all-targets --locked i cargo fmt --all -- --check przez nix develop oraz git diff --check. Nie sprawdzano skrótów w środowisku graficznym użytkownika.

## 2026-10-19 — rekrutacja Miner w Base

Dodano koszt MinerUnit::RECRUIT_COST: roboczo 10 Wood i 15 Metal za każdą pozycję. SkirmishState::can_recruit_miner/recruit_miner sprawdza własną Base, współrzędne, miejsce w liczniku kolejki i środki, po czym opłaca zamówienie. Niedobór dowolnego zasobu lub niewłaściwy cel nie pobiera opłaty ani nie zmienia kolejki. Każda baza ma niezależny licznik queued_miners, recruitment_ticks i flagę waiting_for_departure; dla identycznych Minerów nie jest potrzebna lista obiektów ani utrata Copy struktur.

Rekrutacja trwa 304 ticki (10032 ms, około 10 sekund). Zgodnie z odpowiedzią użytkownika pierwsza pozycja może dojść do 100% na zajętej bazie i czeka tam na wolną kratkę. Nie postępują wtedy kolejne pozycje. Po pojawieniu się górnika flaga waiting_for_departure zatrzymuje następne odliczanie na 0% do jego wyjścia z bazy. BoardState::advance_recruitment działa po ruchu istniejących jednostek, więc nie nadpisuje zajętego pola i może wykorzystać właśnie zwolnioną bazę. Nowy Miner ma właściciela bazy, jej współrzędne, pełne HP, uprawnienia i pustą kolejkę zadań. Bazy przeciwnika używają jego zasobów; brak automatycznej rekrutacji AI.

Alt+Space otwiera panel wyłącznie na własnej bazie, poza rysowaniem trasy i innym modalem. Enter dopisuje Minera, jeśli są środki. Panel pokazuje koszt, czas, liczbę zamówionych (włącznie z aktywnym), postęp i status Recruiting/Ready/Paused. Esc lub Alt+Space zamyka widok bez kasowania kolejki. Ruch mapy i otwarcie budowania są w tym trybie blokowane, globalna muzyka i wyjście pozostają dostępne. Licznik i postęp są również pod Tab.

Dodano tests/recruitment.rs z 8 testami. Cały zestaw 81 testów przeszedł w debug i release, w tym własność i opłaty, brak przedwczesnego pojawienia się jednostki, czekanie przy 100%, blokowanie kolejnych pozycji i wznowienie po wyjściu, brak nadpisania także przy obcej jednostce w bazie, niezależne bazy, zadania nowego górnika oraz Alt+Space/Enter/Esc i renderowanie panelu. Końcowe cargo fmt --all -- --check, cargo check --all-targets --locked i cargo clippy --all-targets --locked przez nix develop przeszły; nadal 47 istniejących ostrzeżeń biblioteki. git diff --check przeszedł. Uzupełniono README, CODE_MAP, TESTING i AGENTS; zachowano wcześniejsze staged i unstaged zmiany użytkownika.

Nie wykonywano ręcznej pełnej rozgrywki ani ponownego budowania paczek. Cena jest wstępna do balansowania. Anulowanie pozycji rekrutacji, inne typy jednostek i pełna mechanika limitu populacji nie były częścią tej zmiany.

## 2026-10-19 — czas, koszt i anulowanie budowy Mine

Potwierdzono, że wyjątek Mine dotyczy tylko połączenia z tunelem/bazą: obecność własnego sąsiedniego budowniczego nadal jest obowiązkowa, więc ruda na środku mapy bez jednostki nie pozwala rozpocząć budowy. Wydzielono wspólne BoardState::has_adjacent_builder i dodano regresję rzeczywistej zdalnej rudy bez pobierania opłaty.

Mine::COST wynosi roboczo 20 Wood i 10 Metal. SkirmishState::can_start_mine_construction/start_mine_construction sprawdza dostępne środki i teren, po czym opłaca rozpoczęcie. Resources::spend sprawdza oba zasoby przed odjęciem; nieudana lub powtórzona próba nie pobiera środków. Nowa struktura MineConstruction przechowuje oryginalną rudę i postęp. Budowa trwa 304 ticki (10032 ms, około 10 sekund) aktywnej obecności własnego budowniczego na sąsiedniej kratce. Odejście pauzuje, powrót wznawia; niedokończona kopalnia nie wydobywa i nie przyjmuje jednostek. Ukończenie zmienia ją w gotową Mine z zachowaną rezerwą.

Panel Ctrl+Space pokazuje koszt, czas, procent, pracę lub brak budowniczego oraz Delete/Cancel construction. Po rozpoczęciu panel pozostaje otwarty; Esc lub Ctrl+Space tylko go zamyka. Procent jest także na kratce i pod Tab. SkirmishState::cancel_mine_construction przywraca oryginalne neutralne Ore i zwraca pełen koszt wyłącznie właścicielowi aktywnego placu. Nie można zwrócić kosztu dwukrotnie, anulować cudzego placu ani sprzedać ukończonej kopalni.

Dodano 7 regresji i dostosowano wcześniejsze testy budowania: construction.rs ma 16 testów, cały projekt 73. Wszystkie przeszły w debug i release. Kontrole cargo fmt --all -- --check, cargo check --all-targets --locked i cargo clippy --all-targets --locked przez nix develop zakończyły się sukcesem; nadal 47 istniejących ostrzeżeń biblioteki. git diff --check przeszedł. Zaktualizowano README, CODE_MAP, TESTING i instrukcje agenta, zachowując poprzednie zmiany w drzewie roboczym.

Nie wykonywano ręcznej pełnej rozgrywki ani ponownego budowania paczek. Koszt i czas są parametrami wstępnymi; anulowanie zwraca pełny koszt niezależnie od postępu, a pasywne dostawy drewna działają osobno.

## 2026-10-19 — budowanie Mine i bonus od obsadzenia górnikiem

Dodano Unit::get_can_build (domyślnie false), włączone dla Miner i delegowane przez Units/Option. Wspólne BoardState::can_build_at sprawdza sąsiedniego własnego budowniczego w czterech kierunkach, granice, brak jednostki na celu i teren: zwykłe budynki wymagają kamienia przy tunelu lub własnej bazie, a Mine wymaga neutralnego, niewyczerpanego Ore bez wymogu połączenia. BoardState::build_mine_at ponownie waliduje i zachowuje zawartość złoża. UI Ctrl+Space udostępnia Enter/Build Mine tylko dla poprawnego celu; udana budowa zamyka widok. Na razie budowa jest natychmiastowa i bez kosztów, bo kosztów ani czasu nie ustalono. Inne typy budynków nie są jeszcze dodane.

Structures::can_enter współdzieli regułę wejścia między planowaniem i wykonywaniem trasy. Mine akceptuje własnego Minera; ruch do i z kopalni jest Moving i trwa około 3 sekund. Jednostka pozostaje na kratce, zajmując jej standardowy pojedynczy slot, bez kopiowania pracownika do osobnego stanu budynku. Mine::production_per_cycle/produce_with_miner zwiększa wydobycie z 1 do 2 Metal na cykl przy obecności własnego górnika; po wyjściu bonus znika. Wydobycie nadal ogranicza rezerwa Ore, a metal trafia do właściciela. Panel Tab pokazuje tempo i dopisek Miner przy aktywnym bonusie, a opis jednostki pokazuje uprawnienie budowania.

Dodano tests/construction.rs (9 testów) i dostosowano test nieprawidłowego celu w context_keybindings.rs. Razem 66 testów przeszło w debug i release. Testy obejmują uprawnienia, sąsiedztwo przez krawędź, wrogą jednostkę, wyjątek Ore, połączenie dla kamienia, granice i zajęte/wyczerpane złoża, budowę Enter w TUI, Moving do/z kopalni, wzrost i zanik bonusu oraz ostatnią jednostkę rudy. Końcowe cargo check --all-targets --locked, cargo fmt --all -- --check i cargo clippy --all-targets --locked przez nix develop przeszły; Clippy nadal ma 47 istniejących ostrzeżeń biblioteki. git diff --check przeszedł. Zaktualizowano README, CODE_MAP, TESTING i AGENTS, zachowując wcześniejsze zmiany zasobów w drzewie roboczym.

Nie wykonywano ręcznej pełnej rozgrywki ani ponownego budowania paczek dystrybucyjnych. Koszty budowy, czas budowy i balans bonusu pozostają do późniejszego ustalenia.

## 2026-10-19 — rzeczywiste zasoby, model Mine i kontekstowa legenda

Zmieniono Iron na Metal w CLI, ustawieniach i nagłówku; --starting-metal zachowuje -i oraz alias --starting-iron. SkirmishState ma osobne magazyny Resources (u32) dla gracza i przeciwnika, inicjalizowane z CLI. TUI czyta aktualny stan magazynu, nie wartości startowe. Gracz dostaje 5 Wood co 152 ticki (około 5 sekund); dodawanie chroni przed przepełnieniem. Metal nie przyrasta bez kopalni.

Dodano Structures::Mine, które przechowuje właściciela i oryginalne Ore wraz z pozostałą zawartością. Structures::build_mine dopuszcza wyłącznie neutralną rudę; tick co 31 wywołań wydobywa do 1 Metal i kredytuje rzeczywistego właściciela. Wyczerpane złoże nie produkuje więcej. Side panel pokazuje zawartość Ore kopalni. Oba początkowe złoża pozostają na mapie, lecz są neutralne. Kopalnie nie są rozmieszczane na planszy. Zgodnie z doprecyzowaniem użytkownika widok Ctrl+Space jest wyłącznie podglądem, bez stawiania budynków, kosztów ani budowania przez Enter.

Wspólne available_skirmish_actions określa konteksty mapy, rysowania trasy i budowania dla wejścia i legendy. Własnego górnika można zaznaczyć spacją; na trasie widoczne są ruch, potwierdzenie i cofanie/anulowanie. Esc anuluje trasę lub zamyka podgląd budowania, w normalnym trybie wraca do menu. Ctrl+Space obsługuje też NUL starszych terminali. Wyjście i muzyka pozostają globalne. Legenda dobiera opisy i szerokości kolumn do tekstu; wysokość stopki pozostaje stała, bez zmiany geometrii planszy.

Dodano tests/resources.rs (7 testów) i tests/context_keybindings.rs (4 testy), poprawiono test neutralności złóż i CLI; razem 57 testów. Debug i release przeszły w całości, w tym renderowanie bieżących zasobów, wyczerpanie złoża, przypisanie wydobycia właścicielowi, brak automatycznej budowy i czytelność legendy przy 120 kolumnach. Końcowe cargo check --all-targets --locked, cargo fmt --all -- --check i cargo clippy --all-targets --locked przez nix develop zakończyły się sukcesem; Clippy zgłasza 47 istniejących ostrzeżeń biblioteki. git diff --check przeszedł. Zaktualizowano README, CODE_MAP, TESTING i instrukcje agenta.

Nie dodano faktycznego stawiania kopalń ani wymiany z powierzchnią. Wartości przychodów są wstępnymi stałymi do balansowania. Nie wykonywano ręcznej pełnej rozgrywki ani budowania paczek dystrybucyjnych; brak nowych zależności Cargo.

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

Dodano `rustfmt.toml` z uzgodnionymi opcjami: edycja i styl 2024, `max_width = 100`, `hard_tabs = false`, `tab_spaces = 4`, zakończenia Unix, sortowanie importów i modułów oraz domyślne heurystyki krótkich konstrukcji. Zastosowano `nix develop -c cargo fmt --all` do źródeł i testów (56 zmienionych plików Rust). Zaktualizowano instrukcje agentów i mapę konfiguracji projektu.

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

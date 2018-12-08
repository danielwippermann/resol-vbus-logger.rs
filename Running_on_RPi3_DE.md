# `resol-vbus-logger.rs` auf einem Raspberry Pi 3

Dieses Dokument erläutert die Schritte, die notwendig sind, um den `resol-vbus-logger.rs`
auf einem Raspberry Pi 3 zu betreiben.


## Was ist `resol-vbus-logger.rs`?

`resol-vbus-logger.rs` ist der Name eines GitHub-Projekt. Darin enthalten ist der Quellcode
für ein Programm, dessen Aufgabe es ist, Daten vom RESOL VBus zu empfangen und auf verschiedene
Wege zu verarbeiten und / oder aufzuzeichnen.

Dieses Programm ist in der Programmiersprache "Rust" entwickelt (daher auch die Endung auf
".rs") und eignet sich deshalb auch gut dafür, auf einem resourcenbegrenzten System wie dem
Raspberry Pi betrieben zu werden.

Aktuell können mit dem Programm VBus-Daten aus folgenden Quellen empfangen werden:

- über das Netzwerk (z.B. durch Datenlogger oder das Kommunikationsmodul KM2)
- über einen VBus/USB-Adapter

Folgende Datenverarbeitungen und -aufzeichnungen sind aktuell umgesetzt:

- Langzeitspeicherung in eine CSV-Datei
- Langzeitspeicherung in eine SQLite-Datenbank
- Momentaufnahme in eine PNG-Datei
- Momentaufnahme in eine einfache Textdatei

Manche diese Verarbeitungen erfordern nur eine Konfiguration über die Konfigurationsdatei
`config.toml`, andere wiederum benötigen Eingriff in den Quellcode (speziell die PNG-Erstellung).


## Voraussetzungen

Für den Betrieb des Programms auf einem Raspberry Pi wird folgendes benötigt:

- Raspberry Pi mit aktuellem Raspbian auf SD-Karte (getestet wurde "2018-11-13-raspbian-stretch.img")
- Zugang zu einem Regler mit RESOL VBus über eine der oben beschriebenen Quellen
- Rust-Toolchain
- ein bischen Zeit


## Ersteinrichtungsschritte

Die folgenden Befehle müssen über das Linux-Terminal des Raspberry Pis abgesetzt werden.

Für das erfolgreiche Kompilieren später muss ein Paket installiert sein, das nicht Bestandteil
der Raspbian-Standarddistribution ist:

    sudo apt-get install libudev-dev

Die Rust-Toolchain selber lässt sich mit dem folgenden Befehl installieren (ich weiß, ich bin
auch kein Freund von `curl ... | sh`-Kombinationen, aber so steht es in der
[Dokumentation](https://www.rust-lang.org/)...):

    curl https://sh.rustup.rs -sSf | sh

Dieses Skript installiert die aktuelle Rust-Toolchain für den Raspberry Pi. Am Ende des
Installationsvorgangs kommt der Hinweis, dass man entweder ein neues Terminal aufmachen kann
oder einfach mit

    source $HOME/.cargo/env

dem aktuellen Terminal auch noch Zugang zu der Rust-Toolchain geben kann. Danach sollte der
Befehl

    cargo -V

die Version des Rust-Buildtools "cargo" ausgeben.

Den Quellcode für das Programm gibt es wie erwähnt bei GitHub. Den aktuellen Stand kann
man sich mit dem folgenden Befehl auf den Raspberry Pi holen:

    git clone https://github.com/danielwippermann/resol-vbus-logger.rs

Dabei wird im aktuellen Verzeichnis ein Unterverzeichnis `resol-vbus-logger.rs` angelegt.
Für die weiteren Schritte wird davon ausgegangen, dass in dieses Verzeichnis gewechselt wurde:

    cd resol-vbus-logger.rs

Es gibt mindest zwei verschiedene Arten, das Programm zu kompilieren: der Debug-Mode, der
weniger Zeit zum Kompilieren braucht, aber auch ein deutlich langsameres Programm erstellt und
der Release-Mode, der ein hochoptimiertes Programm erzeugt, dafür aber deutlich mehr Zeit
benötigt. Es empfiehlt sich, für die ersten Tests mit der Debug-Variante zu arbeiten und wenn
sichergestellt ist, dass das Programm die gewünschten Funktionen erfüllt, sich einmal eine
Release-Variante zu erstellen.

Die Debug-Variante kann mit

    cargo build

erstellt werden. Das Ergebnis liegt nach erfolgreicher Kompilierung in `target/debug/logger`.

Die Release-Variante kann mit

    cargo build --release

erstellt werden. Das Ergebnis liegt dann in `target/release/logger`.

Cargo kompiliert nicht nur den Quellcode des `resol-vbus-logger.rs`, sondern lädt und übersetzt
auf alle benötigten Abhängigkeiten. Das erste Kompilieren dauert deshalb deutlich länger als alle
nachfolgenden.

Nachdem die Debug-Variante gebaut ist, muss noch die Konfigurationsdatei `config.toml` auf die 
persönlichen Bedürfnisse angepasst werden. Die Datei selber existiert nicht in dem geklonten
Quellcode, dafür aber eine Beispieldatei, die als Vorlage genutzt werden kann. Der folgende Befehl
kopiert die Vorlage an die entsprechende Stelle:

    cp config.toml.example config.toml


## Anpassungen und Test-Phase

Nun kann die `config.toml` mit dem Editor der Wahl bearbeitet werden. Das Dateiformat erinnert
an eine Windows-INI-Datei und ist relativ einfach zu pflegen. Mehr Informationen über TOML gibt
es in der entsprechenden [Dokumentation](https://github.com/toml-lang/toml/tree/v0.4.0).

Die Beispiel-Datei ist kommentiert, um die Funktion der einzelnen Felder zu erläutern. Ein paar
Einstellungen werden notwendig sein:

- bei VBus-Verbindung über das Netzwerk müssen die Einstellungen für `address`, `password` und
  `channel` konfiguriert werden und die Einstellung `path` muss mit einem `#` davor
  auskommentiert sein
- bei VBus-Verbindung über einen VBus/USB-Adapter muss die Einstellung `path` konfiguriert werden
  (z.B. auf `/dev/ttyACM0`) und die Einstellung `address` muss mit einem `#` davor
  auskommentiert sein
- alle Generatoren und Logger sind mit ihrem entsprechenden `..._tick_interval = 0` deaktiviert.
  Die gewünschten Generatoren und Logger müssten ein entsprechendes Interval in Sekunden bekommen
  und die restlichen Einstellungen geprüft und ggf. angepasst werden

Nach erfolgreicher Konfiguration kann das Programm jetzt gestartet werden. Gerade am Anfang ist
es sinnvoll, die Debug-Ausgaben im Terminal zu aktivieren, um mögliche Probleme zu entdecken.
Dafür wird das Programm wie folgt gestartet:

    RUST_LOG=debug target/debug/logger

Danach sieht die Terminalausgabe so ähnlich wie folgt aus:

    pi@raspberrypi:~/resol-vbus-logger.rs $ RUST_LOG=debug target/debug/logger
    [2018-12-03T14:34:44Z DEBUG logger] Loading config
    [2018-12-03T14:34:44Z DEBUG logger] Using serial port
    [2018-12-03T14:34:44Z DEBUG logger] Connecting serial port
    [2018-12-03T14:34:44Z DEBUG logger] Creating live data stream
    [2018-12-03T14:34:46Z DEBUG logger] Received new packet, need to resettle...
    [2018-12-03T14:34:46Z DEBUG logger] Received new packet, need to resettle...
    [2018-12-03T14:34:47Z DEBUG logger] Received new packet, need to resettle...
    [2018-12-03T14:34:51Z DEBUG logger] Settling: 1 / 9 -> 11.11%
    [2018-12-03T14:34:52Z DEBUG logger] Settling: 2 / 9 -> 22.22%
    [2018-12-03T14:34:56Z DEBUG logger] Settling: 3 / 9 -> 33.33%
    [2018-12-03T14:34:57Z DEBUG logger] Settling: 4 / 9 -> 44.44%
    [2018-12-03T14:35:01Z DEBUG logger] Settling: 5 / 9 -> 55.56%
    [2018-12-03T14:35:02Z DEBUG logger] Settling: 6 / 9 -> 66.67%
    [2018-12-03T14:35:06Z DEBUG logger] Settling: 7 / 9 -> 77.78%
    [2018-12-03T14:35:07Z DEBUG logger] Settling: 8 / 9 -> 88.89%
    [2018-12-03T14:35:11Z DEBUG logger] Settling: 9 / 9 -> 100.00%
    [2018-12-03T14:35:12Z DEBUG logger] Settled ["00_6521_7210_10_0200", "00_7210_6521_10_0100", "00_0010_7211_10_0100"]
    [2018-12-03T14:35:20Z DEBUG logger] PNG Tick
    [2018-12-03T14:35:30Z DEBUG logger] PNG Tick
    [2018-12-03T14:35:40Z DEBUG logger] PNG Tick
    [2018-12-03T14:35:50Z DEBUG logger] PNG Tick
    [2018-12-03T14:36:00Z DEBUG logger] PNG Tick
    [2018-12-03T14:36:05Z DEBUG logger] CSV tick
    [2018-12-03T14:36:05Z DEBUG logger::csv_generator] Needs header: is new file = true, ID hash differs = true, filename = TextData_20181203.log
    [2018-12-03T14:36:10Z DEBUG logger] PNG Tick
    [2018-12-03T14:36:20Z DEBUG logger] PNG Tick
    [2018-12-03T14:36:30Z DEBUG logger] PNG Tick
    [2018-12-03T14:36:40Z DEBUG logger] PNG Tick
    [2018-12-03T14:36:50Z DEBUG logger] PNG Tick
    [2018-12-03T14:37:00Z DEBUG logger] PNG Tick
    [2018-12-03T14:37:04Z DEBUG logger] CSV tick

Nachdem die Konfiguration geladen und die Verbindung über den seriellen Port zum VBus/USB-Adapter
hergestellt wurde, beginnt die sogenannte "Settling"-Phase. In diese Zeit werden noch keine Daten
weiterverarbeitet oder aufgezeichnet, sondern es wird sich erst einmal ein Überblick über die
auf dem VBus zur Verfügung stehenden Daten verschafft. Jedes Mal, wenn ein bisher unbekanntes
VBus-Paket empfangen wird, verlängert sich diese Phase noch einmal. Wenn lange genug nur
bekannte Pakete empfangen wurden, endet diese Phase mit der Ausgabe der Liste der empfangenen
Pakete:

    [2018-12-03T14:35:12Z DEBUG logger] Settled ["00_6521_7210_10_0200", "00_7210_6521_10_0100", "00_0010_7211_10_0100"]

Diese Liste kann bei Bedarf in die `config.toml` unter dem Wert `known_packet_ids` übernommen
werden, um spätere Aufrufe des Programms schneller durch die "Settling"-Phase zu bekommen.


## Produktivbetrieb

In dem Beispiel oben war das `png_tick_interval` auf 10 Sekunden  und das `csv_tick_interval`
auf 60 Sekunden eingestellt. In den Debugausgaben nach der "Settling"-Phase kann man die
unterschiedlichen Ticks sehen. Was aber bei Zeitstempel `2018-12-03T14:36:05Z` besonders auffällt,
ist dass der vorherige Schritt zum Erstellen der PNG ca. 5 Sekunden gedauert hat. In der Zeit ist 
auch die Prozessorlast stark gestiegen. Das verbessert sich aber, sobald nach dem ganzen
Feintuning an dem Programm die Release-Variante erstellen werden kann. Wenn die mit

    cargo build --release

erstellt wurde und mit

    RUST_LOG=debug target/release/logger

gestartet wird, werden PNG und CSV in der selben Sekunde erstellt und auch die
Prozessorlasterhöhung während dieser Zeit ist nicht mehr so deutlich wie bei der Debug-Variante.

Um das Programm dann später ohne Debugausgaben zu starten, kann man einfach den Teil mit `RUST_LOG=debug` weglassen:

    target/release/logger

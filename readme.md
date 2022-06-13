*Deutsch/German*
Ein kleines Programm, in das man eine Liste von Netzen mit gegebener Hostanzahl eingeben kann.
Bei entsprechender Angabe des IP-Adressbereichs wird dann alles gesubnettet und als Tabelle ausgegeben.

Es gibt zwar kein CLI, aber man kann die Daten eben im Code anpassen und dann ausführen.

Gehe in die src/main.rs, und passe die erforderlichen Netze in Zeile 25 an Tuple. In der ersten Komponente wird der Name des Netzes festgelegt, in der zweiten die Anzahl der nötigen Hosts.
Beispiel: Netz "Test" soll mindestens 3000 Hosts bekommen. Dafür schreibt man einfach `("Test", 3000)`.
Achtung, die Einträge werden mit Komma getrennt.

In Zeilen 20-21 wird der IP-Adressbereich festgelegt.
In Zeile 20 einfach die vier Komponenten der IP-Adresse eintippen, in Zeile 21 den Präfix der Subnetzmaske.

Dann halt noch ausführen :)
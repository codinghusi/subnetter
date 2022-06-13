*Deutsch/German*
Ein kleines Programm, in das man eine Liste von Netzen mit gegebener Hostanzahl eingeben kann.
Bei entsprechender Angabe des IP-Adressbereichs wird dann alles gesubnettet und als Tabelle ausgegeben.

Gehe in die src/main.rs, und passe die erforderlichen Netze in Zeile 38 an.
In der ersten Komponente des Tupels wird der Name des Netzes festgelegt, in der zweiten die Anzahl der nötigen Hosts.
Beispiel: Netz "Test" soll mindestens 3000 Hosts bekommen. Dafür schreibt man einfach `("Test", 3000)`.
Achtung, die Einträge werden mit Komma getrennt.

Dieses Projekt wurde für ein Abschlussprojekt in der FH Aachen (Datennetze und IT-Sicherheit) angefertigt.
Dort wird der IP-Adressbereich aus dem Vornamen und Nachnamen generiert.

Die Binary kann mit dem entsprechenden Namen ausgeführt werden.
Für genauere Hilfe kann die Hilfe aufgerufen werden: `subnetter.exe --help`
Beispiel: `subnetter.exe --vorname Max --nachname Mustermann`
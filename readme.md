*Deutsch/German*
Ein kleines Programm, in das man eine Liste von Netzen mit gegebener Hostanzahl eingeben kann.
Bei entsprechender Angabe des IP-Adressbereichs wird dann alles gesubnettet und als Tabelle ausgegeben.

Es gibt zwar kein CLI, aber man kann die Daten eben im Code anpassen und dann ausführen.

Gehe in die src/main.rs, und passe die erforderlichen Netze in Zeile 38 an.
In der ersten Komponente des Tupels wird der Name des Netzes festgelegt, in der zweiten die Anzahl der nötigen Hosts.
Beispiel: Netz "Test" soll mindestens 3000 Hosts bekommen. Dafür schreibt man einfach `("Test", 3000)`.
Achtung, die Einträge werden mit Komma getrennt.

Dieses Projekt wurde für ein Abschlussprojekt in der FH Aachen (Datennetze und IT-Sicherheit) angefertigt.
Dort wird der IP-Adressbereich aus dem Vornamen und Nachnamen generiert.
In der Datei src/main.rs in Zeile 33 muss dafür der Vor- und Nachname angepasst werden.

Dann halt noch ausführen :)
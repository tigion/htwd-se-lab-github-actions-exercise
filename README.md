# HTWD | Übung: GitHub Actions mit Unit-Tests

Das Repository enthält Übungsprojekte für die Praktikumsaufgabe zu [GitHub
Actions][08-github-actions] im Fach Software Engineering II.

[08-github-actions]: https://www.informatik.htw-dresden.de/~zirkelba/praktika/se/git-docs-collab/08-github-actions.html#_aufgabe_2_unit_tests

Enthalten sind einfache Anwendungen mit Unit-Tests in **Java**
([Maven], [JUnit]) und **Python** ([unittest]). Beide Beispiele bestehen aus
einer `Town`-Klasse mit zugehörigen Unit-Tests und stammen aus dem Praktikum
"Unit-Tests mit Java und Python".

[Maven]: https://maven.apache.org/
[JUnit]: https://junit.org/junit5/docs/current/user-guide/
[unittest]: https://docs.python.org/3/library/unittest.html

> [!WARNING]
> Ein Test schlägt bewusst fehl, da dieser im Zuge der Aufgabenbearbeitung
> angepasst wird.

## Übungsprojekte

Die Beispielprojekte für **Java** und **Python** befinden sich in getrennten
Verzeichnissen und enthalten jeweils eine `README.md`-Datei mit Informationen:

| Projekt                               | Beschreibung                                 |
| ------------------------------------- | -------------------------------------------- |
| [`example-java/`](/example-java/)     | Enthält die Java-Anwendung mit Unit-Tests.   |
| [`example-python/`](/example-python/) | Enthält die Python-Anwendung mit Unit-Tests. |

Für jedes Projekt werden im Laufe der Aufgabenbearbeitung die zugehörigen
**GitHub Action Workflows** im Verzeichnis `.github/workflows/` erstellt.

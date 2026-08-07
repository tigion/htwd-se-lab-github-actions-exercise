# HTWD | Übung: GitHub Actions mit Unit-Tests

Das Repository enthält Übungsprojekte für die Praktikumsaufgabe zu [GitHub
Actions][08-github-actions] im Fach Software Engineering II.

[08-github-actions]: https://www.informatik.htw-dresden.de/~zirkelba/praktika/se/git-docs-collab/08-github-actions.html#_aufgabe_2_unit_tests

Enthalten sind einfache Anwendungen mit Unit-Tests in **Java**, **Python** und
**Rust**. Die Beispiele bestehen aus einer `Town`-Klasse mit zugehörigen
Unit-Tests und stammen aus dem Praktikum "Unit-Tests mit Java und Python".

> [!WARNING]
> Ein Test schlägt bewusst fehl, da dieser im Zuge der Aufgabenbearbeitung
> angepasst wird.

## Übungsprojekte

Die folgenden Beispielprojekte zeigen die Umsetzung mit **Java**, **Python**
und **Rust**. Jedes Projekt enthält eine eigene `README.md` mit Informationen
zu Anforderungen, Ausführung und Tests.

| Projekt                               | Beschreibung                                 |
| ------------------------------------- | -------------------------------------------- |
| [`example-java/`](/example-java/)     | Enthält die Java-Anwendung mit Unit-Tests.   |
| [`example-python/`](/example-python/) | Enthält die Python-Anwendung mit Unit-Tests. |
| [`example-rust/`](/example-rust/)     | Enthält die Rust-Anwendung mit Unit-Tests.   |

Für jedes Projekt werden im Laufe der Aufgabenbearbeitung die zugehörigen
**GitHub Action Workflows** im Verzeichnis `.github/workflows/` erstellt.

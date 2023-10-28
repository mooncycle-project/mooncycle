# nix-bevy-wasm-trunk

[![deploy](https://github.com/j-brn/nix-bevy-wasm-trunk/actions/workflows/deploy.yml/badge.svg)](https://github.com/j-brn/nix-bevy-wasm-trunk/actions/workflows/deploy.yml)
[![test](https://github.com/j-brn/nix-bevy-wasm-trunk/actions/workflows/test.yml/badge.svg)](https://github.com/j-brn/nix-bevy-wasm-trunk/actions/workflows/test.yml)

[Bevy](https://github.com/bevyengine/bevy) breakout example with nix build and github pages deployment

## Prerequisites

- [Nix](https://github.com/NixOS/nix) has to be installed and
  [flakes and the experimental nix command have to be enabled](https://nixos.wiki/wiki/Flakes#Enable_flakes)
- (optional) install and setup [direnv](https://direnv.net/) to automatically enter the dev shell in this directory

## Usage

- `nix build` to build
- `nix flake check` to run tests
- `nix develop` to enter the dev shell (happens automatically when using direnv)
- `trunk serve` to start the development server

## TODOS

Eigenschaften Kreisel:
- `v: Vec2` Geschwindigkeit der Position in x und y
- `a: Vec2` Beschleundigung der Position in x und y
- `vr: f32` Rotation Speed (Einheit? 0-1 für Prozent)
- `tilt: Vec2`: Neigung (x und y, 0 - 1)
- `mass: iu8`: Masse
- `friction: f32`: Reibung (0 - 1)

Fragen:
- Rotieren die Spieler Kreisel in eine fest vorgegebene Richtung oder kann die geändert werden. Je nachdem ändert sich das Verhalten bei Kollision.


Task 1: Darstellung Kreisel 
- Drehgeschwindigkeit (Drehendes Sprite, evtl. mit einem schwarzen Punkt am Rand)
- Neigung (Pfeil)
- Geschwindigkeit? (zweiter Pfeil, macht es einfacher die Geschwindikeit von Gegnern einzuschätzen)
- Controls
- Drehgeschindigkeit (Healthbar über dem Kreisel), sollte für eigenen Kreisel ausblendbar sein
- Darstellung "Game Over"

Task 2: UI
- Start Menü
- Debug Mode: Direkter Start, Slider für die Eigenschaften des Kreisels, ein Gegner der auch über Slider gesteuert werden kann
- Score (wie wird der berechnet? Nur die Masse für den Anfang?)
- Geschwindigkeit des eigenen Kreisels als große Healthbar


Task 3: Gegner
- Sollten auch Kreisel sein die in unterschiedlichen Richtungen rotieren
- Einfärbung je nach Drehrichtung
- "KI": Nur dumme Kreisel oder sollen diese versuchen den Spieler dumm zu rammen, mit einer Strategie zu rammen

Task 4: Kreiselphysic
- Wie bewegt sich ein Kreisel ohne Kollision fort?
- Tilt verhalten
- Beschleunigungsverhalten
- Drift verhalten

Task 5: Kollisionen
- Kollisionen mit statischen Objekten (Wand)
- Kollisionen mit anderen Kreiseln
- Wie wirkt sich eine Kollision auf die einzelnen Attribute aus
- Wenn zwei Kreisel kollidieren, hat der schnellere Kreisel den Vorteil, er kann Masse vom anderen Kreisel anziehen

### Physik

- Eine hohe Masse verlangsamt das Neigung
- Ein gekippter Kreisel kippt immer weiter
  - Eine hohe Drehgeschwindgkeit veringert die Neigung
    - Bei voller `rv` ändert sich die Neigung nicht alleine
    - Bei einer `rv`  gegen 0 fällt der Kreisel um
  - Eine hohe Neigung erhöht beschleunigt die Neigung
    - Um so weiter der Kreisel aus der Balance ist, fällt er leichter um
- Drift
  - Eine hohe Drehgeschwindkeit verringert den Drift
    - Bei voller `rv` gibt es keinen Drift
  - Eine hohe Neigung erhöht erhöht den Drift
    - Wenn der Kreisel aufrecht steht gibt es keinen Drift
- Beschleunigung abhängig von der Neigung und der Masse
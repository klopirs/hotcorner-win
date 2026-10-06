# HotCorner Win

Le projet s'inspire du système de Hot Corners disponible sur Linux.

## Fonctionnalités

- Configuration des 4 coins de l'écran
- Afficher le bureau avec `Win + D`
- Ouvrir la vue des tâches avec `Win + Tab`
- Verrouiller Windows
- Ouvrir une application
- Régler le délai d'activation
- Régler la taille de la zone de détection
- Support multi-écrans
- Sauvegarde de la configuration
- Fonctionnement en arrière-plan avec le System Tray

## Compilation

Le projet nécessite Rust et Cargo.

```bash
cargo run
```

Pour compiler l'exécutable :

```bash
cargo build --release
```

L'exécutable sera disponible dans :

```text
target/release/hotcorner-win.exe
```

## Structure

```text
src/
├── ui/
│   ├── mod.rs
│   └── settings.rs
├── action.rs
├── config.rs
├── hotcorner.rs
├── main.rs
└── tray.rs
```

## Configuration

Les paramètres sont sauvegardés automatiquement dans :

```text
config.json
```

Ils peuvent être modifiés directement depuis l'interface de l'application.

## Technologies

- Rust
- Win32 API
- egui / eframe
- tray-icon
- Serde
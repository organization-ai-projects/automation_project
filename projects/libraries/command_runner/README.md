# Command Runner

`command_runner` est une bibliothèque Rust conçue pour exécuter des commandes système de manière robuste et ergonomique. Elle fournit des outils pour gérer les erreurs, capturer les sorties, et journaliser les exécutions.

## Fonctionnalités

- **Exécution stricte ou permissive** :
  - `run_cmd_ok` : Retourne une erreur si la commande échoue.
  - `run_cmd_allow_failure` : Retourne toujours la sortie, même en cas d'échec.
- **Gestion des erreurs** :
  - Types d'erreurs détaillés (`CommandError`).
  - Journalisation des commandes exécutées.
- **Troncature sécurisée** :
  - Les sorties longues sont tronquées de manière UTF-8 sûre.

## Installation

Ajoutez la dépendance suivante à votre `Cargo.toml` :

```toml
[dependencies]
command_runner = "0.1.0"
```

## Utilisation

### Exemple de base

```rust
use command_runner::{run_cmd_ok, CommandError};
use std::path::Path;

fn main() -> Result<(), CommandError> {
    let repo_path = Path::new("/chemin/vers/repo");
    let mut logs = Vec::new();

    let output = run_cmd_ok(repo_path, "git", &["status"], &mut logs)?;

    println!("Statut: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
```

### Modes d'exécution

- **Strict** :
  - Utilisez `run_cmd_ok` pour des commandes où un code de sortie non nul est considéré comme une erreur.
- **Permissif** :
  - Utilisez `run_cmd_allow_failure` pour capturer la sortie même si la commande échoue.

### Gestion des erreurs

Les erreurs sont encapsulées dans le type `CommandError` :

- `InvalidInput` : Entrée invalide pour la commande.
- `Io` : Erreur d'entrée/sortie lors de l'exécution.
- `NonZeroExit` : La commande a échoué avec un code de sortie non nul.

### Journalisation

Les journaux des commandes exécutées peuvent être collectés dans un `Vec<String>` :

```rust
let mut logs = Vec::new();
run_cmd_ok(repo_path, "ls", &["-la"], &mut logs)?;
for log in logs {
    println!("{}", log);
}
```

## Contribuer

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou une pull request sur le dépôt GitHub.

Pour plus de détails sur le workflow Git/GitHub utilisé dans ce projet, consultez la [documentation sur le versioning](../../../docs/versioning/git-github.md).

## Licence

Ce projet est sous licence MIT. Consultez le fichier `LICENSE` pour plus de détails.

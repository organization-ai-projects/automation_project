# Protocol Library

Une bibliothèque Rust pour la communication basée sur des commandes et événements typés avec validation et métadonnées.

## Version

Version actuelle : **1.0.0**

## Caractéristiques

- ✅ **Commandes et événements typés** - Types définis pour une meilleure sécurité et clarté
- ✅ **Validation robuste** - Validation complète avec messages d'erreur descriptifs
- ✅ **Métadonnées automatiques** - Timestamps et IDs uniques générés automatiquement
- ✅ **Sérialisation** - Support complet de serde pour JSON/binaire
- ✅ **Sécurité** - Limites de taille et validation de format pour prévenir les abus
- ✅ **Documentation complète** - Docs inline et exemples

## Installation

Ajoutez à votre `Cargo.toml` :

```toml
[dependencies]
protocol = { path = "../path/to/protocol" }
```

## Utilisation

### Créer et valider une commande

```rust
use protocol::{Command, CommandType};

// Créer une nouvelle commande
let cmd = Command::new(
    "execute_task".to_string(),
    CommandType::Execute,
    r#"{"task": "example", "params": {}}"#.to_string()
);

// Valider la commande
match cmd.validate() {
    Ok(()) => println!("Commande valide!"),
    Err(e) => eprintln!("Erreur de validation: {}", e),
}
```

### Créer et valider un événement

```rust
use protocol::{Event, EventType};

// Créer un nouvel événement
let event = Event::new(
    "task_completed".to_string(),
    EventType::Completed,
    r#"{"result": "success", "duration_ms": 1234}"#.to_string()
);

// Valider l'événement
if let Err(e) = event.validate() {
    eprintln!("Événement invalide: {}", e);
}
```

### Types de commandes disponibles

- `Execute` - Exécuter une tâche ou opération
- `Query` - Interroger pour obtenir des informations
- `Update` - Mettre à jour des données existantes
- `Delete` - Supprimer des données ou ressources
- `Create` - Créer de nouvelles ressources
- `Subscribe` - S'abonner à des événements ou mises à jour
- `Unsubscribe` - Se désabonner d'événements ou mises à jour
- `Configure` - Commande de configuration
- `Custom` - Type de commande personnalisé

### Types d'événements disponibles

- `Started` / `Stopped` - Démarrage/arrêt du système
- `Created` / `Updated` / `Deleted` - Modifications de données
- `Error` / `Warning` / `Info` - Niveaux de log
- `Completed` / `Failed` - Résultats de tâches
- `Progress` - Mise à jour de progression
- `StateChanged` - Changement d'état
- `Custom` - Type d'événement personnalisé

### Métadonnées

Les métadonnées sont automatiquement générées avec :

- **Timestamp** : millisecondes depuis UNIX epoch
- **ID unique** : combinaison de timestamp et compteur atomique

```rust
use protocol::Metadata;

// Créer avec timestamp actuel
let metadata = Metadata::now();

// Accéder aux données
println!("Timestamp: {}", metadata.timestamp);
println!("ID: {}", metadata.id);
println!("Format lisible: {}", metadata.timestamp_to_string());
```

### Gestion des erreurs

La bibliothèque fournit des erreurs de validation détaillées :

```rust
use protocol::{Command, CommandType, ValidationError};

let cmd = Command::new(
    "test command with spaces!".to_string(), // Nom invalide
    CommandType::Execute,
    "payload".to_string()
);

match cmd.validate() {
    Err(ValidationError::InvalidNameFormat(name)) => {
        println!("Nom invalide: {}", name);
    }
    Err(e) => println!("Autre erreur: {}", e),
    Ok(()) => println!("OK"),
}
```

Types d'erreurs :

- `EmptyName` - Nom vide ou contenant seulement des espaces
- `EmptyPayload` - Payload/données vides
- `InvalidNameFormat` - Nom contient des caractères invalides
- `PayloadTooLarge` - Payload dépasse la taille maximale (10 MB)
- `NameTooLong` - Nom dépasse la longueur maximale (256 caractères)
- `InvalidTimestamp` - Timestamp invalide (trop loin dans le futur)

## Limites de sécurité

Pour prévenir les abus et attaques :

### Commandes

- Longueur maximale du nom : **256 caractères**
- Taille maximale du payload : **10 MB**
- Caractères autorisés dans le nom : alphanumériques, `_`, `-`, `.`

### Événements

- Longueur maximale du nom : **256 caractères**
- Taille maximale des données : **10 MB**
- Caractères autorisés dans le nom : alphanumériques, `_`, `-`, `.`

### Timestamps

- Dérive maximale dans le futur : **1 heure**

## Tests

Lancer les tests :

```bash
cargo test -p protocol
```

## Évolutions futures possibles

- Support de payloads structurés avec `serde_json::Value`
- Compression des payloads volumineux
- Chiffrement des données sensibles
- Signatures cryptographiques pour l'authenticité
- Support des schémas de validation personnalisés

## Licence

À définir

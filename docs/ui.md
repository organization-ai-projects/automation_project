# UI

## 1. Règles

- Toute UI produit est un **bundle WASM** chargé par `central_ui`.
- Une UI ne dépend jamais d’un backend produit.
- Toute action passe par `engine` via `protocol`.

## 2. Contrat minimal UI

- Connexion WS à `engine`
- Auth user session
- Send Command / receive Events

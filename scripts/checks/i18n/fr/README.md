# Scripts de verification

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les scripts de validation et de controle pour le repository.

## Verifications disponibles

### check_stable_deps.sh

**Objectif**: appliquer la regle 2 de la structure stable/unstable - un produit stable ne doit pas dependre d'un produit unstable.

**Utilisation**:

```bash
./scripts/checks/check_stable_deps.sh
```

**Comportement**:

- Scanne tous les `Cargo.toml` dans `projects/products/stable/`
- Detecte les dependances `path` vers `projects/products/unstable/`
- Remonte les violations trouvees
- Retourne `0` en succes, `1` en echec

**Integration CI**: execute automatiquement via `.github/workflows/check_stable_deps.yml`.

**Documentation liee**: `projects/products/README.md`.

### check_layer_boundaries.sh

**Objectif**: faire respecter les frontieres de dependances entre couches du workspace pour eviter la derive d'architecture.

**Utilisation**:

```bash
./scripts/checks/check_layer_boundaries.sh
```

**Comportement**:

- Lance `cargo metadata` pour inspecter le graphe de dependances
- Classe les crates par chemin:
  - `projects/libraries/*` => `library`
  - `projects/products/*` => `product`
- Echoue si une dependance `library -> product` est detectee

**Integration CI**: execute automatiquement dans `.github/workflows/ci_reusable.yml`.

**Documentation liee**: `documentation/technical_documentation/library_layer_boundaries.md`.

### analyze_layer_anomalies.sh

**Objectif**: outil d'analyse semi-automatisee pour aider les decisions d'architecture sur le modele strict adjacent-only.

**Utilisation**:

```bash
./scripts/checks/analyze_layer_anomalies.sh \
  --json-out /tmp/layer_anomalies.json
```

Options utiles:

- `--map-file <path>` pour surcharger les hypotheses provisoires crate->couche.
- `--protocol-layer <L1|L2|UNDECIDED>` est deprecie et ignoree (conservee pour compatibilite).
- `--fail-on-anomaly true` pour l'utiliser en mode bloquant experimental.

**Comportement**:

- Lance `cargo metadata` et extrait les aretes de dependances workspace
- Construit une vue provisoire des couches (surchageable par map-file)
- Remonte:
  - dependances `library -> product`
  - aretes laterales / montantes / non-adjacentes
  - crates non mappees et hotspots ambigus
  - signaux de cycle (signal base sur `tsort`)
- Peut produire une sortie lisible et un export JSON.

## Ajouter un nouveau check

1. Creer le script dans ce repertoire
2. Le rendre executable: `chmod +x script_name.sh`
3. Documenter le script dans ce `README`
4. Ajouter un workflow CI si necessaire dans `.github/workflows/`
5. Respecter la convention de code retour: `0 = succes`, `non-zero = echec`

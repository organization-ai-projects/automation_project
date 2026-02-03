# Structure de Documentation Multi-Langue

Ce document décrit la structure de documentation multi-langue implémentée dans ce dépôt.

## Vue d'ensemble

Le dépôt supporte maintenant la documentation en plusieurs langues :
- **Anglais (en)** : Source de vérité canonique
- **Français (fr)** : Traduction

## Structure par Type de Documentation

### 1. Documentation au Niveau Racine

Les fichiers de documentation au niveau racine ont des variantes linguistiques avec le suffixe `.fr.md` :

```
README.md           # Anglais (canonique)
README.fr.md        # Traduction française
CONTRIBUTING.md     # Anglais (canonique)
CONTRIBUTING.fr.md  # Traduction française
```

Chaque fichier inclut un lien vers sa variante linguistique en haut.

### 2. Documentation Technique Principale (`/documentation/`)

Le dossier de documentation principal utilise une structure hiérarchique avec des sélecteurs de langue :

```
documentation/
├── TOC.md              # Index anglais
├── TOC.fr.md           # Index français
└── technical_documentation/
    ├── assets/         # Ressources partagées (non dupliquées par langue)
    │   └── architecture_bootstrap.png
    ├── en/             # Documentation anglaise (canonique)
    │   ├── TOC.md
    │   ├── ARCHITECTURE.md
    │   ├── documentation.md
    │   ├── metadata.md
    │   ├── registry.md
    │   ├── system_processes.md
    │   └── projects/
    │       ├── TOC.md
    │       ├── projects_libraries.md
    │       └── projects_products.md
    └── fr/             # Traductions françaises
        ├── TOC.md
        ├── ARCHITECTURE.md
        ├── documentation.md
        ├── metadata.md
        ├── registry.md
        ├── system_processes.md
        └── projects/
            ├── TOC.md
            ├── projects_libraries.md
            └── projects_products.md
```

**Points clés :**
- Les ressources sont partagées au niveau `technical_documentation/assets/`
- Pas de duplication des ressources par langue
- Les liens vers les ressources utilisent `../assets/` depuis les dossiers de langue

### 3. Documentation des Bibliothèques (`/projects/libraries/*/documentation/`)

La documentation de chaque bibliothèque suit le même modèle :

```
projects/libraries/<nom_bibliothèque>/
├── README.md
├── README.fr.md
└── documentation/
    ├── en/
    │   ├── TOC.md
    │   └── *.md (fichiers de documentation)
    └── fr/
        ├── TOC.md
        └── *.md (traductions)
```

**Exemple** : bibliothèque `common_json` :
```
projects/libraries/common_json/
├── README.md
├── README.fr.md
└── documentation/
    ├── en/
    │   ├── TOC.md
    │   ├── access.md
    │   ├── deserialize.md
    │   ├── error.md
    │   ├── json_array_builder.md
    │   ├── json_object_builder.md
    │   ├── macros.md
    │   ├── merge.md
    │   ├── serialize.md
    │   └── value.md
    └── fr/
        ├── TOC.md
        ├── access.md
        ├── deserialize.md
        ├── error.md
        ├── json_array_builder.md
        ├── json_object_builder.md
        ├── macros.md
        ├── merge.md
        ├── serialize.md
        └── value.md
```

### 4. Documentation des Produits (`/projects/products/*/documentation/`)

Les produits suivent la même structure que les bibliothèques :

```
projects/products/<nom_produit>/
├── README.md
├── README.fr.md
└── documentation/
    ├── en/
    │   ├── TOC.md
    │   └── *.md
    └── fr/
        ├── TOC.md
        └── *.md
```

### 5. Documentation des Scripts (`/scripts/`)

Les scripts utilisent une structure plate avec le suffixe `.fr.md` pour les variantes françaises :

```
scripts/
├── README.md
├── README.fr.md
├── TOC.md
├── TOC.fr.md
├── automation/
│   ├── README.md
│   ├── README.fr.md
│   ├── TOC.md
│   └── TOC.fr.md
└── versioning/
    ├── README.md
    ├── README.fr.md
    ├── TOC.md
    └── TOC.fr.md
```

**Justification** : La documentation des scripts est principalement composée de fichiers README, donc une structure plate avec des suffixes de langue est plus pratique que des sous-répertoires.

### 6. Documentation des Outils (`/tools/`)

Les outils suivent la même structure plate que les scripts :

```
tools/bot_ci_harness/
├── README.md
└── README.fr.md
```

## Règles de Navigation

### Références Croisées

1. Les **documents anglais** pointent vers d'autres documents anglais
2. Les **documents français** pointent vers d'autres documents français
3. Les sélecteurs de langue en haut des documents majeurs permettent de changer de langue

### Modèles de Liens

- Docs racine → Documentation : `documentation/TOC.md` (EN) ou `documentation/TOC.fr.md` (FR)
- Documentation → Racine : `../README.md` (EN) ou `../README.fr.md` (FR)
- Dans les dossiers doc : Liens relatifs dans le même dossier de langue
- Ressources : Toujours utiliser un chemin relatif vers le dossier `assets/` partagé

### Exemples

De `documentation/technical_documentation/en/ARCHITECTURE.md` :
```markdown
![Bootstrap](../assets/architecture_bootstrap.png)
[Back to Technical TOC](TOC.md)
```

De `projects/libraries/common_json/documentation/en/TOC.md` :
```markdown
[Back to common_json README](../../README.md)
```

De `projects/libraries/common_json/documentation/fr/TOC.md` :
```markdown
[Retour à common_json README](../../README.fr.md)
```

## Ajout de Nouvelle Documentation

### Pour un nouveau document dans un emplacement existant :

1. Créer la version anglaise dans le dossier `en/`
2. Créer la traduction française dans le dossier `fr/`
3. Mettre à jour le fichier TOC.md correspondant dans les deux langues

### Pour une nouvelle bibliothèque/produit :

1. Créer `README.md` et `README.fr.md` à la racine
2. Créer les dossiers `documentation/en/` et `documentation/fr/`
3. Ajouter `TOC.md` dans les deux dossiers de langue
4. Ajouter les fichiers de documentation en suivant le modèle ci-dessus

### Pour les scripts :

1. Créer `README.md` et `README.fr.md` dans le même répertoire
2. Créer `TOC.md` et `TOC.fr.md` si nécessaire
3. S'assurer que les liens dans les fichiers français pointent vers les variantes françaises

## Directives de Traduction

1. **L'anglais est canonique** : Tout le contenu provient de l'anglais
2. **Pas de changements de contenu en traduction** : Les versions françaises doivent traduire, pas réécrire
3. **Maintenir la structure** : Garder les mêmes sections, en-têtes et organisation
4. **Mettre à jour les liens** : S'assurer que les docs françaises pointent vers les variantes françaises
5. **Ne pas dupliquer les ressources** : Utiliser les dossiers de ressources partagées

## Résumé

- **Plus de 200 fichiers de documentation** supportent maintenant l'anglais et le français
- **Zéro duplication de ressources** - toutes les images/ressources sont partagées
- **Structure cohérente** dans tous les types de documentation
- **Références croisées fonctionnelles** entre tous les documents
- **Facile à étendre** pour des langues supplémentaires à l'avenir

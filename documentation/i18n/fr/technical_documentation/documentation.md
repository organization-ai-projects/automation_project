# Documentation automatisee

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

- [Retour au TOC technique](TOC.md)

## Introduction

Ce document decrit les objectifs et les fonctionnalites de la documentation automatisee dans `automation_project`.

---

## 1. Documentation automatisee

### 1.0 Convention

La documentation au niveau crate vit dans un dossier `documentation/` a la racine de la crate.
Les regles de contribution vivent a la racine du repository (`CONTRIBUTING.md`).

### 1.1 Objectifs

La documentation est un composant essentiel du projet et doit etre generee automatiquement pour rester a jour et coherente avec le code.

### 1.2 Ownership

- La documentation workspace vit dans `documentation/technical_documentation/`.
- La documentation crate vit dans `projects/**/documentation/` et doit se concentrer sur l'usage specifique de la crate.

### 1.3 Fonctionnalites detaillees

Les fonctionnalites suivantes sont prevues pour la documentation automatisee :

1. **Generation automatique** :
   - Utiliser `cargo doc` pour produire la documentation Rust standard.
   - Enrichir la documentation avec des exemples de code, des schemas et des explications detaillees.

2. **Export multi-format (optionnel)** :
   - **HTML** : pour consultation en ligne.
   - **Markdown** : pour integration dans les depots Git.
   - **PDF** : pour livrables ou consultation hors ligne.

   L'export vers d'autres formats peut etre ajoute plus tard selon les besoins.

3. **Integration aux workflows (planifie)** :
   - Generation automatique de la documentation pour les modules ajoutes ou modifies dans les workflows.
   - Enrichissement des exemples via des workflows symboliques et neuraux.

4. **Compatibilite et standards** :
   - Respect des formats standardises comme Markdown et HTML.
   - Documentation des dependances critiques et des versions minimales requises.

5. **Verification et qualite (planifie)** :
   - Definir des regles de linting specifiques au projet.
   - Automatiser les verifications de conventions via :
     - **Clippy** : pour les regles Rust standard.
     - Des regles personnalisees adaptees au projet.
   - Generer des rapports detailles sur les violations detectees et les suggestions d'amelioration.
   - Proposer des corrections automatiques quand c'est possible.

> Ces fonctionnalites visent une documentation complete, a jour et adaptee aux besoins varies des utilisateurs et des developpeurs.

La documentation automatisee est consideree comme un artefact de premier rang du systeme, au meme niveau que le code ou les workflows.

### 1.3 Exemples

#### Exemple 1 : generer la documentation

Pour generer la documentation de tout le workspace, utilisez :

```bash
cargo doc --workspace --open
```

Cela produit une documentation HTML pour toutes les crates du workspace et l'ouvre dans le navigateur par defaut.

#### Exemple 2 : exporter en PDF

Actuellement, l'export PDF necessite un outil tiers. Etapes :

1. Generer la documentation HTML avec `cargo doc`.
2. Convertir le HTML en PDF avec un outil comme `wkhtmltopdf` :

```bash
wkhtmltopdf target/doc/index.html documentation.pdf
```

#### Exemple 3 : lint de la documentation

Pour verifier la conformite de la documentation aux standards, lancez :

```bash
cargo clippy -- -D warnings
```

Cela applique les regles de linting et remonte les problemes detectes dans la documentation.

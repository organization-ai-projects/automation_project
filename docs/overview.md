# Vue d'ensemble

## Introduction

Ce document présente une vue d'ensemble du projet `automation_project`, un workspace d'automatisation avancé. Pour des détails spécifiques, consultez les sections suivantes :

- [Organisation multi-projets](projects/organization.md)
- [Principes non négociables](principles.md)
- [Documentation automatisée](documentation.md)
- [Orchestration des workflows](workflows.md)
- [Planification consolidée](planning.md)

Cette vue d’ensemble ne décrit pas les détails d’implémentation, volontairement couverts dans les documents spécialisés.

---

## 1. Objectif

L’objectif de ce projet est de construire un **workspace d’automatisation avancé** (type Google / Microsoft) capable d’orchestrer **plusieurs projets simultanément**, avec une automatisation avancée du cycle de développement logiciel.

Le système vise à automatiser :

- La génération de code
- Le linting et la validation structurelle
- La documentation
- L’application et l’évolution des bonnes pratiques
- L’orchestration de workflows complets (analyse → génération → validation → itération)

Le projet est **100 % Rust**, avec :

- une base **symbolique forte** (règles, structures, invariants)
- une composante **neuronale optionnelle et activable** (Burn)

---

## 2. Concept fondamental : `automation_project`

### 2.1 Définition

**`automation_project` est le workspace racine.**

Ce n’est pas un simple projet, mais un **environnement global outillé**, capable de gérer **N projets hétérogènes**.

> `automation_project` = conteneur racine + registry + tooling + orchestration

Il peut contenir :

- des **produits finaux** (applications, services, outils)
- des **libraries / SDK** (réutilisables, versionnées)

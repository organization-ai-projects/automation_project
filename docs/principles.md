# Principes non négociables

## Introduction

Ce document énumère les principes fondamentaux qui guident le développement du projet `automation_project`. Pour une vue d'ensemble, consultez [Vue d'ensemble](overview.md).

---

## 1. Principes non négociables

Ces principes s’appliquent dès les premières versions et priment sur toute considération de rapidité ou de confort.

- Multi-projets **dès le design**
- Isolation stricte des états
- Symbolique prioritaire
- Neuronal optionnel
- APIs claires et stables
- Architecture pensée long terme
- Aucune dépendance circulaire entre les crates :
  - `engine` ne dépend jamais de `ui`
  - `ai` ne dépend jamais de `engine`
  - `symbolic` et `neural` ne connaissent pas le workspace

Toute évolution du projet doit préserver ces principes ou justifier explicitement leur révision.

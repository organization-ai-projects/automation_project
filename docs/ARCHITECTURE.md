# ARCHITECTURE

## 1. Vue d’ensemble

Ce document fournit une vue d’ensemble de l’architecture du projet `automation_project`, en se concentrant sur l’arborescence des projets.

---

## 2. Structure des projets

```plaintext
automation_project/
├── projects/
│   ├── products/
│   │   ├── core/
│   │   │   ├── launcher/         # Bootstrap initial
│   │   │   ├── engine/           # Hub d’exécution et workflows
│   │   │   ├── central_ui/       # Interface utilisateur centrale
│   │   │   └── watcher/          # Superviseur global
│   │   ├── app/                 # Application exemple (avec dépendances, non requise au fonctionnement du workspace)
│   │   └── admin-ui/            # Interface d’administration (exemple, non requise au fonctionnement du workspace)
│   └── libraries/
│       ├── common/               # Types, helpers neutres
│       ├── protocol/             # Contrat de communication WS
│       ├── security/             # Authentification et permissions
│       ├── symbolic/             # Logique symbolique
│       ├── neural/               # Logique neuronale (optionnelle)
│       ├── ai/                   # Orchestrateur IA
│       └── ui/                   # Composants UI communs
└── .automation_project/
    ├── registry.json             # Registry centralisé des produits et UIs
    ├── settings.json
    ├── cache/
    └── logs/
```

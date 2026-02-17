# Documentation des scripts Git

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les scripts qui utilisent **uniquement** la CLI `git`.

## Workflows

Pour la documentation complete des workflows:

- **[Sync After PR Merge](sync_after_pr.md)** - Synchroniser les branches locales apres un merge PR, en manuel ou via `cleanup_after_pr.sh`

## Role dans le projet

Ce repertoire couvre les operations Git pures, independantes d'une plateforme d'hebergement.
Il interagit principalement avec:

- Les repositories Git locaux
- Les remotes Git
- Les orchestrateurs parents
- Les conventions de message de commit

## Structure du repertoire

```plaintext
git/
├── README.md (ce fichier, version EN canonique)
├── TOC.md
├── sync_after_pr.md           # Workflow de sync apres merge PR
├── create_branch.sh           # Creation de branches avec validation
├── delete_branch.sh           # Suppression de branches
├── push_branch.sh             # Push vers remote
├── clean_branches.sh          # Nettoyage des branches obsoletes
├── clean_local_gone.sh        # Suppression des branches locales [gone]
├── create_work_branch.sh      # Creation de branches de travail
├── finish_branch.sh           # Cloture de branches de travail
├── add_commit_push.sh         # Add/commit/push avec validation
├── create_after_delete.sh     # Recreation de branche depuis base
└── cleanup_after_pr.sh        # Nettoyage/sync apres merge PR
```

## Fichiers

- `README.md`: Ce document.
- `TOC.md`: Index des scripts Git.
- `sync_after_pr.md`: Workflow de sync apres merge.
- `create_branch.sh`: Creation de branche avec validation.
- `delete_branch.sh`: Suppression de branche.
- `push_branch.sh`: Push de branche.
- `clean_branches.sh`: Nettoyage de branches obsoletes.
- `clean_local_gone.sh`: Nettoyage des branches a remote disparu.
- `create_work_branch.sh`: Creation de branche de travail.
- `finish_branch.sh`: Cloture de branche de travail.
- `add_commit_push.sh`: Add/commit/push avec validation du message.
- `create_after_delete.sh`: Recreation de branche.
- `cleanup_after_pr.sh`: Nettoyage/synchronisation apres merge PR.

## Validation des messages de commit

Le script `add_commit_push.sh` applique le format conventionnel:

`<type>(<scope>): <message>` ou `<type>: <message>`

Types autorises:
`feature`, `feat`, `fix`, `fixture`, `doc`, `docs`, `refactor`, `test`, `tests`, `chore`

Exemples:

- `feat(auth): add user authentication`
- `fix: resolve null pointer exception`
- `docs(readme): update installation instructions`
- `docs(.github): add default PR template`

Bypass (deconseille):

- `./add_commit_push.sh "message" --no-verify`
- `SKIP_COMMIT_VALIDATION=1 git commit -m "message"`

## Portee

Les scripts de ce repertoire doivent:

- Utiliser uniquement `git` (pas `gh` ni autre CLI de forge)
- Effectuer des operations Git pures (branches/commits/working tree)
- Rester portables quel que soit l'hebergeur Git
- Appliquer les conventions du projet (noms de branches, format de commit)

Si un script doit interagir avec GitHub, il doit aller dans `file_versioning/github/` ou dans une couche orchestrateur adaptee.

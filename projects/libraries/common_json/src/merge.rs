// Documentation déplacée dans docs/merge.md. Référez-vous à ce fichier pour les détails.

use crate::Json;
use crate::array_diff::ArrayDiff;
use crate::json_diff::JsonDiff;
use crate::merge_strategy::MergeStrategy;
use crate::value::JsonMap;
use std::collections::HashMap;

/// Fusionne deux valeurs JSON avec la stratégie spécifiée.
///
/// # Stratégies
///
/// - `Replace` : retourne simplement `source`
/// - `DeepMerge` : fusionne récursivement les objets
/// - `Concat` : comme `DeepMerge` mais concatène les tableaux
pub fn merge(target: &Json, source: &Json, strategy: MergeStrategy) -> Json {
    match strategy {
        MergeStrategy::Replace => source.clone(),
        MergeStrategy::DeepMerge => deep_merge(target, source),
        MergeStrategy::Concat => concat_merge(target, source),
    }
}

/// Fusion profonde de deux valeurs JSON.
///
/// - Les objets sont fusionnés récursivement
/// - Les tableaux et autres types sont remplacés par la source
pub fn deep_merge(target: &Json, source: &Json) -> Json {
    match (target, source) {
        (Json::Object(target_map), Json::Object(source_map)) => {
            let mut result = target_map.clone();
            for (key, source_value) in source_map {
                let merged_value = match result.get(key) {
                    Some(target_value) => deep_merge(target_value, source_value),
                    None => source_value.clone(),
                };
                result.insert(key.clone(), merged_value);
            }
            Json::Object(result)
        }
        _ => source.clone(),
    }
}

/// Fusion avec concaténation des tableaux.
///
/// - Les objets sont fusionnés récursivement
/// - Les tableaux sont concaténés (target + source)
/// - Les autres types sont remplacés par la source
pub fn concat_merge(target: &Json, source: &Json) -> Json {
    match (target, source) {
        (Json::Object(target_map), Json::Object(source_map)) => {
            let mut result = target_map.clone();
            for (key, source_value) in source_map {
                let merged_value = match result.get(key) {
                    Some(target_value) => concat_merge(target_value, source_value),
                    None => source_value.clone(),
                };
                result.insert(key.clone(), merged_value);
            }
            Json::Object(result)
        }
        (Json::Array(target_arr), Json::Array(source_arr)) => {
            let mut result = target_arr.clone();
            result.extend(source_arr.iter().cloned());
            Json::Array(result)
        }
        _ => source.clone(),
    }
}

/// Calcule les différences entre deux valeurs JSON.
///
/// Compare `target` (ancienne valeur) à `source` (nouvelle valeur).
pub fn diff(target: &Json, source: &Json) -> JsonDiff {
    if target == source {
        return JsonDiff::Same;
    }

    match (target, source) {
        (Json::Object(target_map), Json::Object(source_map)) => {
            let mut differences = HashMap::new();

            for (key, target_value) in target_map {
                match source_map.get(key) {
                    Some(source_value) => {
                        let field_diff = diff(target_value, source_value);
                        if !matches!(field_diff, JsonDiff::Same) {
                            differences.insert(key.clone(), field_diff);
                        }
                    }
                    None => {
                        differences.insert(key.clone(), JsonDiff::Removed(target_value.clone()));
                    }
                }
            }

            for (key, source_value) in source_map {
                if !target_map.contains_key(key) {
                    differences.insert(key.clone(), JsonDiff::Added(source_value.clone()));
                }
            }

            if differences.is_empty() {
                JsonDiff::Same
            } else {
                JsonDiff::Object(differences)
            }
        }
        (Json::Array(target_arr), Json::Array(source_arr)) => {
            let mut array_diffs = Vec::new();
            let max_len = target_arr.len().max(source_arr.len());

            for i in 0..max_len {
                let diff = match (target_arr.get(i), source_arr.get(i)) {
                    (Some(t), Some(s)) if t == s => ArrayDiff::Same(t.clone()),
                    (Some(t), Some(s)) => ArrayDiff::Changed {
                        from: t.clone(),
                        to: s.clone(),
                    },
                    (Some(t), None) => ArrayDiff::Removed(t.clone()),
                    (None, Some(s)) => ArrayDiff::Added(s.clone()),
                    (None, None) => unreachable!(),
                };
                array_diffs.push(diff);
            }

            if array_diffs.iter().all(|d| matches!(d, ArrayDiff::Same(_))) {
                JsonDiff::Same
            } else {
                JsonDiff::Array(array_diffs)
            }
        }
        _ => JsonDiff::Changed {
            from: target.clone(),
            to: source.clone(),
        },
    }
}

/// Vérifie si un JSON contient un autre (pour filtrage/matching).
///
/// La vérification est récursive :
/// - Pour les objets : tous les champs de `needle` doivent exister dans `haystack` avec les mêmes valeurs
/// - Pour les tableaux : tous les éléments de `needle` doivent être présents dans `haystack`
/// - Pour les primitives : égalité stricte
pub fn contains(haystack: &Json, needle: &Json) -> bool {
    match (haystack, needle) {
        (Json::Object(h), Json::Object(n)) => n
            .iter()
            .all(|(key, value)| h.get(key).is_some_and(|h_value| contains(h_value, value))),
        (Json::Array(h), Json::Array(n)) => n
            .iter()
            .all(|needle_item| h.iter().any(|h_item| contains(h_item, needle_item))),
        _ => haystack == needle,
    }
}

/// Aplatit un objet JSON imbriqué en clés avec notation pointée.
///
/// Transforme une structure hiérarchique en objet plat où les clés
/// représentent le chemin complet vers chaque valeur.
///
/// # Limitations
///
/// - Seuls les objets sont aplatis, pas les tableaux
/// - Les clés contenant des points peuvent causer des ambiguïtés
pub fn flatten(value: &Json) -> Json {
    let mut result = JsonMap::new();
    flatten_recursive(value, String::new(), &mut result);
    Json::Object(result)
}

fn flatten_recursive(value: &Json, prefix: String, result: &mut JsonMap) {
    match value {
        Json::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_recursive(val, new_prefix, result);
            }
        }
        _ => {
            if !prefix.is_empty() {
                result.insert(prefix, value.clone());
            }
        }
    }
}

/// Reconstruit un objet imbriqué depuis des clés avec notation pointée.
///
/// Opération inverse de [`flatten`].
pub fn unflatten(value: &Json) -> Json {
    match value {
        Json::Object(map) => {
            let mut result = Json::Object(JsonMap::new());
            for (key, val) in map {
                set_nested_value(&mut result, key, val.clone());
            }
            result
        }
        _ => value.clone(),
    }
}

fn set_nested_value(root: &mut Json, path: &str, value: Json) {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = root;

    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            if let Json::Object(map) = current {
                map.insert((*part).to_string(), value);
            }
            return;
        }

        if let Json::Object(map) = current {
            if !map.contains_key(*part) {
                map.insert((*part).to_string(), Json::Object(JsonMap::new()));
            }
            current = map.get_mut(*part).unwrap();
        }
    }
}

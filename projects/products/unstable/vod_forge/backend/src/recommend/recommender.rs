use crate::catalog::Catalog;
use crate::playback::History;
use crate::recommend::recommend_rule::RecommendRule;
use std::collections::HashSet;

pub struct Recommender;

impl Recommender {
    pub fn recommend(
        catalog: &Catalog,
        history: &History,
        profile: &str,
        rule: &RecommendRule,
    ) -> Vec<String> {
        let watched: HashSet<String> = history
            .entries
            .iter()
            .filter(|e| e.profile_id.0 == profile)
            .map(|e| e.episode_id.clone())
            .collect();

        let mut result: Vec<String> = Vec::new();

        for title in catalog.titles.values() {
            for season in title.seasons.values() {
                for episode in season.episodes.values() {
                    if rule.unwatched_only && watched.contains(&episode.id) {
                        continue;
                    }
                    result.push(episode.id.clone());
                }
            }
        }

        result.sort();
        result
    }
}

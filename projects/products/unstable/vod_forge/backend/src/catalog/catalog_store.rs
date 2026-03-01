use crate::catalog::catalog::Catalog;
use crate::catalog::episode::Episode;
use crate::catalog::season::Season;
use crate::catalog::title::Title;
use crate::catalog::title_id::TitleId;
use crate::diagnostics::BackendError;

pub struct CatalogStore {
    pub catalog: Catalog,
}

impl CatalogStore {
    pub fn new() -> Self {
        CatalogStore {
            catalog: Catalog::default(),
        }
    }

    pub fn add_title(&mut self, title_id: &str, name: &str, year: u16) -> Result<(), BackendError> {
        let id = TitleId::from(title_id);
        if self.catalog.titles.contains_key(&id) {
            return Err(BackendError::Catalog(format!(
                "title {} already exists",
                title_id
            )));
        }
        let title = Title {
            id: id.clone(),
            name: name.to_string(),
            year,
            seasons: std::collections::BTreeMap::new(),
        };
        self.catalog.titles.insert(id, title);
        Ok(())
    }

    pub fn add_episode(
        &mut self,
        title_id: &str,
        season_num: u32,
        episode_num: u32,
        name: &str,
        duration_secs: u32,
    ) -> Result<(), BackendError> {
        let id = TitleId::from(title_id);
        let title = self
            .catalog
            .titles
            .get_mut(&id)
            .ok_or_else(|| BackendError::Catalog(format!("title {} not found", title_id)))?;
        let season = title.seasons.entry(season_num).or_insert_with(|| Season {
            number: season_num,
            episodes: std::collections::BTreeMap::new(),
        });
        let ep_id = format!("{}-s{:02}e{:02}", title_id, season_num, episode_num);
        let episode = Episode {
            id: ep_id,
            season: season_num,
            number: episode_num,
            name: name.to_string(),
            duration_secs,
        };
        season.episodes.insert(episode_num, episode);
        Ok(())
    }

    pub fn list(&self) -> Vec<&Title> {
        self.catalog.titles.values().collect()
    }
}

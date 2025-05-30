use std::path::Path;

use derive_getters::Getters;
use derive_more::From;
use orion_error::{ErrorOwe, ErrorWith};

use crate::{error::SpecResult, types::Persistable};

#[derive(Getters, Clone, Debug, From)]
pub struct GxlProject {
    main: String,
}

impl Persistable<GxlProject> for GxlProject {
    fn save_to(&self, path: &Path, _name: Option<String>) -> SpecResult<()> {
        let path = path.join("_gal");
        std::fs::create_dir_all(&path).owe_res().with(&path)?;
        std::fs::write(path.join(crate::const_vars::WORK_GXL), self.main.as_str()).owe_res()?;
        Ok(())
    }

    fn load_from(path: &Path) -> SpecResult<GxlProject> {
        let path = path.join("_gal/work.gxl");
        let main = std::fs::read_to_string(path).owe_res()?;
        Ok(Self { main })
    }
}

use std::path::PathBuf;

use async_trait::async_trait;
use derive_getters::Getters;
use orion_error::{ErrorOwe, ErrorWith, WithContext};
use orion_exchange::vars::{ValueDict, ValueType};

use crate::{
    addr::{GitAddr, path_file_name},
    error::{SpecReason, SpecResult, ToErr},
    module::TargetNodeType,
    tpl::{TPlEngineType, TplRender},
    types::{AsyncUpdateable, JsonAble, Localizable, Persistable, TomlAble},
};

use super::{refs::ModuleSpecRef, spec::ModuleSpec};

#[derive(Getters, Clone, Debug)]
pub struct RunningModule {
    name: String,
    spec: ModuleSpecRef,
    value: ValueDict,
    local: Option<PathBuf>,
}
impl Persistable<RunningModule> for RunningModule {
    fn save_to(&self, path: &PathBuf) -> SpecResult<()> {
        let root = path.join(self.name());
        std::fs::create_dir_all(&root).owe_conf()?;
        let spec_path = root.join("spec.toml");
        self.spec.save_toml(&spec_path)?;
        let json_path = root.join("value.json");
        self.value.save_json(&json_path)?;
        Ok(())
    }

    fn load_from(path: &PathBuf) -> SpecResult<Self> {
        let name = path_file_name(path)?;
        let spec_path = path.join("spec.toml");
        let spec = ModuleSpecRef::from_toml(&spec_path)?;
        let json_path = path.join("value.json");
        let value = ValueDict::from_json(&json_path)?;
        Ok(Self {
            name,
            spec,
            value,
            local: Some(path.clone()),
        })
    }
}
impl RunningModule {
    pub fn new(spec: ModuleSpecRef, value: ValueDict) -> Self {
        let name = spec.name().clone();
        Self {
            name,
            spec,
            value,
            local: None,
        }
    }
    pub async fn update(&self) -> SpecResult<()> {
        let mut ctx = WithContext::want("modul update");
        let local = self
            .local
            .clone()
            .ok_or(SpecReason::Miss("local-path".into()).to_err().with(&ctx))?;
        let spec = local.join("spec");
        if spec.exists() {
            ctx.with_path("spec", &spec);
            std::fs::remove_dir_all(&spec).owe_res().with(&ctx)?;
        }
        ctx.with_path("local", &local);
        self.spec.update_rename(&local, "spec").await.with(&ctx)?;

        Ok(())
    }
}

#[async_trait]
impl Localizable for RunningModule {
    async fn localize(&self) -> SpecResult<()> {
        let mut ctx = WithContext::want("modul localize");
        let local = self
            .local
            .clone()
            .ok_or(SpecReason::Miss("local-path".into()).to_err().with(&ctx))?;
        let tpl = local.join("spec");
        let dst = local.join("local");
        let data = local.join("value.json");
        ctx.with_path("dst", &dst);
        let spec = ModuleSpec::load_from(&tpl)?;
        spec.update_local(&tpl).await?;
        TplRender::render_path(TPlEngineType::Handlebars, &tpl, &dst, &data).with(&ctx)?;
        Ok(())
    }
}
pub fn make_modins_example() -> SpecResult<RunningModule> {
    let name = "postgresql";
    let spec = ModuleSpecRef::from(
        name,
        GitAddr::from("https://e.coding.net/dy-sec/galaxy-open/modspec.git").path(name),
        TargetNodeType::Host,
    );
    let mut dict = ValueDict::new();
    dict.insert("KEY", ValueType::from(name));
    dict.insert("CACHE_SIZE", ValueType::from(4));
    let sys = RunningModule::new(spec, dict);
    Ok(sys)
}

pub fn make_modins_new(name: &str, spec_center: &str) -> SpecResult<RunningModule> {
    let spec = ModuleSpecRef::from(
        name,
        GitAddr::from(spec_center).path(name),
        TargetNodeType::Host,
    );
    let mut dict = ValueDict::new();
    dict.insert("KEY", ValueType::from("postgresql"));
    dict.insert("CACHE_SIZE", ValueType::from(4));
    let sys = RunningModule::new(spec, dict);
    Ok(sys)
}
#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use crate::{
        const_vars::MODULES_INS_ROOT,
        error::SpecResult,
        module::work::make_modins_example,
        types::{Localizable, Persistable},
    };

    use super::RunningModule;

    #[tokio::test]
    async fn test_mod_running() -> SpecResult<()> {
        let sys = make_modins_example()?;
        let path = PathBuf::from(MODULES_INS_ROOT);
        sys.save_to(&path)?;
        let sys = RunningModule::load_from(&path.join(sys.spec.name()))?;
        sys.update().await?;
        sys.localize().await?;
        Ok(())
    }
}

use std::path::PathBuf;

use async_trait::async_trait;
use derive_getters::Getters;
use orion_error::{ErrorOwe, ErrorWith, WithContext};
use orion_exchange::vars::VarCollection;

use crate::{
    action::act::Actions,
    addr::path_file_name,
    artifact::Artifact,
    conf::ConfSpec,
    error::SpecResult,
    resource::CaculateResSpec,
    software::LogsSpec,
    types::{AsyncUpdateable, Persistable, TomlAble},
};

#[derive(Getters, Clone, Debug)]
pub struct ModTargetSpec {
    target: String,
    artifact: Artifact,
    actions: Actions,
    conf_spec: ConfSpec,
    logs_spec: LogsSpec,
    res_spec: CaculateResSpec,
    vars: VarCollection,
}

#[async_trait]
impl AsyncUpdateable for ModTargetSpec {
    async fn update_local(&self, path: &PathBuf) -> SpecResult<PathBuf> {
        self.conf_spec.update_local(path).await?;
        Ok(path.clone())
    }
}

impl Persistable<ModTargetSpec> for ModTargetSpec {
    fn save_to(&self, root: &PathBuf) -> SpecResult<()> {
        let target_path = root.join(self.target());
        std::fs::create_dir_all(&target_path)
            .owe_conf()
            .with(format!("path: {}", target_path.display()))?;
        let artifact_path = target_path.join("artifact.toml");
        self.artifact.save_toml(&artifact_path)?;

        self.actions.save_to(&target_path)?;
        let spec_path = target_path.join("conf_spec.toml");
        self.conf_spec.save_toml(&spec_path)?;
        let spec_path = target_path.join("logs_spec.toml");
        self.logs_spec.save_toml(&spec_path)?;

        let spec_path = target_path.join("res_spec.toml");
        self.res_spec.save_toml(&spec_path)?;
        let vars_path = target_path.join("vars.toml");
        self.vars.save_toml(&vars_path)?;
        Ok(())
    }

    fn load_from(target_path: &PathBuf) -> SpecResult<Self> {
        //target: &str
        let mut ctx = WithContext::want("load mod spec");
        //let target_path = root.join(target);
        let artifact_path = target_path.join("artifact.toml");
        ctx.with("artifact", format!("{}", artifact_path.display()));
        let artifact = Artifact::from_toml(&artifact_path).with(&ctx)?;

        let actions = Actions::load_from(&target_path)?;
        let spec_path = target_path.join("conf_spec.toml");
        let conf_spec = ConfSpec::from_toml(&spec_path)?;
        let spec_path = target_path.join("logs_spec.toml");
        let logs_spec = LogsSpec::from_toml(&spec_path)?;
        let spec_path = target_path.join("res_spec.toml");
        let res_spec = CaculateResSpec::from_toml(&spec_path)?;
        let vars_path = target_path.join("vars.toml");
        let vars = VarCollection::from_toml(&vars_path)?;
        let target = path_file_name(target_path)?;

        Ok(Self {
            target,
            artifact,
            actions,
            conf_spec,
            logs_spec,
            res_spec,
            vars,
        })
    }
}
impl ModTargetSpec {
    pub fn init<S: Into<String>>(
        target: S,
        artifact: Artifact,
        actions: Actions,
        conf_spec: ConfSpec,
        res_spec: CaculateResSpec,
        vars: VarCollection,
    ) -> Self {
        Self {
            target: target.into(),
            actions,
            artifact,
            conf_spec,
            logs_spec: LogsSpec::tpl_init(),
            res_spec,
            vars,
        }
    }
}

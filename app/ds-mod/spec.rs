use orion_error::{ErrorConv, ErrorOwe};
use orion_syspec::error::SpecResult;
use orion_syspec::infra::configure_dfx_logging;
use orion_syspec::module::proj::ModProject;
use orion_syspec::module::spec::make_mod_spec_example;
use orion_syspec::types::{Localizable, LocalizeOptions, Persistable};
use orion_syspec::update::UpdateOptions;
use orion_syspec::vars::ValueDict;
use std::path::PathBuf;

use crate::args::{self};

pub async fn do_mod_cmd(cmd: args::GxModCmd) -> SpecResult<()> {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    match cmd {
        args::GxModCmd::Example => {
            let spec = make_mod_spec_example().err_conv()?;
            spec.save_to(&PathBuf::from("./"), None).err_conv()?;
        }
        args::GxModCmd::New(spec_args) => {
            let project_dir = current_dir.join(spec_args.name());
            std::fs::create_dir(&project_dir).owe_res()?;
            configure_dfx_logging(&spec_args);
            let spec = ModProject::make_new(&project_dir, spec_args.name.as_str()).err_conv()?;
            spec.save().err_conv()?;
        }
        args::GxModCmd::Update(dfx) => {
            configure_dfx_logging(&dfx);
            let spec = ModProject::load(&current_dir).err_conv()?;
            let options = UpdateOptions::from((dfx.force, ValueDict::default()));
            spec.update(&options).await.err_conv()?;
        }
        args::GxModCmd::Localize(args) => {
            configure_dfx_logging(&args);
            let spec = ModProject::load(&current_dir).err_conv()?;
            let dict = spec.load_global_value(args.value())?;
            spec.localize(None, LocalizeOptions::new(dict, args.use_default_value))
                .await
                .err_conv()?;
        }
    }
    Ok(())
}

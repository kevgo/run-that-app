use super::App;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::platform::{Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use big_s::S;
use const_format::formatcp;

pub struct Deadcode {}

impl App for Deadcode {
    fn name(&self) -> &'static str {
        "deadcode"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "deadcode.exe",
            Os::Linux | Os::MacOS => "deadcode",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://pkg.go.dev/golang.org/x/tools/cmd/deadcode")
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        compile_go(CompileArgs {
            import_path: format!("golang.org/x/tools/cmd/deadcode@v{version}"),
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }

    fn latest_version(&self, _output: &dyn Output) -> Result<String> {
        Ok(S("0.16.1"))
    }

    fn versions(&self, _amount: u8, _output: &dyn Output) -> Result<Vec<String>> {
        Ok(vec![S("0.16.1")])
    }
}

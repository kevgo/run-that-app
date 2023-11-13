use super::InstallationMethod;

pub struct CompileFromRustSource {}

impl InstallationMethod for CompileFromRustSource {
    fn install(
        &self,
        _output: &dyn crate::output::Output,
    ) -> crate::error::Result<Option<crate::yard::Executable>> {
        todo!()
    }
}

use super::InstallationMethod;

pub struct CompileFromGoSource {}

impl InstallationMethod for CompileFromGoSource {
    fn install(
        &self,
        _output: &dyn crate::output::Output,
    ) -> crate::error::Result<Option<crate::yard::Executable>> {
        todo!()
    }
}

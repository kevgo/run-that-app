use crate::Output;

pub fn version(output: &dyn Output) {
    output.print("version");
}

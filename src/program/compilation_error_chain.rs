pub struct CompilationErrorChain;

impl CompilationErrorChain {
    pub fn none<T>(self) -> Option<T> {
        None
    }

    pub fn void(self) -> () {
        ()
    }
}
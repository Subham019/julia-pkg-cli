pub fn status() {
    crate::common::run_julia("using Pkg; Pkg.status()", &[]);
}

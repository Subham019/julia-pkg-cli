pub fn fetch() {
    crate::common::run_julia(
        "using Pkg; Pkg.autoprecompilation_enabled(false); Pkg.instantiate()", 
        &[],
    );
}
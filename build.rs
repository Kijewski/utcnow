fn main() {
    autocfg::rerun_path("build.rs");

    let ac = autocfg::new();
    ac.emit_has_trait("core::panic::RefUnwindSafe");
    ac.emit_has_trait("core::panic::UnwindSafe");
}

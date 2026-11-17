//! Integration tests for SigmaOS-emacs-typography-theme-directory.

use sigmaos_emacs_typography_theme_directory_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}

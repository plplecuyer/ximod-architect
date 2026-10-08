//! Tree selection of the active document: the step, group, option and
//! conditional-install set currently selected in the project tree.

/// Selected node of the project tree. Every index below a given level is
/// only meaningful when that level is `Some` (see `XimodApp::select_step`
/// and friends, which keep the levels consistent).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Selection {
    pub step: Option<usize>,
    pub group: Option<usize>,
    pub plugin: Option<usize>,
    pub cond_pattern: Option<usize>,
}

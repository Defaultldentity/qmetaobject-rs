use qttypes::QString;

/// Refer to the Qt documentation for QQuickStyle
pub struct QQuickStyle {}

impl QQuickStyle {
    /// Refer to the Qt documentation for QQuickStyle::setStyle
    pub fn set_style(style: QString) {
        std::env::set_var::<_, String>("QT_QUICK_CONTROLS_STYLE", style.into());
    }
}

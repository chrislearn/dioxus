#![doc = include_str!("../../../notes/README.md")]

pub use dioxus_core as core;

#[cfg(feature = "hooks")]
pub use dioxus_hooks as hooks;

pub mod events {
    #[cfg(feature = "html")]
    pub use dioxus_html::{on::*, KeyCode};
}

#[cfg(feature = "html")]
pub use dioxus_html as html;

#[cfg(feature = "macro")]
pub use dioxus_rsx as rsx;

#[cfg(feature = "macro")]
pub use dioxus_core_macro as core_macro;

pub mod prelude {
    #[cfg(feature = "hooks")]
    pub use crate::hooks::*;

    pub use dioxus_core::prelude::*;

    #[cfg(feature = "macro")]
    pub use dioxus_core_macro::{format_args_f, inline_props, rsx, Props};

    #[cfg(feature = "html")]
    pub use dioxus_html as dioxus_elements;

    #[cfg(feature = "html")]
    pub use dioxus_elements::{GlobalAttributes, SvgAttributes};

    #[cfg(feature = "hot-reload")]
    pub use dioxus_rsx_interpreter::{
        captuered_context::{CapturedContext, FormattedArg, IfmtArgs},
        get_line_num, resolve_scope, CodeLocation, RsxContext,
    };
}

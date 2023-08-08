#![allow(unused)]
#![allow(missing_docs)]
#![allow(clippy::module_name_repetitions)]

use std::fmt::{self, Display, Formatter};

use crate::typed_attributes;

typed_attributes! {
    [pub Hx] {
        // REST
        hx_get: String,
        hx_post: String,
        hx_put: String,
        hx_delete: String,
        hx_patch: String,
        hx_headers: String,
        // Core
        hx_boost: bool,
        hx_on: String,
        hx_push_url: String,
        hx_select: String,
        hx_select_oob: String,
        hx_swap: HtmxSwapTarget,
        hx_swap_oob: String,
        hx_target: HtmxTarget,
        hx_trigger: String,
        hx_vals: String,
        // Misc
        hx_confirm: String,
        hx_disable: bool,
        hx_disinherit: String,
        hx_encoding: String,
        hx_ext: String,
        hx_history: bool,
        hx_history_elt: bool,
        hx_include: String,
        hx_indicator: String,
        hx_params: HtmxParamsFilter,
        hx_preserve: String,
        hx_prompt: String,
        hx_replace_url: HtmxReplaceUrl,
        hx_sync: HtmxSyncStrategy,
        hx_validate: bool
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HtmxSwapTarget {
    InnerHtml,
    OuterHtml,
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd,
    Delete,
    None,
}

impl Display for HtmxSwapTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InnerHtml => f.write_str("innerHTML"),
            Self::OuterHtml => f.write_str("outerHTML"),
            Self::BeforeBegin => f.write_str("beforebegin"),
            Self::AfterBegin => f.write_str("afterbegin"),
            Self::BeforeEnd => f.write_str("beforeend"),
            Self::AfterEnd => f.write_str("afterend"),
            Self::Delete => f.write_str("delete"),
            Self::None => f.write_str("none"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmxTarget {
    Selector(String),
    This,
    Closest(String),
    Find(String),
    Next(String),
    Previous(String),
}

impl Display for HtmxTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::This => "this",
                Self::Selector(css)
                | Self::Closest(css)
                | Self::Next(css)
                | Self::Previous(css)
                | Self::Find(css) => css,
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmxParamsFilter {
    IncludeAll,
    None,
    Not(Vec<String>),
    Only(Vec<String>),
}

impl Display for HtmxParamsFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncludeAll => f.write_str("*"),
            Self::None => f.write_str("none"),
            Self::Not(params) => f.write_fmt(format_args!("not {}", params.join(","))),
            Self::Only(params) => f.write_fmt(format_args!("{}", params.join(","))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmxReplaceUrl {
    True,
    False,
    Url(String),
}

impl Display for HtmxReplaceUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::Url(uri) => f.write_fmt(format_args!("{uri}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmxSyncStrategy {
    None(String),
    Drop(String),
    Abort(String),
    Replace(String),
    Queue(QueueSyncStrategy),
}

impl Display for HtmxSyncStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::None(css) => f.write_fmt(format_args!("{css}")),
            Self::Drop(css) => f.write_fmt(format_args!("{css}:drop")),
            Self::Abort(css) => f.write_fmt(format_args!("{css}:abort")),
            Self::Replace(css) => f.write_fmt(format_args!("{css}:replace")),
            Self::Queue(queue) => f.write_fmt(format_args!("{queue}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueSyncStrategy {
    First(String),
    Last(String),
    All(String),
}

impl Display for QueueSyncStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::First(css) => f.write_fmt(format_args!("{css}:queue first")),
            Self::Last(css) => f.write_fmt(format_args!("{css}:queue last")),
            Self::All(css) => f.write_fmt(format_args!("{css}:queue all")),
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HtmxCssClass {
    HtmxAdded,
    HtmxIndicator,
    HtmxRequest,
    HtmxSettling,
    HtmxSwapping,
}

impl Display for HtmxCssClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::HtmxAdded => f.write_str("htmx-added"),
            Self::HtmxIndicator => f.write_str("htmx-indicator"),
            Self::HtmxRequest => f.write_str("htmx-request"),
            Self::HtmxSettling => f.write_str("htmx-settling"),
            Self::HtmxSwapping => f.write_str("htmx-swapping"),
        }
    }
}

//! Steps component — daisyUI `steps`.

use crate::utils::class::build_class;
use crate::variants::color::Color;
use leptos::prelude::*;

/// Orientation for steps layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    /// Get the CSS class for this orientation.
    pub fn class(&self) -> &'static str {
        match self {
            Self::Horizontal => "steps-horizontal",
            Self::Vertical => "steps-vertical",
        }
    }
}

/// Data structure for a step.
#[derive(Clone, Debug)]
pub struct StepData {
    /// The label/text to display for the step.
    pub label: String,
    /// Optional description text.
    pub description: Option<String>,
    /// Optional custom icon/content.
    pub icon: Option<String>,
}

impl StepData {
    /// Create a new step with just a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            icon: None,
        }
    }

    /// Create a step with label and description.
    pub fn with_description(label: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: Some(description.into()),
            icon: None,
        }
    }

    /// Create a step with a custom icon.
    pub fn with_icon(label: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            icon: Some(icon.into()),
        }
    }
}

/// A daisyUI steps component for showing progress through a sequence.
#[component]
pub fn Steps(
    /// The steps to display.
    #[prop(into)]
    steps: Vec<StepData>,
    /// The index of the current/active step (0-based).
    #[prop(optional, into)]
    current_step: usize,
    /// Orientation of the steps (horizontal or vertical).
    #[prop(optional, into)]
    orientation: Option<Orientation>,
    /// Color for the active/completed steps.
    #[prop(optional, into)]
    color: Option<Color>,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();

    if let Some(o) = orientation {
        mods.push(o.class().to_string());
    }

    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "steps",
        &refs,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let color_class = color.map(|c| c.class("step")).unwrap_or_default();

    view! {
        <ul class={cls}>
            {steps.into_iter().enumerate().map(|(index, step)| {
                let is_active = index == current_step;
                let is_completed = index < current_step;
                let step_cls = if is_active && !color_class.is_empty() {
                    format!("step {} step-primary", color_class)
                } else if is_active {
                    "step step-primary".to_string()
                } else if is_completed && !color_class.is_empty() {
                    format!("step {} step-primary", color_class)
                } else if is_completed {
                    "step step-primary".to_string()
                } else {
                    "step".to_string()
                };

                view! {
                    <li class={step_cls} data-content={step.icon.clone()}>
                        {if let Some(desc) = &step.description {
                            view! {
                                <div>
                                    <div class="step-title">{step.label.clone()}</div>
                                    <div class="step-desc">{desc.clone()}</div>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="step-title">{step.label.clone()}</div>
                            }.into_any()
                        }}
                    </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}

/// A single step item (for use when building steps manually).
#[component]
pub fn Step(
    children: Children,
    /// Whether this step is active.
    #[prop(optional)]
    active: bool,
    /// Color for this step when active.
    #[prop(optional, into)]
    color: Option<Color>,
    /// Custom data-content attribute value.
    #[prop(optional, into)]
    data_content: Option<String>,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();

    if active {
        let color_class = color
            .map(|c| c.class("step"))
            .unwrap_or_else(|| "step-primary".to_string());
        if !color_class.is_empty() {
            mods.push(color_class);
        } else {
            mods.push("step-primary".to_string());
        }
    }

    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "step",
        &refs,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <li class={cls} data-content={data_content}>
            {children()}
        </li>
    }
}

use leptos::prelude::*;

/// Component types that can be placed on the canvas.
#[derive(Clone, Debug, PartialEq)]
enum ComponentKind {
    Button,
    Card,
    Badge,
    Alert,
    Input,
    Toggle,
    Checkbox,
    Select,
    Textarea,
    Progress,
    Loading,
    Steps,
    Tooltip,
    Divider,
}

impl ComponentKind {
    fn label(&self) -> &'static str {
        match self {
            Self::Button => "Button",
            Self::Card => "Card",
            Self::Badge => "Badge",
            Self::Alert => "Alert",
            Self::Input => "Input",
            Self::Toggle => "Toggle",
            Self::Checkbox => "Checkbox",
            Self::Select => "Select",
            Self::Textarea => "Textarea",
            Self::Progress => "Progress",
            Self::Loading => "Loading",
            Self::Steps => "Steps",
            Self::Tooltip => "Tooltip",
            Self::Divider => "Divider",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Button => "🔘",
            Self::Card => "🃏",
            Self::Badge => "🏷️",
            Self::Alert => "⚠️",
            Self::Input => "📝",
            Self::Toggle => "🔀",
            Self::Checkbox => "☑️",
            Self::Select => "📋",
            Self::Textarea => "📄",
            Self::Progress => "📊",
            Self::Loading => "⏳",
            Self::Steps => "👣",
            Self::Tooltip => "💬",
            Self::Divider => "➖",
        }
    }

    fn all() -> &'static [ComponentKind] {
        &[
            ComponentKind::Button,
            ComponentKind::Card,
            ComponentKind::Badge,
            ComponentKind::Alert,
            ComponentKind::Input,
            ComponentKind::Toggle,
            ComponentKind::Checkbox,
            ComponentKind::Select,
            ComponentKind::Textarea,
            ComponentKind::Progress,
            ComponentKind::Loading,
            ComponentKind::Steps,
            ComponentKind::Tooltip,
            ComponentKind::Divider,
        ]
    }
}

/// A placed component on the canvas.
#[derive(Clone, Debug)]
struct PlacedComponent {
    id: usize,
    kind: ComponentKind,
    color: String,
    size: String,
    variant: String,
    text: String,
}

#[component]
pub fn PlaygroundPage() -> impl IntoView {
    let (components, set_components) = signal(Vec::<PlacedComponent>::new());
    let (next_id, set_next_id) = signal(1usize);
    let (selected_id, set_selected_id) = signal(Option::<usize>::None);
    let (preview_theme, set_preview_theme) = signal("".to_string());

    // Add a component to the canvas
    let add_component = move |kind: ComponentKind| {
        let id = next_id.get_untracked();
        set_next_id.set(id + 1);
        let text = match &kind {
            ComponentKind::Button => "Click me".to_string(),
            ComponentKind::Card => "Card Title".to_string(),
            ComponentKind::Badge => "New".to_string(),
            ComponentKind::Alert => "This is an alert".to_string(),
            ComponentKind::Input => "placeholder...".to_string(),
            ComponentKind::Textarea => "Type here...".to_string(),
            ComponentKind::Tooltip => "Hover me".to_string(),
            _ => String::new(),
        };
        set_components.update(|v| {
            v.push(PlacedComponent {
                id,
                kind,
                color: "primary".to_string(),
                size: "md".to_string(),
                variant: "default".to_string(),
                text,
            });
        });
        set_selected_id.set(Some(id));
    };

    // Remove a component
    let remove_component = move |id: usize| {
        set_components.update(|v| v.retain(|c| c.id != id));
        if selected_id.get_untracked() == Some(id) {
            set_selected_id.set(None);
        }
    };

    // Move component up/down
    let move_component = move |id: usize, dir: i32| {
        set_components.update(|v| {
            if let Some(idx) = v.iter().position(|c| c.id == id) {
                let new_idx = idx as i32 + dir;
                if new_idx >= 0 && (new_idx as usize) < v.len() {
                    v.swap(idx, new_idx as usize);
                }
            }
        });
    };

    // Generate code from canvas
    let generated_code = move || {
        let comps = components.get();
        if comps.is_empty() {
            return "// Add components from the palette to generate code".to_string();
        }
        let mut lines = vec![
            "view! {".to_string(),
            "    <div class=\"space-y-4\">".to_string(),
        ];
        for c in &comps {
            let code = generate_code_for(c);
            lines.push(code);
        }
        lines.push("    </div>".to_string());
        lines.push("}".to_string());
        lines.join("\n")
    };

    let (code_copied, _set_code_copied) = signal(false);

    let preview_themes = [
        "",
        "light",
        "dark",
        "cupcake",
        "synthwave",
        "cyberpunk",
        "valentine",
        "dracula",
        "nord",
        "sunset",
        "caramellatte",
        "abyss",
    ];

    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between flex-wrap gap-4">
                <div>
                    <h1 class="text-3xl font-bold">"🛝 Playground"</h1>
                    <p class="text-base-content/60 text-sm mt-1">"Click components to add them to the canvas. Select to edit properties."</p>
                </div>
                <div class="flex items-center gap-2">
                    <label class="text-sm font-medium">"Preview theme:"</label>
                    <select
                        class="select select-bordered select-sm"
                        on:change=move |ev| set_preview_theme.set(event_target_value(&ev))
                    >
                        <option value="">"Current"</option>
                        {preview_themes.iter().skip(1).map(|t| {
                            let t = *t;
                            view! { <option value={t}>{t}</option> }
                        }).collect_view()}
                    </select>
                    <button
                        class="btn btn-sm btn-error btn-outline"
                        on:click=move |_| {
                            set_components.set(Vec::new());
                            set_selected_id.set(None);
                        }
                    >
                        "Clear All"
                    </button>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-[16rem_1fr_16rem] gap-4 min-h-[32rem]">
                // --- Component Palette ---
                <div class="bg-base-100 rounded-box border border-base-300 p-4">
                    <h3 class="font-bold mb-3 text-sm uppercase tracking-wider text-base-content/50">"Palette"</h3>
                    <div class="flex flex-col gap-1.5">
                        {ComponentKind::all().iter().map(|kind| {
                            let kind = kind.clone();
                            let label = kind.label();
                            let icon = kind.icon();
                            view! {
                                <button
                                    class="btn btn-sm btn-ghost justify-start gap-2 text-left"
                                    on:click=move |_| add_component(kind.clone())
                                >
                                    <span>{icon}</span>
                                    <span>{label}</span>
                                </button>
                            }
                        }).collect_view()}
                    </div>
                    // Component tree
                    <div class="divider text-xs">"TREE"</div>
                    <div class="text-sm space-y-0.5">
                        {move || {
                            let comps = components.get();
                            if comps.is_empty() {
                                view! { <p class="text-base-content/40 text-xs italic">"Empty canvas"</p> }.into_any()
                            } else {
                                comps.iter().map(|c| {
                                    let id = c.id;
                                    let icon = c.kind.icon();
                                    let label = c.kind.label();
                                    let is_sel = move || selected_id.get() == Some(id);
                                    view! {
                                        <button
                                            class="flex items-center gap-1.5 w-full text-left px-2 py-0.5 rounded text-xs hover:bg-base-200"
                                            class:bg-primary=is_sel
                                            class:text-primary-content=is_sel
                                            on:click=move |_| set_selected_id.set(Some(id))
                                        >
                                            <span>{icon}</span>
                                            <span>{format!("{} #{}", label, id)}</span>
                                        </button>
                                    }
                                }).collect_view().into_any()
                            }
                        }}
                    </div>
                </div>

                // --- Canvas ---
                <div
                    class="bg-base-100 rounded-box border border-base-300 border-dashed p-4 min-h-[28rem] flex flex-col"
                    data-theme=move || {
                        let t = preview_theme.get();
                        if t.is_empty() { None } else { Some(t) }
                    }
                >
                    <h3 class="font-bold text-sm uppercase tracking-wider text-base-content/50 mb-3">"Canvas"</h3>
                    <div class="flex-1 space-y-3">
                        {move || {
                            let comps = components.get();
                            if comps.is_empty() {
                                view! {
                                    <div class="flex items-center justify-center h-full text-base-content/30">
                                        <div class="text-center space-y-2">
                                            <p class="text-4xl">"🎨"</p>
                                            <p>"Click components from the palette"</p>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                comps.iter().map(|c| {
                                    let id = c.id;
                                    let is_sel = move || selected_id.get() == Some(id);
                                    let rendered = render_component(c);
                                    view! {
                                        <div
                                            class="relative group cursor-pointer rounded p-2 -m-1 transition-colors"
                                            class:ring-2=is_sel
                                            class:ring-primary=is_sel
                                            class:bg-base-200=is_sel
                                            on:click=move |_| set_selected_id.set(Some(id))
                                        >
                                            {rendered}
                                            // Hover controls
                                            <div class="absolute -top-2 -right-2 opacity-0 group-hover:opacity-100 transition-opacity flex gap-0.5">
                                                <button
                                                    class="btn btn-xs btn-circle btn-ghost bg-base-300"
                                                    on:click=move |ev| {
                                                        ev.stop_propagation();
                                                        move_component(id, -1);
                                                    }
                                                >"↑"</button>
                                                <button
                                                    class="btn btn-xs btn-circle btn-ghost bg-base-300"
                                                    on:click=move |ev| {
                                                        ev.stop_propagation();
                                                        move_component(id, 1);
                                                    }
                                                >"↓"</button>
                                                <button
                                                    class="btn btn-xs btn-circle btn-error"
                                                    on:click=move |ev| {
                                                        ev.stop_propagation();
                                                        remove_component(id);
                                                    }
                                                >"×"</button>
                                            </div>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }
                        }}
                    </div>
                </div>

                // --- Properties Panel ---
                <div class="bg-base-100 rounded-box border border-base-300 p-4">
                    <h3 class="font-bold text-sm uppercase tracking-wider text-base-content/50 mb-3">"Properties"</h3>
                    {move || {
                        let sel = selected_id.get();
                        let comps = components.get();
                        match sel.and_then(|id| comps.iter().find(|c| c.id == id)) {
                            None => view! {
                                <p class="text-sm text-base-content/40 italic">"Select a component"</p>
                            }.into_any(),
                            Some(comp) => {
                                let id = comp.id;
                                let kind_label = comp.kind.label();
                                let current_color = comp.color.clone();
                                let current_size = comp.size.clone();
                                let current_variant = comp.variant.clone();
                                let current_text = comp.text.clone();
                                let has_text = matches!(comp.kind,
                                    ComponentKind::Button | ComponentKind::Card |
                                    ComponentKind::Badge | ComponentKind::Alert |
                                    ComponentKind::Input | ComponentKind::Textarea |
                                    ComponentKind::Tooltip
                                );
                                let has_size = matches!(comp.kind,
                                    ComponentKind::Button | ComponentKind::Badge |
                                    ComponentKind::Input | ComponentKind::Toggle |
                                    ComponentKind::Checkbox | ComponentKind::Select |
                                    ComponentKind::Textarea | ComponentKind::Loading
                                );
                                let has_variant = matches!(comp.kind,
                                    ComponentKind::Button
                                );

                                view! {
                                    <div class="space-y-4">
                                        <div>
                                            <span class="text-xs text-base-content/50">"Component"</span>
                                            <p class="font-bold">{kind_label} " #" {id}</p>
                                        </div>

                                        // Color
                                        <div class="form-control">
                                            <label class="label py-0.5"><span class="label-text text-xs">"Color"</span></label>
                                            <select
                                                class="select select-bordered select-xs"
                                                on:change=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    set_components.update(|v| {
                                                        if let Some(c) = v.iter_mut().find(|c| c.id == id) {
                                                            c.color = val;
                                                        }
                                                    });
                                                }
                                            >
                                                {["primary", "secondary", "accent", "neutral", "info", "success", "warning", "error", "ghost"].iter().map(|c| {
                                                    let c = *c;
                                                    let selected = c == current_color;
                                                    view! { <option value={c} selected=selected>{c}</option> }
                                                }).collect_view()}
                                            </select>
                                        </div>

                                        // Size
                                        {has_size.then(|| {
                                            let current_size = current_size.clone();
                                            view! {
                                                <div class="form-control">
                                                    <label class="label py-0.5"><span class="label-text text-xs">"Size"</span></label>
                                                    <select
                                                        class="select select-bordered select-xs"
                                                        on:change=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_components.update(|v| {
                                                                if let Some(c) = v.iter_mut().find(|c| c.id == id) {
                                                                    c.size = val;
                                                                }
                                                            });
                                                        }
                                                    >
                                                        {["xs", "sm", "md", "lg"].iter().map(|s| {
                                                            let s = *s;
                                                            let selected = s == current_size;
                                                            view! { <option value={s} selected=selected>{s}</option> }
                                                        }).collect_view()}
                                                    </select>
                                                </div>
                                            }
                                        })}

                                        // Variant (for buttons)
                                        {has_variant.then(|| {
                                            let current_variant = current_variant.clone();
                                            view! {
                                                <div class="form-control">
                                                    <label class="label py-0.5"><span class="label-text text-xs">"Variant"</span></label>
                                                    <select
                                                        class="select select-bordered select-xs"
                                                        on:change=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_components.update(|v| {
                                                                if let Some(c) = v.iter_mut().find(|c| c.id == id) {
                                                                    c.variant = val;
                                                                }
                                                            });
                                                        }
                                                    >
                                                        {["default", "outline", "link", "glass", "wide", "block"].iter().map(|v| {
                                                            let v = *v;
                                                            let selected = v == current_variant;
                                                            view! { <option value={v} selected=selected>{v}</option> }
                                                        }).collect_view()}
                                                    </select>
                                                </div>
                                            }
                                        })}

                                        // Text content
                                        {has_text.then(|| {
                                            view! {
                                                <div class="form-control">
                                                    <label class="label py-0.5"><span class="label-text text-xs">"Text"</span></label>
                                                    <input
                                                        type="text"
                                                        class="input input-bordered input-xs"
                                                        prop:value=current_text
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_components.update(|v| {
                                                                if let Some(c) = v.iter_mut().find(|c| c.id == id) {
                                                                    c.text = val;
                                                                }
                                                            });
                                                        }
                                                    />
                                                </div>
                                            }
                                        })}

                                        // Delete button
                                        <button
                                            class="btn btn-xs btn-error btn-outline w-full"
                                            on:click=move |_| remove_component(id)
                                        >"Delete"</button>
                                    </div>
                                }.into_any()
                            }
                        }
                    }}
                </div>
            </div>

            // --- Generated Code ---
            <div class="bg-base-100 rounded-box border border-base-300 p-4">
                <div class="flex items-center justify-between mb-3">
                    <h3 class="font-bold text-sm uppercase tracking-wider text-base-content/50">"Generated Leptos Code"</h3>
                    <button
                        class="btn btn-sm btn-ghost"
                        on:click=move |_| {
                            #[cfg(any(feature = "csr", feature = "hydrate"))]
                            {
                                let text = generated_code();
                                let _ = leptos::prelude::window().navigator().clipboard().write_text(&text);
                                _set_code_copied.set(true);
                                set_timeout(move || _set_code_copied.set(false), std::time::Duration::from_secs(2));
                            }
                        }
                    >
                        {move || if code_copied.get() { "✓ Copied!" } else { "📋 Copy" }}
                    </button>
                </div>
                <div class="mockup-code text-xs overflow-x-auto">
                    <pre><code>{generated_code}</code></pre>
                </div>
            </div>
        </div>
    }
}

/// Generate Leptos code for a placed component.
fn generate_code_for(c: &PlacedComponent) -> String {
    match c.kind {
        ComponentKind::Button => {
            let variant_str = match c.variant.as_str() {
                "outline" => " variant=Variant::Outline",
                "link" => " variant=Variant::Link",
                "glass" => " variant=Variant::Glass",
                "wide" => " class=\"btn-wide\"",
                "block" => " class=\"btn-block\"",
                _ => "",
            };
            format!(
                "        <Button color=Color::{} size=Size::{}{}>{:?}</Button>",
                capitalize(&c.color),
                capitalize(&c.size),
                variant_str,
                c.text
            )
        }
        ComponentKind::Card => {
            format!(
                "        <Card>\n            <CardBody>\n                <CardTitle>{:?}</CardTitle>\n                <p>\"Card content\"</p>\n            </CardBody>\n        </Card>",
                c.text
            )
        }
        ComponentKind::Badge => {
            format!(
                "        <Badge color=Color::{}>{:?}</Badge>",
                capitalize(&c.color),
                c.text
            )
        }
        ComponentKind::Alert => {
            format!(
                "        <Alert variant=AlertVariant::{}>\n            <span>{:?}</span>\n        </Alert>",
                capitalize(&c.color),
                c.text
            )
        }
        ComponentKind::Input => {
            format!("        <Input placeholder={:?} size=Size::{} />", c.text, capitalize(&c.size))
        }
        ComponentKind::Toggle => {
            format!("        <Toggle color=Color::{} />", capitalize(&c.color))
        }
        ComponentKind::Checkbox => {
            format!("        <Checkbox color=Color::{} />", capitalize(&c.color))
        }
        ComponentKind::Select => {
            "        <Select>\n            <option>\"Option 1\"</option>\n            <option>\"Option 2\"</option>\n            <option>\"Option 3\"</option>\n        </Select>".to_string()
        }
        ComponentKind::Textarea => {
            format!("        <Textarea placeholder={:?} />", c.text)
        }
        ComponentKind::Progress => {
            format!("        <Progress color=Color::{} value=65 max=100 />", capitalize(&c.color))
        }
        ComponentKind::Loading => {
            format!("        <Loading color=Color::{} />", capitalize(&c.color))
        }
        ComponentKind::Steps => {
            "        <Steps>\n            <Step color=Color::Primary>\"Step 1\"</Step>\n            <Step color=Color::Primary>\"Step 2\"</Step>\n            <Step>\"Step 3\"</Step>\n        </Steps>".to_string()
        }
        ComponentKind::Tooltip => {
            format!(
                "        <Tooltip tip=\"Tooltip text\">\n            <button class=\"btn\">{:?}</button>\n        </Tooltip>",
                c.text
            )
        }
        ComponentKind::Divider => "        <Divider />".to_string(),
    }
}

/// Render a component preview on the canvas.
fn render_component(comp: &PlacedComponent) -> impl IntoView {
    let color = &comp.color;
    let size = &comp.size;
    let variant = &comp.variant;
    let text = &comp.text;

    match comp.kind {
        ComponentKind::Button => {
            let variant_cls = match variant.as_str() {
                "outline" => " btn-outline",
                "link" => " btn-link",
                "glass" => " glass",
                "wide" => " btn-wide",
                "block" => " btn-block",
                _ => "",
            };
            let cls = format!("btn btn-{} btn-{}{}", color, size, variant_cls);
            view! { <button class={cls}>{text.clone()}</button> }.into_any()
        }
        ComponentKind::Card => view! {
            <div class="card bg-base-200 shadow-sm">
                <div class="card-body p-4">
                    <h4 class="card-title text-sm">{text.clone()}</h4>
                    <p class="text-xs text-base-content/60">"Card content goes here"</p>
                </div>
            </div>
        }
        .into_any(),
        ComponentKind::Badge => {
            let cls = format!("badge badge-{}", color);
            view! { <span class={cls}>{text.clone()}</span> }.into_any()
        }
        ComponentKind::Alert => {
            let cls = format!("alert alert-{} py-2", color);
            view! { <div class={cls}><span class="text-sm">{text.clone()}</span></div> }.into_any()
        }
        ComponentKind::Input => {
            let cls = format!("input input-bordered input-{}", size);
            let text = text.clone();
            view! { <input type="text" class={cls} placeholder={text} /> }.into_any()
        }
        ComponentKind::Toggle => {
            let cls = format!("toggle toggle-{} toggle-{}", color, size);
            view! { <input type="checkbox" class={cls} checked /> }.into_any()
        }
        ComponentKind::Checkbox => {
            let cls = format!("checkbox checkbox-{} checkbox-{}", color, size);
            view! { <input type="checkbox" class={cls} checked /> }.into_any()
        }
        ComponentKind::Select => {
            let cls = format!("select select-bordered select-{}", size);
            view! {
                <select class={cls}>
                    <option>"Option 1"</option>
                    <option>"Option 2"</option>
                    <option>"Option 3"</option>
                </select>
            }
            .into_any()
        }
        ComponentKind::Textarea => {
            let cls = format!("textarea textarea-bordered textarea-{}", size);
            let text = text.clone();
            view! { <textarea class={cls} placeholder={text}></textarea> }.into_any()
        }
        ComponentKind::Progress => {
            let cls = format!("progress progress-{} w-full", color);
            view! { <progress class={cls} value="65" max="100"></progress> }.into_any()
        }
        ComponentKind::Loading => {
            let cls = format!("loading loading-spinner loading-{} text-{}", size, color);
            view! { <span class={cls}></span> }.into_any()
        }
        ComponentKind::Steps => view! {
            <ul class="steps w-full">
                <li class=format!("step step-{}", color)>"Step 1"</li>
                <li class=format!("step step-{}", color)>"Step 2"</li>
                <li class="step">"Step 3"</li>
            </ul>
        }
        .into_any(),
        ComponentKind::Tooltip => {
            let text = text.clone();
            view! {
                <div class=format!("tooltip tooltip-{}", color) data-tip="Tooltip text">
                    <button class="btn btn-sm">{text}</button>
                </div>
            }
            .into_any()
        }
        ComponentKind::Divider => view! { <div class="divider">"—"</div> }.into_any(),
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

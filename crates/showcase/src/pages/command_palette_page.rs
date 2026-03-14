use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CommandPalettePage() -> impl IntoView {
    let open = RwSignal::new(false);
    let (selected, set_selected) = signal(String::new());

    let items = vec![
        CommandPaletteItem {
            id: "home".into(),
            label: "Go to Home".into(),
            group: Some("Navigation".into()),
            description: Some("Return to the home page".into()),
        },
        CommandPaletteItem {
            id: "settings".into(),
            label: "Open Settings".into(),
            group: Some("Navigation".into()),
            description: Some("Configure your preferences".into()),
        },
        CommandPaletteItem {
            id: "search".into(),
            label: "Search Files".into(),
            group: Some("Actions".into()),
            description: None,
        },
        CommandPaletteItem {
            id: "theme".into(),
            label: "Change Theme".into(),
            group: Some("Actions".into()),
            description: Some("Switch between light and dark themes".into()),
        },
        CommandPaletteItem {
            id: "help".into(),
            label: "Help & Documentation".into(),
            group: Some("Support".into()),
            description: Some("View the documentation".into()),
        },
    ];

    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Command Palette"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A Cmd+K / Ctrl+K search palette with keyboard navigation, filtering, and item grouping."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Interactive Demo"
                    description="Press Cmd+K (Mac) or Ctrl+K (Windows/Linux) to open, or click the button below."
                    code=r##"let open = RwSignal::new(false);

let items = vec![
    CommandPaletteItem {
        id: "home".into(),
        label: "Go to Home".into(),
        group: Some("Navigation".into()),
        description: Some("Return to the home page".into()),
    },
    // ...
];

view! {
    <Button on_click=move |_| open.set(true)>"Open Palette"</Button>
    <CommandPalette
        open=open
        items=items
        on_select=Callback::new(move |id| { /* handle */ })
    />
}"##
                >
                    <div class="flex flex-col gap-4">
                        <div class="flex items-center gap-4">
                            <Button
                                color=Color::Primary
                                on_click=Callback::new(move |_| open.set(true))
                            >
                                "Open Command Palette"
                            </Button>
                            <span class="text-sm text-base-content/60">
                                "or press "
                                <kbd class="kbd kbd-sm">"Cmd"</kbd>
                                " + "
                                <kbd class="kbd kbd-sm">"K"</kbd>
                            </span>
                        </div>
                        <Show when=move || !selected.get().is_empty()>
                            <p class="text-sm">"Selected: "<code>{move || selected.get()}</code></p>
                        </Show>
                    </div>

                    <CommandPalette
                        open=open
                        items=items.clone()
                        on_select=Callback::new(move |id: String| set_selected.set(id))
                        placeholder="Type a command..."
                        keyboard_shortcut=false
                    />
                </ComponentPreview>
            </section>
        </div>
    }
}

use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FabPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"FAB"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Floating action button patterns including speed dial, labels, flower layout, and open-state helpers."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable
                    entries=vec![
                        ClassEntry { name: "fab", type_label: "base", description: "Floating action button container" },
                        ClassEntry { name: "fab-flower", type_label: "modifier", description: "Quarter-circle (flower) action layout" },
                        ClassEntry { name: "fab-close", type_label: "part", description: "Close wrapper shown in open state" },
                        ClassEntry { name: "fab-main-action", type_label: "part", description: "Main action wrapper shown in open state" },
                    ]
                />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic FAB"
                    code=r#"<Fab class=\"fab-bottom fab-end\">
    <FabTrigger class=\"btn-primary\">"+"</FabTrigger>
    <FabAction class=\"btn-secondary\">"✉"</FabAction>
    <FabAction class=\"btn-accent\">"⭐"</FabAction>
</Fab>"#
                >
                    <div class="relative min-h-64 w-full rounded-box border border-base-300 bg-base-100">
                        <Fab class="fab-bottom fab-end">
                            <FabTrigger class="btn-primary">"+"</FabTrigger>
                            <FabAction class="btn-secondary">"✉"</FabAction>
                            <FabAction class="btn-accent">"⭐"</FabAction>
                        </Fab>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With labels"
                    code=r#"<Fab class=\"fab-bottom fab-end\">
    <FabTrigger class=\"btn-primary\">"☰"</FabTrigger>
    <FabAction class=\"btn-info\" label="Chat">"💬"</FabAction>
    <FabAction class=\"btn-success\" label="Share">"↗"</FabAction>
    <FabAction class=\"btn-warning\" label="Star">"★"</FabAction>
</Fab>"#
                >
                    <div class="relative min-h-72 w-full rounded-box border border-base-300 bg-base-100">
                        <Fab class="fab-bottom fab-end">
                            <FabTrigger class="btn-primary">"☰"</FabTrigger>
                            <FabAction class="btn-info" label="Chat">"💬"</FabAction>
                            <FabAction class="btn-success" label="Share">"↗"</FabAction>
                            <FabAction class="btn-warning" label="Star">"★"</FabAction>
                        </Fab>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Flower + close and main action"
                    code=r#"<Fab flower=true class=\"fab-bottom fab-end\">
    <FabClose label="Close">
        <Button class="btn btn-circle btn-outline">"✕"</Button>
    </FabClose>
    <FabMainAction>
        <Button class="btn btn-circle btn-primary">"✚"</Button>
    </FabMainAction>
    <FabAction class=\"btn-secondary\" label="Mail">"✉"</FabAction>
    <FabAction class=\"btn-accent\" label="Like">"♥"</FabAction>
</Fab>"#
                >
                    <div class="relative min-h-72 w-full rounded-box border border-base-300 bg-base-100">
                        <Fab flower=true class="fab-bottom fab-end">
                            <FabClose label="Close">
                                <Button class="btn btn-circle btn-outline">"✕"</Button>
                            </FabClose>
                            <FabMainAction>
                                <Button class="btn btn-circle btn-primary">"✚"</Button>
                            </FabMainAction>
                            <FabAction class="btn-secondary" label="Mail">"✉"</FabAction>
                            <FabAction class="btn-accent" label="Like">"♥"</FabAction>
                        </Fab>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

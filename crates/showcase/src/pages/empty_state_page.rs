use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn EmptyStatePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Empty State"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Centered placeholder shown when a content area has no data."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r#"<EmptyState message="No items found" />"#
                >
                    <EmptyState message="No items found" />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Description"
                    code=r#"<EmptyState message="No results" description="Try adjusting your search filters." />"#
                >
                    <EmptyState message="No results" description="Try adjusting your search filters." />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Action"
                    code=r#"<EmptyState message="No projects yet" description="Get started by creating your first project.">
    <Button color=Color::Primary>"Create Project"</Button>
</EmptyState>"#
                >
                    <EmptyState message="No projects yet" description="Get started by creating your first project.">
                        <Button color=Color::Primary>"Create Project"</Button>
                    </EmptyState>
                </ComponentPreview>
            </section>
        </div>
    }
}

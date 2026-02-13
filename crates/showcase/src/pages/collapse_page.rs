use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CollapsePage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Accordion / Collapse"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "DaisyUI collapse patterns with checkbox, details, and radio triggers."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "collapse", type_label: "base", description: "Container for collapsible content" },
                    ClassEntry { name: "collapse-title", type_label: "base", description: "Title/trigger area of the collapse" },
                    ClassEntry { name: "collapse-content", type_label: "base", description: "Content area that expands and collapses" },
                    ClassEntry { name: "collapse-arrow", type_label: "modifier", description: "Adds arrow indicator to collapse title" },
                    ClassEntry { name: "collapse-plus", type_label: "modifier", description: "Adds plus/minus indicator to collapse title" },
                    ClassEntry { name: "collapse-open", type_label: "state", description: "Forces collapse to stay open" },
                    ClassEntry { name: "collapse-close", type_label: "state", description: "Forces collapse to stay closed" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic collapses"
                    code=r#"<Collapse
    class="bg-base-100 border border-base-300"
    title="Click to open this one and close others"
    trigger={CollapseTrigger::Radio}
    radio_name="collapse-basic"
    open=true
>
    <p>"This item starts open. Radio trigger keeps one item open in the group."</p>
</Collapse>

<Collapse
    class="bg-base-100 border border-base-300"
    title="Click to open this one and close others"
    trigger={CollapseTrigger::Radio}
    radio_name="collapse-basic"
>
    <p>"Use this for FAQ and settings groups where only one item should expand."</p>
</Collapse>"#
                    description="This item starts open. Radio trigger keeps one item open in the group."
                >
                    <div class="space-y-3 max-w-3xl w-full">
                        <Collapse
                            class="bg-base-100 border border-base-300"
                            title="Click to open this one and close others"
                            trigger={CollapseTrigger::Radio}
                            radio_name="collapse-basic"
                            open=true
                        >
                            <p>"This item starts open. Radio trigger keeps one item open in the group."</p>
                        </Collapse>
                        <Collapse
                            class="bg-base-100 border border-base-300"
                            title="Click to open this one and close others"
                            trigger={CollapseTrigger::Radio}
                            radio_name="collapse-basic"
                        >
                            <p>"Use this for FAQ and settings groups where only one item should expand."</p>
                        </Collapse>
                        <Collapse
                            class="bg-base-100 border border-base-300"
                            title="Click to open this one and close others"
                            trigger={CollapseTrigger::Radio}
                            radio_name="collapse-basic"
                        >
                            <p>"The component still supports arbitrary children and nested controls."</p>
                        </Collapse>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With icons"
                    code=r#"<Collapse
    class="bg-base-100 border border-base-300"
    title="Arrow indicator"
    trigger={CollapseTrigger::Checkbox}
    icon={CollapseIcon::Arrow}
>
    <p>"Uses daisyUI's `collapse-arrow` modifier."</p>
</Collapse>

<Collapse
    class="bg-base-100 border border-base-300"
    title="Plus / minus indicator"
    trigger={CollapseTrigger::Checkbox}
    icon={CollapseIcon::Plus}
>
    <p>"Uses daisyUI's `collapse-plus` modifier."</p>
</Collapse>"#
                >
                    <div class="space-y-3 max-w-3xl w-full">
                        <Collapse
                            class="bg-base-100 border border-base-300"
                            title="Arrow indicator"
                            trigger={CollapseTrigger::Checkbox}
                            icon={CollapseIcon::Arrow}
                        >
                            <p>"Uses daisyUI's `collapse-arrow` modifier."</p>
                        </Collapse>
                        <Collapse
                            class="bg-base-100 border border-base-300"
                            title="Plus / minus indicator"
                            trigger={CollapseTrigger::Checkbox}
                            icon={CollapseIcon::Plus}
                        >
                            <p>"Uses daisyUI's `collapse-plus` modifier."</p>
                        </Collapse>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Forced state classes"
                    code=r#"<Collapse
    class="bg-base-100 border border-base-300"
    title="Always open"
    trigger={CollapseTrigger::Checkbox}
    state={CollapseState::Open}
>
    <p>"This uses the `collapse-open` class."</p>
</Collapse>

<Collapse
    class="bg-base-100 border border-base-300"
    title="Always closed"
    trigger={CollapseTrigger::Checkbox}
    state={CollapseState::Close}
>
    <p>"This uses the `collapse-close` class."</p>
</Collapse>"#
                >
                    <div class="space-y-3 max-w-3xl w-full">
                        <Collapse
                            class="bg-base-100 border border-base-300"
                            title="Always open"
                            trigger={CollapseTrigger::Checkbox}
                            state={CollapseState::Open}
                        >
                            <p>"This uses the `collapse-open` class."</p>
                        </Collapse>
                        <Collapse
                            class="bg-base-100 border border-base-300"
                            title="Always closed"
                            trigger={CollapseTrigger::Checkbox}
                            state={CollapseState::Close}
                        >
                            <p>"This uses the `collapse-close` class."</p>
                        </Collapse>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Details / summary trigger"
                    code=r#"<Collapse
    class="bg-base-100 border border-base-300"
    title="Native details element"
    trigger={CollapseTrigger::Details}
>
    <p>"This variant uses semantic `<details>` and `<summary>` markup."</p>
</Collapse>

<Collapse
    class="bg-base-100 border border-base-300"
    title="Open by default"
    trigger={CollapseTrigger::Details}
    open=true
>
    <div class="space-y-3">
        <p>"You can still render actions inside content."</p>
        <div class="flex gap-2">
            <Button size={Size::Small} color={Color::Primary}>"Save"</Button>
            <Button size={Size::Small} variant={Variant::Ghost}>"Cancel"</Button>
        </div>
    </div>
</Collapse>"#
                >
                    <div class="space-y-3 max-w-3xl w-full">
                        <Collapse
                            class="bg-base-100 border border-base-300"
                            title="Native details element"
                            trigger={CollapseTrigger::Details}
                        >
                            <p>"This variant uses semantic `<details>` and `<summary>` markup."</p>
                        </Collapse>
                        <Collapse
                            class="bg-base-100 border border-base-300"
                            title="Open by default"
                            trigger={CollapseTrigger::Details}
                            open=true
                        >
                            <div class="space-y-3">
                                <p>"You can still render actions inside content."</p>
                                <div class="flex gap-2">
                                    <Button size={Size::Small} color={Color::Primary}>"Save"</Button>
                                    <Button size={Size::Small} variant={Variant::Ghost}>"Cancel"</Button>
                                </div>
                            </div>
                        </Collapse>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Accordion wrapper + parts"
                    code=r#"<Accordion>
    <Collapse
        class="join-item border border-base-300 bg-base-100"
        title="Managed with Accordion container"
        trigger={CollapseTrigger::Radio}
        radio_name="collapse-accordion"
        icon={CollapseIcon::Arrow}
        open=true
    >
        <p>"`Accordion` applies the recommended join classes for grouped elements."</p>
    </Collapse>
    <Collapse
        class="join-item border border-base-300 bg-base-100"
        title="Second accordion item"
        trigger={CollapseTrigger::Radio}
        radio_name="collapse-accordion"
        icon={CollapseIcon::Arrow}
    >
        <p>"Use `join-item` on each collapse panel for the classic stacked accordion look."</p>
    </Collapse>
</Accordion>

<div class="collapse collapse-plus border border-base-300 bg-base-100">
    <input type="checkbox" />
    <CollapseTitle>
        <div class="flex items-center gap-2">
            <span class="badge badge-info badge-sm">"Part"</span>
            "Custom title content with `CollapseTitle`"
        </div>
    </CollapseTitle>
    <CollapseContent>
        <p>"Use `CollapseTitle` and `CollapseContent` for full control."</p>
    </CollapseContent>
</div>"#
                >
                    <div class="w-full max-w-3xl">
                        <Accordion class="max-w-3xl">
                            <Collapse
                                class="join-item border border-base-300 bg-base-100"
                                title="Managed with Accordion container"
                                trigger={CollapseTrigger::Radio}
                                radio_name="collapse-accordion"
                                icon={CollapseIcon::Arrow}
                                open=true
                            >
                                <p>"`Accordion` applies the recommended join classes for grouped elements."</p>
                            </Collapse>
                            <Collapse
                                class="join-item border border-base-300 bg-base-100"
                                title="Second accordion item"
                                trigger={CollapseTrigger::Radio}
                                radio_name="collapse-accordion"
                                icon={CollapseIcon::Arrow}
                            >
                                <p>"Use `join-item` on each collapse panel for the classic stacked accordion look."</p>
                            </Collapse>
                        </Accordion>
                    </div>

                    <div class="mt-8 collapse collapse-plus border border-base-300 bg-base-100 max-w-3xl">
                        <input type="checkbox" />
                        <CollapseTitle>
                            <div class="flex items-center gap-2">
                                <span class="badge badge-info badge-sm">"Part"</span>
                                "Custom title content with `CollapseTitle`"
                            </div>
                        </CollapseTitle>
                        <CollapseContent>
                            <p>
                                "Use `CollapseTitle` and `CollapseContent` when you need full control over inner markup."
                            </p>
                        </CollapseContent>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

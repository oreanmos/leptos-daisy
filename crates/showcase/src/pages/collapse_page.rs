use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CollapsePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Collapse"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Checkbox-based Collapse"</h2>
                <Collapse title="Click to expand" trigger={CollapseTrigger::Checkbox}>
                    <p>"This content is revealed when you click the title. It uses a checkbox input for the toggle mechanism."</p>
                    <p class="mt-2">"You can put any content here, including other components."</p>
                </Collapse>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Details-based Collapse"</h2>
                <Collapse title="Click to toggle" trigger={CollapseTrigger::Details}>
                    <p>"This collapse uses the native HTML details/summary elements for the toggle behavior."</p>
                    <div class="mt-4">
                        <Button color={Color::Primary}>"Action Button"</Button>
                    </div>
                </Collapse>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Radio-based Collapse (Accordion)"</h2>
                <div class="space-y-2">
                    <Collapse
                        title="Accordion Item 1"
                        trigger={CollapseTrigger::Radio}
                        radio_name="accordion-demo"
                    >
                        <p>"This is the first accordion item. Only one item can be open at a time."</p>
                    </Collapse>
                    <Collapse
                        title="Accordion Item 2"
                        trigger={CollapseTrigger::Radio}
                        radio_name="accordion-demo"
                    >
                        <p>"This is the second accordion item. Opening this will close the first one."</p>
                    </Collapse>
                    <Collapse
                        title="Accordion Item 3"
                        trigger={CollapseTrigger::Radio}
                        radio_name="accordion-demo"
                    >
                        <p>"This is the third accordion item. Radio buttons ensure only one is selected."</p>
                    </Collapse>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Open by Default"</h2>
                <Collapse title="This starts open" trigger={CollapseTrigger::Details} open=true>
                    <p>"This collapse starts in the open state because of the open prop."</p>
                </Collapse>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Nested Content"</h2>
                <Collapse title="Complex Content" trigger={CollapseTrigger::Details}>
                    <div class="space-y-4">
                        <p>"You can include complex nested content:"</p>
                        <ul class="list-disc list-inside">
                            <li>"List items"</li>
                            <li>"More content"</li>
                            <li>"Even more"</li>
                        </ul>
                        <div class="flex gap-2">
                            <Button size={Size::Small} color={Color::Primary}>"Save"</Button>
                            <Button size={Size::Small} variant={Variant::Ghost}>"Cancel"</Button>
                        </div>
                    </div>
                </Collapse>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Using CollapseTitle and CollapseContent"</h2>
                <div class="collapse bg-base-200">
                    <input type="checkbox" />
                    <CollapseTitle>
                        <div class="flex items-center gap-2">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z" />
                            </svg>
                            "Custom Title with Icon"
                        </div>
                    </CollapseTitle>
                    <CollapseContent>
                        <p>"This example uses the CollapseTitle and CollapseContent components for more control over the structure."</p>
                    </CollapseContent>
                </div>
            </section>
        </div>
    }
}

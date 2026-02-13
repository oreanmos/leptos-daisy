use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FieldsetPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Fieldset"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Group related form inputs with optional legend labels."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Legend"
                    code=r##"<Fieldset legend="Personal Information">
    <div class="space-y-4">
        <div>
            <Label for_id="name">"Name"</Label>
            <Input placeholder="Enter your name" />
        </div>
        <div>
            <Label for_id="email">"Email"</Label>
            <Input placeholder="Enter your email" />
        </div>
    </div>
</Fieldset>"##
                >
                    <Fieldset legend="Personal Information">
                        <div class="space-y-4">
                            <div>
                                <Label for_id="name">"Name"</Label>
                                <Input placeholder="Enter your name" />
                            </div>
                            <div>
                                <Label for_id="email">"Email"</Label>
                                <Input placeholder="Enter your email" />
                            </div>
                        </div>
                    </Fieldset>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Without Legend"
                    code=r##"<Fieldset>
    <div class="space-y-4">
        <div>
            <Label for_id="username">"Username"</Label>
            <Input placeholder="Enter username" />
        </div>
        <div>
            <Label for_id="password">"Password"</Label>
            <Input placeholder="Enter password" />
        </div>
    </div>
</Fieldset>"##
                >
                    <Fieldset>
                        <div class="space-y-4">
                            <div>
                                <Label for_id="username">"Username"</Label>
                                <Input placeholder="Enter username" />
                            </div>
                            <div>
                                <Label for_id="password">"Password"</Label>
                                <Input placeholder="Enter password" />
                            </div>
                        </div>
                    </Fieldset>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Multiple Fieldsets"
                    code=r##"<Fieldset legend="Billing Address">
    <div class="space-y-2">
        <Input placeholder="Street Address" />
        <Input placeholder="City" />
        <Input placeholder="Postal Code" />
    </div>
</Fieldset>

<Fieldset legend="Shipping Address">
    <div class="space-y-2">
        <Input placeholder="Street Address" />
        <Input placeholder="City" />
        <Input placeholder="Postal Code" />
    </div>
</Fieldset>"##
                >
                    <div class="space-y-4">
                        <Fieldset legend="Billing Address">
                            <div class="space-y-2">
                                <Input placeholder="Street Address" />
                                <Input placeholder="City" />
                                <Input placeholder="Postal Code" />
                            </div>
                        </Fieldset>
                        <Fieldset legend="Shipping Address">
                            <div class="space-y-2">
                                <Input placeholder="Street Address" />
                                <Input placeholder="City" />
                                <Input placeholder="Postal Code" />
                            </div>
                        </Fieldset>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

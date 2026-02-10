use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FieldsetPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Fieldset"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Legend"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Without Legend"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Multiple Fieldsets"</h2>
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
            </section>
        </div>
    }
}

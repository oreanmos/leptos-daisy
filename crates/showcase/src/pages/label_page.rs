use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn LabelPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Label"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Form labels for inputs with support for floating labels and form integration."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Standard Labels"
                    code=r#"<div>
    <Label for_id="input1">"Username"</Label>
    <Input id="input1" placeholder="Enter username" />
</div>
<div>
    <Label for_id="input2">"Email Address"</Label>
    <Input id="input2" placeholder="Enter email" />
</div>
<div>
    <Label for_id="input3">"Password"</Label>
    <Input id="input3" placeholder="Enter password" />
</div>"#
                >
                    <div class="space-y-4">
                        <div>
                            <Label for_id="input1">"Username"</Label>
                            <Input id="input1" placeholder="Enter username" />
                        </div>
                        <div>
                            <Label for_id="input2">"Email Address"</Label>
                            <Input id="input2" placeholder="Enter email" />
                        </div>
                        <div>
                            <Label for_id="input3">"Password"</Label>
                            <Input id="input3" placeholder="Enter password" />
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Floating Labels"
                    code=r#"<label class="floating-label">
    <span>"Your Name"</span>
    <Input placeholder="Enter your name" />
</label>
<label class="floating-label">
    <span>"Your Email"</span>
    <Input placeholder="Enter your email" />
</label>
<label class="floating-label">
    <span>"Your Message"</span>
    <Textarea placeholder="Enter your message" />
</label>"#
                >
                    <div class="space-y-4">
                        <label class="floating-label">
                            <span>"Your Name"</span>
                            <Input placeholder="Enter your name" />
                        </label>
                        <label class="floating-label">
                            <span>"Your Email"</span>
                            <Input placeholder="Enter your email" />
                        </label>
                        <label class="floating-label">
                            <span>"Your Message"</span>
                            <Textarea placeholder="Enter your message" />
                        </label>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Labels with Form Elements"
                    code=r#"<Fieldset legend="Contact Form">
    <div class="space-y-4">
        <div>
            <Label for_id="fullname">"Full Name"</Label>
            <Input id="fullname" placeholder="John Doe" />
        </div>
        <div>
            <Label for_id="phone">"Phone Number"</Label>
            <Input id="phone" placeholder="+1 (555) 123-4567" />
        </div>
        <div>
            <Label for_id="subject">"Subject"</Label>
            <Select>
                <option>"General Inquiry"</option>
                <option>"Support Request"</option>
                <option>"Feedback"</option>
            </Select>
        </div>
    </div>
</Fieldset>"#
                >
                    <Fieldset legend="Contact Form">
                        <div class="space-y-4">
                            <div>
                                <Label for_id="fullname">"Full Name"</Label>
                                <Input id="fullname" placeholder="John Doe" />
                            </div>
                            <div>
                                <Label for_id="phone">"Phone Number"</Label>
                                <Input id="phone" placeholder="+1 (555) 123-4567" />
                            </div>
                            <div>
                                <Label for_id="subject">"Subject"</Label>
                                <Select>
                                    <option>"General Inquiry"</option>
                                    <option>"Support Request"</option>
                                    <option>"Feedback"</option>
                                </Select>
                            </div>
                        </div>
                    </Fieldset>
                </ComponentPreview>
            </section>
        </div>
    }
}

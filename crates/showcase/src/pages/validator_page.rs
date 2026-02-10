use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ValidatorPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Validator"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"No Validation"</h2>
                <Validator>
                    <Input placeholder="Enter text" />
                </Validator>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Error State"</h2>
                <Validator state={ValidatorState::Error} hint="This field is required">
                    <Input placeholder="Enter text" />
                </Validator>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Success State"</h2>
                <Validator state={ValidatorState::Success} hint="Looks good!">
                    <Input placeholder="Enter text" />
                </Validator>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Different Inputs"</h2>
                <div class="space-y-4">
                    <Validator state={ValidatorState::Error} hint="Invalid email format">
                        <Label>"Email"</Label>
                        <Input placeholder="email@example.com" />
                    </Validator>
                    <Validator state={ValidatorState::Success} hint="Username is available">
                        <Label>"Username"</Label>
                        <Input placeholder="username" />
                    </Validator>
                    <Validator state={ValidatorState::Error} hint="Password is too weak">
                        <Label>"Password"</Label>
                        <Input placeholder="Enter password" />
                    </Validator>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Form Example"</h2>
                <Fieldset legend="Registration">
                    <div class="space-y-4">
                        <Validator state={ValidatorState::Success} hint="Valid email">
                            <Label>"Email"</Label>
                            <Input placeholder="your@email.com" />
                        </Validator>
                        <Validator state={ValidatorState::Error} hint="Must be at least 8 characters">
                            <Label>"Password"</Label>
                            <Input placeholder="Create password" />
                        </Validator>
                        <Validator>
                            <Label>"Confirm Password"</Label>
                            <Input placeholder="Confirm password" />
                        </Validator>
                        <Button color={Color::Primary} class="w-full">"Register"</Button>
                    </div>
                </Fieldset>
            </section>
        </div>
    }
}

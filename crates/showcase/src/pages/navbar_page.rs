use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn NavbarPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Navbar"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <Navbar>
                    <NavbarStart>
                        <a class="btn btn-ghost text-xl">"daisyUI"</a>
                    </NavbarStart>
                    <NavbarCenter>
                        <a class="btn btn-ghost">"Center"</a>
                    </NavbarCenter>
                    <NavbarEnd>
                        <Button>"Action"</Button>
                    </NavbarEnd>
                </Navbar>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Colors"</h2>
                <Navbar class="bg-primary text-primary-content rounded-box">
                    <NavbarStart>
                        <a class="btn btn-ghost text-xl">"Primary Navbar"</a>
                    </NavbarStart>
                    <NavbarEnd>
                        <Button class="btn-ghost">"Button"</Button>
                    </NavbarEnd>
                </Navbar>
            </section>
        </div>
    }
}

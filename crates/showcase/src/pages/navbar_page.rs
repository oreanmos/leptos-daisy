use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn NavbarPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Navbar"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Common daisyUI navbar patterns for app headers, marketing navigation, and responsive actions."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic 3-slot navbar"</h2>
                <Navbar class="bg-base-100 rounded-box shadow-sm border border-base-300">
                    <NavbarStart>
                        <a class="btn btn-ghost text-xl">"daisyUI"</a>
                    </NavbarStart>
                    <NavbarCenter class="hidden lg:flex">
                        <ul class="menu menu-horizontal px-1 gap-1">
                            <li><a>"Features"</a></li>
                            <li><a>"Pricing"</a></li>
                            <li><a>"Docs"</a></li>
                        </ul>
                    </NavbarCenter>
                    <NavbarEnd>
                        <Button size={Size::Small}>"Login"</Button>
                    </NavbarEnd>
                </Navbar>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Glass / brand variants"</h2>
                <div class="space-y-4">
                    <Navbar glass=true rounded=true class="bg-primary/80 text-primary-content backdrop-blur">
                        <NavbarStart><a class="btn btn-ghost text-xl">"Primary"</a></NavbarStart>
                        <NavbarEnd><Button class="btn-ghost btn-sm">"Dashboard"</Button></NavbarEnd>
                    </Navbar>
                    <Navbar class="bg-neutral text-neutral-content rounded-box">
                        <NavbarStart><a class="btn btn-ghost text-lg">"Neutral"</a></NavbarStart>
                        <NavbarEnd class="gap-2">
                            <button class="btn btn-ghost btn-sm">"Sign in"</button>
                            <button class="btn btn-sm btn-primary">"Get started"</button>
                        </NavbarEnd>
                    </Navbar>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"With search and actions"</h2>
                <Navbar class="bg-base-100 rounded-box border border-base-300">
                    <NavbarStart>
                        <button class="btn btn-ghost btn-circle lg:hidden">
                            <span class="text-lg">"☰"</span>
                        </button>
                        <a class="btn btn-ghost text-lg">"Acme"</a>
                    </NavbarStart>
                    <NavbarCenter class="hidden md:flex">
                        <input class="input input-bordered input-sm w-72" placeholder="Search docs..." />
                    </NavbarCenter>
                    <NavbarEnd class="gap-1">
                        <button class="btn btn-ghost btn-circle">
                            <span class="indicator-item badge badge-xs badge-primary"></span>
                            "🔔"
                        </button>
                        <div class="avatar placeholder">
                            <div class="bg-base-300 text-base-content rounded-full w-9">
                                <span class="text-xs">"JD"</span>
                            </div>
                        </div>
                    </NavbarEnd>
                </Navbar>
            </section>
        </div>
    }
}

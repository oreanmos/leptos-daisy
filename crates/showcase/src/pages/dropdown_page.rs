use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn DropdownPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Dropdown"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "DaisyUI dropdown positions, trigger behaviors, and menu item states."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic dropdown"</h2>
                <Dropdown>
                    <DropdownTrigger>
                        <Button>"Open menu"</Button>
                    </DropdownTrigger>
                    <DropdownContent>
                        <DropdownItem href="/profile">"Profile"</DropdownItem>
                        <DropdownItem href="/settings" active=true>"Settings"</DropdownItem>
                        <DropdownItem href="/billing">"Billing"</DropdownItem>
                        <DropdownItem disabled=true>"Disabled action"</DropdownItem>
                    </DropdownContent>
                </Dropdown>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Hover vs click"</h2>
                <div class="flex flex-wrap gap-4">
                    <Dropdown hover={DropdownHover::Hover}>
                        <DropdownTrigger>
                            <Button color={Color::Primary}>"Hover trigger"</Button>
                        </DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"Team"</DropdownItem>
                            <DropdownItem>"Projects"</DropdownItem>
                            <DropdownItem>"Messages"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>

                    <Dropdown hover={DropdownHover::Click}>
                        <DropdownTrigger>
                            <Button variant={Variant::Outline}>"Click trigger"</Button>
                        </DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"Account"</DropdownItem>
                            <DropdownItem>"Notifications"</DropdownItem>
                            <DropdownItem>"Logout"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Positions"</h2>
                <div class="flex flex-wrap gap-4">
                    <Dropdown position={DropdownPosition::Top}>
                        <DropdownTrigger><Button>"Top"</Button></DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"Top item"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                    <Dropdown position={DropdownPosition::Bottom}>
                        <DropdownTrigger><Button>"Bottom"</Button></DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"Bottom item"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                    <Dropdown position={DropdownPosition::Start}>
                        <DropdownTrigger><Button>"Start"</Button></DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"Start item"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                    <Dropdown position={DropdownPosition::End}>
                        <DropdownTrigger><Button>"End"</Button></DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"End item"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                    <Dropdown position={DropdownPosition::Center}>
                        <DropdownTrigger><Button>"Center"</Button></DropdownTrigger>
                        <DropdownContent class="dropdown-center">
                            <DropdownItem>"Centered item"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Forced state"</h2>
                <div class="flex flex-wrap gap-4">
                    <Dropdown state={DropdownState::Open}>
                        <DropdownTrigger><Button color={Color::Success}>"Forced open"</Button></DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"This menu is forced open"</DropdownItem>
                            <DropdownItem>"Useful for demos"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>

                    <Dropdown state={DropdownState::Close}>
                        <DropdownTrigger><Button color={Color::Error}>"Forced closed"</Button></DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"This won't open"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                </div>
            </section>
        </div>
    }
}

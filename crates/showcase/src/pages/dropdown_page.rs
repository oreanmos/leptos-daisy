use crate::components::component_preview::ComponentPreview;
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
                <ComponentPreview
                    title="Basic dropdown"
                    code=r##"<Dropdown>
    <DropdownTrigger>
        <Button>"Open menu"</Button>
    </DropdownTrigger>
    <DropdownContent>
        <DropdownItem href="#">"Profile"</DropdownItem>
        <DropdownItem href="#" active=true>"Settings"</DropdownItem>
        <DropdownItem href="#">"Billing"</DropdownItem>
        <DropdownItem disabled=true>"Disabled action"</DropdownItem>
    </DropdownContent>
</Dropdown>"##
                >
                    <div class="h-64 flex items-start justify-center">
                        <Dropdown>
                            <DropdownTrigger>
                                <Button>"Open menu"</Button>
                            </DropdownTrigger>
                            <DropdownContent>
                                <DropdownItem href="#">"Profile"</DropdownItem>
                                <DropdownItem href="#" active=true>"Settings"</DropdownItem>
                                <DropdownItem href="#">"Billing"</DropdownItem>
                                <DropdownItem disabled=true>"Disabled action"</DropdownItem>
                            </DropdownContent>
                        </Dropdown>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Hover vs click"
                    code=r#"<Dropdown hover={DropdownHover::Hover}>
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
</Dropdown>"#
                >
                    <div class="h-64 flex flex-wrap gap-4 items-start justify-center">
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
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Positions"
                    code=r#"<Dropdown position={DropdownPosition::Top}>
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
</Dropdown>"#
                >
                    <div class="h-96 flex flex-wrap gap-4 items-center justify-center p-12">
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
                            <DropdownContent class="dropdown-center bg-base-200">
                                <DropdownItem>"Centered item"</DropdownItem>
                            </DropdownContent>
                        </Dropdown>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Forced state"
                    code=r#"<Dropdown state={DropdownState::Open}>
    <DropdownTrigger>
        <Button color={Color::Success}>"Forced open"</Button>
    </DropdownTrigger>
    <DropdownContent>
        <DropdownItem>"This menu is forced open"</DropdownItem>
        <DropdownItem>"Useful for demos"</DropdownItem>
    </DropdownContent>
</Dropdown>

<Dropdown state={DropdownState::Close}>
    <DropdownTrigger>
        <Button color={Color::Error}>"Forced closed"</Button>
    </DropdownTrigger>
    <DropdownContent>
        <DropdownItem>"This won't open"</DropdownItem>
    </DropdownContent>
</Dropdown>"#
                >
                    <div class="h-48 flex flex-wrap gap-4 items-start justify-center">
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
                </ComponentPreview>
            </section>
        </div>
    }
}

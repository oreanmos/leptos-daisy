use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn DropdownPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Dropdown"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <Dropdown>
                    <DropdownTrigger>
                        <Button>"Click me"</Button>
                    </DropdownTrigger>
                    <DropdownContent>
                        <DropdownItem>"Item 1"</DropdownItem>
                        <DropdownItem>"Item 2"</DropdownItem>
                        <DropdownItem>"Item 3"</DropdownItem>
                    </DropdownContent>
                </Dropdown>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Hover"</h2>
                <Dropdown hover={DropdownHover::Hover}>
                    <DropdownTrigger>
                        <Button color={Color::Primary}>"Hover me"</Button>
                    </DropdownTrigger>
                    <DropdownContent>
                        <DropdownItem>"Hover Item 1"</DropdownItem>
                        <DropdownItem>"Hover Item 2"</DropdownItem>
                    </DropdownContent>
                </Dropdown>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Positions"</h2>
                <div class="flex flex-wrap gap-4">
                    <Dropdown position={DropdownPosition::Top}>
                        <DropdownTrigger><Button>"Top"</Button></DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"Top item"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                    <Dropdown position={DropdownPosition::End}>
                        <DropdownTrigger><Button>"End"</Button></DropdownTrigger>
                        <DropdownContent>
                            <DropdownItem>"End item"</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                </div>
            </section>
        </div>
    }
}

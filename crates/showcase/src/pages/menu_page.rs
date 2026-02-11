use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MenuPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Menu"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Vertical, horizontal, responsive, and nested menu patterns for sidebars and app navigation."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Vertical menu"</h2>
                <Menu class="bg-base-200 w-64 rounded-box">
                    <MenuTitle>"Workspace"</MenuTitle>
                    <MenuItem active=true><a>"Dashboard"</a></MenuItem>
                    <MenuItem><a>"Projects"</a></MenuItem>
                    <MenuItem><a>"Tasks"</a></MenuItem>
                    <MenuItem disabled=true><a>"Archived"</a></MenuItem>
                </Menu>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Horizontal menu"</h2>
                <Menu horizontal=true class="bg-base-200 rounded-box">
                    <MenuItem active=true><a>"Overview"</a></MenuItem>
                    <MenuItem><a>"Reports"</a></MenuItem>
                    <MenuItem><a>"Team"</a></MenuItem>
                    <MenuItem><a>"Billing"</a></MenuItem>
                </Menu>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Responsive menu"</h2>
                <Menu responsive=true class="bg-base-200 rounded-box">
                    <MenuItem active=true><a>"Home"</a></MenuItem>
                    <MenuItem><a>"Features"</a></MenuItem>
                    <MenuItem><a>"Pricing"</a></MenuItem>
                    <MenuItem><a>"Contact"</a></MenuItem>
                </Menu>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="grid gap-4 md:grid-cols-3">
                    <Menu size={Size::ExtraSmall} class="bg-base-200 rounded-box">
                        <MenuItem><a>"XS Item"</a></MenuItem>
                        <MenuItem><a>"Another"</a></MenuItem>
                    </Menu>
                    <Menu size={Size::Small} class="bg-base-200 rounded-box">
                        <MenuItem><a>"SM Item"</a></MenuItem>
                        <MenuItem><a>"Another"</a></MenuItem>
                    </Menu>
                    <Menu size={Size::Large} class="bg-base-200 rounded-box">
                        <MenuItem><a>"LG Item"</a></MenuItem>
                        <MenuItem><a>"Another"</a></MenuItem>
                    </Menu>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Nested submenu"</h2>
                <Menu class="bg-base-200 w-72 rounded-box">
                    <MenuTitle>"Navigation"</MenuTitle>
                    <MenuItem><a>"Home"</a></MenuItem>
                    <SubMenu title="Products">
                        <MenuItem><a>"Analytics"</a></MenuItem>
                        <MenuItem><a>"Engagement"</a></MenuItem>
                        <SubMenu title="Developer">
                            <MenuItem><a>"API"</a></MenuItem>
                            <MenuItem><a>"SDK"</a></MenuItem>
                        </SubMenu>
                    </SubMenu>
                    <MenuItem><a>"Support"</a></MenuItem>
                </Menu>
            </section>
        </div>
    }
}

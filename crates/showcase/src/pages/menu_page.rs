use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MenuPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Menu"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical Menu (Default)"</h2>
                <Menu class="bg-base-200 w-56 rounded-box">
                    <MenuItem><a>"Item 1"</a></MenuItem>
                    <MenuItem><a>"Item 2"</a></MenuItem>
                    <MenuItem><a>"Item 3"</a></MenuItem>
                </Menu>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Horizontal Menu"</h2>
                <Menu horizontal=true class="bg-base-200 rounded-box">
                    <MenuItem><a>"Home"</a></MenuItem>
                    <MenuItem><a>"About"</a></MenuItem>
                    <MenuItem><a>"Services"</a></MenuItem>
                    <MenuItem><a>"Contact"</a></MenuItem>
                </Menu>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Extra Small"</span>
                        <Menu size={Size::ExtraSmall} class="bg-base-200 w-56 rounded-box">
                            <MenuItem><a>"Item 1"</a></MenuItem>
                            <MenuItem><a>"Item 2"</a></MenuItem>
                        </Menu>
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Small"</span>
                        <Menu size={Size::Small} class="bg-base-200 w-56 rounded-box">
                            <MenuItem><a>"Item 1"</a></MenuItem>
                            <MenuItem><a>"Item 2"</a></MenuItem>
                        </Menu>
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Large"</span>
                        <Menu size={Size::Large} class="bg-base-200 w-56 rounded-box">
                            <MenuItem><a>"Item 1"</a></MenuItem>
                            <MenuItem><a>"Item 2"</a></MenuItem>
                        </Menu>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Titles"</h2>
                <Menu class="bg-base-200 w-56 rounded-box">
                    <MenuTitle>"Main"</MenuTitle>
                    <MenuItem><a>"Dashboard"</a></MenuItem>
                    <MenuItem><a>"Profile"</a></MenuItem>
                    <MenuTitle>"Settings"</MenuTitle>
                    <MenuItem><a>"General"</a></MenuItem>
                    <MenuItem><a>"Security"</a></MenuItem>
                </Menu>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Active and Disabled Items"</h2>
                <Menu class="bg-base-200 w-56 rounded-box">
                    <MenuItem active=true><a>"Active Item"</a></MenuItem>
                    <MenuItem><a>"Normal Item"</a></MenuItem>
                    <MenuItem disabled=true><a>"Disabled Item"</a></MenuItem>
                </Menu>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Dropdowns"</h2>
                <Menu class="bg-base-200 w-56 rounded-box">
                    <MenuItem><a>"Parent"</a></MenuItem>
                    <MenuDropdown trigger=Box::new(|| view! { <a>"Level 1"</a> }.into_any())>
                        <MenuItem><a>"Level 2"</a></MenuItem>
                        <MenuDropdown trigger=Box::new(|| view! { <a>"Level 2"</a> }.into_any())>
                            <MenuItem><a>"Level 3"</a></MenuItem>
                            <MenuItem><a>"Level 3"</a></MenuItem>
                        </MenuDropdown>
                    </MenuDropdown>
                    <MenuItem><a>"Parent"</a></MenuItem>
                </Menu>
            </section>
        </div>
    }
}

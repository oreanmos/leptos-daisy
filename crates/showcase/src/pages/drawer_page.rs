use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn DrawerPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
             <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Drawer"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Drawer is a grid layout that can show/hide a sidebar on the left or right side of the page."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "drawer", type_label: "base", description: "Base drawer container" },
                    ClassEntry { name: "drawer-toggle", type_label: "base part", description: "Hidden checkbox for state" },
                    ClassEntry { name: "drawer-content", type_label: "base part", description: "Main content area" },
                    ClassEntry { name: "drawer-side", type_label: "base part", description: "Sidebar area" },
                    ClassEntry { name: "drawer-overlay", type_label: "base part", description: "Overlay behind drawer" },
                    ClassEntry { name: "drawer-open", type_label: "state", description: "Forces drawer open" },
                    ClassEntry { name: "drawer-end", type_label: "modifier", description: "Drawer on right side" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Left Drawer"
                    code=r#"<Drawer id="drawer-left">
    <DrawerContent>
        <div class="p-4 bg-base-200 h-full">
            <label for="drawer-left" class="btn btn-primary drawer-button">
                "Open Drawer"
            </label>
            <p class="mt-4">"Main content area."</p>
        </div>
    </DrawerContent>
    <DrawerSide>
        <DrawerOverlay drawer_id="drawer-left" />
        <div class="bg-base-100 min-h-full w-64 p-4">
            <h2 class="text-xl font-bold mb-4">"Drawer Menu"</h2>
            <ul class="menu p-0">
                <li><a>"Home"</a></li>
                <li><a>"About"</a></li>
                <li><a>"Services"</a></li>
                <li><a>"Contact"</a></li>
            </ul>
        </div>
    </DrawerSide>
</Drawer>"#
                >
                    <div class="h-64 border border-base-300 rounded-box overflow-hidden relative">
                         <Drawer id="drawer-left" class="absolute inset-0 w-full h-full">
                            <DrawerContent>
                                <div class="p-4 bg-base-200 h-full flex flex-col items-center justify-center">
                                    <label for="drawer-left" class="btn btn-primary drawer-button">"Open Drawer"</label>
                                    <p class="mt-4">"Main content area."</p>
                                </div>
                            </DrawerContent>
                            <DrawerSide class="absolute h-full">
                                <DrawerOverlay drawer_id="drawer-left" />
                                <div class="bg-base-100 min-h-full w-64 p-4 border-r border-base-300">
                                    <h2 class="text-xl font-bold mb-4">"Drawer Menu"</h2>
                                    <ul class="menu p-0">
                                        <li><a>"Home"</a></li>
                                        <li><a>"About"</a></li>
                                        <li><a>"Services"</a></li>
                                        <li><a>"Contact"</a></li>
                                    </ul>
                                </div>
                            </DrawerSide>
                        </Drawer>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Right Drawer"
                     code=r#"<Drawer id="drawer-right" end=true>
    <DrawerContent>
        <div class="p-4 bg-base-200 h-full">
            <label for="drawer-right" class="btn btn-secondary drawer-button">
                "Open Right Drawer"
            </label>
        </div>
    </DrawerContent>
    <DrawerSide>
        <DrawerOverlay drawer_id="drawer-right" />
        <div class="bg-base-100 min-h-full w-64 p-4">
            <h2 class="text-xl font-bold mb-4">"Settings"</h2>
            <div class="space-y-4">
                <div>
                    <label class="label">"Theme"</label>
                    <Select>
                        <option>"Light"</option>
                        <option>"Dark"</option>
                        <option>"Auto"</option>
                    </Select>
                </div>
            </div>
        </div>
    </DrawerSide>
</Drawer>"#
                >
                    <div class="h-64 border border-base-300 rounded-box overflow-hidden relative">
                        <Drawer id="drawer-right" end=true class="absolute inset-0 w-full h-full">
                            <DrawerContent>
                                <div class="p-4 bg-base-200 h-full flex flex-col items-center justify-center">
                                    <label for="drawer-right" class="btn btn-secondary drawer-button">"Open Right Drawer"</label>
                                </div>
                            </DrawerContent>
                            <DrawerSide class="absolute h-full">
                                <DrawerOverlay drawer_id="drawer-right" />
                                <div class="bg-base-100 min-h-full w-64 p-4 border-l border-base-300">
                                    <h2 class="text-xl font-bold mb-4">"Settings"</h2>
                                    <div class="space-y-4">
                                        <div>
                                            <label class="label">"Theme"</label>
                                            <Select>
                                                <option>"Light"</option>
                                                <option>"Dark"</option>
                                                <option>"Auto"</option>
                                            </Select>
                                        </div>
                                    </div>
                                </div>
                            </DrawerSide>
                        </Drawer>
                    </div>
                </ComponentPreview>
            </section>

             <section class="space-y-4">
                <ComponentPreview
                    title="Drawer with Navigation"
                    code=r#"<Drawer id="drawer-nav">
    <DrawerContent>
        <div class="p-4 bg-base-200 h-full">
            <div class="navbar bg-base-100 rounded-lg mb-4">
                <div class="flex-none">
                    <label for="drawer-nav" class="btn btn-square btn-ghost">
                        // Hamburger icon SVG
                    </label>
                </div>
                <div class="flex-1">
                    <span class="text-xl font-bold">"My App"</span>
                </div>
            </div>
            <p>"Click hamburger menu"</p>
        </div>
    </DrawerContent>
    <DrawerSide>
        <DrawerOverlay drawer_id="drawer-nav" />
        <div class="bg-base-100 min-h-full w-64 p-4">
            <div class="flex items-center justify-between mb-4">
                <h2 class="text-xl font-bold">"Navigation"</h2>
                <label for="drawer-nav" class="btn btn-sm btn-circle btn-ghost">"✕"</label>
            </div>
            <Menu class="w-full p-0">
                <MenuItem active=true><a>"Dashboard"</a></MenuItem>
                <MenuItem><a>"Projects"</a></MenuItem>
                <MenuItem><a>"Messages"</a></MenuItem>
            </Menu>
        </div>
    </DrawerSide>
</Drawer>"#
                >
                    <div class="h-80 border border-base-300 rounded-box overflow-hidden relative">
                        <Drawer id="drawer-nav" class="absolute inset-0 w-full h-full">
                            <DrawerContent>
                                <div class="p-4 bg-base-200 h-full">
                                    <div class="navbar bg-base-100 rounded-lg mb-4 shadow-sm">
                                        <div class="flex-none">
                                            <label for="drawer-nav" class="btn btn-square btn-ghost">
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                                                </svg>
                                            </label>
                                        </div>
                                        <div class="flex-1">
                                            <span class="text-xl font-bold">"My App"</span>
                                        </div>
                                    </div>
                                    <div class="flex justify-center items-center h-32">
                                        <p>"Click hamburger menu"</p>
                                    </div>
                                </div>
                            </DrawerContent>
                            <DrawerSide class="absolute h-full">
                                <DrawerOverlay drawer_id="drawer-nav" />
                                <div class="bg-base-100 min-h-full w-64 p-4 border-r border-base-300">
                                    <div class="flex items-center justify-between mb-4">
                                        <h2 class="text-xl font-bold">"Navigation"</h2>
                                        <label for="drawer-nav" class="btn btn-sm btn-circle btn-ghost">"✕"</label>
                                    </div>
                                    <Menu class="w-full p-0">
                                        <MenuItem active=true><a>"Dashboard"</a></MenuItem>
                                        <MenuItem><a>"Projects"</a></MenuItem>
                                        <MenuItem><a>"Messages"</a></MenuItem>
                                    </Menu>
                                </div>
                            </DrawerSide>
                        </Drawer>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Programmatic Control"
                    code=r#"<Drawer id="drawer-programmatic">
    <DrawerContent>
        <div class="p-4 bg-base-200 h-full">
            <label for="drawer-programmatic" class="btn btn-primary drawer-button">
                "Toggle Drawer"
            </label>
            <p class="mt-4">"Main Content"</p>
        </div>
    </DrawerContent>
    <DrawerSide>
        <DrawerOverlay drawer_id="drawer-programmatic" />
        <div class="bg-base-100 h-full w-64 p-4">
            <h3 class="font-bold">"Controlled Drawer"</h3>
            <p class="py-4">"This drawer is toggled via the checkbox."</p>
            <label for="drawer-programmatic" class="btn btn-sm">"Close"</label>
        </div>
    </DrawerSide>
</Drawer>"#
                >
                    <div class="h-64 border border-base-300 rounded-box overflow-hidden relative">
                        <Drawer id="drawer-programmatic" class="absolute inset-0 w-full h-full">
                            <DrawerContent>
                                <div class="p-4 bg-base-200 h-full flex items-center justify-center">
                                    <label for="drawer-programmatic" class="btn btn-primary drawer-button">"Toggle Drawer"</label>
                                </div>
                            </DrawerContent>
                            <DrawerSide class="absolute h-full">
                                <DrawerOverlay drawer_id="drawer-programmatic" />
                                <div class="bg-base-100 h-full w-64 p-4 border-r border-base-300">
                                    <h3 class="font-bold">"Controlled Drawer"</h3>
                                    <p class="py-4">"This drawer is toggled via the checkbox."</p>
                                    <label for="drawer-programmatic" class="btn btn-sm">"Close"</label>
                                </div>
                            </DrawerSide>
                        </Drawer>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

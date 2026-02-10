use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn DrawerPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Drawer"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Left Drawer"</h2>
                <Drawer id="drawer-left">
                    <DrawerContent>
                        <div class="p-4 bg-base-200 h-64 flex flex-col">
                            <label for="drawer-left" class="btn btn-primary drawer-button">"Open Drawer"</label>
                            <p class="mt-4">"Main content area. Click the button to open the drawer."</p>
                        </div>
                    </DrawerContent>
                    <DrawerSide>
                        <DrawerOverlay drawer_id="drawer-left" />
                        <div class="bg-base-200 min-h-full w-80 p-4">
                            <h2 class="text-xl font-bold mb-4">"Drawer Menu"</h2>
                            <ul class="menu">
                                <li><a>"Home"</a></li>
                                <li><a>"About"</a></li>
                                <li><a>"Services"</a></li>
                                <li><a>"Contact"</a></li>
                            </ul>
                        </div>
                    </DrawerSide>
                </Drawer>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Right Drawer"</h2>
                <Drawer id="drawer-right" end=true>
                    <DrawerContent>
                        <div class="p-4 bg-base-200 h-64 flex flex-col">
                            <label for="drawer-right" class="btn btn-secondary drawer-button">"Open Right Drawer"</label>
                            <p class="mt-4">"Main content area. Click the button to open the drawer from the right."</p>
                        </div>
                    </DrawerContent>
                    <DrawerSide>
                        <DrawerOverlay drawer_id="drawer-right" />
                        <div class="bg-base-200 min-h-full w-80 p-4">
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
                                <div>
                                    <label class="label">"Notifications"</label>
                                    <Toggle />
                                </div>
                            </div>
                        </div>
                    </DrawerSide>
                </Drawer>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Drawer with Navigation"</h2>
                <Drawer id="drawer-nav">
                    <DrawerContent>
                        <div class="p-4 bg-base-200 h-64">
                            <div class="navbar bg-base-100 rounded-lg mb-4">
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
                            <p>"Click the hamburger menu to open the navigation drawer."</p>
                        </div>
                    </DrawerContent>
                    <DrawerSide>
                        <DrawerOverlay drawer_id="drawer-nav" />
                        <div class="bg-base-200 min-h-full w-80 p-4">
                            <div class="flex items-center justify-between mb-4">
                                <h2 class="text-xl font-bold">"Navigation"</h2>
                                <label for="drawer-nav" class="btn btn-sm btn-circle">"✕"</label>
                            </div>
                            <Menu class="w-full">
                                <MenuItem active=true><a>"Dashboard"</a></MenuItem>
                                <MenuItem><a>"Projects"</a></MenuItem>
                                <MenuItem><a>"Tasks"</a></MenuItem>
                                <MenuItem><a>"Calendar"</a></MenuItem>
                                <MenuItem><a>"Messages"</a></MenuItem>
                                <MenuTitle>"Settings"</MenuTitle>
                                <MenuItem><a>"Profile"</a></MenuItem>
                                <MenuItem><a>"Preferences"</a></MenuItem>
                            </Menu>
                        </div>
                    </DrawerSide>
                </Drawer>
            </section>
        </div>
    }
}

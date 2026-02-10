use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MockupWindowPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Mockup Window"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Window"</h2>
                <MockupWindow title="My Application">
                    <div class="p-8 bg-base-200">
                        <p>"This is content inside a window mockup."</p>
                    </div>
                </MockupWindow>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Title Bar"</h2>
                <MockupWindow title="Terminal">
                    <div class="bg-neutral text-neutral-content p-4 font-mono text-sm">
                        <p>"$ echo 'Hello, World!'"</p>
                        <p>"Hello, World!"</p>
                        <p>"$ _"</p>
                    </div>
                </MockupWindow>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Editor Window"</h2>
                <MockupWindow title="main.rs - leptos-daisyui">
                    <div class="bg-base-300 p-4 font-mono text-sm">
                        <div class="flex">
                            <div class="text-base-content/30 pr-4 text-right select-none">
                                <div>"1"</div>
                                <div>"2"</div>
                                <div>"3"</div>
                                <div>"4"</div>
                                <div>"5"</div>
                                <div>"6"</div>
                                <div>"7"</div>
                                <div>"8"</div>
                            </div>
                            <div class="text-base-content">
                                <div><span class="text-purple-500">"use"</span> " leptos::prelude::*;"</div>
                                <div><span class="text-purple-500">"use"</span> " leptos_daisyui::prelude::*;"</div>
                                <div>""</div>
                                <div><span class="text-yellow-500">"#[component]"</span></div>
                                <div><span class="text-blue-500">"fn"</span> " App() " <span class="text-blue-500">"->"</span> " impl IntoView {"</div>
                                <div>"    " <span class="text-yellow-500">"view!"</span> " {"</div>
                                <div>"        <Button>"<span class="text-green-500">"\"Click me\""</span>"</Button>"</div>
                                <div>"    }"</div>
                                <div>"}"</div>
                            </div>
                        </div>
                    </div>
                </MockupWindow>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"File Explorer"</h2>
                <MockupWindow title="Project Explorer">
                    <div class="flex h-64 bg-base-200">
                        <div class="w-48 bg-base-300 p-2 text-sm">
                            <div class="font-semibold mb-2">"src"</div>
                            <div class="pl-4 space-y-1">
                                <div class="flex items-center gap-1">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
                                    </svg>
                                    "main.rs"
                                </div>
                                <div class="flex items-center gap-1">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
                                    </svg>
                                    "lib.rs"
                                </div>
                                <div class="flex items-center gap-1">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                                    </svg>
                                    "components/"
                                </div>
                            </div>
                        </div>
                        <div class="flex-1 p-4">
                            <p class="text-base-content/70">"Select a file to view its contents"</p>
                        </div>
                    </div>
                </MockupWindow>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Settings Window"</h2>
                <MockupWindow title="Settings">
                    <div class="bg-base-200 p-4">
                        <div class="space-y-4 max-w-md">
                            <div class="flex items-center justify-between">
                                <span>"Dark Mode"</span>
                                <Toggle checked=true />
                            </div>
                            <div class="flex items-center justify-between">
                                <span>"Notifications"</span>
                                <Toggle checked=true />
                            </div>
                            <div class="flex items-center justify-between">
                                <span>"Auto-save"</span>
                                <Toggle />
                            </div>
                            <Divider />
                            <div>
                                <Label>"Language"</Label>
                                <Select>
                                    <option>"English"</option>
                                    <option>"Spanish"</option>
                                    <option>"French"</option>
                                </Select>
                            </div>
                        </div>
                    </div>
                </MockupWindow>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Alert Dialog"</h2>
                <MockupWindow title="Warning" class="max-w-md mx-auto">
                    <div class="bg-base-200 p-6">
                        <div class="flex items-start gap-4">
                            <div class="text-warning">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                                </svg>
                            </div>
                            <div>
                                <h3 class="font-bold">"Unsaved Changes"</h3>
                                <p class="text-sm text-base-content/70 mt-1">"You have unsaved changes. Are you sure you want to exit?"</p>
                                <div class="flex gap-2 mt-4">
                                    <Button size={Size::Small} variant={Variant::Ghost}>"Cancel"</Button>
                                    <Button size={Size::Small} color={Color::Warning}>"Discard"</Button>
                                </div>
                            </div>
                        </div>
                    </div>
                </MockupWindow>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Multiple Windows"</h2>
                <div class="space-y-4">
                    <MockupWindow title="Window 1" class="opacity-75">
                        <div class="h-20 bg-base-200 p-4">
                            <p>"Background window"</p>
                        </div>
                    </MockupWindow>
                    <MockupWindow title="Active Window">
                        <div class="h-32 bg-base-200 p-4">
                            <p>"Foreground window with focus"</p>
                        </div>
                    </MockupWindow>
                </div>
            </section>
        </div>
    }
}

use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn DockPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Dock"</h1>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "dock", type_label: "base", description: "Base class for dock" },
                    ClassEntry { name: "dock-xs", type_label: "size", description: "Extra small dock" },
                    ClassEntry { name: "dock-sm", type_label: "size", description: "Small dock" },
                    ClassEntry { name: "dock-md", type_label: "size", description: "Medium dock (default)" },
                    ClassEntry { name: "dock-lg", type_label: "size", description: "Large dock" },
                ] />
            </section>

            <section>
                <ComponentPreview
                    title="Basic Dock"
                    code=r#"<Dock items={vec![
    DockItem::active("Home", "<svg>...</svg>"),
    DockItem::new("Search", "<svg>...</svg>"),
    DockItem::new("Favorites", "<svg>...</svg>"),
    DockItem::new("Profile", "<svg>...</svg>"),
]} />"#
                >
                    <div class="relative h-32 bg-base-200 rounded-lg">
                        <div class="absolute bottom-0 left-0 right-0">
                            <Dock items={vec![
                                DockItem::active("Home", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'><path stroke-linecap='round' stroke-linejoin='round' d='M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25'/></svg>"),
                                DockItem::new("Search", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'><path stroke-linecap='round' stroke-linejoin='round' d='M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z'/></svg>"),
                                DockItem::new("Favorites", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'><path stroke-linecap='round' stroke-linejoin='round' d='M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z'/></svg>"),
                                DockItem::new("Profile", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'><path stroke-linecap='round' stroke-linejoin='round' d='M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z'/></svg>"),
                            ]} />
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Extra Small Dock"
                    code=r#"<Dock size={Size::ExtraSmall} items={vec![
    DockItem::active("Home", "<svg>...</svg>"),
    DockItem::new("Search", "<svg>...</svg>"),
    DockItem::new("Profile", "<svg>...</svg>"),
]} />"#
                >
                    <div class="relative h-24 bg-base-200 rounded-lg">
                        <div class="absolute bottom-0 left-0 right-0">
                            <Dock
                                size={Size::ExtraSmall}
                                items={vec![
                                    DockItem::active("Home", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-4 h-4'><path stroke-linecap='round' stroke-linejoin='round' d='M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25'/></svg>"),
                                    DockItem::new("Search", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-4 h-4'><path stroke-linecap='round' stroke-linejoin='round' d='M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z'/></svg>"),
                                    DockItem::new("Profile", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-4 h-4'><path stroke-linecap='round' stroke-linejoin='round' d='M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z'/></svg>"),
                                ]}
                            />
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Large Dock"
                    code=r#"<Dock size={Size::Large} items={vec![
    DockItem::active("Home", "<svg>...</svg>"),
    DockItem::new("Search", "<svg>...</svg>"),
    DockItem::new("Profile", "<svg>...</svg>"),
]} />"#
                >
                    <div class="relative h-32 bg-base-200 rounded-lg">
                        <div class="absolute bottom-0 left-0 right-0">
                            <Dock
                                size={Size::Large}
                                items={vec![
                                    DockItem::active("Home", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-8 h-8'><path stroke-linecap='round' stroke-linejoin='round' d='M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25'/></svg>"),
                                    DockItem::new("Search", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-8 h-8'><path stroke-linecap='round' stroke-linejoin='round' d='M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z'/></svg>"),
                                    DockItem::new("Profile", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-8 h-8'><path stroke-linecap='round' stroke-linejoin='round' d='M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z'/></svg>"),
                                ]}
                            />
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With Hrefs"
                    code=r#"<Dock items={vec![
    DockItem::new("Home", "<svg>...</svg>").with_href("/"),
    DockItem::new("Button", "<svg>...</svg>"),
    DockItem::active("Active", "<svg>...</svg>"),
]} />"#
                >
                    <div class="relative h-32 bg-base-200 rounded-lg">
                        <div class="absolute bottom-0 left-0 right-0">
                            <Dock items={vec![
                                DockItem::new("Home", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'><path stroke-linecap='round' stroke-linejoin='round' d='M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25'/></svg>").with_href("/"),
                                DockItem::new("Button", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'><path stroke-linecap='round' stroke-linejoin='round' d='M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z'/></svg>"),
                                DockItem::active("Active", "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'><path stroke-linecap='round' stroke-linejoin='round' d='M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z'/></svg>"),
                            ]} />
                        </div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

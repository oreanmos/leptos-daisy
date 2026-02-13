use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;

use leptos_daisyui::prelude::*;

use crate::pages::*;

/// All 35 daisyUI built-in themes.
const THEMES: &[&str] = &[
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
    "dim",
    "nord",
    "sunset",
    "caramellatte",
    "abyss",
    "silk",
];

/// Navigation items: (category, icon, items).
#[allow(clippy::type_complexity)]
const NAV_SECTIONS: &[(&str, &str, &[(&str, &str)])] = &[
    (
        "Actions",
        "🎬",
        &[
            ("Button", "/button"),
            ("Dropdown", "/dropdown"),
            ("Modal", "/modal"),
            ("Swap", "/swap"),
        ],
    ),
    (
        "Data Display",
        "📊",
        &[
            ("Alert", "/alert"),
            ("Avatar", "/avatar"),
            ("Badge", "/badge"),
            ("Card", "/card"),
            ("Carousel", "/carousel"),
            ("Chat", "/chat"),
            ("Collapse", "/collapse"),
            ("Countdown", "/countdown"),
            ("Diff", "/diff"),
            ("Kbd", "/kbd"),
            ("List", "/list"),
            ("Progress", "/progress"),
            ("Radial Progress", "/radial-progress"),
            ("Rating", "/rating"),
            ("Stat", "/stat"),
            ("Status", "/status"),
            ("Table", "/table"),
            ("Timeline", "/timeline"),
        ],
    ),
    (
        "Data Input",
        "📝",
        &[
            ("Checkbox", "/checkbox"),
            ("Fieldset", "/fieldset"),
            ("File Input", "/file-input"),
            ("Filter", "/filter"),
            ("Input", "/input"),
            ("Label", "/label"),
            ("Radio", "/radio"),
            ("Range", "/range"),
            ("Select", "/select"),
            ("Textarea", "/textarea"),
            ("Toggle", "/toggle"),
            ("Validator", "/validator"),
        ],
    ),
    (
        "Feedback",
        "💬",
        &[
            ("Loading", "/loading"),
            ("Skeleton", "/skeleton"),
            ("Toast", "/toast"),
        ],
    ),
    (
        "Layout",
        "📐",
        &[
            ("Artboard", "/artboard"),
            ("Divider", "/divider"),
            ("Drawer", "/drawer"),
            ("Footer", "/footer"),
            ("Hero", "/hero"),
            ("Indicator", "/indicator"),
            ("Join", "/join"),
            ("Mask", "/mask"),
            ("Stack", "/stack"),
            ("DaisyUI Layout", "/layout"),
            ("Stacked Layout", "/layouts/stacked"),
            ("Sidebar Layout", "/layouts/sidebar"),
            ("Multi-Column", "/layouts/multi-column"),
        ],
    ),
    (
        "Mockups",
        "🖼️",
        &[
            ("Browser", "/mockup-browser"),
            ("Code", "/mockup-code"),
            ("Phone", "/mockup-phone"),
            ("Window", "/mockup-window"),
        ],
    ),
    (
        "Navigation",
        "🧭",
        &[
            ("Bottom Navigation", "/bottom-navigation"),
            ("Breadcrumbs", "/breadcrumbs"),
            ("Dock", "/dock"),
            ("Link", "/link"),
            ("Menu", "/menu"),
            ("Navbar", "/navbar"),
            ("Pagination", "/pagination"),
            ("Steps", "/steps"),
            ("Tab", "/tab"),
        ],
    ),
    (
        "Overlay",
        "🔲",
        &[("Backdrop", "/backdrop"), ("Tooltip", "/tooltip")],
    ),
    (
        "Theming",
        "🎨",
        &[
            ("Theme Showcase", "/themes"),
            ("Theme Creator", "/themes/creator"),
            ("Theme Guide", "/themes/guide"),
        ],
    ),
    ("Tools", "🛠️", &[("Playground", "/playground")]),
];

/// Sets `data-theme` on the `<html>` element so daisyUI themes work globally.
fn set_html_theme(theme: &str) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        if let Some(doc) = leptos::prelude::document().document_element() {
            let _ = doc.set_attribute("data-theme", theme);
        }
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        let _ = theme;
    }
}

/// Read current theme from localStorage, or default to "light".
fn load_saved_theme() -> String {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        leptos::prelude::window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|s| s.get_item("daisy-theme").ok().flatten())
            .unwrap_or_else(|| "light".to_string())
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        "light".to_string()
    }
}

/// Save theme to localStorage.
fn save_theme(theme: &str) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        if let Ok(Some(storage)) = leptos::prelude::window().local_storage() {
            let _ = storage.set_item("daisy-theme", theme);
        }
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        let _ = theme;
    }
}

/// Component to determine if a nav item is active based on current path
#[component]
fn NavItemWithActiveState(href: String, children: Children) -> impl IntoView {
    let location = use_location();
    let href_clone = href.clone();

    // Keep active route status reactive to client-side navigation changes.
    let is_active =
        Signal::derive(move || location.pathname.with(|pathname| pathname == &href_clone));

    view! {
        <SidebarLayoutNavItem href={href} active=is_active>
            {children()}
        </SidebarLayoutNavItem>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let initial_theme = load_saved_theme();
    set_html_theme(&initial_theme);
    let (theme, set_theme) = signal(initial_theme);

    // Apply theme to <html> whenever it changes
    Effect::new(move || {
        let t = theme.get();
        set_html_theme(&t);
        save_theme(&t);
    });

    view! {
        <Router>
            <SidebarLayout
                variant=SidebarLayoutVariant::Light
                width=SidebarLayoutWidth::Narrow
                open_on_large=true
            >
                <SidebarLayoutContent>
                    // Top bar with mobile menu and theme selector
                    <SidebarLayoutTopBar>
                        <SidebarLayoutTopBarStart>
                            <SidebarLayoutMobileMenuButton />
                            <A href="/" attr:class="btn btn-ghost text-xl gap-2">
                                <span>"🌼"</span>
                                <span class="hidden sm:inline">"leptos-daisyui"</span>
                            </A>
                        </SidebarLayoutTopBarStart>
                        <SidebarLayoutTopBarCenter>
                            <span class="text-lg font-semibold opacity-70">"Component Showcase"</span>
                        </SidebarLayoutTopBarCenter>
                        <SidebarLayoutTopBarEnd>
                            <ThemeSelector theme=theme set_theme=set_theme />
                        </SidebarLayoutTopBarEnd>
                    </SidebarLayoutTopBar>

                    // Main content area
                    <SidebarLayoutMain>
                        <div class="p-4 md:p-6 max-w-6xl w-full mx-auto">
                            <Routes fallback=|| view! { <p class="text-error text-xl">"404 — Page not found"</p> }>
                                <Route path=path!("/") view=HomePage />
                                // Actions
                                <Route path=path!("/button") view=ButtonPage />
                                <Route path=path!("/dropdown") view=DropdownPage />
                                <Route path=path!("/modal") view=ModalPage />
                                <Route path=path!("/swap") view=SwapPage />
                                // Data Display
                                <Route path=path!("/alert") view=AlertPage />
                                <Route path=path!("/avatar") view=AvatarPage />
                                <Route path=path!("/badge") view=BadgePage />
                                <Route path=path!("/card") view=CardPage />
                                <Route path=path!("/carousel") view=CarouselPage />
                                <Route path=path!("/chat") view=ChatPage />
                                <Route path=path!("/collapse") view=CollapsePage />
                                <Route path=path!("/countdown") view=CountdownPage />
                                <Route path=path!("/diff") view=DiffPage />
                                <Route path=path!("/kbd") view=KbdPage />
                                <Route path=path!("/list") view=ListPage />
                                <Route path=path!("/progress") view=ProgressPage />
                                <Route path=path!("/radial-progress") view=RadialProgressPage />
                                <Route path=path!("/rating") view=RatingPage />
                                <Route path=path!("/skeleton") view=SkeletonPage />
                                <Route path=path!("/stat") view=StatPage />
                                <Route path=path!("/status") view=StatusPage />
                                <Route path=path!("/table") view=TablePage />
                                <Route path=path!("/timeline") view=TimelinePage />
                                // Data Input
                                <Route path=path!("/checkbox") view=CheckboxPage />
                                <Route path=path!("/fieldset") view=FieldsetPage />
                                <Route path=path!("/file-input") view=FileInputPage />
                                <Route path=path!("/filter") view=FilterPage />
                                <Route path=path!("/input") view=InputPage />
                                <Route path=path!("/label") view=LabelPage />
                                <Route path=path!("/radio") view=RadioPage />
                                <Route path=path!("/range") view=RangePage />
                                <Route path=path!("/select") view=SelectPage />
                                <Route path=path!("/textarea") view=TextareaPage />
                                <Route path=path!("/toggle") view=TogglePage />
                                <Route path=path!("/validator") view=ValidatorPage />
                                // Feedback
                                <Route path=path!("/loading") view=LoadingPage />
                                <Route path=path!("/toast") view=ToastPage />
                                // Layout
                                <Route path=path!("/artboard") view=ArtboardPage />
                                <Route path=path!("/divider") view=DividerPage />
                                <Route path=path!("/drawer") view=DrawerPage />
                                <Route path=path!("/footer") view=FooterPage />
                                <Route path=path!("/hero") view=HeroPage />
                                <Route path=path!("/indicator") view=IndicatorPage />
                                <Route path=path!("/join") view=JoinPage />
                                <Route path=path!("/mask") view=MaskPage />
                                <Route path=path!("/stack") view=StackPage />
                                <Route path=path!("/layout") view=LayoutPage />
                                <Route path=path!("/layouts/stacked") view=StackedLayoutPage />
                                <Route path=path!("/layouts/sidebar") view=SidebarLayoutPage />
                                <Route path=path!("/layouts/multi-column") view=MultiColumnPage />
                                // Mockups
                                <Route path=path!("/mockup-browser") view=MockupBrowserPage />
                                <Route path=path!("/mockup-code") view=MockupCodePage />
                                <Route path=path!("/mockup-phone") view=MockupPhonePage />
                                <Route path=path!("/mockup-window") view=MockupWindowPage />
                                // Navigation
                                <Route path=path!("/bottom-navigation") view=BottomNavigationPage />
                                <Route path=path!("/breadcrumbs") view=BreadcrumbsPage />
                                <Route path=path!("/dock") view=DockPage />
                                <Route path=path!("/link") view=LinkPage />
                                <Route path=path!("/menu") view=MenuPage />
                                <Route path=path!("/navbar") view=NavbarPage />
                                <Route path=path!("/pagination") view=PaginationPage />
                                <Route path=path!("/steps") view=StepsPage />
                                <Route path=path!("/tab") view=TabPage />
                                // Overlay
                                <Route path=path!("/backdrop") view=BackdropPage />
                                <Route path=path!("/tooltip") view=TooltipPage />
                                // Theming
                                <Route path=path!("/themes") view=ThemeShowcasePage />
                                <Route path=path!("/themes/creator") view=ThemeCreatorPage />
                                <Route path=path!("/themes/guide") view=ThemeGuidePage />
                                // Tools
                                <Route path=path!("/playground") view=PlaygroundPage />
                            </Routes>
                        </div>
                    </SidebarLayoutMain>
                </SidebarLayoutContent>

                // Sidebar
                <SidebarLayoutSide>
                    <SidebarLayoutOverlay />
                    <SidebarLayoutPanel>
                        <SidebarLayoutHeader>
                            <A href="/" attr:class="btn btn-ghost text-xl gap-2 -ml-2">
                                <span>"🌼"</span>
                                <span>"leptos-daisyui"</span>
                            </A>
                        </SidebarLayoutHeader>
                        <SidebarLayoutNav>
                            {NAV_SECTIONS.iter().map(|(category, icon, items)| {
                                let category = *category;
                                let icon = *icon;
                                let items = *items;
                                view! {
                                    <SidebarLayoutNavSection title={format!("{} {}", icon, category)}>
                                        {items.iter().map(|(label, path)| {
                                            let path = path.to_string();
                                            let label = *label;
                                            view! {
                                                <NavItemWithActiveState href={path}>
                                                    {label}
                                                </NavItemWithActiveState>
                                            }
                                        }).collect_view()}
                                    </SidebarLayoutNavSection>
                                }
                            }).collect_view()}
                        </SidebarLayoutNav>
                        <SidebarLayoutFooter>
                            <div class="text-xs opacity-60 text-center">
                                <p>"Built with Leptos + daisyUI"</p>
                                <p class="mt-1">"v0.1.0"</p>
                            </div>
                        </SidebarLayoutFooter>
                    </SidebarLayoutPanel>
                </SidebarLayoutSide>
            </SidebarLayout>
        </Router>
    }
}

/// Theme selector dropdown with live preview swatches.
#[component]
fn ThemeSelector(theme: ReadSignal<String>, set_theme: WriteSignal<String>) -> impl IntoView {
    view! {
        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" aria-label="Choose theme" class="btn btn-ghost btn-sm gap-1">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M4.098 19.902a3.75 3.75 0 005.304 0l6.401-6.402M6.75 21A3.75 3.75 0 013 17.25V4.125C3 3.504 3.504 3 4.125 3h5.25c.621 0 1.125.504 1.125 1.125v4.072M6.75 21a3.75 3.75 0 003.75-3.75V8.197M6.75 21h13.125c.621 0 1.125-.504 1.125-1.125v-5.25c0-.621-.504-1.125-1.125-1.125h-4.072M10.5 8.197l2.88-2.88c.438-.439 1.15-.439 1.59 0l3.712 3.713c.44.44.44 1.152 0 1.59l-2.879 2.88M6.75 17.25h.008v.008H6.75v-.008z" />
                </svg>
                <span class="hidden sm:inline">{move || theme.get()}</span>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20" class="w-4 h-4">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M6 8l4 4 4-4" />
                </svg>
            </div>
            <div class="dropdown-content bg-base-200 rounded-box z-50 w-56 max-h-96 overflow-y-auto shadow-2xl p-2 mt-2">
                <div class="grid grid-cols-1 gap-1">
                    {THEMES.iter().map(|t| {
                        let t = *t;
                        let is_active = move || theme.get() == t;
                        view! {
                            <button
                                class="outline-base-content text-start outline-offset-4 w-full"
                                class:outline=is_active
                                class:outline-2=is_active
                                data-theme={t}
                                on:click=move |_| set_theme.set(t.to_string())
                            >
                                <div class="bg-base-100 rounded-btn flex items-center gap-2 px-3 py-2">
                                    <div class="flex gap-1">
                                        <div class="bg-primary rounded-badge w-2 h-4"></div>
                                        <div class="bg-secondary rounded-badge w-2 h-4"></div>
                                        <div class="bg-accent rounded-badge w-2 h-4"></div>
                                        <div class="bg-neutral rounded-badge w-2 h-4"></div>
                                    </div>
                                    <span class="text-base-content text-sm flex-1">{t}</span>
                                    {move || is_active().then(|| view! {
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-4 h-4 stroke-current text-success">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 13l4 4L19 7" />
                                        </svg>
                                    })}
                                </div>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

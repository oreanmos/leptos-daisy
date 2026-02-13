use leptos::prelude::*;
use leptos_daisyui::prelude::*;

use crate::components::component_preview::ComponentPreview;

#[component]
pub fn SidebarLayoutPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Sidebar Layout"</h1>
            <p class="text-base-content/70">
                "A responsive sidebar layout using daisyUI's drawer component. "
                "Features mobile drawer, static desktop sidebar, and Tailwind Plus inspired patterns."
            </p>

            // --- Light Variant ---
            <section class="space-y-4">
                <ComponentPreview
                    title="Light Variant"
                    code=r#"<SidebarLayout variant=SidebarLayoutVariant::Light open_on_large=true>
  <SidebarLayoutContent>
    <SidebarLayoutTopBar>
      <SidebarLayoutTopBarStart>
        <SidebarLayoutMobileMenuButton />
        <span class="font-semibold">"Dashboard"</span>
      </SidebarLayoutTopBarStart>
      <SidebarLayoutTopBarEnd>...</SidebarLayoutTopBarEnd>
    </SidebarLayoutTopBar>
    <SidebarLayoutMain>...</SidebarLayoutMain>
  </SidebarLayoutContent>
  <SidebarLayoutSide>
    <SidebarLayoutOverlay />
    <SidebarLayoutPanel>
      <SidebarLayoutHeader>...</SidebarLayoutHeader>
      <SidebarLayoutNav>
        <SidebarLayoutNavSection title="Main">
          <SidebarLayoutNavItem href="/" active=true>"Dashboard"</SidebarLayoutNavItem>
          <SidebarLayoutNavItem href="/projects">"Projects"</SidebarLayoutNavItem>
        </SidebarLayoutNavSection>
      </SidebarLayoutNav>
      <SidebarLayoutFooter>...</SidebarLayoutFooter>
    </SidebarLayoutPanel>
  </SidebarLayoutSide>
</SidebarLayout>"#
                >
                    <div class="border border-base-300 rounded-box overflow-hidden w-full">
                        <div class="h-[24rem]">
                            <SidebarLayout variant=SidebarLayoutVariant::Light open_on_large=true>
                                <SidebarLayoutContent>
                                    <SidebarLayoutTopBar>
                                        <SidebarLayoutTopBarStart>
                                            <SidebarLayoutMobileMenuButton />
                                            <span class="font-semibold">"Dashboard"</span>
                                        </SidebarLayoutTopBarStart>
                                        <SidebarLayoutTopBarEnd>
                                            <button class="btn btn-ghost btn-circle btn-sm">
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                                                </svg>
                                            </button>
                                        </SidebarLayoutTopBarEnd>
                                    </SidebarLayoutTopBar>
                                    <SidebarLayoutMain>
                                        <div class="p-6 space-y-4">
                                            <h3 class="text-2xl font-bold">"Welcome Back"</h3>
                                            <div class="grid grid-cols-2 gap-4">
                                                <div class="bg-base-200 rounded-box p-4 h-24 flex items-center justify-center">"Widget A"</div>
                                                <div class="bg-base-200 rounded-box p-4 h-24 flex items-center justify-center">"Widget B"</div>
                                                <div class="bg-base-200 rounded-box p-4 h-24 flex items-center justify-center">"Widget C"</div>
                                                <div class="bg-base-200 rounded-box p-4 h-24 flex items-center justify-center">"Widget D"</div>
                                            </div>
                                        </div>
                                    </SidebarLayoutMain>
                                </SidebarLayoutContent>
                                <SidebarLayoutSide>
                                    <SidebarLayoutOverlay />
                                    <SidebarLayoutPanel>
                                        <SidebarLayoutHeader>
                                            <span class="text-xl font-bold">"My App"</span>
                                        </SidebarLayoutHeader>
                                        <SidebarLayoutNav>
                                            <SidebarLayoutNavSection title="Main">
                                                <SidebarLayoutNavItem href="/" active=true
                                                    unsafe_icon_html="<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor'><path stroke-linecap='round' stroke-linejoin='round' d='M2.25 12l8.954-8.955a1.126 1.126 0 011.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25' /></svg>">
                                                    "Dashboard"
                                                </SidebarLayoutNavItem>
                                                <SidebarLayoutNavItem href="/projects"
                                                    unsafe_icon_html="<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor'><path stroke-linecap='round' stroke-linejoin='round' d='M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z' /></svg>">
                                                    "Projects"
                                                </SidebarLayoutNavItem>
                                                <SidebarLayoutNavItem href="/calendar"
                                                    unsafe_icon_html="<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor'><path stroke-linecap='round' stroke-linejoin='round' d='M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5' /></svg>">
                                                    "Calendar"
                                                </SidebarLayoutNavItem>
                                            </SidebarLayoutNavSection>
                                            <SidebarLayoutNavSection title="Settings">
                                                <SidebarLayoutNavItem href="/profile"
                                                    unsafe_icon_html="<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor'><path stroke-linecap='round' stroke-linejoin='round' d='M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z' /></svg>">
                                                    "Profile"
                                                </SidebarLayoutNavItem>
                                            </SidebarLayoutNavSection>
                                        </SidebarLayoutNav>
                                        <SidebarLayoutFooter>
                                            <div class="flex items-center gap-3">
                                                <div class="avatar placeholder">
                                                    <div class="bg-neutral text-neutral-content w-10 rounded-full">
                                                        <span>"JD"</span>
                                                    </div>
                                                </div>
                                                <div class="flex-1 min-w-0">
                                                    <div class="font-medium truncate">"John Doe"</div>
                                                    <div class="text-sm opacity-60 truncate">"john@example.com"</div>
                                                </div>
                                            </div>
                                        </SidebarLayoutFooter>
                                    </SidebarLayoutPanel>
                                </SidebarLayoutSide>
                            </SidebarLayout>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            // --- Dark Variant ---
            <section class="space-y-4">
                <ComponentPreview
                    title="Dark Variant"
                    code=r#"<SidebarLayout variant=SidebarLayoutVariant::Dark open_on_large=true>
  <SidebarLayoutContent>
    <SidebarLayoutTopBar>
      <SidebarLayoutTopBarStart>
        <SidebarLayoutMobileMenuButton />
        <span class="font-semibold">"Analytics"</span>
      </SidebarLayoutTopBarStart>
    </SidebarLayoutTopBar>
    <SidebarLayoutMain>...</SidebarLayoutMain>
  </SidebarLayoutContent>
  <SidebarLayoutSide>
    <SidebarLayoutOverlay />
    <SidebarLayoutPanel>
      <SidebarLayoutHeader>...</SidebarLayoutHeader>
      <SidebarLayoutNav>
        <SidebarLayoutNavSection>
          <SidebarLayoutNavItem href="/overview" active=true>"Overview"</SidebarLayoutNavItem>
          <SidebarLayoutNavItem href="/analytics">"Analytics"</SidebarLayoutNavItem>
        </SidebarLayoutNavSection>
      </SidebarLayoutNav>
    </SidebarLayoutPanel>
  </SidebarLayoutSide>
</SidebarLayout>"#
                >
                    <div class="border border-base-300 rounded-box overflow-hidden w-full">
                        <div class="h-[24rem]">
                            <SidebarLayout variant=SidebarLayoutVariant::Dark open_on_large=true>
                                <SidebarLayoutContent>
                                    <SidebarLayoutTopBar>
                                        <SidebarLayoutTopBarStart>
                                            <SidebarLayoutMobileMenuButton />
                                            <span class="font-semibold">"Analytics"</span>
                                        </SidebarLayoutTopBarStart>
                                    </SidebarLayoutTopBar>
                                    <SidebarLayoutMain>
                                        <div class="p-6">
                                            <h3 class="text-2xl font-bold">"Dark Sidebar"</h3>
                                            <p class="mt-2 text-base-content/60">"Uses bg-neutral for a dark sidebar appearance."</p>
                                        </div>
                                    </SidebarLayoutMain>
                                </SidebarLayoutContent>
                                <SidebarLayoutSide>
                                    <SidebarLayoutOverlay />
                                    <SidebarLayoutPanel>
                                        <SidebarLayoutHeader>
                                            <span class="text-xl font-bold">"Admin"</span>
                                        </SidebarLayoutHeader>
                                        <SidebarLayoutNav>
                                            <SidebarLayoutNavSection>
                                                <SidebarLayoutNavItem href="/overview" active=true>
                                                    "Overview"
                                                </SidebarLayoutNavItem>
                                                <SidebarLayoutNavItem href="/analytics">
                                                    "Analytics"
                                                </SidebarLayoutNavItem>
                                                <SidebarLayoutNavItem href="/users">
                                                    "Users"
                                                </SidebarLayoutNavItem>
                                            </SidebarLayoutNavSection>
                                        </SidebarLayoutNav>
                                    </SidebarLayoutPanel>
                                </SidebarLayoutSide>
                            </SidebarLayout>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            // --- Primary Variant ---
            <section class="space-y-4">
                <ComponentPreview
                    title="Primary Variant"
                    code=r#"<SidebarLayout variant=SidebarLayoutVariant::Primary open_on_large=true>
  <SidebarLayoutContent>
    <SidebarLayoutTopBar>
      <SidebarLayoutTopBarStart>
        <SidebarLayoutMobileMenuButton />
        <span class="font-semibold">"Brand"</span>
      </SidebarLayoutTopBarStart>
    </SidebarLayoutTopBar>
    <SidebarLayoutMain>...</SidebarLayoutMain>
  </SidebarLayoutContent>
  <SidebarLayoutSide>
    <SidebarLayoutOverlay />
    <SidebarLayoutPanel>
      <SidebarLayoutHeader>...</SidebarLayoutHeader>
      <SidebarLayoutNav>
        <SidebarLayoutNavSection>
          <SidebarLayoutNavItem href="/home" active=true>"Home"</SidebarLayoutNavItem>
          <SidebarLayoutNavItem href="/features">"Features"</SidebarLayoutNavItem>
        </SidebarLayoutNavSection>
      </SidebarLayoutNav>
    </SidebarLayoutPanel>
  </SidebarLayoutSide>
</SidebarLayout>"#
                >
                    <div class="border border-base-300 rounded-box overflow-hidden w-full">
                        <div class="h-[24rem]">
                            <SidebarLayout variant=SidebarLayoutVariant::Primary open_on_large=true>
                                <SidebarLayoutContent>
                                    <SidebarLayoutTopBar>
                                        <SidebarLayoutTopBarStart>
                                            <SidebarLayoutMobileMenuButton />
                                            <span class="font-semibold">"Brand"</span>
                                        </SidebarLayoutTopBarStart>
                                    </SidebarLayoutTopBar>
                                    <SidebarLayoutMain>
                                        <div class="p-6">
                                            <h3 class="text-2xl font-bold">"Primary Sidebar"</h3>
                                            <p class="mt-2 text-base-content/60">"Uses bg-primary for a branded sidebar appearance."</p>
                                        </div>
                                    </SidebarLayoutMain>
                                </SidebarLayoutContent>
                                <SidebarLayoutSide>
                                    <SidebarLayoutOverlay />
                                    <SidebarLayoutPanel>
                                        <SidebarLayoutHeader>
                                            <span class="text-xl font-bold">"Brand"</span>
                                        </SidebarLayoutHeader>
                                        <SidebarLayoutNav>
                                            <SidebarLayoutNavSection>
                                                <SidebarLayoutNavItem href="/home" active=true>
                                                    "Home"
                                                </SidebarLayoutNavItem>
                                                <SidebarLayoutNavItem href="/features">
                                                    "Features"
                                                </SidebarLayoutNavItem>
                                                <SidebarLayoutNavItem href="/pricing">
                                                    "Pricing"
                                                </SidebarLayoutNavItem>
                                            </SidebarLayoutNavSection>
                                        </SidebarLayoutNav>
                                    </SidebarLayoutPanel>
                                </SidebarLayoutSide>
                            </SidebarLayout>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            // --- Width Variations ---
            <section class="space-y-4">
                <ComponentPreview
                    title="Width Variations"
                    code=r#"// Narrow (w-64)
<SidebarLayout width=SidebarLayoutWidth::Narrow open_on_large=true>
  ...
</SidebarLayout>

// Default (w-72)
<SidebarLayout width=SidebarLayoutWidth::Default open_on_large=true>
  ...
</SidebarLayout>

// Wide (w-80)
<SidebarLayout width=SidebarLayoutWidth::Wide open_on_large=true>
  ...
</SidebarLayout>"#
                >
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 w-full">
                        // Narrow
                        <div class="border border-base-300 rounded-box overflow-hidden">
                            <div class="h-[16rem]">
                                <SidebarLayout width=SidebarLayoutWidth::Narrow open_on_large=true>
                                    <SidebarLayoutContent>
                                        <SidebarLayoutMain>
                                            <div class="p-4">
                                                <p class="text-sm text-base-content/60">"Narrow (w-64)"</p>
                                            </div>
                                        </SidebarLayoutMain>
                                    </SidebarLayoutContent>
                                    <SidebarLayoutSide>
                                        <SidebarLayoutOverlay />
                                        <SidebarLayoutPanel>
                                            <SidebarLayoutHeader>
                                                <span class="font-bold">"Narrow"</span>
                                            </SidebarLayoutHeader>
                                            <SidebarLayoutNav>
                                                <SidebarLayoutNavSection>
                                                    <SidebarLayoutNavItem href="/" active=true>"Home"</SidebarLayoutNavItem>
                                                    <SidebarLayoutNavItem href="/about">"About"</SidebarLayoutNavItem>
                                                </SidebarLayoutNavSection>
                                            </SidebarLayoutNav>
                                        </SidebarLayoutPanel>
                                    </SidebarLayoutSide>
                                </SidebarLayout>
                            </div>
                        </div>
                        // Default
                        <div class="border border-base-300 rounded-box overflow-hidden">
                            <div class="h-[16rem]">
                                <SidebarLayout width=SidebarLayoutWidth::Default open_on_large=true>
                                    <SidebarLayoutContent>
                                        <SidebarLayoutMain>
                                            <div class="p-4">
                                                <p class="text-sm text-base-content/60">"Default (w-72)"</p>
                                            </div>
                                        </SidebarLayoutMain>
                                    </SidebarLayoutContent>
                                    <SidebarLayoutSide>
                                        <SidebarLayoutOverlay />
                                        <SidebarLayoutPanel>
                                            <SidebarLayoutHeader>
                                                <span class="font-bold">"Default"</span>
                                            </SidebarLayoutHeader>
                                            <SidebarLayoutNav>
                                                <SidebarLayoutNavSection>
                                                    <SidebarLayoutNavItem href="/" active=true>"Home"</SidebarLayoutNavItem>
                                                    <SidebarLayoutNavItem href="/about">"About"</SidebarLayoutNavItem>
                                                </SidebarLayoutNavSection>
                                            </SidebarLayoutNav>
                                        </SidebarLayoutPanel>
                                    </SidebarLayoutSide>
                                </SidebarLayout>
                            </div>
                        </div>
                        // Wide
                        <div class="border border-base-300 rounded-box overflow-hidden">
                            <div class="h-[16rem]">
                                <SidebarLayout width=SidebarLayoutWidth::Wide open_on_large=true>
                                    <SidebarLayoutContent>
                                        <SidebarLayoutMain>
                                            <div class="p-4">
                                                <p class="text-sm text-base-content/60">"Wide (w-80)"</p>
                                            </div>
                                        </SidebarLayoutMain>
                                    </SidebarLayoutContent>
                                    <SidebarLayoutSide>
                                        <SidebarLayoutOverlay />
                                        <SidebarLayoutPanel>
                                            <SidebarLayoutHeader>
                                                <span class="font-bold">"Wide"</span>
                                            </SidebarLayoutHeader>
                                            <SidebarLayoutNav>
                                                <SidebarLayoutNavSection>
                                                    <SidebarLayoutNavItem href="/" active=true>"Home"</SidebarLayoutNavItem>
                                                    <SidebarLayoutNavItem href="/about">"About"</SidebarLayoutNavItem>
                                                </SidebarLayoutNavSection>
                                            </SidebarLayoutNav>
                                        </SidebarLayoutPanel>
                                    </SidebarLayoutSide>
                                </SidebarLayout>
                            </div>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            // --- Usage ---
            <section class="space-y-4">
                <ComponentPreview
                    title="Usage"
                    code=r#"<SidebarLayout variant=SidebarLayoutVariant::Dark open_on_large=true>
  <SidebarLayoutContent>
    <SidebarLayoutTopBar>
      <SidebarLayoutMobileMenuButton />
    </SidebarLayoutTopBar>
    <SidebarLayoutMain>
      // Main content
    </SidebarLayoutMain>
  </SidebarLayoutContent>
  <SidebarLayoutSide>
    <SidebarLayoutOverlay />
    <SidebarLayoutPanel>
      <SidebarLayoutHeader>"Logo"</SidebarLayoutHeader>
      <SidebarLayoutNav>
        <SidebarLayoutNavSection title="Main">
          <SidebarLayoutNavItem href="/" active=true>"Home"</SidebarLayoutNavItem>
        </SidebarLayoutNavSection>
      </SidebarLayoutNav>
      <SidebarLayoutFooter>"User"</SidebarLayoutFooter>
    </SidebarLayoutPanel>
  </SidebarLayoutSide>
</SidebarLayout>"#
                >
                    <div class="w-full text-base-content/70 text-sm">
                        "See the variants above for live examples of the SidebarLayout component."
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

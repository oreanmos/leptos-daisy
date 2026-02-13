use leptos::prelude::*;
use leptos_daisyui::prelude::*;

use crate::components::component_preview::ComponentPreview;

#[component]
pub fn StackedLayoutPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Stacked Layout"</h1>
            <p class="text-base-content/70">
                "Full-width page shell with header, navigation, content area, and optional footer. "
                "Inspired by Tailwind Plus "stacked" application shell layouts."
            </p>

            // --- Example 1: Simple header with nav tabs (light) ---
            <ComponentPreview
                title="1. Simple Header with Nav Tabs (Light)"
                description="Basic light header with horizontal tab navigation."
                code=r#"<StackedLayout variant=StackedLayoutVariant::Light>
  <StackedLayoutHeader>
    <NavbarStart>...</NavbarStart>
    <NavbarCenter>
      <StackedLayoutNav>
        <StackedLayoutNavItem href="/" active=true>"Dashboard"</StackedLayoutNavItem>
      </StackedLayoutNav>
    </NavbarCenter>
    <NavbarEnd>...</NavbarEnd>
  </StackedLayoutHeader>
  <StackedLayoutContent class="p-6">
    // Main content
  </StackedLayoutContent>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::Light class="min-h-[20rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"MyApp"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutNav>
                                    <StackedLayoutNavItem href="/" active=true>"Dashboard"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/team">"Team"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/projects">"Projects"</StackedLayoutNavItem>
                                </StackedLayoutNav>
                            </NavbarCenter>
                            <NavbarEnd>
                                <button class="btn btn-ghost btn-circle">
                                    <div class="avatar placeholder">
                                        <div class="bg-neutral text-neutral-content w-8 rounded-full">
                                            <span class="text-xs">"JD"</span>
                                        </div>
                                    </div>
                                </button>
                            </NavbarEnd>
                        </StackedLayoutHeader>
                        <StackedLayoutContent class="p-6">
                            <div class="max-w-4xl mx-auto space-y-4">
                                <h3 class="text-2xl font-bold">"Dashboard"</h3>
                                <p class="text-base-content/70">"Welcome to your dashboard."</p>
                            </div>
                        </StackedLayoutContent>
                    </StackedLayout>
                </div>
            </ComponentPreview>

            // --- Example 2: Header with border and nav tabs (light) ---
            <ComponentPreview
                title="2. Header with Border and Nav Tabs (Light)"
                description="Light header with bottom border for visual separation."
                code=r#"<StackedLayout variant=StackedLayoutVariant::LightBordered>
  <StackedLayoutHeader>
    <NavbarStart>...</NavbarStart>
    <NavbarCenter>
      <StackedLayoutNav>
        <StackedLayoutNavItem href="/" active=true>"Home"</StackedLayoutNavItem>
      </StackedLayoutNav>
    </NavbarCenter>
    <NavbarEnd>
      <Button size=Size::Small color=Color::Primary>"Sign Up"</Button>
    </NavbarEnd>
  </StackedLayoutHeader>
  <StackedLayoutContent class="p-6">
    // Content area
  </StackedLayoutContent>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::LightBordered class="min-h-[20rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"BorderedApp"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutNav>
                                    <StackedLayoutNavItem href="/" active=true>"Home"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/features">"Features"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/pricing">"Pricing"</StackedLayoutNavItem>
                                </StackedLayoutNav>
                            </NavbarCenter>
                            <NavbarEnd>
                                <Button size=Size::Small color=Color::Primary>"Sign Up"</Button>
                            </NavbarEnd>
                        </StackedLayoutHeader>
                        <StackedLayoutContent class="p-6">
                            <div class="max-w-4xl mx-auto">
                                <p class="text-base-content/70">"Content area with bordered header above."</p>
                            </div>
                        </StackedLayoutContent>
                    </StackedLayout>
                </div>
            </ComponentPreview>

            // --- Example 3: Dark header with nav tabs ---
            <ComponentPreview
                title="3. Dark Header with Nav Tabs"
                description="Dark neutral background header with tab navigation."
                code=r#"<StackedLayout variant=StackedLayoutVariant::Dark>
  <StackedLayoutHeader>
    <NavbarStart>...</NavbarStart>
    <NavbarCenter>
      <StackedLayoutNav>
        <StackedLayoutNavItem href="/" active=true>"Overview"</StackedLayoutNavItem>
      </StackedLayoutNav>
    </NavbarCenter>
    <NavbarEnd>...</NavbarEnd>
  </StackedLayoutHeader>
  <StackedLayoutContent class="p-6 bg-base-100">
    // Content below dark navbar
  </StackedLayoutContent>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::Dark class="min-h-[20rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"DarkMode"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutNav>
                                    <StackedLayoutNavItem href="/" active=true>"Overview"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/analytics">"Analytics"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/reports">"Reports"</StackedLayoutNavItem>
                                </StackedLayoutNav>
                            </NavbarCenter>
                            <NavbarEnd>
                                <button class="btn btn-ghost btn-circle text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                                    </svg>
                                </button>
                            </NavbarEnd>
                        </StackedLayoutHeader>
                        <StackedLayoutContent class="p-6 bg-base-100">
                            <div class="max-w-4xl mx-auto">
                                <p class="text-base-content/70">"Content below the dark navbar."</p>
                            </div>
                        </StackedLayoutContent>
                    </StackedLayout>
                </div>
            </ComponentPreview>

            // --- Example 4: Indigo branded header with nav tabs ---
            <ComponentPreview
                title="4. Indigo Branded Header with Nav Tabs"
                description="Primary-colored header with tab navigation."
                code=r#"<StackedLayout variant=StackedLayoutVariant::Primary>
  <StackedLayoutHeader>
    <NavbarStart>...</NavbarStart>
    <NavbarCenter>
      <StackedLayoutNav>
        <StackedLayoutNavItem href="/" active=true>"Products"</StackedLayoutNavItem>
      </StackedLayoutNav>
    </NavbarCenter>
    <NavbarEnd>...</NavbarEnd>
  </StackedLayoutHeader>
  <StackedLayoutContent class="p-6 bg-base-100">
    // Content below branded header
  </StackedLayoutContent>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::Primary class="min-h-[20rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"BrandApp"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutNav>
                                    <StackedLayoutNavItem href="/" active=true>"Products"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/solutions">"Solutions"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/resources">"Resources"</StackedLayoutNavItem>
                                </StackedLayoutNav>
                            </NavbarCenter>
                            <NavbarEnd>
                                <button class="btn btn-ghost btn-circle text-primary-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                                    </svg>
                                </button>
                            </NavbarEnd>
                        </StackedLayoutHeader>
                        <StackedLayoutContent class="p-6 bg-base-100">
                            <div class="max-w-4xl mx-auto">
                                <p class="text-base-content/70">"Content below the branded primary header."</p>
                            </div>
                        </StackedLayoutContent>
                    </StackedLayout>
                </div>
            </ComponentPreview>

            // --- Example 5: Dark header with overlap content area ---
            <ComponentPreview
                title="5. Dark Header with Overlap Content Area"
                description="Dark header with content card overlapping into header."
                code=r#"<StackedLayout variant=StackedLayoutVariant::Dark>
  <StackedLayoutHeader>...</StackedLayoutHeader>
  <StackedLayoutContent overlap=true>
    // Content overlaps header
  </StackedLayoutContent>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::Dark class="min-h-[24rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"OverlapDark"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutNav>
                                    <StackedLayoutNavItem href="/" active=true>"Dashboard"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/settings">"Settings"</StackedLayoutNavItem>
                                </StackedLayoutNav>
                            </NavbarCenter>
                        </StackedLayoutHeader>
                        <StackedLayoutContent overlap=true class="px-6 pb-6">
                            <div class="max-w-4xl mx-auto">
                                <div class="bg-base-100 rounded-box shadow-lg p-6">
                                    <h3 class="text-2xl font-bold mb-4">"Overlapping Content Card"</h3>
                                    <p class="text-base-content/70">"This card overlaps the dark header above, creating a layered visual effect."</p>
                                    <div class="mt-4 grid grid-cols-3 gap-4">
                                        <div class="stat bg-base-200 rounded-box">
                                            <div class="stat-title">"Users"</div>
                                            <div class="stat-value">"1,234"</div>
                                        </div>
                                        <div class="stat bg-base-200 rounded-box">
                                            <div class="stat-title">"Revenue"</div>
                                            <div class="stat-value text-primary">"$12K"</div>
                                        </div>
                                        <div class="stat bg-base-200 rounded-box">
                                            <div class="stat-title">"Growth"</div>
                                            <div class="stat-value text-success">"+8%"</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </StackedLayoutContent>
                    </StackedLayout>
                </div>
            </ComponentPreview>

            // --- Example 6: Indigo branded header with overlap content area ---
            <ComponentPreview
                title="6. Indigo Branded Header with Overlap Content Area"
                description="Primary header with overlapping content card."
                code=r#"<StackedLayout variant=StackedLayoutVariant::Primary>
  <StackedLayoutHeader>...</StackedLayoutHeader>
  <StackedLayoutContent overlap=true>
    // Content overlaps branded header
  </StackedLayoutContent>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::Primary class="min-h-[24rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"OverlapBrand"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutNav>
                                    <StackedLayoutNavItem href="/" active=true>"Overview"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/details">"Details"</StackedLayoutNavItem>
                                </StackedLayoutNav>
                            </NavbarCenter>
                        </StackedLayoutHeader>
                        <StackedLayoutContent overlap=true class="px-6 pb-6">
                            <div class="max-w-4xl mx-auto">
                                <div class="bg-base-100 rounded-box shadow-lg p-6">
                                    <h3 class="text-2xl font-bold mb-4">"Primary Header Overlap"</h3>
                                    <p class="text-base-content/70">"Content card overlapping the branded primary header."</p>
                                    <div class="mt-4 flex gap-4">
                                        <Button color=Color::Primary>"Get Started"</Button>
                                        <Button variant=Variant::Ghost>"Learn More"</Button>
                                    </div>
                                </div>
                            </div>
                        </StackedLayoutContent>
                    </StackedLayout>
                </div>
            </ComponentPreview>

            // --- Example 7: Indigo branded header with search and secondary nav ---
            <ComponentPreview
                title="7. Indigo Branded Header with Search and Secondary Nav"
                description="Primary header with search input and secondary navigation."
                code=r#"<StackedLayout variant=StackedLayoutVariant::Primary>
  <StackedLayoutHeader>
    <NavbarStart>...</NavbarStart>
    <NavbarCenter>
      <StackedLayoutSearch placeholder="Search products..." />
    </NavbarCenter>
    <NavbarEnd>...</NavbarEnd>
  </StackedLayoutHeader>
  <StackedLayoutContent class="p-6 bg-base-100">
    // Content with breadcrumbs
  </StackedLayoutContent>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::Primary class="min-h-[20rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"SearchBrand"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutSearch placeholder="Search products..." />
                            </NavbarCenter>
                            <NavbarEnd>
                                <button class="btn btn-ghost btn-circle text-primary-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" />
                                    </svg>
                                </button>
                            </NavbarEnd>
                        </StackedLayoutHeader>
                        <StackedLayoutContent class="p-6 bg-base-100">
                            <div class="max-w-4xl mx-auto">
                                <div class="flex items-center gap-4 mb-4 text-sm">
                                    <a href="/" class="text-primary hover:underline">"Home"</a>
                                    <span class="text-base-content/40">"/"</span>
                                    <a href="/products" class="text-primary hover:underline">"Products"</a>
                                    <span class="text-base-content/40">"/"</span>
                                    <span class="text-base-content/70">"All Items"</span>
                                </div>
                                <p class="text-base-content/70">"Secondary navigation (breadcrumbs) shown below the header with search."</p>
                            </div>
                        </StackedLayoutContent>
                    </StackedLayout>
                </div>
            </ComponentPreview>

            // --- Example 8: Dark header with compact nav ---
            <ComponentPreview
                title="8. Dark Header with Compact Nav"
                description="Dark header with compact minimal navigation."
                code=r#"<StackedLayout variant=StackedLayoutVariant::Dark>
  <StackedLayoutHeader>
    <NavbarStart>...</NavbarStart>
    <NavbarCenter>
      <StackedLayoutNav style=NavStyle::Compact>
        <StackedLayoutNavItem href="/" active=true>"Home"</StackedLayoutNavItem>
      </StackedLayoutNav>
    </NavbarCenter>
    <NavbarEnd>...</NavbarEnd>
  </StackedLayoutHeader>
  <StackedLayoutContent class="p-6 bg-base-100">
    // Content area
  </StackedLayoutContent>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::Dark class="min-h-[20rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"CompactDark"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutNav style=NavStyle::Compact>
                                    <StackedLayoutNavItem href="/" active=true>"Home"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/docs">"Docs"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/api">"API"</StackedLayoutNavItem>
                                    <StackedLayoutNavItem href="/help">"Help"</StackedLayoutNavItem>
                                </StackedLayoutNav>
                            </NavbarCenter>
                            <NavbarEnd>
                                <Button size=Size::Small variant=Variant::Outline class="border-neutral-content/30 text-neutral-content hover:bg-neutral-content/10">"Login"</Button>
                            </NavbarEnd>
                        </StackedLayoutHeader>
                        <StackedLayoutContent class="p-6 bg-base-100">
                            <div class="max-w-4xl mx-auto">
                                <p class="text-base-content/70">"Content below the dark header with compact navigation."</p>
                            </div>
                        </StackedLayoutContent>
                    </StackedLayout>
                </div>
            </ComponentPreview>

            // --- Example 9: Indigo header with centered search and footer ---
            <ComponentPreview
                title="9. Indigo Header with Centered Search and Footer"
                description="Primary header with centered search and page footer."
                code=r#"<StackedLayout variant=StackedLayoutVariant::Primary>
  <StackedLayoutHeader>...</StackedLayoutHeader>
  <StackedLayoutContent>...</StackedLayoutContent>
  <StackedLayoutFooter>
    // Footer content
  </StackedLayoutFooter>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::Primary class="min-h-[24rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"FullApp"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutSearch placeholder="Search..." class="w-80" />
                            </NavbarCenter>
                            <NavbarEnd>
                                <div class="flex items-center gap-2">
                                    <button class="btn btn-ghost btn-circle text-primary-content">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                                        </svg>
                                    </button>
                                    <div class="avatar">
                                        <div class="w-8 rounded-full bg-primary-content/20">
                                            <span class="text-primary-content text-sm">"U"</span>
                                        </div>
                                    </div>
                                </div>
                            </NavbarEnd>
                        </StackedLayoutHeader>
                        <StackedLayoutContent class="p-6 bg-base-100">
                            <div class="max-w-4xl mx-auto">
                                <h3 class="text-xl font-bold mb-4">"Main Content"</h3>
                                <p class="text-base-content/70">"This layout includes a footer below the content area."</p>
                            </div>
                        </StackedLayoutContent>
                        <StackedLayoutFooter>
                            <div class="flex items-center justify-between w-full max-w-4xl">
                                <p class="text-sm text-base-content/60">"© 2024 FullApp Inc. All rights reserved."</p>
                                <div class="flex gap-4 text-sm">
                                    <a href="/privacy" class="hover:text-primary">"Privacy"</a>
                                    <a href="/terms" class="hover:text-primary">"Terms"</a>
                                    <a href="/contact" class="hover:text-primary">"Contact"</a>
                                </div>
                            </div>
                        </StackedLayoutFooter>
                    </StackedLayout>
                </div>
            </ComponentPreview>

            // --- Usage Example ---
            <ComponentPreview
                title="Usage"
                code=r#"<StackedLayout variant=StackedLayoutVariant::Primary>
  <StackedLayoutHeader>
    <NavbarStart>
      <span class="text-xl font-bold">"Logo"</span>
    </NavbarStart>
    <NavbarCenter>
      <StackedLayoutNav>
        <StackedLayoutNavItem href="/" active=true>"Home"</StackedLayoutNavItem>
      </StackedLayoutNav>
    </NavbarCenter>
  </StackedLayoutHeader>
  <StackedLayoutContent>
    <p>"Main content here"</p>
  </StackedLayoutContent>
</StackedLayout>"#
            >
                <div class="border border-base-300 rounded-box overflow-hidden w-full">
                    <StackedLayout variant=StackedLayoutVariant::Primary class="min-h-[16rem]">
                        <StackedLayoutHeader>
                            <NavbarStart>
                                <span class="text-xl font-bold px-4">"Logo"</span>
                            </NavbarStart>
                            <NavbarCenter>
                                <StackedLayoutNav>
                                    <StackedLayoutNavItem href="/" active=true>"Home"</StackedLayoutNavItem>
                                </StackedLayoutNav>
                            </NavbarCenter>
                        </StackedLayoutHeader>
                        <StackedLayoutContent class="p-6 bg-base-100">
                            <div class="max-w-4xl mx-auto">
                                <p class="text-base-content/70">"Main content here"</p>
                            </div>
                        </StackedLayoutContent>
                    </StackedLayout>
                </div>
            </ComponentPreview>
        </div>
    }
}

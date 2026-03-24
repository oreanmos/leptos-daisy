use leptos::prelude::*;
use leptos_daisyui::prelude::*;

use crate::components::component_preview::ComponentPreview;

#[component]
pub fn MultiColumnPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Multi-Column Layout"</h1>
            <p class="text-base-content/70">
                "Flexible multi-column layouts with a main content area and side panels. "
                "Inspired by Tailwind Plus multi-column application layouts."
            </p>

            // --- Pattern 1: Sidebar with Secondary Column ---
            <section class="space-y-4">
                <ComponentPreview
                    title="Pattern 1: Sidebar with Secondary Column"
                    description="Left sidebar + secondary navigation column + main content. Ideal for complex applications with hierarchical navigation."
                    code=r#"<SidebarShell>
  <Sidebar width=SidebarWidth::Narrow>
    // Icon-only navigation
  </Sidebar>
  <SecondaryColumn width=SecondaryColumnWidth::Wide variant=ColumnVariant::Muted>
    // Secondary navigation or filters
  </SecondaryColumn>
  <MainColumn>
    // Main content area
  </MainColumn>
</SidebarShell>"#
                >
                    <div class="border border-base-300 rounded-box overflow-hidden w-full">
                        <div class="h-[24rem] flex">
                            // Left Sidebar
                            <div class="w-16 bg-neutral flex flex-col items-center py-4 gap-4 shrink-0">
                                <div class="w-10 h-10 bg-primary rounded-lg flex items-center justify-center text-primary-content font-bold">"A"</div>
                                <div class="w-10 h-10 bg-neutral-focus rounded-lg flex items-center justify-center text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                                    </svg>
                                </div>
                                <div class="w-10 h-10 hover:bg-neutral-focus rounded-lg flex items-center justify-center text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                                    </svg>
                                </div>
                                <div class="w-10 h-10 hover:bg-neutral-focus rounded-lg flex items-center justify-center text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                                    </svg>
                                </div>
                            </div>
                            // Secondary Column
                            <SecondaryColumn width=SecondaryColumnWidth::Wide variant=ColumnVariant::Muted bordered=true>
                                <div class="p-4">
                                    <h3 class="font-bold text-sm mb-4">"Projects"</h3>
                                    <ul class="menu menu-sm gap-1">
                                        <li><a class="active">"Website Redesign"</a></li>
                                        <li><a>"Mobile App"</a></li>
                                        <li><a>"API Integration"</a></li>
                                        <li><a>"Documentation"</a></li>
                                        <li><a>"Marketing Site"</a></li>
                                    </ul>
                                    <div class="divider"></div>
                                    <h3 class="font-bold text-sm mb-2">"Filters"</h3>
                                    <div class="space-y-2">
                                        <label class="flex items-center gap-2 cursor-pointer">
                                            <input type="checkbox" class="checkbox checkbox-sm" checked />
                                            <span class="text-sm">"Active"</span>
                                        </label>
                                        <label class="flex items-center gap-2 cursor-pointer">
                                            <input type="checkbox" class="checkbox checkbox-sm" />
                                            <span class="text-sm">"Archived"</span>
                                        </label>
                                    </div>
                                </div>
                            </SecondaryColumn>
                            // Main Content
                            <MainColumn>
                                <div class="p-6 space-y-4">
                                    <div class="flex items-center justify-between">
                                        <h3 class="text-xl font-bold">"Website Redesign"</h3>
                                        <button class="btn btn-primary btn-sm">"New Task"</button>
                                    </div>
                                    <div class="grid grid-cols-2 gap-4">
                                        <div class="bg-base-200 rounded-box p-4">
                                            <div class="text-sm text-base-content/60">"Progress"</div>
                                            <div class="text-2xl font-bold">"75%"</div>
                                            <progress class="progress progress-primary w-full mt-2" value="75" max="100"></progress>
                                        </div>
                                        <div class="bg-base-200 rounded-box p-4">
                                            <div class="text-sm text-base-content/60">"Tasks"</div>
                                            <div class="text-2xl font-bold">"12/16"</div>
                                            <div class="text-sm text-success mt-2">"4 remaining"</div>
                                        </div>
                                    </div>
                                </div>
                            </MainColumn>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            // --- Pattern 2: Three-Column with Header ---
            <section class="space-y-4">
                <ComponentPreview
                    title="Pattern 2: Three-Column with Header"
                    description="Header bar + left sidebar + main content + right panel. Perfect for dashboard-style applications with activity feeds."
                    code=r#"<StackedShell>
  <PageHeader>
    // Top navigation bar
  </PageHeader>
  <MultiColumnShell>
    <Panel width=PanelWidth::Small>
      // Left sidebar
    </Panel>
    <MainColumn>
      // Main content
    </MainColumn>
    <RightPanel width=RightPanelWidth::Default breakpoint=Breakpoint::Large>
      // Right panel (lg+)
    </RightPanel>
  </MultiColumnShell>
</StackedShell>"#
                >
                    <div class="border border-base-300 rounded-box overflow-hidden w-full">
                        <div class="h-[26rem] flex flex-col">
                            // Header
                            <PageHeader class="py-3">
                                <div class="flex items-center justify-between w-full">
                                    <div class="flex items-center gap-4">
                                        <button class="btn btn-ghost btn-sm btn-square">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" />
                                            </svg>
                                        </button>
                                        <span class="font-bold">"Dashboard"</span>
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <button class="btn btn-ghost btn-sm btn-circle">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                                            </svg>
                                        </button>
                                        <div class="avatar placeholder">
                                            <div class="bg-primary text-primary-content w-8 rounded-full">
                                                <span class="text-xs">"JD"</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </PageHeader>
                            // Content Area
                            <div class="flex flex-1 overflow-hidden">
                                // Left Panel
                                <Panel width=PanelWidth::Small class="border-r bg-base-200">
                                    <div class="p-4">
                                        <h4 class="font-bold mb-3">"Channels"</h4>
                                        <ul class="menu menu-sm gap-0.5">
                                            <li><a class="active">"# general"</a></li>
                                            <li><a>"# engineering"</a></li>
                                            <li><a>"# design"</a></li>
                                            <li><a>"# marketing"</a></li>
                                        </ul>
                                    </div>
                                </Panel>
                                // Main Content
                                <MainColumn>
                                    <div class="p-6 space-y-3">
                                        <h3 class="text-xl font-bold">"# general"</h3>
                                        <div class="divider"></div>
                                        <div class="space-y-3">
                                            <div class="chat chat-start">
                                                <div class="chat-bubble chat-bubble-primary">"Hey team! Sprint planning at 2pm."</div>
                                            </div>
                                            <div class="chat chat-end">
                                                <div class="chat-bubble">"Sounds good! I'll prepare the backlog."</div>
                                            </div>
                                            <div class="chat chat-start">
                                                <div class="chat-bubble chat-bubble-primary">"Perfect, see you there 👍"</div>
                                            </div>
                                        </div>
                                    </div>
                                </MainColumn>
                                // Right Panel
                                <RightPanel width=RightPanelWidth::Default breakpoint=Breakpoint::Large variant=ColumnVariant::Muted class="border-l">
                                    <div class="p-4">
                                        <h4 class="font-bold mb-3">"Members"</h4>
                                        <ul class="space-y-2">
                                            <li class="flex items-center gap-2">
                                                <div class="badge badge-xs badge-success"></div>
                                                <span class="text-sm">"Alice"</span>
                                            </li>
                                            <li class="flex items-center gap-2">
                                                <div class="badge badge-xs badge-success"></div>
                                                <span class="text-sm">"Bob"</span>
                                            </li>
                                            <li class="flex items-center gap-2">
                                                <div class="badge badge-xs badge-warning"></div>
                                                <span class="text-sm">"Charlie"</span>
                                            </li>
                                        </ul>
                                    </div>
                                </RightPanel>
                            </div>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            // --- Pattern 3: Narrow Sidebar with Secondary ---
            <section class="space-y-4">
                <ComponentPreview
                    title="Pattern 3: Narrow Sidebar with Secondary"
                    description="Icon-only sidebar + secondary column + main content + right panel. Maximizes screen real estate while maintaining navigation depth."
                    code=r#"<SidebarShell>
  <Sidebar width=SidebarWidth::Narrow>
    // Icon-only navigation
  </Sidebar>
  <SecondaryColumn width=SecondaryColumnWidth::Default>
    // Secondary navigation
  </SecondaryColumn>
  <MainColumn>
    // Main content
  </MainColumn>
  <RightPanel width=RightPanelWidth::Narrow breakpoint=Breakpoint::ExtraLarge>
    // Right panel (xl+)
  </RightPanel>
</SidebarShell>"#
                >
                    <div class="border border-base-300 rounded-box overflow-hidden w-full">
                        <div class="h-[24rem] flex">
                            // Icon Sidebar
                            <div class="w-16 bg-neutral flex flex-col items-center py-4 gap-2 shrink-0">
                                <div class="w-10 h-10 bg-primary rounded-lg flex items-center justify-center text-primary-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                                    </svg>
                                </div>
                                <div class="w-10 h-10 bg-neutral-focus rounded-lg flex items-center justify-center text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                                    </svg>
                                </div>
                                <div class="w-10 h-10 hover:bg-neutral-focus rounded-lg flex items-center justify-center text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                                    </svg>
                                </div>
                                <div class="w-10 h-10 hover:bg-neutral-focus rounded-lg flex items-center justify-center text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                    </svg>
                                </div>
                            </div>
                            // Secondary Column
                            <SecondaryColumn width=SecondaryColumnWidth::Default variant=ColumnVariant::Light bordered=true>
                                <div class="p-4">
                                    <div class="flex items-center justify-between mb-4">
                                        <h3 class="font-bold">"Inbox"</h3>
                                        <span class="badge badge-sm badge-primary">"12"</span>
                                    </div>
                                    <ul class="menu menu-sm gap-1">
                                        <li><a class="active">"All Messages"</a></li>
                                        <li><a>"Unread"</a></li>
                                        <li><a>"Starred"</a></li>
                                        <li><a>"Sent"</a></li>
                                        <li><a>"Drafts"</a></li>
                                    </ul>
                                    <div class="divider"></div>
                                    <h4 class="font-bold text-sm mb-2">"Labels"</h4>
                                    <div class="flex flex-wrap gap-1">
                                        <span class="badge badge-sm">"Work"</span>
                                        <span class="badge badge-sm badge-secondary">"Personal"</span>
                                        <span class="badge badge-sm badge-accent">"Urgent"</span>
                                    </div>
                                </div>
                            </SecondaryColumn>
                            // Main Content
                            <MainColumn>
                                <div class="p-6">
                                    <div class="flex items-center justify-between mb-4">
                                        <h3 class="text-xl font-bold">"All Messages"</h3>
                                        <div class="flex gap-2">
                                            <button class="btn btn-ghost btn-sm">"Filter"</button>
                                            <button class="btn btn-primary btn-sm">"Compose"</button>
                                        </div>
                                    </div>
                                    <div class="space-y-2">
                                        <div class="bg-base-200 rounded-box p-3 flex items-center gap-3">
                                            <div class="avatar placeholder">
                                                <div class="bg-neutral text-neutral-content w-10 rounded-full">
                                                    <span>"AS"</span>
                                                </div>
                                            </div>
                                            <div class="flex-1 min-w-0">
                                                <div class="flex items-center justify-between">
                                                    <span class="font-semibold">"Alice Smith"</span>
                                                    <span class="text-xs text-base-content/50">"10:30 AM"</span>
                                                </div>
                                                <p class="text-sm text-base-content/70 truncate">"Meeting rescheduled to 3pm tomorrow"</p>
                                            </div>
                                        </div>
                                        <div class="bg-base-100 border border-base-300 rounded-box p-3 flex items-center gap-3">
                                            <div class="avatar placeholder">
                                                <div class="bg-primary text-primary-content w-10 rounded-full">
                                                    <span>"BJ"</span>
                                                </div>
                                            </div>
                                            <div class="flex-1 min-w-0">
                                                <div class="flex items-center justify-between">
                                                    <span class="font-semibold">"Bob Johnson"</span>
                                                    <span class="text-xs text-base-content/50">"Yesterday"</span>
                                                </div>
                                                <p class="text-sm text-base-content/70 truncate">"Please review the attached document"</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </MainColumn>
                            // Right Panel
                            <RightPanel width=RightPanelWidth::Narrow breakpoint=Breakpoint::ExtraLarge variant=ColumnVariant::Muted class="border-l">
                                <div class="p-4">
                                    <h4 class="font-bold mb-3">"Quick Stats"</h4>
                                    <div class="space-y-3">
                                        <div class="bg-base-100 rounded-box p-3">
                                            <div class="text-sm text-base-content/60">"Total"</div>
                                            <div class="text-xl font-bold">"1,234"</div>
                                        </div>
                                        <div class="bg-base-100 rounded-box p-3">
                                            <div class="text-sm text-base-content/60">"Unread"</div>
                                            <div class="text-xl font-bold">"12"</div>
                                        </div>
                                    </div>
                                </div>
                            </RightPanel>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            // --- Pattern 4: Narrow Sidebar with Header and Secondary ---
            <section class="space-y-4">
                <ComponentPreview
                    title="Pattern 4: Narrow Sidebar with Header and Secondary"
                    description="Icon sidebar + header + secondary column + main + right panel. The most comprehensive layout for complex enterprise applications."
                    code=r#"<SidebarShell>
  <Sidebar width=SidebarWidth::Narrow>
    // Icon-only sidebar
  </Sidebar>
  <div class="flex-1 flex flex-col">
    <PageHeader>
      // Top navigation
    </PageHeader>
    <MultiColumnShell>
      <SecondaryColumn width=SecondaryColumnWidth::Narrow>
        // Secondary nav
      </SecondaryColumn>
      <MainColumn>
        // Main content
      </MainColumn>
      <RightPanel breakpoint=Breakpoint::ExtraLarge>
        // Right panel (xl+)
      </RightPanel>
    </MultiColumnShell>
  </div>
</SidebarShell>"#
                >
                    <div class="border border-base-300 rounded-box overflow-hidden w-full">
                        <div class="h-[28rem] flex">
                            // Icon Sidebar
                            <div class="w-16 bg-neutral flex flex-col items-center py-4 gap-2 shrink-0">
                                <div class="w-10 h-10 bg-primary rounded-lg flex items-center justify-center text-primary-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                                    </svg>
                                </div>
                                <div class="w-10 h-10 hover:bg-neutral-focus rounded-lg flex items-center justify-center text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                                    </svg>
                                </div>
                                <div class="w-10 h-10 bg-neutral-focus rounded-lg flex items-center justify-center text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                                    </svg>
                                </div>
                                <div class="mt-auto w-10 h-10 hover:bg-neutral-focus rounded-lg flex items-center justify-center text-neutral-content">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                    </svg>
                                </div>
                            </div>
                            // Main Content Area with Header
                            <div class="flex-1 flex flex-col min-w-0">
                                // Header
                                <div class="border-b border-base-300 bg-base-100 px-4 py-3 flex items-center justify-between">
                                    <div class="flex items-center gap-4">
                                        <h2 class="font-bold">"Team Management"</h2>
                                        <div class="tabs tabs-box tabs-sm">
                                            <a class="tab tab-active">"Members"</a>
                                            <a class="tab">"Groups"</a>
                                            <a class="tab">"Permissions"</a>
                                        </div>
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <div class="input input-sm input-bordered flex items-center gap-2">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-base-content/50" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                                            </svg>
                                            <span class="text-sm text-base-content/50">"Search..."</span>
                                        </div>
                                        <button class="btn btn-primary btn-sm">"Add Member"</button>
                                    </div>
                                </div>
                                // Content
                                <div class="flex flex-1 overflow-hidden">
                                    // Secondary Column
                                    <SecondaryColumn width=SecondaryColumnWidth::Narrow variant=ColumnVariant::Light bordered=true>
                                        <div class="p-4">
                                            <h4 class="font-bold text-sm mb-3">"Departments"</h4>
                                            <ul class="menu menu-sm gap-1">
                                                <li><a class="active">"All Members"</a></li>
                                                <li><a>"Engineering"</a></li>
                                                <li><a>"Design"</a></li>
                                                <li><a>"Marketing"</a></li>
                                                <li><a>"Sales"</a></li>
                                                <li><a>"HR"</a></li>
                                            </ul>
                                            <div class="divider"></div>
                                            <h4 class="font-bold text-sm mb-3">"Status"</h4>
                                            <ul class="menu menu-sm gap-1">
                                                <li><a>"Active"</a></li>
                                                <li><a>"On Leave"</a></li>
                                                <li><a>"Inactive"</a></li>
                                            </ul>
                                        </div>
                                    </SecondaryColumn>
                                    // Main Content
                                    <MainColumn>
                                        <div class="p-6">
                                            <div class="overflow-x-auto">
                                                <table class="table table-sm">
                                                    <thead>
                                                        <tr>
                                                            <th>"Name"</th>
                                                            <th>"Role"</th>
                                                            <th>"Department"</th>
                                                            <th>"Status"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        <tr>
                                                            <td>
                                                                <div class="flex items-center gap-2">
                                                                    <div class="avatar placeholder">
                                                                        <div class="bg-neutral text-neutral-content w-8 rounded-full">
                                                                            <span class="text-xs">"AS"</span>
                                                                        </div>
                                                                    </div>
                                                                    "Alice Smith"
                                                                </div>
                                                            </td>
                                                            <td>"Engineering Manager"</td>
                                                            <td>"Engineering"</td>
                                                            <td><span class="badge badge-sm badge-success">"Active"</span></td>
                                                        </tr>
                                                        <tr>
                                                            <td>
                                                                <div class="flex items-center gap-2">
                                                                    <div class="avatar placeholder">
                                                                        <div class="bg-primary text-primary-content w-8 rounded-full">
                                                                            <span class="text-xs">"BJ"</span>
                                                                        </div>
                                                                    </div>
                                                                    "Bob Johnson"
                                                                </div>
                                                            </td>
                                                            <td>"Senior Developer"</td>
                                                            <td>"Engineering"</td>
                                                            <td><span class="badge badge-sm badge-success">"Active"</span></td>
                                                        </tr>
                                                        <tr>
                                                            <td>
                                                                <div class="flex items-center gap-2">
                                                                    <div class="avatar placeholder">
                                                                        <div class="bg-secondary text-secondary-content w-8 rounded-full">
                                                                            <span class="text-xs">"CW"</span>
                                                                        </div>
                                                                    </div>
                                                                    "Carol White"
                                                                </div>
                                                            </td>
                                                            <td>"Designer"</td>
                                                            <td>"Design"</td>
                                                            <td><span class="badge badge-sm badge-warning">"On Leave"</span></td>
                                                        </tr>
                                                    </tbody>
                                                </table>
                                            </div>
                                        </div>
                                    </MainColumn>
                                    // Right Panel
                                    <RightPanel width=RightPanelWidth::Default breakpoint=Breakpoint::ExtraLarge variant=ColumnVariant::Muted class="border-l">
                                        <div class="p-4">
                                            <h4 class="font-bold mb-3">"Activity"</h4>
                                            <ul class="space-y-3 text-sm">
                                                <li class="flex gap-3">
                                                    <div class="badge badge-xs badge-success mt-1"></div>
                                                    <div>
                                                        <p>"Alice joined the team"</p>
                                                        <p class="text-xs text-base-content/50">"2 hours ago"</p>
                                                    </div>
                                                </li>
                                                <li class="flex gap-3">
                                                    <div class="badge badge-xs badge-info mt-1"></div>
                                                    <div>
                                                        <p>"Bob updated permissions"</p>
                                                        <p class="text-xs text-base-content/50">"5 hours ago"</p>
                                                    </div>
                                                </li>
                                                <li class="flex gap-3">
                                                    <div class="badge badge-xs badge-warning mt-1"></div>
                                                    <div>
                                                        <p>"Carol requested leave"</p>
                                                        <p class="text-xs text-base-content/50">"Yesterday"</p>
                                                    </div>
                                                </li>
                                            </ul>
                                        </div>
                                    </RightPanel>
                                </div>
                            </div>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            // --- Basic Examples ---
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Examples"</h2>

                // Two Column: Main + Right Panel
                <ComponentPreview
                    title="Main + Right Panel"
                    code=r#"<MultiColumnShell>
  <MainColumn>
    // Primary content area
  </MainColumn>
  <Panel class="border-l p-4 bg-base-200">
    // Right activity panel
  </Panel>
</MultiColumnShell>"#
                >
                    <div class="border border-base-300 rounded-box overflow-hidden w-full">
                        <MultiColumnShell class="h-[20rem]">
                            <MainColumn>
                                <div class="p-6 space-y-4">
                                    <h3 class="text-xl font-bold">"Main Content"</h3>
                                    <p class="text-base-content/60">"This is the primary content area that takes up remaining space."</p>
                                    <div class="grid grid-cols-2 gap-4">
                                        <div class="bg-base-200 rounded-box p-4 h-20 flex items-center justify-center">"Card 1"</div>
                                        <div class="bg-base-200 rounded-box p-4 h-20 flex items-center justify-center">"Card 2"</div>
                                    </div>
                                </div>
                            </MainColumn>
                            <Panel class="border-l p-4 bg-base-200">
                                <h4 class="font-bold mb-3">"Activity"</h4>
                                <ul class="space-y-2 text-sm">
                                    <li class="flex items-center gap-2">
                                        <div class="badge badge-xs badge-success"></div>
                                        "User signed up"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <div class="badge badge-xs badge-info"></div>
                                        "New comment"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <div class="badge badge-xs badge-warning"></div>
                                        "Payment pending"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <div class="badge badge-xs badge-error"></div>
                                        "Server alert"
                                    </li>
                                </ul>
                            </Panel>
                        </MultiColumnShell>
                    </div>
                </ComponentPreview>

                // Three Column: Left Panel + Main + Right Panel
                <ComponentPreview
                    title="Three Columns"
                    description="Left navigation panel, main content, and right details panel."
                    code=r#"<MultiColumnShell>
  <Panel width=PanelWidth::Small class="border-r p-4 bg-base-200">
    // Left navigation
  </Panel>
  <MainColumn>
    // Main content
  </MainColumn>
  <Panel class="border-l p-4 bg-base-200">
    // Right details panel
  </Panel>
</MultiColumnShell>"#
                >
                    <div class="border border-base-300 rounded-box overflow-hidden w-full">
                        <MultiColumnShell class="h-[22rem]">
                            <Panel width=PanelWidth::Small class="border-r p-4 bg-base-200">
                                <h4 class="font-bold mb-3">"Channels"</h4>
                                <ul class="menu menu-sm gap-0.5">
                                    <li><a class="active">"# general"</a></li>
                                    <li><a>"# random"</a></li>
                                    <li><a>"# engineering"</a></li>
                                    <li><a>"# design"</a></li>
                                    <li><a>"# marketing"</a></li>
                                </ul>
                            </Panel>
                            <MainColumn>
                                <div class="p-6 space-y-3">
                                    <h3 class="text-xl font-bold">"# general"</h3>
                                    <div class="divider"></div>
                                    <div class="space-y-3">
                                        <div class="chat chat-start">
                                            <div class="chat-bubble chat-bubble-primary">"Hey team! Sprint planning at 2pm."</div>
                                        </div>
                                        <div class="chat chat-end">
                                            <div class="chat-bubble">"Sounds good! I'll prepare the backlog."</div>
                                        </div>
                                        <div class="chat chat-start">
                                            <div class="chat-bubble chat-bubble-primary">"Perfect, see you there 👍"</div>
                                        </div>
                                    </div>
                                </div>
                            </MainColumn>
                            <Panel class="border-l p-4 bg-base-200">
                                <h4 class="font-bold mb-3">"Members"</h4>
                                <ul class="space-y-2">
                                    <li class="flex items-center gap-2">
                                        <div class="badge badge-xs badge-success"></div>
                                        <span class="text-sm">"Alice"</span>
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <div class="badge badge-xs badge-success"></div>
                                        <span class="text-sm">"Bob"</span>
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <div class="badge badge-xs badge-warning"></div>
                                        <span class="text-sm">"Charlie"</span>
                                    </li>
                                </ul>
                            </Panel>
                        </MultiColumnShell>
                    </div>
                </ComponentPreview>
            </section>

            // --- Component Reference ---
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Component Reference"</h2>

                // Width Options
                <div class="overflow-x-auto">
                    <h3 class="text-lg font-medium mb-2">"Panel Widths"</h3>
                    <table class="table table-sm">
                        <thead>
                            <tr>
                                <th>"Enum"</th>
                                <th>"Width"</th>
                                <th>"Use Case"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr><td><code class="badge badge-ghost">"PanelWidth::ExtraSmall"</code></td><td>"w-48 (12rem)"</td><td>"Icon-only, mini nav"</td></tr>
                            <tr><td><code class="badge badge-ghost">"PanelWidth::Small"</code></td><td>"w-56 (14rem)"</td><td>"Compact lists, tags"</td></tr>
                            <tr><td><code class="badge badge-ghost">"PanelWidth::Medium"</code></td><td>"w-64 (16rem)"</td><td>"Standard sidebar (default)"</td></tr>
                            <tr><td><code class="badge badge-ghost">"PanelWidth::Large"</code></td><td>"w-80 (20rem)"</td><td>"Detail panels, forms"</td></tr>
                            <tr><td><code class="badge badge-ghost">"PanelWidth::ExtraLarge"</code></td><td>"w-96 (24rem)"</td><td>"Rich content, previews"</td></tr>
                        </tbody>
                    </table>
                </div>

                <div class="overflow-x-auto">
                    <h3 class="text-lg font-medium mb-2">"SecondaryColumn Widths"</h3>
                    <table class="table table-sm">
                        <thead>
                            <tr>
                                <th>"Enum"</th>
                                <th>"Width"</th>
                                <th>"Use Case"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr><td><code class="badge badge-ghost">"SecondaryColumnWidth::Narrow"</code></td><td>"w-64 (16rem)"</td><td>"Compact secondary nav"</td></tr>
                            <tr><td><code class="badge badge-ghost">"SecondaryColumnWidth::Default"</code></td><td>"w-80 (20rem)"</td><td>"Standard secondary column"</td></tr>
                            <tr><td><code class="badge badge-ghost">"SecondaryColumnWidth::Wide"</code></td><td>"w-96 (24rem)"</td><td>"Rich secondary content"</td></tr>
                        </tbody>
                    </table>
                </div>

                <div class="overflow-x-auto">
                    <h3 class="text-lg font-medium mb-2">"RightPanel Options"</h3>
                    <table class="table table-sm">
                        <thead>
                            <tr>
                                <th>"Enum"</th>
                                <th>"Value"</th>
                                <th>"Description"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr><td><code class="badge badge-ghost">"RightPanelWidth::Narrow"</code></td><td>"w-64"</td><td>"Compact right panel"</td></tr>
                            <tr><td><code class="badge badge-ghost">"RightPanelWidth::Default"</code></td><td>"w-80"</td><td>"Standard right panel"</td></tr>
                            <tr><td><code class="badge badge-ghost">"RightPanelWidth::Wide"</code></td><td>"w-96"</td><td>"Wide right panel"</td></tr>
                            <tr><td colspan="3" class="bg-base-200"></td></tr>
                            <tr><td><code class="badge badge-ghost">"Breakpoint::Large"</code></td><td>"lg"</td><td>"Show on large screens and up"</td></tr>
                            <tr><td><code class="badge badge-ghost">"Breakpoint::ExtraLarge"</code></td><td>"xl"</td><td>"Show on extra large screens and up"</td></tr>
                            <tr><td colspan="3" class="bg-base-200"></td></tr>
                            <tr><td><code class="badge badge-ghost">"ColumnVariant::Default"</code></td><td>"transparent"</td><td>"No background color"</td></tr>
                            <tr><td><code class="badge badge-ghost">"ColumnVariant::Light"</code></td><td>"bg-base-100"</td><td>"Light background"</td></tr>
                            <tr><td><code class="badge badge-ghost">"ColumnVariant::Muted"</code></td><td>"bg-base-200"</td><td>"Muted/subtle background"</td></tr>
                            <tr><td><code class="badge badge-ghost">"ColumnVariant::Dark"</code></td><td>"bg-neutral"</td><td>"Dark background"</td></tr>
                        </tbody>
                    </table>
                </div>
            </section>
        </div>
    }
}

use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn BottomNavigationPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Bottom Navigation"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Mobile-style bottom navigation bars with icons, labels, and active states."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r##"<BottomNavigation class="absolute bottom-0 w-full">
    <button>
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
        </svg>
    </button>
    <button class="active">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
    </button>
    <button>
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        </svg>
    </button>
</BottomNavigation>"##
                >
                    <div class="h-64 relative border border-base-300 rounded-box overflow-hidden bg-base-200">
                         <div class="absolute inset-0 flex items-center justify-center">
                            <p class="text-base-content/50">"App Content"</p>
                         </div>
                        <BottomNavigation class="absolute bottom-0 w-full">
                            <button>
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" /></svg>
                            </button>
                            <button class="active">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                            </button>
                            <button>
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                            </button>
                        </BottomNavigation>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Labels"
                    code=r##"<BottomNavigation class="absolute bottom-0 w-full">
    <button class="active">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <BottomNavigationLabel>"Info"</BottomNavigationLabel>
    </button>
    <button>
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        </svg>
        <BottomNavigationLabel>"Stats"</BottomNavigationLabel>
    </button>
</BottomNavigation>"##
                >
                    <div class="h-64 relative border border-base-300 rounded-box overflow-hidden bg-base-200">
                        <BottomNavigation class="absolute bottom-0 w-full">
                            <button>
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" /></svg>
                                <BottomNavigationLabel>"Home"</BottomNavigationLabel>
                            </button>
                            <button class="active">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                <BottomNavigationLabel>"Info"</BottomNavigationLabel>
                            </button>
                            <button>
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                                <BottomNavigationLabel>"Stats"</BottomNavigationLabel>
                            </button>
                        </BottomNavigation>
                    </div>
                </ComponentPreview>
            </section>

             <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r##"<BottomNavigation class="absolute bottom-0 w-full">
    <button class="text-primary">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">...</svg>
        <BottomNavigationLabel>"Home"</BottomNavigationLabel>
    </button>
    <button class="active text-secondary border-secondary bg-secondary/10">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">...</svg>
        <BottomNavigationLabel>"Info"</BottomNavigationLabel>
    </button>
    <button class="text-accent">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">...</svg>
        <BottomNavigationLabel>"Stats"</BottomNavigationLabel>
    </button>
</BottomNavigation>"##
                >
                    <div class="h-64 relative border border-base-300 rounded-box overflow-hidden bg-base-200">
                        <BottomNavigation class="absolute bottom-0 w-full">
                            <button class="text-primary">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" /></svg>
                                 <BottomNavigationLabel>"Home"</BottomNavigationLabel>
                            </button>
                            <button class="active text-secondary border-secondary bg-secondary/10">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                <BottomNavigationLabel>"Info"</BottomNavigationLabel>
                            </button>
                            <button class="text-accent">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                                 <BottomNavigationLabel>"Stats"</BottomNavigationLabel>
                            </button>
                        </BottomNavigation>
                    </div>
                </ComponentPreview>
            </section>

             <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<BottomNavigation size={Size::ExtraSmall}>
    <button class="active">
        <BottomNavigationLabel>"XS"</BottomNavigationLabel>
    </button>
    <button>
        <BottomNavigationLabel>"XS"</BottomNavigationLabel>
    </button>
</BottomNavigation>

<BottomNavigation size={Size::Large}>
    <button class="active">
        <BottomNavigationLabel>"LG"</BottomNavigationLabel>
    </button>
    <button>
        <BottomNavigationLabel>"LG"</BottomNavigationLabel>
    </button>
</BottomNavigation>"##
                >
                    <div class="h-64 relative border border-base-300 rounded-box overflow-hidden bg-base-200 flex flex-col items-center justify-center p-4">
                         <p class="mb-2">"Small (xs)"</p>
                        <BottomNavigation size={Size::ExtraSmall} class="relative w-full max-w-sm mb-4 bg-base-100 rounded-box">
                             <button class="active">
                                 <BottomNavigationLabel>"XS"</BottomNavigationLabel>
                             </button>
                             <button>
                                 <BottomNavigationLabel>"XS"</BottomNavigationLabel>
                             </button>
                        </BottomNavigation>

                        <p class="mb-2">"Large (lg)"</p>
                        <BottomNavigation size={Size::Large} class="relative w-full max-w-sm bg-base-100 rounded-box">
                             <button class="active">
                                 <BottomNavigationLabel>"LG"</BottomNavigationLabel>
                             </button>
                             <button>
                                 <BottomNavigationLabel>"LG"</BottomNavigationLabel>
                             </button>
                        </BottomNavigation>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

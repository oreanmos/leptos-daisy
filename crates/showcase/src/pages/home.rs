use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="prose max-w-none">
            <h1>"🌼 leptos-daisyui Showcase"</h1>
            <p class="text-lg">"A comprehensive Leptos component library wrapping all 65 daisyUI components."</p>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mt-8">
                // Actions
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"🎬 Actions"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/button">"Button"</A></li>
                            <li><A href="/dropdown">"Dropdown"</A></li>
                            <li><A href="/modal">"Modal"</A></li>
                            <li><A href="/swap">"Swap"</A></li>
                        </ul>
                    </div>
                </div>

                // Data Display
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"📊 Data Display"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/alert">"Alert"</A></li>
                            <li><A href="/avatar">"Avatar"</A></li>
                            <li><A href="/badge">"Badge"</A></li>
                            <li><A href="/card">"Card"</A></li>
                            <li><A href="/carousel">"Carousel"</A></li>
                            <li><A href="/chat">"Chat"</A></li>
                            <li><A href="/collapse">"Collapse"</A></li>
                            <li><A href="/countdown">"Countdown"</A></li>
                            <li><A href="/diff">"Diff"</A></li>
                            <li><A href="/kbd">"Kbd"</A></li>
                            <li><A href="/list">"List"</A></li>
                            <li><A href="/progress">"Progress"</A></li>
                            <li><A href="/radial-progress">"Radial Progress"</A></li>
                            <li><A href="/rating">"Rating"</A></li>
                            <li><A href="/skeleton">"Skeleton"</A></li>
                            <li><A href="/stat">"Stat"</A></li>
                            <li><A href="/status">"Status"</A></li>
                            <li><A href="/table">"Table"</A></li>
                            <li><A href="/timeline">"Timeline"</A></li>
                        </ul>
                    </div>
                </div>

                // Data Input
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"📝 Data Input"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/checkbox">"Checkbox"</A></li>
                            <li><A href="/fieldset">"Fieldset"</A></li>
                            <li><A href="/file-input">"File Input"</A></li>
                            <li><A href="/filter">"Filter"</A></li>
                            <li><A href="/input">"Input"</A></li>
                            <li><A href="/label">"Label"</A></li>
                            <li><A href="/radio">"Radio"</A></li>
                            <li><A href="/range">"Range"</A></li>
                            <li><A href="/select">"Select"</A></li>
                            <li><A href="/textarea">"Textarea"</A></li>
                            <li><A href="/toggle">"Toggle"</A></li>
                            <li><A href="/validator">"Validator"</A></li>
                        </ul>
                    </div>
                </div>

                // Feedback
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"💬 Feedback"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/loading">"Loading"</A></li>
                            <li><A href="/skeleton">"Skeleton"</A></li>
                            <li><A href="/toast">"Toast"</A></li>
                        </ul>
                    </div>
                </div>

                // Layout
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"📐 Layout"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/artboard">"Artboard"</A></li>
                            <li><A href="/divider">"Divider"</A></li>
                            <li><A href="/drawer">"Drawer"</A></li>
                            <li><A href="/footer">"Footer"</A></li>
                            <li><A href="/hero">"Hero"</A></li>
                            <li><A href="/indicator">"Indicator"</A></li>
                            <li><A href="/join">"Join"</A></li>
                            <li><A href="/mask">"Mask"</A></li>
                            <li><A href="/stack">"Stack"</A></li>
                        </ul>
                    </div>
                </div>

                // Mockups
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"🖼️ Mockups"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/mockup-browser">"Browser"</A></li>
                            <li><A href="/mockup-code">"Code"</A></li>
                            <li><A href="/mockup-phone">"Phone"</A></li>
                            <li><A href="/mockup-window">"Window"</A></li>
                        </ul>
                    </div>
                </div>

                // Navigation
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"🧭 Navigation"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/breadcrumbs">"Breadcrumbs"</A></li>
                            <li><A href="/dock">"Dock"</A></li>
                            <li><A href="/link">"Link"</A></li>
                            <li><A href="/menu">"Menu"</A></li>
                            <li><A href="/navbar">"Navbar"</A></li>
                            <li><A href="/pagination">"Pagination"</A></li>
                            <li><A href="/steps">"Steps"</A></li>
                            <li><A href="/tab">"Tab"</A></li>
                        </ul>
                    </div>
                </div>

                // Overlay
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"🔲 Overlay"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/backdrop">"Backdrop"</A></li>
                            <li><A href="/tooltip">"Tooltip"</A></li>
                        </ul>
                    </div>
                </div>

                // Layout Components
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"🏗️ Layout Components"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/layout">"DaisyUI Layout"</A></li>
                            <li><A href="/layouts/stacked">"Stacked Layout"</A></li>
                            <li><A href="/layouts/sidebar">"Sidebar Layout"</A></li>
                            <li><A href="/layouts/multi-column">"Multi-Column"</A></li>
                        </ul>
                    </div>
                </div>

                // Theming
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"🎨 Theming"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/themes">"Theme Showcase"</A></li>
                            <li><A href="/themes/creator">"Theme Creator"</A></li>
                            <li><A href="/themes/guide">"Theme Guide"</A></li>
                        </ul>
                    </div>
                </div>

                // Tools
                <div class="card bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"🛠️ Tools"</h2>
                        <ul class="menu menu-sm">
                            <li><A href="/playground">"Playground"</A></li>
                        </ul>
                    </div>
                </div>
            </div>

            <div class="mt-12 p-6 bg-base-200 rounded-lg">
                <h2>"📚 Quick Stats"</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mt-4">
                    <div class="text-center">
                        <div class="text-4xl font-bold text-primary">"65+"</div>
                        <div class="text-sm opacity-70">"Components"</div>
                    </div>
                    <div class="text-center">
                        <div class="text-4xl font-bold text-secondary">"35"</div>
                        <div class="text-sm opacity-70">"Themes"</div>
                    </div>
                    <div class="text-center">
                        <div class="text-4xl font-bold text-accent">"3"</div>
                        <div class="text-sm opacity-70">"Render Modes"</div>
                    </div>
                    <div class="text-center">
                        <div class="text-4xl font-bold text-info">"0"</div>
                        <div class="text-sm opacity-70">"Dependencies"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

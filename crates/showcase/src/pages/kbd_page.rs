use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn KbdPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Keyboard (Kbd)"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Display keyboard keys and shortcuts with proper styling."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r#"<Kbd size={KbdSize::ExtraSmall}>"XS"</Kbd>
<Kbd size={KbdSize::Small}>"SM"</Kbd>
<Kbd size={KbdSize::Default}>"MD"</Kbd>
<Kbd size={KbdSize::Medium}>"MD+"</Kbd>
<Kbd size={KbdSize::Large}>"LG"</Kbd>
<Kbd size={KbdSize::ExtraLarge}>"XL"</Kbd>"#
                >
                    <div class="flex flex-wrap gap-4 items-center">
                        <div class="flex flex-col items-center gap-2">
                            <Kbd size={KbdSize::ExtraSmall}>"XS"</Kbd>
                            <span class="text-xs text-base-content/70">"Extra Small"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Kbd size={KbdSize::Small}>"SM"</Kbd>
                            <span class="text-xs text-base-content/70">"Small"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Kbd size={KbdSize::Default}>"MD"</Kbd>
                            <span class="text-xs text-base-content/70">"Default"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Kbd size={KbdSize::Medium}>"MD+"</Kbd>
                            <span class="text-xs text-base-content/70">"Medium"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Kbd size={KbdSize::Large}>"LG"</Kbd>
                            <span class="text-xs text-base-content/70">"Large"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Kbd size={KbdSize::ExtraLarge}>"XL"</Kbd>
                            <span class="text-xs text-base-content/70">"Extra Large"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Keyboard Shortcuts"
                    code=r#"<p>"Press "<Kbd>"Ctrl"</Kbd>" + "<Kbd>"C"</Kbd>" to copy"</p>
<p>"Press "<Kbd>"Ctrl"</Kbd>" + "<Kbd>"V"</Kbd>" to paste"</p>
<p>"Press "<Kbd>"Ctrl"</Kbd>" + "<Kbd>"Z"</Kbd>" to undo"</p>
<p>"Press "<Kbd>"Ctrl"</Kbd>" + "<Kbd>"Shift"</Kbd>" + "<Kbd>"T"</Kbd>" to reopen tab"</p>"#
                >
                    <div class="space-y-2">
                        <p>"Press "<Kbd>"Ctrl"</Kbd>" + "<Kbd>"C"</Kbd>" to copy"</p>
                        <p>"Press "<Kbd>"Ctrl"</Kbd>" + "<Kbd>"V"</Kbd>" to paste"</p>
                        <p>"Press "<Kbd>"Ctrl"</Kbd>" + "<Kbd>"Z"</Kbd>" to undo"</p>
                        <p>"Press "<Kbd>"Ctrl"</Kbd>" + "<Kbd>"Shift"</Kbd>" + "<Kbd>"T"</Kbd>" to reopen tab"</p>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Function Keys"
                    code=r#"<Kbd>"F1"</Kbd>
<Kbd>"F2"</Kbd>
<Kbd>"F3"</Kbd>
<Kbd>"F4"</Kbd>
<Kbd>"F5"</Kbd>
<Kbd>"F6"</Kbd>
<Kbd>"F7"</Kbd>
<Kbd>"F8"</Kbd>
<Kbd>"F9"</Kbd>
<Kbd>"F10"</Kbd>
<Kbd>"F11"</Kbd>
<Kbd>"F12"</Kbd>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Kbd>"F1"</Kbd>
                        <Kbd>"F2"</Kbd>
                        <Kbd>"F3"</Kbd>
                        <Kbd>"F4"</Kbd>
                        <Kbd>"F5"</Kbd>
                        <Kbd>"F6"</Kbd>
                        <Kbd>"F7"</Kbd>
                        <Kbd>"F8"</Kbd>
                        <Kbd>"F9"</Kbd>
                        <Kbd>"F10"</Kbd>
                        <Kbd>"F11"</Kbd>
                        <Kbd>"F12"</Kbd>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Arrow Keys"
                    code=r#"<Kbd>"↑"</Kbd>
<Kbd>"↓"</Kbd>
<Kbd>"←"</Kbd>
<Kbd>"→"</Kbd>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Kbd>"↑"</Kbd>
                        <Kbd>"↓"</Kbd>
                        <Kbd>"←"</Kbd>
                        <Kbd>"→"</Kbd>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Special Keys"
                    code=r#"<Kbd>"Enter"</Kbd>
<Kbd>"Space"</Kbd>
<Kbd>"Tab"</Kbd>
<Kbd>"Shift"</Kbd>
<Kbd>"Ctrl"</Kbd>
<Kbd>"Alt"</Kbd>
<Kbd>"Esc"</Kbd>
<Kbd>"Cmd"</Kbd>
<Kbd>"Opt"</Kbd>
<Kbd>"Del"</Kbd>
<Kbd>"Backspace"</Kbd>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Kbd>"Enter"</Kbd>
                        <Kbd>"Space"</Kbd>
                        <Kbd>"Tab"</Kbd>
                        <Kbd>"Shift"</Kbd>
                        <Kbd>"Ctrl"</Kbd>
                        <Kbd>"Alt"</Kbd>
                        <Kbd>"Esc"</Kbd>
                        <Kbd>"Cmd"</Kbd>
                        <Kbd>"Opt"</Kbd>
                        <Kbd>"Del"</Kbd>
                        <Kbd>"Backspace"</Kbd>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="In Context"
                    code=r#"<div class="bg-base-200 p-4 rounded-lg">
    <p class="mb-2">"To save a file:"</p>
    <div class="flex items-center gap-2">
        <Kbd size={KbdSize::Small}>"Ctrl"</Kbd>
        <span>"+"</span>
        <Kbd size={KbdSize::Small}>"S"</Kbd>
    </div>
    <p class="mt-4 mb-2">"To select all:"</p>
    <div class="flex items-center gap-2">
        <Kbd size={KbdSize::Small}>"Ctrl"</Kbd>
        <span>"+"</span>
        <Kbd size={KbdSize::Small}>"A"</Kbd>
    </div>
</div>"#
                >
                    <div class="bg-base-200 p-4 rounded-lg">
                        <p class="mb-2">"To save a file:"</p>
                        <div class="flex items-center gap-2">
                            <Kbd size={KbdSize::Small}>"Ctrl"</Kbd>
                            <span>"+"</span>
                            <Kbd size={KbdSize::Small}>"S"</Kbd>
                        </div>
                        <p class="mt-4 mb-2">"To select all:"</p>
                        <div class="flex items-center gap-2">
                            <Kbd size={KbdSize::Small}>"Ctrl"</Kbd>
                            <span>"+"</span>
                            <Kbd size={KbdSize::Small}>"A"</Kbd>
                        </div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

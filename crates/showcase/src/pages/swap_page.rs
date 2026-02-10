use leptos::prelude::*;
use leptos_daisyui::prelude::*;
use std::sync::Arc;

fn sun_icon() -> impl IntoView {
    view! {
        <div class="text-warning">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-10 h-10 fill-current">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" />
            </svg>
        </div>
    }
}

fn moon_icon() -> impl IntoView {
    view! {
        <div class="text-warning">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-10 h-10 fill-current">
                <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z" />
            </svg>
        </div>
    }
}

fn like_icon() -> impl IntoView {
    view! {
        <Button color={Color::Error}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z" />
            </svg>
            "Unlike"
        </Button>
    }
}

fn unlike_icon() -> impl IntoView {
    view! {
        <Button variant={Variant::Outline}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z" />
            </svg>
            "Like"
        </Button>
    }
}

fn check_icon() -> impl IntoView {
    view! {
        <div class="text-success">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-12 h-12">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
        </div>
    }
}

fn minus_icon() -> impl IntoView {
    view! {
        <div class="text-base-content/50">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-12 h-12">
                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
        </div>
    }
}

fn volume_on_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19.114 5.636a9 9 0 010 12.728M16.463 8.288a5.25 5.25 0 010 7.424M6.75 8.25l4.72-4.72a.75.75 0 011.28.53v15.88a.75.75 0 01-1.28.53l-4.72-4.72H4.51c-.88 0-1.704-.507-1.938-1.354A9.01 9.01 0 012.25 12c0-.83.112-1.633.322-2.396C2.806 8.756 3.63 8.25 4.51 8.25H6.75z" />
        </svg>
    }
}

fn volume_off_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
            <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 9.75L19.5 12m0 0l2.25 2.25M19.5 12l2.25-2.25M19.5 12l-2.25 2.25m-10.5-6l4.72-4.72a.75.75 0 011.28.53v15.88a.75.75 0 01-1.28.53l-4.72-4.72H4.51c-.88 0-1.704-.507-1.938-1.354A9.01 9.01 0 012.25 12c0-.83.112-1.633.322-2.396C2.806 8.756 3.63 8.25 4.51 8.25H6.75z" />
        </svg>
    }
}

fn pause_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-8 h-8">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25v13.5m-7.5-13.5v13.5" />
        </svg>
    }
}

fn play_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-8 h-8">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.348a1.125 1.125 0 010 1.971l-11.54 6.347a1.125 1.125 0 01-1.667-.985V5.653z" />
        </svg>
    }
}

#[component]
pub fn SwapPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Swap"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Rotate Effect"</h2>
                <p class="mb-4 text-base-content/70">
                    "The rotate swap effect rotates the content when activated."
                </p>
                <Swap
                    effect={SwapEffect::Rotate}
                    active=true
                    on_icon={Arc::new(|| moon_icon().into_any())}
                    off_icon={Arc::new(|| sun_icon().into_any())}
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Flip Effect"</h2>
                <p class="mb-4 text-base-content/70">
                    "The flip swap effect flips the content horizontally when activated."
                </p>
                <Swap
                    effect={SwapEffect::Flip}
                    active=true
                    on_icon={Arc::new(|| like_icon().into_any())}
                    off_icon={Arc::new(|| unlike_icon().into_any())}
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Checkbox Controlled"</h2>
                <p class="mb-4 text-base-content/70">
                    "Swaps can be controlled by a checkbox for state management."
                </p>
                <div class="flex items-center gap-4">
                    <SwapCheckbox
                        effect={SwapEffect::Rotate}
                        on_icon={Arc::new(|| check_icon().into_any())}
                        off_icon={Arc::new(|| minus_icon().into_any())}
                    />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Volume Control Example"</h2>
                <div class="flex items-center gap-4">
                    <SwapCheckbox
                        effect={SwapEffect::Flip}
                        on_icon={Arc::new(|| volume_on_icon().into_any())}
                        off_icon={Arc::new(|| volume_off_icon().into_any())}
                    />
                    <Range class="w-48" />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Text Swap"</h2>
                <Swap
                    effect={SwapEffect::Flip}
                    active=true
                    on_icon={Arc::new(|| view! { <span class="text-2xl font-bold text-success">"ON"</span> }.into_any())}
                    off_icon={Arc::new(|| view! { <span class="text-2xl font-bold text-base-content/50">"OFF"</span> }.into_any())}
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Play/Pause Button"</h2>
                <SwapCheckbox
                    effect={SwapEffect::Rotate}
                    class="btn btn-circle btn-lg"
                    on_icon={Arc::new(|| pause_icon().into_any())}
                    off_icon={Arc::new(|| play_icon().into_any())}
                />
            </section>
        </div>
    }
}

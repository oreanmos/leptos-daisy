use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn StatusPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Status"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="flex flex-wrap gap-4 items-center">
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::ExtraSmall} />
                        <span class="text-xs text-base-content/70">"XS"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::Small} />
                        <span class="text-xs text-base-content/70">"SM"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::Default} />
                        <span class="text-xs text-base-content/70">"Default"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::Medium} />
                        <span class="text-xs text-base-content/70">"MD"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::Large} />
                        <span class="text-xs text-base-content/70">"LG"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::ExtraLarge} />
                        <span class="text-xs text-base-content/70">"XL"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-wrap gap-4">
                    <div class="flex flex-col items-center gap-2">
                        <Status color={StatusColor::Default} />
                        <span class="text-xs text-base-content/70">"Default"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status color={StatusColor::Primary} />
                        <span class="text-xs text-base-content/70">"Primary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status color={StatusColor::Secondary} />
                        <span class="text-xs text-base-content/70">"Secondary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status color={StatusColor::Accent} />
                        <span class="text-xs text-base-content/70">"Accent"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status color={StatusColor::Neutral} />
                        <span class="text-xs text-base-content/70">"Neutral"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status color={StatusColor::Info} />
                        <span class="text-xs text-base-content/70">"Info"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status color={StatusColor::Success} />
                        <span class="text-xs text-base-content/70">"Success"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status color={StatusColor::Warning} />
                        <span class="text-xs text-base-content/70">"Warning"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status color={StatusColor::Error} />
                        <span class="text-xs text-base-content/70">"Error"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Combinations"</h2>
                <div class="flex flex-wrap gap-4">
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::Large} color={StatusColor::Success} />
                        <span class="text-xs text-base-content/70">"Large Success"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::Large} color={StatusColor::Error} />
                        <span class="text-xs text-base-content/70">"Large Error"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::Medium} color={StatusColor::Warning} />
                        <span class="text-xs text-base-content/70">"Medium Warning"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Status size={StatusSize::ExtraLarge} color={StatusColor::Info} />
                        <span class="text-xs text-base-content/70">"XL Info"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Avatars"</h2>
                <div class="flex flex-wrap gap-4">
                    <div class="relative">
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Online" alt="User" />
                        </Avatar>
                        <div class="absolute bottom-0 right-0">
                            <Status size={StatusSize::Small} color={StatusColor::Success} />
                        </div>
                    </div>
                    <div class="relative">
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Away" alt="User" />
                        </Avatar>
                        <div class="absolute bottom-0 right-0">
                            <Status size={StatusSize::Small} color={StatusColor::Warning} />
                        </div>
                    </div>
                    <div class="relative">
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Busy" alt="User" />
                        </Avatar>
                        <div class="absolute bottom-0 right-0">
                            <Status size={StatusSize::Small} color={StatusColor::Error} />
                        </div>
                    </div>
                    <div class="relative">
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Offline" alt="User" />
                        </Avatar>
                        <div class="absolute bottom-0 right-0">
                            <Status size={StatusSize::Small} color={StatusColor::Neutral} />
                        </div>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"In Context"</h2>
                <div class="space-y-2 max-w-md">
                    <div class="flex items-center gap-3 p-2 bg-base-200 rounded">
                        <Status size={StatusSize::Small} color={StatusColor::Success} />
                        <span>"System operational"</span>
                    </div>
                    <div class="flex items-center gap-3 p-2 bg-base-200 rounded">
                        <Status size={StatusSize::Small} color={StatusColor::Warning} />
                        <span>"High CPU usage"</span>
                    </div>
                    <div class="flex items-center gap-3 p-2 bg-base-200 rounded">
                        <Status size={StatusSize::Small} color={StatusColor::Error} />
                        <span>"Service unavailable"</span>
                    </div>
                    <div class="flex items-center gap-3 p-2 bg-base-200 rounded">
                        <Status size={StatusSize::Small} color={StatusColor::Info} />
                        <span>"Update available"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}

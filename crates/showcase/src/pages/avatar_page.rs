use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn AvatarPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Avatar"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="flex flex-wrap gap-4 items-end">
                    <div class="flex flex-col items-center gap-2">
                        <Avatar size={AvatarSize::ExtraSmall}>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
                        </Avatar>
                        <span class="text-xs text-base-content/70">"XS"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar size={AvatarSize::Small}>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
                        </Avatar>
                        <span class="text-xs text-base-content/70">"SM"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar size={AvatarSize::Medium}>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
                        </Avatar>
                        <span class="text-xs text-base-content/70">"MD"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar size={AvatarSize::Large}>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
                        </Avatar>
                        <span class="text-xs text-base-content/70">"LG"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar size={AvatarSize::ExtraLarge}>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
                        </Avatar>
                        <span class="text-xs text-base-content/70">"XL"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Shapes"</h2>
                <div class="flex flex-wrap gap-4">
                    <Avatar shape={AvatarShape::Default}>
                        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Default" alt="Avatar" />
                    </Avatar>
                    <Avatar shape={AvatarShape::Circle}>
                        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Circle" alt="Avatar" />
                    </Avatar>
                    <Avatar shape={AvatarShape::Rounded}>
                        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Rounded" alt="Avatar" />
                    </Avatar>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Status Indicators"</h2>
                <div class="flex flex-wrap gap-4">
                    <div class="flex flex-col items-center gap-2">
                        <Avatar status={AvatarStatus::Online}>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Online" alt="Avatar" />
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Online"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar status={AvatarStatus::Offline}>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Offline" alt="Avatar" />
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Offline"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Placeholders"</h2>
                <div class="flex flex-wrap gap-4">
                    <div class="flex flex-col items-center gap-2">
                        <Avatar placeholder={AvatarPlaceholder::Neutral}>
                            <AvatarPlaceholderContent>"JD"</AvatarPlaceholderContent>
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Neutral"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar placeholder={AvatarPlaceholder::Primary}>
                            <AvatarPlaceholderContent>"AB"</AvatarPlaceholderContent>
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Primary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar placeholder={AvatarPlaceholder::Secondary}>
                            <AvatarPlaceholderContent>"CD"</AvatarPlaceholderContent>
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Secondary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar placeholder={AvatarPlaceholder::Accent}>
                            <AvatarPlaceholderContent>"EF"</AvatarPlaceholderContent>
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Accent"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar placeholder={AvatarPlaceholder::Info}>
                            <AvatarPlaceholderContent>"GH"</AvatarPlaceholderContent>
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Info"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar placeholder={AvatarPlaceholder::Success}>
                            <AvatarPlaceholderContent>"IJ"</AvatarPlaceholderContent>
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Success"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar placeholder={AvatarPlaceholder::Warning}>
                            <AvatarPlaceholderContent>"KL"</AvatarPlaceholderContent>
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Warning"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Avatar placeholder={AvatarPlaceholder::Error}>
                            <AvatarPlaceholderContent>"MN"</AvatarPlaceholderContent>
                        </Avatar>
                        <span class="text-xs text-base-content/70">"Error"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Avatar Groups"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Horizontal"</span>
                        <AvatarGroup>
                            <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=1" alt="Avatar" /></Avatar>
                            <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=2" alt="Avatar" /></Avatar>
                            <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=3" alt="Avatar" /></Avatar>
                            <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=4" alt="Avatar" /></Avatar>
                            <Avatar placeholder={AvatarPlaceholder::Neutral}>
                                <AvatarPlaceholderContent>"+5"</AvatarPlaceholderContent>
                            </Avatar>
                        </AvatarGroup>
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Vertical"</span>
                        <AvatarGroup vertical=true>
                            <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=5" alt="Avatar" /></Avatar>
                            <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=6" alt="Avatar" /></Avatar>
                            <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=7" alt="Avatar" /></Avatar>
                        </AvatarGroup>
                    </div>
                </div>
            </section>
        </div>
    }
}

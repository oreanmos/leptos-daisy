use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn AvatarPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Avatar"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Avatars are used to show a thumbnail representation of an individual or business in the interface."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r#"<Avatar size={AvatarSize::ExtraSmall}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
</Avatar>
<Avatar size={AvatarSize::Small}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
</Avatar>
<Avatar size={AvatarSize::Medium}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
</Avatar>
<Avatar size={AvatarSize::Large}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
</Avatar>
<Avatar size={AvatarSize::ExtraLarge}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar" />
</Avatar>"#
                >
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
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Shapes"
                     code=r#"<Avatar shape={AvatarShape::Default}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Default" alt="Avatar" />
</Avatar>

<Avatar shape={AvatarShape::Circle}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Circle" alt="Avatar" />
</Avatar>

<Avatar shape={AvatarShape::Rounded}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Rounded" alt="Avatar" />
</Avatar>"#
                >
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
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                 <ComponentPreview
                    title="Status Indicators"
                     code=r#"<Avatar status={AvatarStatus::Online}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Online" alt="Avatar" />
</Avatar>

<Avatar status={AvatarStatus::Offline}>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Offline" alt="Avatar" />
</Avatar>"#
                >
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
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Placeholders"
                     code=r#"<Avatar placeholder={AvatarPlaceholder::Neutral}>
    <AvatarPlaceholderContent>"JD"</AvatarPlaceholderContent>
</Avatar>

<Avatar placeholder={AvatarPlaceholder::Primary}>
    <AvatarPlaceholderContent>"AB"</AvatarPlaceholderContent>
</Avatar>

<Avatar placeholder={AvatarPlaceholder::Secondary}>
    <AvatarPlaceholderContent>"CD"</AvatarPlaceholderContent>
</Avatar>

<Avatar placeholder={AvatarPlaceholder::Accent}>
    <AvatarPlaceholderContent>"EF"</AvatarPlaceholderContent>
</Avatar>"#
                >
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
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Avatar Groups"
                     code=r#"<AvatarGroup>
    <Avatar><img src="..." alt="Avatar" /></Avatar>
    <Avatar><img src="..." alt="Avatar" /></Avatar>
    <Avatar><img src="..." alt="Avatar" /></Avatar>
    <Avatar><img src="..." alt="Avatar" /></Avatar>
    <Avatar placeholder={AvatarPlaceholder::Neutral}>
        <AvatarPlaceholderContent>"+5"</AvatarPlaceholderContent>
    </Avatar>
</AvatarGroup>

<AvatarGroup vertical=true>
    <Avatar><img src="..." alt="Avatar" /></Avatar>
    <Avatar><img src="..." alt="Avatar" /></Avatar>
    <Avatar><img src="..." alt="Avatar" /></Avatar>
</AvatarGroup>"#
                >
                    <div class="space-y-4">
                        <div>
                            <span class="text-sm text-base-content/70 block mb-2">"Horizontal"</span>
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
                            <span class="text-sm text-base-content/70 block mb-2">"Vertical"</span>
                            <AvatarGroup vertical=true>
                                <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=5" alt="Avatar" /></Avatar>
                                <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=6" alt="Avatar" /></Avatar>
                                <Avatar><img src="https://api.dicebear.com/7.x/avataaars/svg?seed=7" alt="Avatar" /></Avatar>
                            </AvatarGroup>
                        </div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

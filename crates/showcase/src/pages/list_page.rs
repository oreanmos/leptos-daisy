use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ListPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"List"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "List components for displaying items in vertical or horizontal layouts with support for columns and complex content."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic List"
                    code=r#"<List class="bg-base-200 rounded-lg max-w-sm">
    <ListItem>"Item 1"</ListItem>
    <ListItem>"Item 2"</ListItem>
    <ListItem>"Item 3"</ListItem>
    <ListItem>"Item 4"</ListItem>
</List>"#
                >
                    <List class="bg-base-200 rounded-lg max-w-sm">
                        <ListItem>"Item 1"</ListItem>
                        <ListItem>"Item 2"</ListItem>
                        <ListItem>"Item 3"</ListItem>
                        <ListItem>"Item 4"</ListItem>
                    </List>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="List Row"
                    code=r#"<List row=true class="bg-base-200 rounded-lg max-w-sm">
    <ListItem>"Row Item 1"</ListItem>
    <ListItem>"Row Item 2"</ListItem>
    <ListItem>"Row Item 3"</ListItem>
</List>"#
                >
                    <List row=true class="bg-base-200 rounded-lg max-w-sm">
                        <ListItem>"Row Item 1"</ListItem>
                        <ListItem>"Row Item 2"</ListItem>
                        <ListItem>"Row Item 3"</ListItem>
                    </List>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="List with Columns"
                    code=r#"<List class="bg-base-200 rounded-lg max-w-md">
    <ListItem>
        <ListCol>
            <div class="font-semibold">"John Doe"</div>
            <div class="text-sm text-base-content/70">"john@example.com"</div>
        </ListCol>
        <ListCol>
            <Badge color={Color::Success}>"Active"</Badge>
        </ListCol>
    </ListItem>
    <ListItem>
        <ListCol>
            <div class="font-semibold">"Jane Smith"</div>
            <div class="text-sm text-base-content/70">"jane@example.com"</div>
        </ListCol>
        <ListCol>
            <Badge color={Color::Warning}>"Pending"</Badge>
        </ListCol>
    </ListItem>
</List>"#
                >
                    <List class="bg-base-200 rounded-lg max-w-md">
                        <ListItem>
                            <ListCol>
                                <div class="font-semibold">"John Doe"</div>
                                <div class="text-sm text-base-content/70">"john@example.com"</div>
                            </ListCol>
                            <ListCol>
                                <Badge color={Color::Success}>"Active"</Badge>
                            </ListCol>
                        </ListItem>
                        <ListItem>
                            <ListCol>
                                <div class="font-semibold">"Jane Smith"</div>
                                <div class="text-sm text-base-content/70">"jane@example.com"</div>
                            </ListCol>
                            <ListCol>
                                <Badge color={Color::Warning}>"Pending"</Badge>
                            </ListCol>
                        </ListItem>
                        <ListItem>
                            <ListCol>
                                <div class="font-semibold">"Bob Johnson"</div>
                                <div class="text-sm text-base-content/70">"bob@example.com"</div>
                            </ListCol>
                            <ListCol>
                                <Badge color={Color::Error}>"Inactive"</Badge>
                            </ListCol>
                        </ListItem>
                    </List>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="List with Growing Column"
                    code=r#"<List class="bg-base-200 rounded-lg max-w-md">
    <ListItem>
        <ListCol grow=true>
            <div class="font-semibold">"Product Name"</div>
            <div class="text-sm text-base-content/70">"Description goes here"</div>
        </ListCol>
        <ListCol>
            <div class="text-right font-bold">"$99.99"</div>
        </ListCol>
    </ListItem>
    <ListItem>
        <ListCol grow=true>
            <div class="font-semibold">"Another Product"</div>
            <div class="text-sm text-base-content/70">"Another description"</div>
        </ListCol>
        <ListCol>
            <div class="text-right font-bold">"$149.99"</div>
        </ListCol>
    </ListItem>
</List>"#
                >
                    <List class="bg-base-200 rounded-lg max-w-md">
                        <ListItem>
                            <ListCol grow=true>
                                <div class="font-semibold">"Product Name"</div>
                                <div class="text-sm text-base-content/70">"Description goes here"</div>
                            </ListCol>
                            <ListCol>
                                <div class="text-right font-bold">"$99.99"</div>
                            </ListCol>
                        </ListItem>
                        <ListItem>
                            <ListCol grow=true>
                                <div class="font-semibold">"Another Product"</div>
                                <div class="text-sm text-base-content/70">"Another description"</div>
                            </ListCol>
                            <ListCol>
                                <div class="text-right font-bold">"$149.99"</div>
                            </ListCol>
                        </ListItem>
                    </List>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Complex List Items"
                    code=r#"<List class="bg-base-200 rounded-lg max-w-md">
    <ListItem>
        <ListCol>
            <Avatar size={AvatarSize::Small}>
                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=1" alt="User" />
            </Avatar>
        </ListCol>
        <ListCol grow=true>
            <div class="font-semibold">"New Message"</div>
            <div class="text-sm text-base-content/70">"Hey, can we meet tomorrow?"</div>
        </ListCol>
        <ListCol>
            <span class="text-xs text-base-content/50">"2m ago"</span>
        </ListCol>
    </ListItem>
</List>"#
                >
                    <List class="bg-base-200 rounded-lg max-w-md">
                        <ListItem>
                            <ListCol>
                                <Avatar size={AvatarSize::Small}>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=1" alt="User" />
                                </Avatar>
                            </ListCol>
                            <ListCol grow=true>
                                <div class="font-semibold">"New Message"</div>
                                <div class="text-sm text-base-content/70">"Hey, can we meet tomorrow?"</div>
                            </ListCol>
                            <ListCol>
                                <span class="text-xs text-base-content/50">"2m ago"</span>
                            </ListCol>
                        </ListItem>
                        <ListItem>
                            <ListCol>
                                <Avatar size={AvatarSize::Small}>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=2" alt="User" />
                                </Avatar>
                            </ListCol>
                            <ListCol grow=true>
                                <div class="font-semibold">"Meeting Reminder"</div>
                                <div class="text-sm text-base-content/70">"Team standup in 15 minutes"</div>
                            </ListCol>
                            <ListCol>
                                <span class="text-xs text-base-content/50">"1h ago"</span>
                            </ListCol>
                        </ListItem>
                    </List>
                </ComponentPreview>
            </section>
        </div>
    }
}

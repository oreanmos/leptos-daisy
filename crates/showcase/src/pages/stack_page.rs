use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn StackPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Stack"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Stack elements on top of each other with overlapping visual effects."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "stack", type_label: "base", description: "Base stacking class" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic Stack"
                    code=r##"<Stack class="w-64">
    <img src="https://picsum.photos/seed/1/200/150" class="rounded-box" alt="Image 1" />
    <img src="https://picsum.photos/seed/2/200/150" class="rounded-box" alt="Image 2" />
    <img src="https://picsum.photos/seed/3/200/150" class="rounded-box" alt="Image 3" />
</Stack>"##
                >
                    <Stack class="w-64">
                        <img src="https://picsum.photos/seed/1/200/150" class="rounded-box" alt="Image 1" />
                        <img src="https://picsum.photos/seed/2/200/150" class="rounded-box" alt="Image 2" />
                        <img src="https://picsum.photos/seed/3/200/150" class="rounded-box" alt="Image 3" />
                    </Stack>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Stacked Avatars"
                    code=r##"<Stack class="w-16">
    <Avatar>
        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=1" alt="User 1" />
    </Avatar>
    <Avatar>
        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=2" alt="User 2" />
    </Avatar>
    <Avatar>
        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=3" alt="User 3" />
    </Avatar>
</Stack>"##
                >
                    <Stack class="w-16">
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=1" alt="User 1" />
                        </Avatar>
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=2" alt="User 2" />
                        </Avatar>
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=3" alt="User 3" />
                        </Avatar>
                    </Stack>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Stacked Cards"
                    code=r##"<Stack class="w-72">
    <Card class="bg-primary text-primary-content shadow-xl">
        <CardBody>
            <CardTitle>"Card 1"</CardTitle>
            <p>"This is the first card in the stack."</p>
        </CardBody>
    </Card>
    <Card class="bg-secondary text-secondary-content shadow-xl">
        <CardBody>
            <CardTitle>"Card 2"</CardTitle>
            <p>"This is the second card in the stack."</p>
        </CardBody>
    </Card>
    <Card class="bg-accent text-accent-content shadow-xl">
        <CardBody>
            <CardTitle>"Card 3"</CardTitle>
            <p>"This is the third card in the stack."</p>
        </CardBody>
    </Card>
</Stack>"##
                >
                    <Stack class="w-72">
                        <Card class="bg-primary text-primary-content shadow-xl">
                            <CardBody>
                                <CardTitle>"Card 1"</CardTitle>
                                <p>"This is the first card in the stack."</p>
                            </CardBody>
                        </Card>
                        <Card class="bg-secondary text-secondary-content shadow-xl">
                            <CardBody>
                                <CardTitle>"Card 2"</CardTitle>
                                <p>"This is the second card in the stack."</p>
                            </CardBody>
                        </Card>
                        <Card class="bg-accent text-accent-content shadow-xl">
                            <CardBody>
                                <CardTitle>"Card 3"</CardTitle>
                                <p>"This is the third card in the stack."</p>
                            </CardBody>
                        </Card>
                    </Stack>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Stack with Notifications"
                    code=r##"<Stack class="w-80">
    <Alert color={Color::Info}>"New message received"</Alert>
    <Alert color={Color::Success}>"Task completed successfully"</Alert>
    <Alert color={Color::Warning}>"Please review your settings"</Alert>
</Stack>"##
                >
                    <Stack class="w-80">
                        <Alert color={Color::Info}>"New message received"</Alert>
                        <Alert color={Color::Success}>"Task completed successfully"</Alert>
                        <Alert color={Color::Warning}>"Please review your settings"</Alert>
                    </Stack>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Interactive Stack"
                    code=r##"<Stack class="w-48 hover:-translate-y-2 transition-transform">
    <div class="bg-base-300 p-4 rounded-lg shadow-lg">
        <p class="font-bold">"Item 1"</p>
        <p class="text-sm">"First item"</p>
    </div>
    <div class="bg-base-200 p-4 rounded-lg shadow-lg">
        <p class="font-bold">"Item 2"</p>
        <p class="text-sm">"Second item"</p>
    </div>
    <div class="bg-base-100 p-4 rounded-lg shadow-lg border">
        <p class="font-bold">"Item 3"</p>
        <p class="text-sm">"Third item"</p>
    </div>
</Stack>"##
                >
                    <div>
                        <span class="text-sm text-base-content/70 block mb-2">"Hover to reveal"</span>
                        <Stack class="w-48 hover:-translate-y-2 transition-transform">
                            <div class="bg-base-300 p-4 rounded-lg shadow-lg">
                                <p class="font-bold">"Item 1"</p>
                                <p class="text-sm">"First item"</p>
                            </div>
                            <div class="bg-base-200 p-4 rounded-lg shadow-lg">
                                <p class="font-bold">"Item 2"</p>
                                <p class="text-sm">"Second item"</p>
                            </div>
                            <div class="bg-base-100 p-4 rounded-lg shadow-lg border">
                                <p class="font-bold">"Item 3"</p>
                                <p class="text-sm">"Third item"</p>
                            </div>
                        </Stack>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

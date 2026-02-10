use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn StackPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Stack"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Stack"</h2>
                <Stack class="w-64">
                    <img src="https://picsum.photos/seed/1/200/150" class="rounded-box" alt="Image 1" />
                    <img src="https://picsum.photos/seed/2/200/150" class="rounded-box" alt="Image 2" />
                    <img src="https://picsum.photos/seed/3/200/150" class="rounded-box" alt="Image 3" />
                </Stack>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Stacked Avatars"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Stacked Cards"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Stack with Notifications"</h2>
                <Stack class="w-80">
                    <Alert variant={AlertVariant::Info}>"New message received"</Alert>
                    <Alert variant={AlertVariant::Success}>"Task completed successfully"</Alert>
                    <Alert variant={AlertVariant::Warning}>"Please review your settings"</Alert>
                </Stack>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Interactive Stack"</h2>
                <div class="flex gap-8">
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
                </div>
            </section>
        </div>
    }
}

use leptos::prelude::*;
use leptos_daisyui::prelude::*;
use std::sync::Arc;

#[component]
pub fn IndicatorPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Indicator"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Indicator"</h2>
                <Indicator item={Arc::new(move || view! { <Badge color={Color::Primary}>"New"</Badge> }.into_any())}>
                    <div class="bg-base-200 p-8 rounded-lg">
                        <p>"Content with an indicator"</p>
                    </div>
                </Indicator>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Position Variations"</h2>
                <div class="flex flex-wrap gap-8">
                    <div class="flex flex-col items-center gap-2">
                        <Indicator
                            horizontal={IndicatorHorizontal::Start}
                            vertical={IndicatorVertical::Top}
                            item={Arc::new(move || view! { <div class="badge badge-sm badge-error">"1"</div> }.into_any())}
                        >
                            <Button>"Top Start"</Button>
                        </Indicator>
                        <span class="text-xs text-base-content/70">"Top Start"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Indicator
                            horizontal={IndicatorHorizontal::Center}
                            vertical={IndicatorVertical::Top}
                            item={Arc::new(move || view! { <div class="badge badge-sm badge-error">"2"</div> }.into_any())}
                        >
                            <Button>"Top Center"</Button>
                        </Indicator>
                        <span class="text-xs text-base-content/70">"Top Center"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Indicator
                            horizontal={IndicatorHorizontal::End}
                            vertical={IndicatorVertical::Top}
                            item={Arc::new(move || view! { <div class="badge badge-sm badge-error">"3"</div> }.into_any())}
                        >
                            <Button>"Top End"</Button>
                        </Indicator>
                        <span class="text-xs text-base-content/70">"Top End"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Indicator
                            horizontal={IndicatorHorizontal::Start}
                            vertical={IndicatorVertical::Middle}
                            item={Arc::new(move || view! { <div class="badge badge-sm badge-primary">"4"</div> }.into_any())}
                        >
                            <Button>"Middle Start"</Button>
                        </Indicator>
                        <span class="text-xs text-base-content/70">"Middle Start"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Indicator
                            horizontal={IndicatorHorizontal::End}
                            vertical={IndicatorVertical::Middle}
                            item={Arc::new(move || view! { <div class="badge badge-sm badge-primary">"5"</div> }.into_any())}
                        >
                            <Button>"Middle End"</Button>
                        </Indicator>
                        <span class="text-xs text-base-content/70">"Middle End"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Badges"</h2>
                <div class="flex flex-wrap gap-8">
                    <Indicator
                        item={Arc::new(move || view! { <Badge color={Color::Error}>"99+"</Badge> }.into_any())}
                    >
                        <Button variant={Variant::Ghost} class="btn-circle">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                            </svg>
                        </Button>
                    </Indicator>
                    <Indicator
                        item={Arc::new(move || view! { <div class="badge badge-sm badge-success">"Live"</div> }.into_any())}
                    >
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=User" alt="User" />
                        </Avatar>
                    </Indicator>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Status Dots"</h2>
                <div class="flex flex-wrap gap-8">
                    <Indicator
                        item={Arc::new(move || view! { <Status size={Size::Small} color={Color::Success} /> }.into_any())}
                    >
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Online" alt="User" />
                        </Avatar>
                    </Indicator>
                    <Indicator
                        item={Arc::new(move || view! { <Status size={Size::Small} color={Color::Warning} /> }.into_any())}
                    >
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Away" alt="User" />
                        </Avatar>
                    </Indicator>
                    <Indicator
                        item={Arc::new(move || view! { <Status size={Size::Small} color={Color::Error} /> }.into_any())}
                    >
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Busy" alt="User" />
                        </Avatar>
                    </Indicator>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"On Cards"</h2>
                <Indicator
                    item={Arc::new(move || view! { <Badge color={Color::Accent}>"Sale"</Badge> }.into_any())}
                >
                    <Card class="bg-base-100 w-64 shadow-xl">
                        <CardBody>
                            <CardTitle>"Product Name"</CardTitle>
                            <p>"Product description goes here."</p>
                            <div class="flex justify-between items-center mt-4">
                                <span class="text-xl font-bold">"$99"</span>
                                <Button size={Size::Small} color={Color::Primary}>"Buy"</Button>
                            </div>
                        </CardBody>
                    </Card>
                </Indicator>
            </section>
        </div>
    }
}

use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn StatPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Stat"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Display statistics and metrics with titles, values, and descriptions."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Horizontal Stats"
                    code=r##"<Stats>
    <Stat>
        <StatTitle>"Total Users"</StatTitle>
        <StatValue>"25.6K"</StatValue>
        <StatDesc>"Jan 1st - Feb 1st"</StatDesc>
    </Stat>
    <Stat>
        <StatTitle>"Page Views"</StatTitle>
        <StatValue>"2.6M"</StatValue>
        <StatDesc>"↗︎ 14% increase"</StatDesc>
    </Stat>
    <Stat>
        <StatTitle>"New Signups"</StatTitle>
        <StatValue>"1,200"</StatValue>
        <StatDesc>"↘︎ 5% decrease"</StatDesc>
    </Stat>
</Stats>"##
                >
                    <Stats>
                        <Stat>
                            <StatTitle>"Total Users"</StatTitle>
                            <StatValue>"25.6K"</StatValue>
                            <StatDesc>"Jan 1st - Feb 1st"</StatDesc>
                        </Stat>
                        <Stat>
                            <StatTitle>"Page Views"</StatTitle>
                            <StatValue>"2.6M"</StatValue>
                            <StatDesc>"↗︎ 14% increase"</StatDesc>
                        </Stat>
                        <Stat>
                            <StatTitle>"New Signups"</StatTitle>
                            <StatValue>"1,200"</StatValue>
                            <StatDesc>"↘︎ 5% decrease"</StatDesc>
                        </Stat>
                    </Stats>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Vertical Stats"
                    code=r##"<Stats vertical=true>
    <Stat>
        <StatTitle>"Revenue"</StatTitle>
        <StatValue>"$89,400"</StatValue>
        <StatDesc>"This month"</StatDesc>
    </Stat>
    <Stat>
        <StatTitle>"Orders"</StatTitle>
        <StatValue>"450"</StatValue>
        <StatDesc>"↗︎ 12% from last month"</StatDesc>
    </Stat>
</Stats>"##
                >
                    <Stats vertical=true>
                        <Stat>
                            <StatTitle>"Revenue"</StatTitle>
                            <StatValue>"$89,400"</StatValue>
                            <StatDesc>"This month"</StatDesc>
                        </Stat>
                        <Stat>
                            <StatTitle>"Orders"</StatTitle>
                            <StatValue>"450"</StatValue>
                            <StatDesc>"↗︎ 12% from last month"</StatDesc>
                        </Stat>
                    </Stats>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Stats with Figures"
                    code=r##"<Stats>
    <Stat>
        <StatFigure class="text-primary">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
            </svg>
        </StatFigure>
        <StatTitle>"Active Users"</StatTitle>
        <StatValue>"4,521"</StatValue>
        <StatDesc>"↗︎ 8% increase"</StatDesc>
    </Stat>
    <Stat>
        <StatFigure class="text-secondary">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 18L9 11.25l4.306 4.307a11.95 11.95 0 015.814-5.519l2.74-1.22m0 0l-5.94-2.28m5.94 2.28l-2.28 5.941" />
            </svg>
        </StatFigure>
        <StatTitle>"Growth"</StatTitle>
        <StatValue>"23.5%"</StatValue>
        <StatDesc>"Year over year"</StatDesc>
    </Stat>
</Stats>"##
                >
                    <Stats>
                        <Stat>
                            <StatFigure class="text-primary">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
                                </svg>
                            </StatFigure>
                            <StatTitle>"Active Users"</StatTitle>
                            <StatValue>"4,521"</StatValue>
                            <StatDesc>"↗︎ 8% increase"</StatDesc>
                        </Stat>
                        <Stat>
                            <StatFigure class="text-secondary">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 18L9 11.25l4.306 4.307a11.95 11.95 0 015.814-5.519l2.74-1.22m0 0l-5.94-2.28m5.94 2.28l-2.28 5.941" />
                                </svg>
                            </StatFigure>
                            <StatTitle>"Growth"</StatTitle>
                            <StatValue>"23.5%"</StatValue>
                            <StatDesc>"Year over year"</StatDesc>
                        </Stat>
                    </Stats>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Stats with Actions"
                    code=r##"<Stats>
    <Stat>
        <StatTitle>"Storage Used"</StatTitle>
        <StatValue>"75%"</StatValue>
        <StatDesc>"7.5 GB of 10 GB"</StatDesc>
        <StatActions>
            <Button size={Size::Small} color={Color::Primary}>"Upgrade"</Button>
        </StatActions>
    </Stat>
    <Stat>
        <StatTitle>"Bandwidth"</StatTitle>
        <StatValue>"45%"</StatValue>
        <StatDesc>"450 GB of 1 TB"</StatDesc>
        <StatActions>
            <Button size={Size::Small} variant={Variant::Ghost}>"View Details"</Button>
        </StatActions>
    </Stat>
</Stats>"##
                >
                    <Stats>
                        <Stat>
                            <StatTitle>"Storage Used"</StatTitle>
                            <StatValue>"75%"</StatValue>
                            <StatDesc>"7.5 GB of 10 GB"</StatDesc>
                            <StatActions>
                                <Button size={Size::Small} color={Color::Primary}>"Upgrade"</Button>
                            </StatActions>
                        </Stat>
                        <Stat>
                            <StatTitle>"Bandwidth"</StatTitle>
                            <StatValue>"45%"</StatValue>
                            <StatDesc>"450 GB of 1 TB"</StatDesc>
                            <StatActions>
                                <Button size={Size::Small} variant={Variant::Ghost}>"View Details"</Button>
                            </StatActions>
                        </Stat>
                    </Stats>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Dashboard Stats Example"
                    code=r##"<Stats class="bg-base-200 p-4 rounded-lg">
    <Stat>
        <StatFigure class="text-primary">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
            </svg>
        </StatFigure>
        <StatTitle>"Total Sales"</StatTitle>
        <StatValue>"$124,500"</StatValue>
        <StatDesc class="text-success">"↗︎ 23% from last month"</StatDesc>
    </Stat>
    <Stat>
        <StatFigure class="text-secondary">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 10.5V6a3.75 3.75 0 10-7.5 0v4.5m11.356-1.993l1.263 12c.07.665-.45 1.243-1.119 1.243H4.25a1.125 1.125 0 01-1.12-1.243l1.264-12A1.125 1.125 0 015.513 7.5h12.974c.576 0 1.059.435 1.119 1.007zM8.625 10.5a.375.375 0 11-.75 0 .375.375 0 01.75 0zm7.5 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" />
            </svg>
        </StatFigure>
        <StatTitle>"Orders"</StatTitle>
        <StatValue>"1,245"</StatValue>
        <StatDesc class="text-success">"↗︎ 15% from last month"</StatDesc>
    </Stat>
    <Stat>
        <StatFigure class="text-accent">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z" />
            </svg>
        </StatFigure>
        <StatTitle>"Rating"</StatTitle>
        <StatValue>"4.8"</StatValue>
        <StatDesc>"Based on 324 reviews"</StatDesc>
    </Stat>
    <Stat>
        <StatFigure class="text-info">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
            </svg>
        </StatFigure>
        <StatTitle>"Customers"</StatTitle>
        <StatValue>"8,549"</StatValue>
        <StatDesc class="text-success">"↗︎ 7% new this month"</StatDesc>
    </Stat>
</Stats>"##
                >
                    <Stats class="bg-base-200 p-4 rounded-lg">
                        <Stat>
                            <StatFigure class="text-primary">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
                                </svg>
                            </StatFigure>
                            <StatTitle>"Total Sales"</StatTitle>
                            <StatValue>"$124,500"</StatValue>
                            <StatDesc class="text-success">"↗︎ 23% from last month"</StatDesc>
                        </Stat>
                        <Stat>
                            <StatFigure class="text-secondary">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 10.5V6a3.75 3.75 0 10-7.5 0v4.5m11.356-1.993l1.263 12c.07.665-.45 1.243-1.119 1.243H4.25a1.125 1.125 0 01-1.12-1.243l1.264-12A1.125 1.125 0 015.513 7.5h12.974c.576 0 1.059.435 1.119 1.007zM8.625 10.5a.375.375 0 11-.75 0 .375.375 0 01.75 0zm7.5 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" />
                                </svg>
                            </StatFigure>
                            <StatTitle>"Orders"</StatTitle>
                            <StatValue>"1,245"</StatValue>
                            <StatDesc class="text-success">"↗︎ 15% from last month"</StatDesc>
                        </Stat>
                        <Stat>
                            <StatFigure class="text-accent">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z" />
                                </svg>
                            </StatFigure>
                            <StatTitle>"Rating"</StatTitle>
                            <StatValue>"4.8"</StatValue>
                            <StatDesc>"Based on 324 reviews"</StatDesc>
                        </Stat>
                        <Stat>
                            <StatFigure class="text-info">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
                                </svg>
                            </StatFigure>
                            <StatTitle>"Customers"</StatTitle>
                            <StatValue>"8,549"</StatValue>
                            <StatDesc class="text-success">"↗︎ 7% new this month"</StatDesc>
                        </Stat>
                    </Stats>
                </ComponentPreview>
            </section>
        </div>
    }
}

use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TimelinePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Timeline"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <Timeline>
                    <TimelineItem connect_end=true>
                        <TimelineStart>"1984"</TimelineStart>
                        <TimelineMiddle />
                        <TimelineEnd>"First Macintosh"</TimelineEnd>
                    </TimelineItem>
                    <TimelineItem connect_start=true connect_end=true>
                        <TimelineStart>"1998"</TimelineStart>
                        <TimelineMiddle />
                        <TimelineEnd>"iMac"</TimelineEnd>
                    </TimelineItem>
                    <TimelineItem connect_start=true>
                        <TimelineStart>"2007"</TimelineStart>
                        <TimelineMiddle />
                        <TimelineEnd>"iPhone"</TimelineEnd>
                    </TimelineItem>
                </Timeline>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical"</h2>
                <Timeline vertical=true>
                    <TimelineItem connect_end=true>
                        <TimelineStart>"Start"</TimelineStart>
                        <TimelineMiddle />
                        <TimelineEnd>"First event"</TimelineEnd>
                    </TimelineItem>
                    <TimelineItem connect_start=true connect_end=true>
                        <TimelineStart>"Middle"</TimelineStart>
                        <TimelineMiddle />
                        <TimelineEnd>"Second event"</TimelineEnd>
                    </TimelineItem>
                    <TimelineItem connect_start=true>
                        <TimelineStart>"End"</TimelineStart>
                        <TimelineMiddle />
                        <TimelineEnd>"Third event"</TimelineEnd>
                    </TimelineItem>
                </Timeline>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Responsive"</h2>
                <Timeline responsive=true>
                    <TimelineItem connect_end=true>
                        <TimelineStart>"2020"</TimelineStart>
                        <TimelineMiddle />
                        <TimelineEnd>"Project started"</TimelineEnd>
                    </TimelineItem>
                    <TimelineItem connect_start=true>
                        <TimelineStart>"2023"</TimelineStart>
                        <TimelineMiddle />
                        <TimelineEnd>"v1.0 released"</TimelineEnd>
                    </TimelineItem>
                </Timeline>
            </section>
        </div>
    }
}

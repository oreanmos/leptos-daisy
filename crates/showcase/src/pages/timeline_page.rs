use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TimelinePage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Timeline"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Horizontal or vertical timeline layouts for showing sequences of events."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "timeline", type_label: "base", description: "Base class for timeline container" },
                    ClassEntry { name: "timeline-start", type_label: "base part", description: "Content positioned at the start of a timeline item" },
                    ClassEntry { name: "timeline-end", type_label: "base part", description: "Content positioned at the end of a timeline item" },
                    ClassEntry { name: "timeline-middle", type_label: "base part", description: "Middle indicator (icon/dot) of a timeline item" },
                    ClassEntry { name: "timeline-snap-icon", type_label: "modifier", description: "Snap the icon to the timeline line" },
                    ClassEntry { name: "timeline-compact", type_label: "modifier", description: "Compact timeline layout without start content" },
                    ClassEntry { name: "timeline-vertical", type_label: "modifier", description: "Display timeline vertically" },
                    ClassEntry { name: "timeline-horizontal", type_label: "modifier", description: "Display timeline horizontally" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic horizontal timeline"
                    code=r##"<Timeline>
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
</Timeline>"##
                >
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
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Vertical timeline"
                    code=r##"<Timeline vertical=true>
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
</Timeline>"##
                >
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
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Responsive timeline"
                    code=r##"<Timeline responsive=true>
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
</Timeline>"##
                >
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
                </ComponentPreview>
            </section>
        </div>
    }
}

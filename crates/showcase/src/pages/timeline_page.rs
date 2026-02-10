use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TimelinePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Timeline"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical Timeline"</h2>
                <Timeline>
                    <TimelineItem>
                        <TimelineStart>"2024-01-01"</TimelineStart>
                        <TimelineMiddle>
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" />
                            </svg>
                        </TimelineMiddle>
                        <TimelineEnd>
                            <TimelineBox>
                                <h3 class="font-bold">"Project Started"</h3>
                                <p class="text-sm">"Initial project kickoff and planning phase."</p>
                            </TimelineBox>
                        </TimelineEnd>
                    </TimelineItem>
                    <TimelineItem>
                        <TimelineStart>"2024-02-15"</TimelineStart>
                        <TimelineMiddle>
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" />
                            </svg>
                        </TimelineMiddle>
                        <TimelineEnd>
                            <TimelineBox>
                                <h3 class="font-bold">"First Milestone"</h3>
                                <p class="text-sm">"Completed initial design and prototyping."</p>
                            </TimelineBox>
                        </TimelineEnd>
                    </TimelineItem>
                    <TimelineItem>
                        <TimelineStart>"2024-03-30"</TimelineStart>
                        <TimelineMiddle>
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" />
                            </svg>
                        </TimelineMiddle>
                        <TimelineEnd>
                            <TimelineBox>
                                <h3 class="font-bold">"Beta Release"</h3>
                                <p class="text-sm">"Released beta version to early adopters."</p>
                            </TimelineBox>
                        </TimelineEnd>
                    </TimelineItem>
                </Timeline>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Horizontal Timeline"</h2>
                <Timeline orientation={TimelineOrientation::Horizontal}>
                    <TimelineItem>
                        <TimelineStart>"Step 1"</TimelineStart>
                        <TimelineMiddle>
                            <div class="w-4 h-4 rounded-full bg-primary"></div>
                        </TimelineMiddle>
                        <TimelineEnd>"Planning"</TimelineEnd>
                    </TimelineItem>
                    <TimelineItem>
                        <TimelineStart>"Step 2"</TimelineStart>
                        <TimelineMiddle>
                            <div class="w-4 h-4 rounded-full bg-primary"></div>
                        </TimelineMiddle>
                        <TimelineEnd>"Design"</TimelineEnd>
                    </TimelineItem>
                    <TimelineItem>
                        <TimelineStart>"Step 3"</TimelineStart>
                        <TimelineMiddle>
                            <div class="w-4 h-4 rounded-full bg-primary"></div>
                        </TimelineMiddle>
                        <TimelineEnd>"Development"</TimelineEnd>
                    </TimelineItem>
                    <TimelineItem>
                        <TimelineStart>"Step 4"</TimelineStart>
                        <TimelineMiddle>
                            <div class="w-4 h-4 rounded-full bg-neutral"></div>
                        </TimelineMiddle>
                        <TimelineEnd>"Launch"</TimelineEnd>
                    </TimelineItem>
                </Timeline>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Timeline with Different Positions"</h2>
                <Timeline>
                    <TimelineItem>
                        <TimelineStart class="text-right">
                            <div class="font-bold">"09:00 AM"</div>
                            <div class="text-sm text-base-content/70">"Morning"</div>
                        </TimelineStart>
                        <TimelineMiddle>
                            <div class="w-3 h-3 rounded-full bg-success"></div>
                        </TimelineMiddle>
                        <TimelineEnd>
                            <div class="font-semibold">"Team Standup"</div>
                            <div class="text-sm text-base-content/70">"Daily sync meeting"</div>
                        </TimelineEnd>
                    </TimelineItem>
                    <TimelineItem>
                        <TimelineStart class="text-right">
                            <div class="font-bold">"11:30 AM"</div>
                            <div class="text-sm text-base-content/70">"Morning"</div>
                        </TimelineStart>
                        <TimelineMiddle>
                            <div class="w-3 h-3 rounded-full bg-info"></div>
                        </TimelineMiddle>
                        <TimelineEnd>
                            <div class="font-semibold">"Code Review"</div>
                            <div class="text-sm text-base-content/70">"Review pull requests"</div>
                        </TimelineEnd>
                    </TimelineItem>
                    <TimelineItem>
                        <TimelineStart class="text-right">
                            <div class="font-bold">"02:00 PM"</div>
                            <div class="text-sm text-base-content/70">"Afternoon"</div>
                        </TimelineStart>
                        <TimelineMiddle>
                            <div class="w-3 h-3 rounded-full bg-warning"></div>
                        </TimelineMiddle>
                        <TimelineEnd>
                            <div class="font-semibold">"Client Meeting"</div>
                            <div class="text-sm text-base-content/70">"Project status update"</div>
                        </TimelineEnd>
                    </TimelineItem>
                </Timeline>
            </section>
        </div>
    }
}

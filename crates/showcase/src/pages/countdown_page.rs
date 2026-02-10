use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CountdownPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Countdown"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Single Value Countdown"</h2>
                <div class="flex flex-wrap gap-8">
                    <div class="flex flex-col items-center gap-2">
                        <Countdown value={42} />
                        <span class="text-sm text-base-content/70">"Value: 42"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Countdown value={0} />
                        <span class="text-sm text-base-content/70">"Value: 0"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Countdown value={99} />
                        <span class="text-sm text-base-content/70">"Value: 99"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Countdown Timer"</h2>
                <div class="flex flex-wrap gap-8">
                    <div class="flex flex-col items-center gap-2">
                        <CountdownTimer
                            minutes={5}
                            seconds={30}
                        />
                        <span class="text-sm text-base-content/70">"Minutes & Seconds"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Full Timer"</h2>
                <div class="flex flex-wrap gap-8">
                    <div class="flex flex-col items-center gap-2">
                        <CountdownTimer
                            days={1}
                            hours={12}
                            minutes={30}
                            seconds={45}
                            show_days=true
                            show_hours=true
                        />
                        <span class="text-sm text-base-content/70">"Days, Hours, Minutes, Seconds"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Styled Countdowns"</h2>
                <div class="flex flex-wrap gap-8">
                    <div class="flex flex-col items-center gap-2">
                        <span class="text-4xl font-mono font-bold text-primary">
                            <Countdown value={25} />
                        </span>
                        <span class="text-sm text-base-content/70">"Primary Color"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <span class="text-4xl font-mono font-bold text-secondary">
                            <Countdown value={50} />
                        </span>
                        <span class="text-sm text-base-content/70">"Secondary Color"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <span class="text-4xl font-mono font-bold text-accent">
                            <Countdown value={75} />
                        </span>
                        <span class="text-sm text-base-content/70">"Accent Color"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"In Context"</h2>
                <div class="bg-base-200 p-6 rounded-lg max-w-md">
                    <h3 class="text-lg font-semibold mb-4">"Flash Sale Ends In:"</h3>
                    <div class="flex justify-center gap-4 text-center">
                        <div>
                            <div class="text-4xl font-mono font-bold">
                                <Countdown value={2} />
                            </div>
                            <div class="text-sm text-base-content/70">"Hours"</div>
                        </div>
                        <div class="text-4xl font-mono">":"</div>
                        <div>
                            <div class="text-4xl font-mono font-bold">
                                <Countdown value={45} />
                            </div>
                            <div class="text-sm text-base-content/70">"Minutes"</div>
                        </div>
                        <div class="text-4xl font-mono">":"</div>
                        <div>
                            <div class="text-4xl font-mono font-bold">
                                <Countdown value={30} />
                            </div>
                            <div class="text-sm text-base-content/70">"Seconds"</div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

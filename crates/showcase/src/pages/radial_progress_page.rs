use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn RadialProgressPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Radial Progress"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Different Values"</h2>
                <div class="flex flex-wrap gap-8">
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={0} />
                        <span class="text-sm text-base-content/70">"0%"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={25} />
                        <span class="text-sm text-base-content/70">"25%"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={50} />
                        <span class="text-sm text-base-content/70">"50%"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={75} />
                        <span class="text-sm text-base-content/70">"75%"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={100} />
                        <span class="text-sm text-base-content/70">"100%"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Custom Sizes"</h2>
                <div class="flex flex-wrap gap-8 items-center">
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={60} size="4rem".to_string() />
                        <span class="text-sm text-base-content/70">"4rem"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={60} size="6rem".to_string() />
                        <span class="text-sm text-base-content/70">"6rem"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={60} size="8rem".to_string() />
                        <span class="text-sm text-base-content/70">"8rem"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={60} size="12rem".to_string() />
                        <span class="text-sm text-base-content/70">"12rem"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Custom Thickness"</h2>
                <div class="flex flex-wrap gap-8 items-center">
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={70} thickness="2px".to_string() />
                        <span class="text-sm text-base-content/70">"2px"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={70} thickness="4px".to_string() />
                        <span class="text-sm text-base-content/70">"4px"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={70} thickness="8px".to_string() />
                        <span class="text-sm text-base-content/70">"8px"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <RadialProgress value={70} thickness="12px".to_string() />
                        <span class="text-sm text-base-content/70">"12px"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Color Variants"</h2>
                <div class="flex flex-wrap gap-8">
                    <div class="flex flex-col items-center gap-2">
                        <div class="radial-progress text-primary" style="--value:80;" role="progressbar">"80%"</div>
                        <span class="text-sm text-base-content/70">"Primary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <div class="radial-progress text-secondary" style="--value:65;" role="progressbar">"65%"</div>
                        <span class="text-sm text-base-content/70">"Secondary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <div class="radial-progress text-accent" style="--value:45;" role="progressbar">"45%"</div>
                        <span class="text-sm text-base-content/70">"Accent"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <div class="radial-progress text-success" style="--value:90;" role="progressbar">"90%"</div>
                        <span class="text-sm text-base-content/70">"Success"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <div class="radial-progress text-warning" style="--value:30;" role="progressbar">"30%"</div>
                        <span class="text-sm text-base-content/70">"Warning"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <div class="radial-progress text-error" style="--value:15;" role="progressbar">"15%"</div>
                        <span class="text-sm text-base-content/70">"Error"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"In Context"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 max-w-2xl">
                    <Card class="bg-base-100 shadow-xl">
                        <CardBody class="items-center text-center">
                            <RadialProgress value={75} size="6rem".to_string() thickness="8px".to_string() />
                            <h3 class="font-bold mt-4">"Storage"</h3>
                            <p class="text-sm text-base-content/70">"75GB of 100GB used"</p>
                        </CardBody>
                    </Card>
                    <Card class="bg-base-100 shadow-xl">
                        <CardBody class="items-center text-center">
                            <RadialProgress value={45} size="6rem".to_string() thickness="8px".to_string() />
                            <h3 class="font-bold mt-4">"Bandwidth"</h3>
                            <p class="text-sm text-base-content/70">"450GB of 1TB used"</p>
                        </CardBody>
                    </Card>
                    <Card class="bg-base-100 shadow-xl">
                        <CardBody class="items-center text-center">
                            <RadialProgress value={90} size="6rem".to_string() thickness="8px".to_string() />
                            <h3 class="font-bold mt-4">"Tasks"</h3>
                            <p class="text-sm text-base-content/70">"18 of 20 completed"</p>
                        </CardBody>
                    </Card>
                </div>
            </section>
        </div>
    }
}

use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn LayoutPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Layout"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Divider"</h2>
                <div class="flex flex-col gap-0">
                    <p>"Content above"</p>
                    <div class="divider">"OR"</div>
                    <p>"Content below"</p>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Stack"</h2>
                <Stack>
                    <div class="bg-primary text-primary-content p-4 rounded-box">"Item 1"</div>
                    <div class="bg-secondary text-secondary-content p-4 rounded-box">"Item 2"</div>
                    <div class="bg-accent text-accent-content p-4 rounded-box">"Item 3"</div>
                </Stack>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Join"</h2>
                <Join>
                    <Button class="join-item">"Button 1"</Button>
                    <Button class="join-item">"Button 2"</Button>
                    <Button class="join-item">"Button 3"</Button>
                </Join>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Grid"</h2>
                <Grid cols={GridCols::Three}>
                    <div class="bg-base-300 p-4 rounded-box text-center">"1"</div>
                    <div class="bg-base-300 p-4 rounded-box text-center">"2"</div>
                    <div class="bg-base-300 p-4 rounded-box text-center">"3"</div>
                    <div class="bg-base-300 p-4 rounded-box text-center">"4"</div>
                    <div class="bg-base-300 p-4 rounded-box text-center">"5"</div>
                    <div class="bg-base-300 p-4 rounded-box text-center">"6"</div>
                </Grid>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Footer"</h2>
                <Footer>
                    <nav>
                        <h6 class="footer-title">"Services"</h6>
                        <a class="link link-hover">"Branding"</a>
                        <a class="link link-hover">"Design"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Company"</h6>
                        <a class="link link-hover">"About"</a>
                        <a class="link link-hover">"Contact"</a>
                    </nav>
                </Footer>
            </section>
        </div>
    }
}

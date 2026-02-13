use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn LayoutPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Layout"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Layout components including divider, stack, join, grid, and footer for organizing page content."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Divider"
                    code=r#"<div class="flex flex-col gap-0">
    <p>"Content above"</p>
    <div class="divider">"OR"</div>
    <p>"Content below"</p>
</div>"#
                >
                    <div class="flex flex-col gap-0">
                        <p>"Content above"</p>
                        <div class="divider">"OR"</div>
                        <p>"Content below"</p>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Stack"
                    code=r#"<Stack>
    <div class="bg-primary text-primary-content p-4 rounded-box">"Item 1"</div>
    <div class="bg-secondary text-secondary-content p-4 rounded-box">"Item 2"</div>
    <div class="bg-accent text-accent-content p-4 rounded-box">"Item 3"</div>
</Stack>"#
                >
                    <Stack>
                        <div class="bg-primary text-primary-content p-4 rounded-box">"Item 1"</div>
                        <div class="bg-secondary text-secondary-content p-4 rounded-box">"Item 2"</div>
                        <div class="bg-accent text-accent-content p-4 rounded-box">"Item 3"</div>
                    </Stack>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Join"
                    code=r#"<Join>
    <Button class="join-item">"Button 1"</Button>
    <Button class="join-item">"Button 2"</Button>
    <Button class="join-item">"Button 3"</Button>
</Join>"#
                >
                    <Join>
                        <Button class="join-item">"Button 1"</Button>
                        <Button class="join-item">"Button 2"</Button>
                        <Button class="join-item">"Button 3"</Button>
                    </Join>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Grid"
                    code=r#"<Grid cols={GridCols::Three}>
    <div class="bg-base-300 p-4 rounded-box text-center">"1"</div>
    <div class="bg-base-300 p-4 rounded-box text-center">"2"</div>
    <div class="bg-base-300 p-4 rounded-box text-center">"3"</div>
    <div class="bg-base-300 p-4 rounded-box text-center">"4"</div>
    <div class="bg-base-300 p-4 rounded-box text-center">"5"</div>
    <div class="bg-base-300 p-4 rounded-box text-center">"6"</div>
</Grid>"#
                >
                    <Grid cols={GridCols::Three}>
                        <div class="bg-base-300 p-4 rounded-box text-center">"1"</div>
                        <div class="bg-base-300 p-4 rounded-box text-center">"2"</div>
                        <div class="bg-base-300 p-4 rounded-box text-center">"3"</div>
                        <div class="bg-base-300 p-4 rounded-box text-center">"4"</div>
                        <div class="bg-base-300 p-4 rounded-box text-center">"5"</div>
                        <div class="bg-base-300 p-4 rounded-box text-center">"6"</div>
                    </Grid>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Footer"
                    code=r#"<Footer>
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
</Footer>"#
                >
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
                </ComponentPreview>
            </section>
        </div>
    }
}

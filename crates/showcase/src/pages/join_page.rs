use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn JoinPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Join"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Group elements together with seamless borders for toolbars, input groups, and navigation."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Horizontal Join"
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
                    title="Vertical Join"
                    code=r#"<Join vertical=true>
    <Button class="join-item">"Button 1"</Button>
    <Button class="join-item">"Button 2"</Button>
    <Button class="join-item">"Button 3"</Button>
</Join>"#
                >
                    <Join vertical=true>
                        <Button class="join-item">"Button 1"</Button>
                        <Button class="join-item">"Button 2"</Button>
                        <Button class="join-item">"Button 3"</Button>
                    </Join>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Join with Input and Button"
                    code=r#"<Join>
    <Input class="join-item" placeholder="Search..." />
    <Button color={Color::Primary} class="join-item">"Search"</Button>
</Join>"#
                >
                    <Join>
                        <Input class="join-item" placeholder="Search..." />
                        <Button color={Color::Primary} class="join-item">"Search"</Button>
                    </Join>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Join with Select"
                    code=r#"<Join>
    <Select class="join-item">
        <option>"Filter by"</option>
        <option>"Name"</option>
        <option>"Date"</option>
        <option>"Price"</option>
    </Select>
    <Input class="join-item" placeholder="Search term..." />
    <Button color={Color::Primary} class="join-item">"Go"</Button>
</Join>"#
                >
                    <Join>
                        <Select class="join-item">
                            <option>"Filter by"</option>
                            <option>"Name"</option>
                            <option>"Date"</option>
                            <option>"Price"</option>
                        </Select>
                        <Input class="join-item" placeholder="Search term..." />
                        <Button color={Color::Primary} class="join-item">"Go"</Button>
                    </Join>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Responsive Join"
                    code=r#"<Join responsive=true>
    <Button class="join-item">"Home"</Button>
    <Button class="join-item">"About"</Button>
    <Button class="join-item">"Services"</Button>
    <Button class="join-item">"Contact"</Button>
</Join>"#
                >
                    <Join responsive=true>
                        <Button class="join-item">"Home"</Button>
                        <Button class="join-item">"About"</Button>
                        <Button class="join-item">"Services"</Button>
                        <Button class="join-item">"Contact"</Button>
                    </Join>
                    <p class="text-sm text-base-content/70 mt-2">"Vertical on mobile, horizontal on desktop"</p>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Join with Pagination Style"
                    code=r#"<Join>
    <Button class="join-item">"First"</Button>
    <Button class="join-item">"Previous"</Button>
    <Button class="join-item btn-active">"1"</Button>
    <Button class="join-item">"2"</Button>
    <Button class="join-item">"3"</Button>
    <Button class="join-item">"Next"</Button>
    <Button class="join-item">"Last"</Button>
</Join>"#
                >
                    <Join>
                        <Button class="join-item">"First"</Button>
                        <Button class="join-item">"Previous"</Button>
                        <Button class="join-item btn-active">"1"</Button>
                        <Button class="join-item">"2"</Button>
                        <Button class="join-item">"3"</Button>
                        <Button class="join-item">"Next"</Button>
                        <Button class="join-item">"Last"</Button>
                    </Join>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Vertical Join with Form Elements"
                    code=r#"<Join vertical=true class="w-full max-w-xs">
    <Input class="join-item" placeholder="Username" />
    <Input class="join-item" placeholder="Password" input_type="password" />
    <Button color={Color::Primary} class="join-item">"Login"</Button>
</Join>"#
                >
                    <Join vertical=true class="w-full max-w-xs">
                        <Input class="join-item" placeholder="Username" />
                        <Input class="join-item" placeholder="Password" input_type="password" />
                        <Button color={Color::Primary} class="join-item">"Login"</Button>
                    </Join>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Join with Radio Buttons"
                    code=r#"<Join>
    <input class="join-item btn" type="radio" name="options" aria-label="Option 1" checked />
    <input class="join-item btn" type="radio" name="options" aria-label="Option 2" />
    <input class="join-item btn" type="radio" name="options" aria-label="Option 3" />
</Join>"#
                >
                    <Join>
                        <input class="join-item btn" type="radio" name="options" aria-label="Option 1" checked />
                        <input class="join-item btn" type="radio" name="options" aria-label="Option 2" />
                        <input class="join-item btn" type="radio" name="options" aria-label="Option 3" />
                    </Join>
                </ComponentPreview>
            </section>
        </div>
    }
}

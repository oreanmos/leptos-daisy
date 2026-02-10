use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn JoinPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Join"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Horizontal Join"</h2>
                <Join>
                    <Button class="join-item">"Button 1"</Button>
                    <Button class="join-item">"Button 2"</Button>
                    <Button class="join-item">"Button 3"</Button>
                </Join>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical Join"</h2>
                <Join vertical=true>
                    <Button class="join-item">"Button 1"</Button>
                    <Button class="join-item">"Button 2"</Button>
                    <Button class="join-item">"Button 3"</Button>
                </Join>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Join with Input and Button"</h2>
                <Join>
                    <Input class="join-item" placeholder="Search..." />
                    <Button color={Color::Primary} class="join-item">"Search"</Button>
                </Join>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Join with Select"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Responsive Join"</h2>
                <Join responsive=true>
                    <Button class="join-item">"Home"</Button>
                    <Button class="join-item">"About"</Button>
                    <Button class="join-item">"Services"</Button>
                    <Button class="join-item">"Contact"</Button>
                </Join>
                <p class="text-sm text-base-content/70 mt-2">"Vertical on mobile, horizontal on desktop"</p>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Join with Pagination Style"</h2>
                <Join>
                    <Button class="join-item">"First"</Button>
                    <Button class="join-item">"Previous"</Button>
                    <Button class="join-item btn-active">"1"</Button>
                    <Button class="join-item">"2"</Button>
                    <Button class="join-item">"3"</Button>
                    <Button class="join-item">"Next"</Button>
                    <Button class="join-item">"Last"</Button>
                </Join>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical Join with Form Elements"</h2>
                <Join vertical=true class="w-full max-w-xs">
                    <Input class="join-item" placeholder="Username" />
                    <Input class="join-item" placeholder="Password" input_type="password" />
                    <Button color={Color::Primary} class="join-item">"Login"</Button>
                </Join>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Join with Radio Buttons"</h2>
                <Join>
                    <input class="join-item btn" type="radio" name="options" aria-label="Option 1" checked />
                    <input class="join-item btn" type="radio" name="options" aria-label="Option 2" />
                    <input class="join-item btn" type="radio" name="options" aria-label="Option 3" />
                </Join>
            </section>
        </div>
    }
}

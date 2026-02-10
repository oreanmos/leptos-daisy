use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FooterPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Footer"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Footer"</h2>
                <Footer class="bg-base-200 p-10">
                    <div>
                        <FooterTitle>"Services"</FooterTitle>
                        <Link href="/" class="link link-hover block">"Branding"</Link>
                        <Link href="/" class="link link-hover block">"Design"</Link>
                        <Link href="/" class="link link-hover block">"Marketing"</Link>
                        <Link href="/" class="link link-hover block">"Advertisement"</Link>
                    </div>
                    <div>
                        <FooterTitle>"Company"</FooterTitle>
                        <Link href="/" class="link link-hover block">"About us"</Link>
                        <Link href="/" class="link link-hover block">"Contact"</Link>
                        <Link href="/" class="link link-hover block">"Jobs"</Link>
                        <Link href="/" class="link link-hover block">"Press kit"</Link>
                    </div>
                    <div>
                        <FooterTitle>"Legal"</FooterTitle>
                        <Link href="/" class="link link-hover block">"Terms of use"</Link>
                        <Link href="/" class="link link-hover block">"Privacy policy"</Link>
                        <Link href="/" class="link link-hover block">"Cookie policy"</Link>
                    </div>
                </Footer>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Centered Footer"</h2>
                <Footer center=true class="bg-base-200 p-10">
                    <div class="text-center">
                        <FooterTitle>"Company Name"</FooterTitle>
                        <p class="max-w-md">"Providing reliable tech since 1992. We are dedicated to delivering the best service to our customers."</p>
                    </div>
                    <div>
                        <div class="grid grid-flow-col gap-4 justify-center">
                            <Link href="/" class="link link-hover">"Twitter"</Link>
                            <Link href="/" class="link link-hover">"GitHub"</Link>
                            <Link href="/" class="link link-hover">"Discord"</Link>
                        </div>
                    </div>
                </Footer>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Horizontal Footer"</h2>
                <Footer horizontal=true class="bg-base-200 p-10">
                    <div class="flex gap-4">
                        <Link href="/" class="link link-hover">"About"</Link>
                        <Link href="/" class="link link-hover">"Contact"</Link>
                        <Link href="/" class="link link-hover">"Privacy"</Link>
                        <Link href="/" class="link link-hover">"Terms"</Link>
                    </div>
                    <div>
                        <p>"© 2024 Company Name. All rights reserved."</p>
                    </div>
                </Footer>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical Footer"</h2>
                <Footer vertical=true class="bg-base-200 p-10">
                    <div>
                        <FooterTitle>"Categories"</FooterTitle>
                        <Link href="/" class="link link-hover block">"Electronics"</Link>
                        <Link href="/" class="link link-hover block">"Clothing"</Link>
                        <Link href="/" class="link link-hover block">"Books"</Link>
                        <Link href="/" class="link link-hover block">"Home & Garden"</Link>
                    </div>
                    <div>
                        <FooterTitle>"Support"</FooterTitle>
                        <Link href="/" class="link link-hover block">"Help Center"</Link>
                        <Link href="/" class="link link-hover block">"Returns"</Link>
                        <Link href="/" class="link link-hover block">"Shipping"</Link>
                        <Link href="/" class="link link-hover block">"Track Order"</Link>
                    </div>
                    <div>
                        <FooterTitle>"Account"</FooterTitle>
                        <Link href="/" class="link link-hover block">"Login"</Link>
                        <Link href="/" class="link link-hover block">"Register"</Link>
                        <Link href="/" class="link link-hover block">"Order History"</Link>
                        <Link href="/" class="link link-hover block">"Wishlist"</Link>
                    </div>
                </Footer>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Footer with Form"</h2>
                <Footer class="bg-base-200 p-10">
                    <div>
                        <FooterTitle>"Newsletter"</FooterTitle>
                        <p class="mb-4">"Subscribe to our newsletter for updates"</p>
                        <div class="join">
                            <Input class="join-item" placeholder="email@example.com" />
                            <Button color={Color::Primary} class="join-item">"Subscribe"</Button>
                        </div>
                    </div>
                    <div>
                        <FooterTitle>"Links"</FooterTitle>
                        <Link href="/" class="link link-hover block">"Home"</Link>
                        <Link href="/" class="link link-hover block">"Products"</Link>
                        <Link href="/" class="link link-hover block">"Services"</Link>
                    </div>
                </Footer>
            </section>
        </div>
    }
}

use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FooterPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Footer"</h1>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "footer", type_label: "base", description: "Base footer class" },
                    ClassEntry { name: "footer-title", type_label: "base part", description: "Title within footer" },
                    ClassEntry { name: "footer-center", type_label: "modifier", description: "Centers footer content" },
                ] />
            </section>

            <section>
                <ComponentPreview
                    title="Basic Footer"
                    code=r#"<Footer class="bg-base-200 p-10">
    <div>
        <FooterTitle>"Services"</FooterTitle>
        <Link href="/" class="link link-hover block">"Branding"</Link>
        <Link href="/" class="link link-hover block">"Design"</Link>
        <Link href="/" class="link link-hover block">"Marketing"</Link>
    </div>
    <div>
        <FooterTitle>"Company"</FooterTitle>
        <Link href="/" class="link link-hover block">"About us"</Link>
        <Link href="/" class="link link-hover block">"Contact"</Link>
    </div>
    <div>
        <FooterTitle>"Legal"</FooterTitle>
        <Link href="/" class="link link-hover block">"Terms of use"</Link>
        <Link href="/" class="link link-hover block">"Privacy policy"</Link>
    </div>
</Footer>"#
                >
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
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Centered Footer"
                    code=r#"<Footer center=true class="bg-base-200 p-10">
    <div class="text-center">
        <FooterTitle>"Company Name"</FooterTitle>
        <p class="max-w-md">"Providing reliable tech since 1992."</p>
    </div>
    <div>
        <div class="grid grid-flow-col gap-4 justify-center">
            <Link href="/" class="link link-hover">"Twitter"</Link>
            <Link href="/" class="link link-hover">"GitHub"</Link>
            <Link href="/" class="link link-hover">"Discord"</Link>
        </div>
    </div>
</Footer>"#
                >
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
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Horizontal Footer"
                    code=r#"<Footer horizontal=true class="bg-base-200 p-10">
    <div class="flex gap-4">
        <Link href="/" class="link link-hover">"About"</Link>
        <Link href="/" class="link link-hover">"Contact"</Link>
        <Link href="/" class="link link-hover">"Privacy"</Link>
        <Link href="/" class="link link-hover">"Terms"</Link>
    </div>
    <div>
        <p>"© 2024 Company Name. All rights reserved."</p>
    </div>
</Footer>"#
                >
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
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Vertical Footer"
                    code=r#"<Footer vertical=true class="bg-base-200 p-10">
    <div>
        <FooterTitle>"Categories"</FooterTitle>
        <Link href="/" class="link link-hover block">"Electronics"</Link>
        <Link href="/" class="link link-hover block">"Clothing"</Link>
        <Link href="/" class="link link-hover block">"Books"</Link>
    </div>
    <div>
        <FooterTitle>"Support"</FooterTitle>
        <Link href="/" class="link link-hover block">"Help Center"</Link>
        <Link href="/" class="link link-hover block">"Returns"</Link>
        <Link href="/" class="link link-hover block">"Shipping"</Link>
    </div>
    <div>
        <FooterTitle>"Account"</FooterTitle>
        <Link href="/" class="link link-hover block">"Login"</Link>
        <Link href="/" class="link link-hover block">"Register"</Link>
    </div>
</Footer>"#
                >
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
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Footer with Form"
                    code=r#"<Footer class="bg-base-200 p-10">
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
</Footer>"#
                >
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
                </ComponentPreview>
            </section>
        </div>
    }
}

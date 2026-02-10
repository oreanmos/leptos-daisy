use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn HeroPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Hero"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Hero"</h2>
                <Hero class="bg-base-200" min_height="400px".to_string()>
                    <HeroContent>
                        <div class="max-w-md">
                            <h1 class="text-5xl font-bold">"Hello there"</h1>
                            <p class="py-6">"Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."</p>
                            <Button color={Color::Primary}>"Get Started"</Button>
                        </div>
                    </HeroContent>
                </Hero>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Centered Hero"</h2>
                <Hero class="bg-base-200" center=true min_height="300px".to_string()>
                    <HeroContent center=true>
                        <div class="max-w-md text-center">
                            <h1 class="text-5xl font-bold">"Welcome"</h1>
                            <p class="py-6">"This hero is centered both horizontally and vertically."</p>
                            <div class="flex gap-2 justify-center">
                                <Button color={Color::Primary}>"Sign Up"</Button>
                                <Button variant={Variant::Outline}>"Learn More"</Button>
                            </div>
                        </div>
                    </HeroContent>
                </Hero>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Hero with Overlay"</h2>
                <Hero
                    overlay=true
                    center=true
                    class="bg-base-200"
                    min_height="400px".to_string()
                >
                    <img
                        src="https://picsum.photos/seed/hero/1200/600"
                        class="absolute inset-0 w-full h-full object-cover"
                        alt="Hero background"
                    />
                    <HeroContent center=true class="relative z-10">
                        <div class="max-w-md text-center text-neutral-content">
                            <h1 class="text-5xl font-bold">"Hero with Overlay"</h1>
                            <p class="py-6">"This hero has a background image with an overlay for better text readability."</p>
                            <Button color={Color::Primary}>"Explore"</Button>
                        </div>
                    </HeroContent>
                </Hero>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Hero with Image"</h2>
                <Hero class="bg-base-200" min_height="500px".to_string()>
                    <HeroContent>
                        <div class="flex flex-col lg:flex-row items-center gap-8">
                            <div class="flex-1">
                                <h1 class="text-5xl font-bold">"Build faster"</h1>
                                <p class="py-6">"Create stunning user interfaces with our comprehensive component library. Built with modern technologies and best practices."</p>
                                <div class="flex gap-2">
                                    <Button color={Color::Primary}>"Start Building"</Button>
                                    <Button variant={Variant::Ghost}>"View Docs"</Button>
                                </div>
                            </div>
                            <div class="flex-1">
                                <img
                                    src="https://picsum.photos/seed/product/500/400"
                                    class="rounded-lg shadow-2xl"
                                    alt="Product screenshot"
                                />
                            </div>
                        </div>
                    </HeroContent>
                </Hero>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Hero with Stats"</h2>
                <Hero class="bg-primary text-primary-content" center=true min_height="400px".to_string()>
                    <HeroContent center=true>
                        <div class="text-center">
                            <h1 class="text-5xl font-bold mb-8">"Trusted by developers"</h1>
                            <Stats class="bg-transparent text-primary-content">
                                <Stat>
                                    <StatValue>"50K+"</StatValue>
                                    <StatTitle>"Downloads"</StatTitle>
                                </Stat>
                                <Stat>
                                    <StatValue>"10K+"</StatValue>
                                    <StatTitle>"Stars"</StatTitle>
                                </Stat>
                                <Stat>
                                    <StatValue>"99%"</StatValue>
                                    <StatTitle>"Satisfaction"</StatTitle>
                                </Stat>
                            </Stats>
                        </div>
                    </HeroContent>
                </Hero>
            </section>
        </div>
    }
}

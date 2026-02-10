use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CarouselPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Carousel"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Carousel"</h2>
                <div class="h-64">
                    <Carousel>
                        <CarouselItem id="slide1" class="h-full">
                            <div class="h-full bg-primary flex items-center justify-center text-primary-content text-4xl font-bold">
                                "Slide 1"
                            </div>
                        </CarouselItem>
                        <CarouselItem id="slide2" class="h-full">
                            <div class="h-full bg-secondary flex items-center justify-center text-secondary-content text-4xl font-bold">
                                "Slide 2"
                            </div>
                        </CarouselItem>
                        <CarouselItem id="slide3" class="h-full">
                            <div class="h-full bg-accent flex items-center justify-center text-accent-content text-4xl font-bold">
                                "Slide 3"
                            </div>
                        </CarouselItem>
                    </Carousel>
                </div>
                <div class="flex justify-center gap-2 mt-4">
                    <a href="#slide1" class="btn btn-xs">"1"</a>
                    <a href="#slide2" class="btn btn-xs">"2"</a>
                    <a href="#slide3" class="btn btn-xs">"3"</a>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Snap Variants"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Snap Start"</span>
                        <div class="h-48">
                            <Carousel snap={CarouselSnap::Start}>
                                <CarouselItem class="w-1/2 h-full">
                                    <div class="h-full bg-primary flex items-center justify-center text-primary-content">
                                        "Item 1"
                                    </div>
                                </CarouselItem>
                                <CarouselItem class="w-1/2 h-full">
                                    <div class="h-full bg-secondary flex items-center justify-center text-secondary-content">
                                        "Item 2"
                                    </div>
                                </CarouselItem>
                                <CarouselItem class="w-1/2 h-full">
                                    <div class="h-full bg-accent flex items-center justify-center text-accent-content">
                                        "Item 3"
                                    </div>
                                </CarouselItem>
                            </Carousel>
                        </div>
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Snap Center"</span>
                        <div class="h-48">
                            <Carousel snap={CarouselSnap::Center}>
                                <CarouselItem class="w-1/2 h-full">
                                    <div class="h-full bg-info flex items-center justify-center text-info-content">
                                        "Item 1"
                                    </div>
                                </CarouselItem>
                                <CarouselItem class="w-1/2 h-full">
                                    <div class="h-full bg-success flex items-center justify-center text-success-content">
                                        "Item 2"
                                    </div>
                                </CarouselItem>
                                <CarouselItem class="w-1/2 h-full">
                                    <div class="h-full bg-warning flex items-center justify-center text-warning-content">
                                        "Item 3"
                                    </div>
                                </CarouselItem>
                            </Carousel>
                        </div>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical Carousel"</h2>
                <div class="h-64">
                    <Carousel orientation={CarouselOrientation::Vertical}>
                        <CarouselItem id="vslide1" class="h-full">
                            <div class="h-full bg-primary flex items-center justify-center text-primary-content text-2xl font-bold">
                                "Vertical 1"
                            </div>
                        </CarouselItem>
                        <CarouselItem id="vslide2" class="h-full">
                            <div class="h-full bg-secondary flex items-center justify-center text-secondary-content text-2xl font-bold">
                                "Vertical 2"
                            </div>
                        </CarouselItem>
                        <CarouselItem id="vslide3" class="h-full">
                            <div class="h-full bg-accent flex items-center justify-center text-accent-content text-2xl font-bold">
                                "Vertical 3"
                            </div>
                        </CarouselItem>
                    </Carousel>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Content"</h2>
                <div class="h-64">
                    <Carousel>
                        <CarouselItem id="content1" class="h-full p-4">
                            <Card class="h-full bg-base-100 shadow-xl">
                                <CardBody>
                                    <CardTitle>"Feature 1"</CardTitle>
                                    <p>"This is a carousel item with a card component inside."</p>
                                </CardBody>
                            </Card>
                        </CarouselItem>
                        <CarouselItem id="content2" class="h-full p-4">
                            <Card class="h-full bg-base-100 shadow-xl">
                                <CardBody>
                                    <CardTitle>"Feature 2"</CardTitle>
                                    <p>"Another carousel item with different content."</p>
                                </CardBody>
                            </Card>
                        </CarouselItem>
                        <CarouselItem id="content3" class="h-full p-4">
                            <Card class="h-full bg-base-100 shadow-xl">
                                <CardBody>
                                    <CardTitle>"Feature 3"</CardTitle>
                                    <p>"You can put any content inside carousel items."</p>
                                </CardBody>
                            </Card>
                        </CarouselItem>
                    </Carousel>
                </div>
                <div class="flex justify-center gap-2 mt-4">
                    <a href="#content1" class="btn btn-circle btn-sm">"1"</a>
                    <a href="#content2" class="btn btn-circle btn-sm">"2"</a>
                    <a href="#content3" class="btn btn-circle btn-sm">"3"</a>
                </div>
            </section>
        </div>
    }
}

use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CarouselPage() -> impl IntoView {
    view! {
        <div class="space-y-12">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Carousel"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Carousel components for displaying scrollable content with snap behavior."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "carousel", type_label: "base", description: "Container for carousel items with scroll snap" },
                    ClassEntry { name: "carousel-item", type_label: "base", description: "Individual item within the carousel" },
                    ClassEntry { name: "carousel-center", type_label: "modifier", description: "Snaps items to center of carousel" },
                    ClassEntry { name: "carousel-end", type_label: "modifier", description: "Snaps items to end of carousel" },
                    ClassEntry { name: "carousel-vertical", type_label: "modifier", description: "Vertical scrolling carousel layout" },
                ] />
            </section>

            <section>
                <ComponentPreview
                    title="Basic Carousel"
                    code=r##"<Carousel class="w-full">
    <CarouselItem id="slide1" class="h-full w-full">
        <div class="h-full w-full bg-primary flex items-center justify-center text-primary-content">
            "Slide 1"
        </div>
    </CarouselItem>
    <CarouselItem id="slide2" class="h-full w-full">
        <div class="h-full w-full bg-secondary flex items-center justify-center text-secondary-content">
            "Slide 2"
        </div>
    </CarouselItem>
    <CarouselItem id="slide3" class="h-full w-full">
        <div class="h-full w-full bg-accent flex items-center justify-center text-accent-content">
            "Slide 3"
        </div>
    </CarouselItem>
</Carousel>

<div class="flex justify-center gap-2">
    <a href="#slide1" class="btn btn-xs">"1"</a>
    <a href="#slide2" class="btn btn-xs">"2"</a>
    <a href="#slide3" class="btn btn-xs">"3"</a>
</div>"##
                >
                    <div class="h-64">
                         <Carousel class="w-full">
                            <CarouselItem id="slide1" class="h-full w-full">
                                <div class="h-full w-full bg-primary flex items-center justify-center text-primary-content text-4xl font-bold">
                                    "Slide 1"
                                </div>
                            </CarouselItem>
                            <CarouselItem id="slide2" class="h-full w-full">
                                <div class="h-full w-full bg-secondary flex items-center justify-center text-secondary-content text-4xl font-bold">
                                    "Slide 2"
                                </div>
                            </CarouselItem>
                            <CarouselItem id="slide3" class="h-full w-full">
                                <div class="h-full w-full bg-accent flex items-center justify-center text-accent-content text-4xl font-bold">
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
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With Content"
                    code=r##"<Carousel class="w-full">
    <CarouselItem id="content1" class="h-full p-4 w-full">
        <Card class="h-full w-full bg-base-100 shadow-xl">
            <CardBody>
                <CardTitle>"Feature 1"</CardTitle>
                <p>"This is a carousel item with a card component inside."</p>
            </CardBody>
        </Card>
    </CarouselItem>
    <CarouselItem id="content2" class="h-full p-4 w-full">
        <Card class="h-full w-full bg-base-100 shadow-xl">
            <CardBody>
                <CardTitle>"Feature 2"</CardTitle>
                <p>"Another carousel item with different content."</p>
            </CardBody>
        </Card>
    </CarouselItem>
</Carousel>"##
                >
                    <div class="h-64">
                        <Carousel class="w-full">
                            <CarouselItem id="content1" class="h-full p-4 w-full">
                                <Card class="h-full w-full bg-base-100 shadow-xl">
                                    <CardBody>
                                        <CardTitle>"Feature 1"</CardTitle>
                                        <p>"This is a carousel item with a card component inside."</p>
                                    </CardBody>
                                </Card>
                            </CarouselItem>
                            <CarouselItem id="content2" class="h-full p-4 w-full">
                                <Card class="h-full w-full bg-base-100 shadow-xl">
                                    <CardBody>
                                        <CardTitle>"Feature 2"</CardTitle>
                                        <p>"Another carousel item with different content."</p>
                                    </CardBody>
                                </Card>
                            </CarouselItem>
                            <CarouselItem id="content3" class="h-full p-4 w-full">
                                <Card class="h-full w-full bg-base-100 shadow-xl">
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
                </ComponentPreview>
            </section>

             <section>
                <ComponentPreview
                    title="Snap Variants"
                    code=r#"<Carousel snap=CarouselSnap::Start class="w-full">
    <CarouselItem class="w-1/2 h-full">
        <div class="h-full w-full bg-primary flex items-center justify-center">
            "Item 1"
        </div>
    </CarouselItem>
    <CarouselItem class="w-1/2 h-full">
        <div class="h-full w-full bg-secondary flex items-center justify-center">
            "Item 2"
        </div>
    </CarouselItem>
</Carousel>

<Carousel snap=CarouselSnap::Center class="w-full">
    <CarouselItem class="w-1/2 h-full">
        <div class="h-full w-full bg-info flex items-center justify-center">
            "Item 1"
        </div>
    </CarouselItem>
    <CarouselItem class="w-1/2 h-full">
        <div class="h-full w-full bg-success flex items-center justify-center">
            "Item 2"
        </div>
    </CarouselItem>
</Carousel>"#
                >
                    <div class="space-y-4">
                        <div>
                            <span class="text-sm text-base-content/70">"Snap Start"</span>
                            <div class="h-48">
                                <Carousel snap=CarouselSnap::Start class="w-full rounded-box">
                                     <CarouselItem class="w-1/2 h-full">
                                        <div class="h-full w-full bg-primary flex items-center justify-center text-primary-content">
                                            "Item 1"
                                        </div>
                                    </CarouselItem>
                                    <CarouselItem class="w-1/2 h-full">
                                        <div class="h-full w-full bg-secondary flex items-center justify-center text-secondary-content">
                                            "Item 2"
                                        </div>
                                    </CarouselItem>
                                    <CarouselItem class="w-1/2 h-full">
                                        <div class="h-full w-full bg-accent flex items-center justify-center text-accent-content">
                                            "Item 3"
                                        </div>
                                    </CarouselItem>
                                </Carousel>
                            </div>
                        </div>

                         <div>
                            <span class="text-sm text-base-content/70">"Snap Center"</span>
                            <div class="h-48">
                                <Carousel snap=CarouselSnap::Center class="w-full space-x-4 bg-neutral rounded-box p-4">
                                     <CarouselItem class="w-1/2 h-full rounded-box">
                                        <div class="h-full w-full bg-info flex items-center justify-center text-info-content rounded-box">
                                            "Item 1"
                                        </div>
                                    </CarouselItem>
                                    <CarouselItem class="w-1/2 h-full rounded-box">
                                        <div class="h-full w-full bg-success flex items-center justify-center text-success-content rounded-box">
                                            "Item 2"
                                        </div>
                                    </CarouselItem>
                                    <CarouselItem class="w-1/2 h-full rounded-box">
                                        <div class="h-full w-full bg-warning flex items-center justify-center text-warning-content rounded-box">
                                            "Item 3"
                                        </div>
                                    </CarouselItem>
                                </Carousel>
                            </div>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Vertical Carousel"
                    code=r##"<Carousel orientation=CarouselOrientation::Vertical class="h-64">
    <CarouselItem id="vslide1" class="h-full w-full">
        <div class="h-full w-full bg-primary flex items-center justify-center text-primary-content">
            "Vertical 1"
        </div>
    </CarouselItem>
    <CarouselItem id="vslide2" class="h-full w-full">
        <div class="h-full w-full bg-secondary flex items-center justify-center text-secondary-content">
            "Vertical 2"
        </div>
    </CarouselItem>
    <CarouselItem id="vslide3" class="h-full w-full">
        <div class="h-full w-full bg-accent flex items-center justify-center text-accent-content">
            "Vertical 3"
        </div>
    </CarouselItem>
</Carousel>"##
                >
                     <div class="h-64">
                         <Carousel orientation=CarouselOrientation::Vertical class="h-64">
                            <CarouselItem id="vslide1" class="h-full w-full">
                                <div class="h-full w-full bg-primary flex items-center justify-center text-primary-content text-2xl font-bold">
                                    "Vertical 1"
                                </div>
                            </CarouselItem>
                            <CarouselItem id="vslide2" class="h-full w-full">
                                <div class="h-full w-full bg-secondary flex items-center justify-center text-secondary-content text-2xl font-bold">
                                    "Vertical 2"
                                </div>
                            </CarouselItem>
                            <CarouselItem id="vslide3" class="h-full w-full">
                                <div class="h-full w-full bg-accent flex items-center justify-center text-accent-content text-2xl font-bold">
                                    "Vertical 3"
                                </div>
                            </CarouselItem>
                        </Carousel>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MaskPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Mask"</h1>

            <section>
                <ComponentPreview
                    title="Geometric Shapes"
                    code=r#"<Mask shape={MaskShape::Squircle}><img src="..." /></Mask>
<Mask shape={MaskShape::Circle}><img src="..." /></Mask>
<Mask shape={MaskShape::Triangle}><img src="..." /></Mask>
<Mask shape={MaskShape::Square}><img src="..." /></Mask>
<Mask shape={MaskShape::Diamond}><img src="..." /></Mask>"#
                >
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Squircle}>
                                <img src="https://picsum.photos/200/200?random=1" alt="Squircle" />
                            </Mask>
                            <span class="text-sm">"Squircle"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Circle}>
                                <img src="https://picsum.photos/200/200?random=2" alt="Circle" />
                            </Mask>
                            <span class="text-sm">"Circle"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Triangle}>
                                <img src="https://picsum.photos/200/200?random=3" alt="Triangle" />
                            </Mask>
                            <span class="text-sm">"Triangle"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Triangle2}>
                                <img src="https://picsum.photos/200/200?random=4" alt="Triangle 2" />
                            </Mask>
                            <span class="text-sm">"Triangle 2"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Triangle3}>
                                <img src="https://picsum.photos/200/200?random=5" alt="Triangle 3" />
                            </Mask>
                            <span class="text-sm">"Triangle 3"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Triangle4}>
                                <img src="https://picsum.photos/200/200?random=6" alt="Triangle 4" />
                            </Mask>
                            <span class="text-sm">"Triangle 4"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Square}>
                                <img src="https://picsum.photos/200/200?random=7" alt="Square" />
                            </Mask>
                            <span class="text-sm">"Square"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Diamond}>
                                <img src="https://picsum.photos/200/200?random=8" alt="Diamond" />
                            </Mask>
                            <span class="text-sm">"Diamond"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Stars"
                    code=r#"<Mask shape={MaskShape::Star}><img src="..." /></Mask>
    <Mask shape={MaskShape::Star2}><img src="..." /></Mask>"#
                >
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Star}>
                                <img src="https://picsum.photos/200/200?random=9" alt="Star" />
                            </Mask>
                            <span class="text-sm">"Star"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Star2}>
                                <img src="https://picsum.photos/200/200?random=10" alt="Star 2" />
                            </Mask>
                            <span class="text-sm">"Star 2"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Hexagons"
                    code=r#"<Mask shape={MaskShape::Hexagon}><img src="..." /></Mask>
    <Mask shape={MaskShape::Hexagon2}><img src="..." /></Mask>"#
                >
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Hexagon}>
                                <img src="https://picsum.photos/200/200?random=11" alt="Hexagon" />
                            </Mask>
                            <span class="text-sm">"Hexagon"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Hexagon2}>
                                <img src="https://picsum.photos/200/200?random=12" alt="Hexagon 2" />
                            </Mask>
                            <span class="text-sm">"Hexagon 2"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Polygons"
                    code=r#"<Mask shape={MaskShape::Pentagon}><img src="..." /></Mask>
<Mask shape={MaskShape::Octagon}><img src="..." /></Mask>
<Mask shape={MaskShape::Decagon}><img src="..." /></Mask>
<Mask shape={MaskShape::Parallelogram}><img src="..." /></Mask>
<Mask shape={MaskShape::Trapezoid}><img src="..." /></Mask>"#
                >
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Pentagon}>
                                <img src="https://picsum.photos/200/200?random=13" alt="Pentagon" />
                            </Mask>
                            <span class="text-sm">"Pentagon"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Octagon}>
                                <img src="https://picsum.photos/200/200?random=14" alt="Octagon" />
                            </Mask>
                            <span class="text-sm">"Octagon"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Decagon}>
                                <img src="https://picsum.photos/200/200?random=15" alt="Decagon" />
                            </Mask>
                            <span class="text-sm">"Decagon"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Parallelogram}>
                                <img src="https://picsum.photos/200/200?random=16" alt="Parallelogram" />
                            </Mask>
                            <span class="text-sm">"Parallelogram"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Parallelogram2}>
                                <img src="https://picsum.photos/200/200?random=17" alt="Parallelogram 2" />
                            </Mask>
                            <span class="text-sm">"Parallelogram 2"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Trapezoid}>
                                <img src="https://picsum.photos/200/200?random=18" alt="Trapezoid" />
                            </Mask>
                            <span class="text-sm">"Trapezoid"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Trapezoid2}>
                                <img src="https://picsum.photos/200/200?random=19" alt="Trapezoid 2" />
                            </Mask>
                            <span class="text-sm">"Trapezoid 2"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Arch & Heart"
                    code=r#"<Mask shape={MaskShape::Arch}><img src="..." /></Mask>
<Mask shape={MaskShape::Arch2}><img src="..." /></Mask>
<Mask shape={MaskShape::Heart}><img src="..." /></Mask>
<Mask shape={MaskShape::Heart2}><img src="..." /></Mask>"#
                >
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Arch}>
                                <img src="https://picsum.photos/200/200?random=20" alt="Arch" />
                            </Mask>
                            <span class="text-sm">"Arch"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Arch2}>
                                <img src="https://picsum.photos/200/200?random=21" alt="Arch 2" />
                            </Mask>
                            <span class="text-sm">"Arch 2"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Heart}>
                                <img src="https://picsum.photos/200/200?random=22" alt="Heart" />
                            </Mask>
                            <span class="text-sm">"Heart"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Mask shape={MaskShape::Heart2}>
                                <img src="https://picsum.photos/200/200?random=23" alt="Heart 2" />
                            </Mask>
                            <span class="text-sm">"Heart 2"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With Avatars"
                    code=r#"<Mask shape={MaskShape::Squircle} class="w-16 h-16">
    <Avatar class="w-full h-full"><img src="..." /></Avatar>
</Mask>
<Mask shape={MaskShape::Circle} class="w-16 h-16">
    <Avatar class="w-full h-full"><img src="..." /></Avatar>
</Mask>"#
                >
                    <div class="flex flex-wrap gap-4 items-center">
                        <Mask shape={MaskShape::Squircle} class="w-16 h-16">
                            <Avatar class="w-full h-full">
                                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=1" alt="User 1" />
                            </Avatar>
                        </Mask>
                        <Mask shape={MaskShape::Circle} class="w-16 h-16">
                            <Avatar class="w-full h-full">
                                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=2" alt="User 2" />
                            </Avatar>
                        </Mask>
                        <Mask shape={MaskShape::Hexagon} class="w-16 h-16">
                            <Avatar class="w-full h-full">
                                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=3" alt="User 3" />
                            </Avatar>
                        </Mask>
                        <Mask shape={MaskShape::Diamond} class="w-16 h-16">
                            <Avatar class="w-full h-full">
                                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=4" alt="User 4" />
                            </Avatar>
                        </Mask>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With Other Content"
                    code=r#"<Mask shape={MaskShape::Circle}>
    <div class="w-32 h-32 bg-primary flex items-center justify-center">"A"</div>
</Mask>
<Mask shape={MaskShape::Triangle}>
    <div class="w-32 h-32 bg-secondary flex items-center justify-center">"B"</div>
</Mask>
<Mask shape={MaskShape::Star}>
    <div class="w-32 h-32 bg-accent flex items-center justify-center">"C"</div>
</Mask>"#
                >
                    <div class="flex flex-wrap gap-4">
                        <Mask shape={MaskShape::Circle}>
                            <div class="w-32 h-32 bg-primary flex items-center justify-center text-primary-content font-bold">
                                "A"
                            </div>
                        </Mask>
                        <Mask shape={MaskShape::Triangle}>
                            <div class="w-32 h-32 bg-secondary flex items-center justify-center text-secondary-content font-bold">
                                "B"
                            </div>
                        </Mask>
                        <Mask shape={MaskShape::Star}>
                            <div class="w-32 h-32 bg-accent flex items-center justify-center text-accent-content font-bold">
                                "C"
                            </div>
                        </Mask>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

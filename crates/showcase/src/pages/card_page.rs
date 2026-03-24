use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CardPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Card"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Cards are used to group and display content in a way that is easily readable."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "card", type_label: "base", description: "Container for card content" },
                    ClassEntry { name: "card-body", type_label: "base", description: "Card body with padding and spacing" },
                    ClassEntry { name: "card-title", type_label: "base", description: "Title text style for card" },
                    ClassEntry { name: "card-actions", type_label: "base", description: "Container for card action buttons" },
                    ClassEntry { name: "card-border", type_label: "modifier", description: "Adds border to card" },
                    ClassEntry { name: "card-sm", type_label: "modifier", description: "Reduces card padding" },
                    ClassEntry { name: "card-side", type_label: "modifier", description: "Horizontal card layout with side image" },
                    ClassEntry { name: "image-full", type_label: "modifier", description: "Makes image cover the entire card as background" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic Card"
                    code=r#"<Card class="bg-base-100 shadow-xl">
    <CardBody>
        <CardTitle>"Card Title"</CardTitle>
        <p>"A basic card with body and title."</p>
        <CardActions>
            <Button color={Color::Primary}>"Action"</Button>
        </CardActions>
    </CardBody>
</Card>

<Card variant=CardVariant::Bordered class="bg-base-100">
    <CardBody>
        <CardTitle>"Bordered Card"</CardTitle>
        <p>"A card with border."</p>
    </CardBody>
</Card>"#
                >
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <Card class="bg-base-100 shadow-xl">
                            <CardBody>
                                <CardTitle>"Card Title"</CardTitle>
                                <p>"A basic card with body and title."</p>
                                <CardActions>
                                    <Button color={Color::Primary}>"Action"</Button>
                                </CardActions>
                            </CardBody>
                        </Card>

                        <Card variant=CardVariant::Bordered class="bg-base-100">
                            <CardBody>
                                <CardTitle>"Bordered Card"</CardTitle>
                                <p>"A card with border."</p>
                            </CardBody>
                        </Card>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Compact Card"
                    code=r#"<Card variant=CardVariant::Compact class="bg-base-100 shadow-xl w-96">
    <CardBody>
        <CardTitle>"Compact"</CardTitle>
        <p>"Less padding."</p>
        <CardActions class="justify-end">
            <Button color={Color::Primary}>"Buy Now"</Button>
        </CardActions>
    </CardBody>
</Card>"#
                >
                    <Card variant=CardVariant::Compact class="bg-base-100 shadow-xl w-96">
                        <CardBody>
                            <CardTitle>"Compact"</CardTitle>
                            <p>"Less padding."</p>
                            <CardActions class="justify-end">
                                <Button color={Color::Primary}>"Buy Now"</Button>
                            </CardActions>
                        </CardBody>
                    </Card>
                </ComponentPreview>
            </section>

             <section class="space-y-4">
                <ComponentPreview
                    title="Card with Image"
                    code=r#"<Card class="bg-base-100 shadow-xl w-96">
    <figure>
        <img
            src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
            alt="Shoes"
        />
    </figure>
    <CardBody>
        <CardTitle>
            "Shoes!"
            <Badge color={Color::Secondary} class="ml-2">"NEW"</Badge>
        </CardTitle>
        <p>"If a dog chews shoes whose shoes does he choose?"</p>
        <CardActions class="justify-end">
            <Badge variant={Variant::Outline}>"Fashion"</Badge>
            <Badge variant={Variant::Outline}>"Products"</Badge>
        </CardActions>
    </CardBody>
</Card>"#
                >
                    <Card class="bg-base-100 shadow-xl w-96">
                        <figure>
                            <img src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp" alt="Shoes" />
                        </figure>
                        <CardBody>
                            <CardTitle>
                                "Shoes!"
                                <Badge color={Color::Secondary} class="ml-2">"NEW"</Badge>
                            </CardTitle>
                            <p>"If a dog chews shoes whose shoes does he choose?"</p>
                            <CardActions class="justify-end">
                                <Badge variant={Variant::Outline}>"Fashion"</Badge>
                                <Badge variant={Variant::Outline}>"Products"</Badge>
                            </CardActions>
                        </CardBody>
                    </Card>
                </ComponentPreview>
            </section>
        </div>
    }
}

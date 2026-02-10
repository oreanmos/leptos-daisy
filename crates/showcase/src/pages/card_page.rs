use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CardPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Card"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Card"</h2>
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

                    <Card bordered=true class="bg-base-100">
                        <CardBody>
                            <CardTitle>"Bordered Card"</CardTitle>
                            <p>"A card with border."</p>
                        </CardBody>
                    </Card>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Compact Card"</h2>
                <Card compact=true class="bg-base-100 shadow-xl">
                    <CardBody>
                        <CardTitle>"Compact"</CardTitle>
                        <p>"Less padding."</p>
                    </CardBody>
                </Card>
            </section>
        </div>
    }
}

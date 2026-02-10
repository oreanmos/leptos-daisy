use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn AlertPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Alert"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Variants"</h2>
                <div class="space-y-2">
                    <Alert variant={AlertVariant::Info}>"This is an info alert."</Alert>
                    <Alert variant={AlertVariant::Success}>"This is a success alert."</Alert>
                    <Alert variant={AlertVariant::Warning}>"This is a warning alert."</Alert>
                    <Alert variant={AlertVariant::Error}>"This is an error alert."</Alert>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Styles"</h2>
                <div class="space-y-2">
                    <Alert variant={AlertVariant::Info} style={AlertStyle::Outline}>"Outline info alert"</Alert>
                    <Alert variant={AlertVariant::Success} style={AlertStyle::Dash}>"Dash success alert"</Alert>
                    <Alert variant={AlertVariant::Warning} style={AlertStyle::Soft}>"Soft warning alert"</Alert>
                </div>
            </section>
        </div>
    }
}

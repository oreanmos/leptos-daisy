use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ModalPage() -> impl IntoView {
    let (show, set_show) = signal(false);

    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Modal"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Modal"</h2>
                <Button color={Color::Primary} on:click=move |_| set_show.set(true)>"Open Modal"</Button>
                <Modal open={show.get()}>
                    <ModalBox>
                        <ModalTitle>"Hello!"</ModalTitle>
                        <p class="py-4">"This is a modal dialog."</p>
                        <ModalActions>
                            <Button on:click=move |_| set_show.set(false)>"Close"</Button>
                        </ModalActions>
                    </ModalBox>
                    <ModalBackdrop />
                </Modal>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"HTML Dialog Method"</h2>
                <p class="text-sm opacity-70">"Use the native dialog element with daisyUI styling. Click the button below."</p>
                <Button color={Color::Secondary} class="mt-2">"Open (use dialog.showModal())"</Button>
            </section>
        </div>
    }
}

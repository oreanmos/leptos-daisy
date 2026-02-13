use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ModalPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Modal"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Dialog-based modal patterns including forced state, placement, and action layouts."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Dialog pattern (daisyUI)"
                    description="This mirrors the docs approach using native showModal() for interactive demo behavior."
                    code=r#"<button class="btn" onclick="my_modal.showModal()">"Open Modal"</button>
<Modal id="my_modal" aria_label="Example modal">
    <ModalBox>
        <h3 class="font-bold text-lg">"Hello!"</h3>
        <p class="py-4">"Press ESC key or click outside to close"</p>
        <ModalAction>
            <form method="dialog">
                <button class="btn">"Close"</button>
            </form>
        </ModalAction>
    </ModalBox>
    <ModalBackdrop />
</Modal>"#
                >
                    <div class="h-48 flex items-center justify-center">
                        <button class="btn" onclick="my_modal.showModal()">"Open Modal"</button>
                        <Modal id="my_modal" aria_label="Example modal">
                            <ModalBox>
                                <h3 class="font-bold text-lg">"Hello!"</h3>
                                <p class="py-4">"Press ESC key or click outside to close"</p>
                                <ModalAction>
                                    <form method="dialog"><button class="btn">"Close"</button></form>
                                </ModalAction>
                            </ModalBox>
                            <ModalBackdrop />
                        </Modal>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Forced open/close states"
                    code=r#"<Modal state={ModalState::Open}>
    <ModalBox class="max-w-md">
        <h3 class="font-bold text-lg">"Forced open"</h3>
        <p class="py-3 text-sm text-base-content/70">
            "`ModalState::Open` applies `modal-open` for static previews."
        </p>
        <ModalAction>
            <button class="btn btn-sm">"Action"</button>
        </ModalAction>
    </ModalBox>
</Modal>

<Modal state={ModalState::Close}>
    <ModalBox class="max-w-md">
        <h3 class="font-bold text-lg">"Forced closed"</h3>
        <p class="py-3 text-sm text-base-content/70">
            "`ModalState::Close` applies `modal-close` for explicit hidden state."
        </p>
    </ModalBox>
</Modal>"#
                    description="ModalState::Open applies modal-open for static previews."
                >
                    <div class="grid gap-4 lg:grid-cols-2">
                        <Modal state={ModalState::Open} class="!static !inset-auto !grid !visible !opacity-100 !pointer-events-auto">
                            <ModalBox class="max-w-md">
                                <h3 class="font-bold text-lg">"Forced open"</h3>
                                <p class="py-3 text-sm text-base-content/70">
                                    "`ModalState::Open` applies `modal-open` for static previews and documentation."
                                </p>
                                <ModalAction>
                                    <button class="btn btn-sm">"Action"</button>
                                </ModalAction>
                            </ModalBox>
                        </Modal>

                       <Modal state={ModalState::Close} class="!static !inset-auto !grid">
                            <ModalBox class="max-w-md">
                                <h3 class="font-bold text-lg">"Forced closed"</h3>
                                <p class="py-3 text-sm text-base-content/70">
                                    "`ModalState::Close` applies `modal-close` for explicit hidden state styling."
                                </p>
                            </ModalBox>
                        </Modal>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Positions"
                    code=r#"<button class="btn btn-outline" onclick="modal_top.showModal()">"Top"</button>
<Modal id="modal_top" position=ModalPosition::Top aria_label="Top modal">
    <ModalBox>
        <h3 class="font-bold text-lg">"Top Modal"</h3>
        <p class="py-3">"Placed with `modal-top`."</p>
        <ModalAction>
            <form method="dialog">
                <button class="btn">"Close"</button>
            </form>
        </ModalAction>
    </ModalBox>
    <ModalBackdrop />
</Modal>

<button class="btn btn-outline" onclick="modal_center.showModal()">"Center"</button>
<Modal id="modal_center" position=ModalPosition::Center aria_label="Center modal">
    <ModalBox>
        <h3 class="font-bold text-lg">"Center Modal"</h3>
        <p class="py-3">"Default centered placement."</p>
        <ModalAction>
            <form method="dialog">
                <button class="btn">"Close"</button>
            </form>
        </ModalAction>
    </ModalBox>
    <ModalBackdrop />
</Modal>

<button class="btn btn-outline" onclick="modal_bottom.showModal()">"Bottom"</button>
<Modal id="modal_bottom" position=ModalPosition::Bottom aria_label="Bottom modal">
    <ModalBox>
        <h3 class="font-bold text-lg">"Bottom Modal"</h3>
        <p class="py-3">"Placed with `modal-bottom`."</p>
        <ModalAction>
            <form method="dialog">
                <button class="btn">"Close"</button>
            </form>
        </ModalAction>
    </ModalBox>
    <ModalBackdrop />
</Modal>"#
                >
                    <div class="h-48 flex flex-wrap gap-2 items-center justify-center">
                        <button class="btn btn-outline" onclick="modal_top.showModal()">"Top"</button>
                        <button class="btn btn-outline" onclick="modal_center.showModal()">"Center"</button>
                        <button class="btn btn-outline" onclick="modal_bottom.showModal()">"Bottom"</button>
                    </div>

                    <Modal id="modal_top" position=ModalPosition::Top aria_label="Top modal">
                        <ModalBox>
                            <h3 class="font-bold text-lg">"Top Modal"</h3>
                            <p class="py-3">"Placed with `modal-top`."</p>
                            <ModalAction>
                                <form method="dialog"><button class="btn">"Close"</button></form>
                            </ModalAction>
                        </ModalBox>
                        <ModalBackdrop />
                    </Modal>

                    <Modal id="modal_center" position=ModalPosition::Center aria_label="Center modal">
                        <ModalBox>
                            <h3 class="font-bold text-lg">"Center Modal"</h3>
                            <p class="py-3">"Default centered placement."</p>
                            <ModalAction>
                                <form method="dialog"><button class="btn">"Close"</button></form>
                            </ModalAction>
                        </ModalBox>
                        <ModalBackdrop />
                    </Modal>

                    <Modal id="modal_bottom" position=ModalPosition::Bottom aria_label="Bottom modal">
                        <ModalBox>
                            <h3 class="font-bold text-lg">"Bottom Modal"</h3>
                            <p class="py-3">"Placed with `modal-bottom`."</p>
                            <ModalAction>
                                <form method="dialog"><button class="btn">"Close"</button></form>
                            </ModalAction>
                        </ModalBox>
                        <ModalBackdrop />
                    </Modal>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Action layouts"
                    code=r#"<Modal state={ModalState::Open}>
    <ModalBox class="max-w-sm">
        <h3 class="font-bold text-lg">"Confirm action"</h3>
        <p class="py-3">"Do you want to continue?"</p>
        <ModalAction>
            <button class="btn btn-ghost btn-sm">"Cancel"</button>
            <button class="btn btn-primary btn-sm">"Confirm"</button>
        </ModalAction>
    </ModalBox>
</Modal>

<Modal state={ModalState::Open}>
    <ModalBox class="max-w-sm">
        <h3 class="font-bold text-lg">"Info modal"</h3>
        <p class="py-3">"Example of a single-action footer."</p>
        <ModalAction>
            <button class="btn btn-sm">"Got it"</button>
        </ModalAction>
    </ModalBox>
</Modal>"#
                >
                    <div class="grid gap-4 md:grid-cols-2">
                        <Modal state={ModalState::Open} class="!static !inset-auto !grid !visible !opacity-100 !pointer-events-auto">
                            <ModalBox class="max-w-sm">
                                <h3 class="font-bold text-lg">"Confirm action"</h3>
                                <p class="py-3">"Do you want to continue?"</p>
                                <ModalAction>
                                    <button class="btn btn-ghost btn-sm">"Cancel"</button>
                                    <button class="btn btn-primary btn-sm">"Confirm"</button>
                                </ModalAction>
                            </ModalBox>
                        </Modal>

                         <Modal state={ModalState::Open} class="!static !inset-auto !grid !visible !opacity-100 !pointer-events-auto">
                            <ModalBox class="max-w-sm">
                                <h3 class="font-bold text-lg">"Info modal"</h3>
                                <p class="py-3">"Example of a single-action footer."</p>
                                <ModalAction>
                                    <button class="btn btn-sm">"Got it"</button>
                                </ModalAction>
                            </ModalBox>
                        </Modal>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

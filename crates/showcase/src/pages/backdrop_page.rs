use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn BackdropPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Backdrop"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A backdrop is used to dim the content behind modals and drawers, providing visual focus on the overlay content."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Modal with Backdrop"
                    code=r#"<Modal id="my-modal" open=true>
    <ModalBox>
        <h3 class="font-bold text-lg">"Modal Title"</h3>
        <p class="py-4">"Modal content here."</p>
        <ModalAction>
            <Button>"Close"</Button>
        </ModalAction>
    </ModalBox>
</Modal>"#
                >
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <Card>
                            <CardBody>
                                <h3 class="card-title">"Modal Usage"</h3>
                                <p>"Backdrops are automatically included with Modal components. They provide visual focus on the modal content."</p>
                            </CardBody>
                        </Card>
                        <Card>
                            <CardBody>
                                <h3 class="card-title">"Drawer Usage"</h3>
                                <p>"Drawers can optionally include backdrops. When enabled, clicking the backdrop closes the drawer."</p>
                            </CardBody>
                        </Card>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Visual Example"
                    code=r#"<div class="relative h-64 bg-base-200 rounded-lg overflow-hidden">
    <div class="absolute inset-0 flex items-center justify-center">
        <p class="text-base-content/50">"Content behind backdrop"</p>
    </div>
    <div class="absolute inset-0 bg-black/50 flex items-center justify-center">
        <div class="bg-base-100 p-6 rounded-lg shadow-xl">
            <h4 class="font-bold mb-2">"Modal Content"</h4>
            <p class="text-sm">"The dark overlay is the backdrop."</p>
        </div>
    </div>
</div>"#
                >
                    <div class="relative h-64 bg-base-200 rounded-lg overflow-hidden">
                        <div class="absolute inset-0 flex items-center justify-center">
                            <p class="text-base-content/50">"Content behind backdrop"</p>
                        </div>
                        <div class="absolute inset-0 bg-black/50 flex items-center justify-center">
                            <div class="bg-base-100 p-6 rounded-lg shadow-xl max-w-sm">
                                <h4 class="font-bold mb-2">"Modal Content"</h4>
                                <p class="text-sm">"The dark overlay is the backdrop. It dims the content underneath."</p>
                            </div>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Modal Example"
                    code=r#"<Modal id="backdrop-modal" open=true>
    <ModalBox>
        <h3 class="font-bold text-lg">"Modal with Backdrop"</h3>
        <p class="py-4">"This modal has a backdrop that dims the content behind it."</p>
        <ModalAction>
            <Button>"Close"</Button>
        </ModalAction>
    </ModalBox>
</Modal>"#
                >
                    <Modal id="backdrop-modal" open=true>
                        <ModalBox>
                            <h3 class="font-bold text-lg">"Modal with Backdrop"</h3>
                            <p class="py-4">"This modal has a backdrop that dims the content behind it."</p>
                            <ModalAction>
                                <Button>"Close"</Button>
                            </ModalAction>
                        </ModalBox>
                    </Modal>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Drawer with Backdrop"
                    code=r#"<Drawer id="drawer-backdrop-demo">
    <DrawerContent>
        <div class="p-8 bg-base-200 h-64">
            <label for="drawer-backdrop-demo" class="btn btn-primary">
                "Open Drawer"
            </label>
        </div>
    </DrawerContent>
    <DrawerSide>
        <DrawerOverlay drawer_id="drawer-backdrop-demo" />
        <ul class="menu p-4 w-80 min-h-full bg-base-200">
            <li><a>"Home"</a></li>
            <li><a>"About"</a></li>
            <li><a>"Services"</a></li>
            <li><a>"Contact"</a></li>
        </ul>
    </DrawerSide>
</Drawer>"#
                >
                    <Drawer id="drawer-backdrop-demo">
                        <DrawerContent>
                            <div class="p-8 bg-base-200 h-64">
                                <label for="drawer-backdrop-demo" class="btn btn-primary drawer-button">
                                    "Open Drawer"
                                </label>
                            </div>
                        </DrawerContent>
                        <DrawerSide>
                            <DrawerOverlay drawer_id="drawer-backdrop-demo" />
                            <ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
                                <li><a>"Home"</a></li>
                                <li><a>"About"</a></li>
                                <li><a>"Services"</a></li>
                                <li><a>"Contact"</a></li>
                            </ul>
                        </DrawerSide>
                    </Drawer>
                </ComponentPreview>
            </section>
        </div>
    }
}

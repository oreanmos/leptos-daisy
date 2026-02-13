use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MockupPhonePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Mockup Phone"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Phone device mockup for displaying mobile interface previews."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "mockup-phone", type_label: "base", description: "Base class for phone mockup container" },
                ] />
            </section>

            <section>
                <ComponentPreview
                    title="Basic Phone"
                    code=r#"<MockupPhone>
    <div class="artboard artboard-demo phone-1 bg-base-200">
        <div class="flex flex-col items-center justify-center h-full">
            <h3 class="text-xl font-bold">"Hello!"</h3>
            <p class="text-sm">"Mobile view"</p>
        </div>
    </div>
</MockupPhone>"#
                >
                    <div class="flex justify-center">
                        <MockupPhone>
                            <div class="artboard artboard-demo phone-1 bg-base-200">
                                <div class="flex flex-col items-center justify-center h-full">
                                    <h3 class="text-xl font-bold">"Hello!"</h3>
                                    <p class="text-sm text-base-content/70">"Mobile view"</p>
                                </div>
                            </div>
                        </MockupPhone>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With App Interface"
                    code=r#"<MockupPhone>
    <div class="artboard artboard-demo phone-1 bg-base-100">
        <div class="navbar bg-primary text-primary-content">
            <div class="flex-1"><span class="text-lg font-bold">"MyApp"</span></div>
        </div>
        <div class="flex-1 p-4">
            <Card><CardBody><h4 class="card-title text-sm">"Welcome!"</h4></CardBody></Card>
        </div>
        <div class="btm-nav btm-nav-sm">...</div>
    </div>
</MockupPhone>"#
                >
                    <div class="flex justify-center">
                        <MockupPhone>
                            <div class="artboard artboard-demo phone-1 bg-base-100">
                                <div class="flex flex-col h-full">
                                    <div class="navbar bg-primary text-primary-content">
                                        <div class="flex-1">
                                            <span class="text-lg font-bold">"MyApp"</span>
                                        </div>
                                        <div class="flex-none">
                                            <Button variant={Variant::Ghost} class="btn-circle btn-sm">
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                                                </svg>
                                            </Button>
                                        </div>
                                    </div>
                                    <div class="flex-1 p-4 space-y-4 overflow-auto">
                                        <Card>
                                            <CardBody>
                                                <h4 class="card-title text-sm">"Welcome!"</h4>
                                                <p class="text-xs">"This is a mobile app interface."</p>
                                            </CardBody>
                                        </Card>
                                        <div class="grid grid-cols-2 gap-2">
                                            <Button size={Size::Small} color={Color::Primary}>"Action 1"</Button>
                                            <Button size={Size::Small} variant={Variant::Outline}>"Action 2"</Button>
                                        </div>
                                    </div>
                                    <div class="btm-nav btm-nav-sm">
                                        <button class="active">
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
                                            </svg>
                                        </button>
                                        <button>
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
                                            </svg>
                                        </button>
                                        <button>
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" />
                                            </svg>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </MockupPhone>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Chat Interface"
                    code=r#"<MockupPhone>
    <div class="artboard artboard-demo phone-1 bg-base-200">
        <div class="navbar bg-base-100 shadow-sm">...</div>
        <div class="flex-1 p-4 space-y-4">
            <Chat position={ChatPosition::Start}>
                <ChatHeader>"Alice"</ChatHeader>
                <ChatBubble>"Hey! How are you?"</ChatBubble>
            </Chat>
            <Chat position={ChatPosition::End}>
                <ChatBubble color={Color::Primary}>"I'm good!"</ChatBubble>
            </Chat>
        </div>
    </div>
</MockupPhone>"#
                >
                    <div class="flex justify-center">
                        <MockupPhone>
                            <div class="artboard artboard-demo phone-1 bg-base-200">
                                <div class="flex flex-col h-full">
                                    <div class="navbar bg-base-100 shadow-sm">
                                        <div class="navbar-start">
                                            <Button variant={Variant::Ghost} class="btn-circle btn-sm">
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5L8.25 12l7.5-7.5" />
                                                </svg>
                                            </Button>
                                        </div>
                                        <div class="navbar-center">
                                            <span class="font-bold">"Alice"</span>
                                        </div>
                                        <div class="navbar-end">
                                            <Avatar class="w-8 h-10" online=true>
                                                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Alice" alt="Alice" />
                                            </Avatar>
                                        </div>
                                    </div>
                                    <div class="flex-1 p-4 space-y-4 overflow-auto">
                                        <Chat position={ChatPosition::Start}>
                                            <ChatHeader>"Alice"</ChatHeader>
                                            <ChatBubble>"Hey! How are you?"</ChatBubble>
                                            <ChatFooter>"10:30 AM"</ChatFooter>
                                        </Chat>
                                        <Chat position={ChatPosition::End}>
                                            <ChatBubble color={Color::Primary}>"I'm good, thanks!"</ChatBubble>
                                            <ChatFooter>"10:31 AM"</ChatFooter>
                                        </Chat>
                                        <Chat position={ChatPosition::Start}>
                                            <ChatBubble>"Want to grab lunch?"</ChatBubble>
                                            <ChatFooter>"10:32 AM"</ChatFooter>
                                        </Chat>
                                    </div>
                                    <div class="p-2 bg-base-100">
                                        <div class="join w-full">
                                            <Input class="join-item w-full" placeholder="Type a message..." />
                                            <Button color={Color::Primary} class="join-item">
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
                                                </svg>
                                            </Button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </MockupPhone>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Login Screen"
                    code=r#"<MockupPhone>
    <div class="artboard artboard-demo phone-1 bg-gradient-to-br from-primary to-secondary">
        <div class="flex flex-col items-center justify-center h-full p-6 text-primary-content">
            <h2 class="text-2xl font-bold mb-6">"Welcome"</h2>
            <div class="w-full space-y-3">
                <Input class="w-full bg-white/10" placeholder="Email" />
                <Input input_type="password" class="w-full bg-white/10" placeholder="Password" />
                <Button class="w-full bg-white text-primary">"Sign In"</Button>
            </div>
        </div>
    </div>
</MockupPhone>"#
                >
                    <div class="flex justify-center">
                        <MockupPhone>
                            <div class="artboard artboard-demo phone-1 bg-gradient-to-br from-primary to-secondary">
                                <div class="flex flex-col items-center justify-center h-full p-6 text-primary-content">
                                    <div class="w-20 h-20 bg-white/20 rounded-2xl flex items-center justify-center mb-6">
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-10 h-10">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" />
                                        </svg>
                                    </div>
                                    <h2 class="text-2xl font-bold mb-6">"Welcome"</h2>
                                    <div class="w-full space-y-3">
                                        <Input class="w-full bg-white/10 border-white/20 placeholder-white/50" placeholder="Email" />
                                        <Input input_type="password" class="w-full bg-white/10 border-white/20 placeholder-white/50" placeholder="Password" />
                                        <Button class="w-full bg-white text-primary hover:bg-white/90">"Sign In"</Button>
                                    </div>
                                    <p class="mt-4 text-sm text-white/70">
                                        "Don't have an account? "<a class="underline">"Sign up"</a>
                                    </p>
                                </div>
                            </div>
                        </MockupPhone>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Different Sizes"
                    code=r#"<MockupPhone>
    <div class="artboard phone-1 bg-base-200">"phone-1"</div>
</MockupPhone>
<MockupPhone>
    <div class="artboard phone-2 bg-base-200">"phone-2"</div>
</MockupPhone>
<MockupPhone>
    <div class="artboard phone-3 bg-base-200">"phone-3"</div>
</MockupPhone>"#
                >
                    <div class="flex flex-wrap justify-center gap-8 items-end">
                        <div class="flex flex-col items-center gap-2">
                            <MockupPhone>
                                <div class="artboard phone-1 bg-base-200 flex items-center justify-center">
                                    <span class="text-xs">"phone-1"</span>
                                </div>
                            </MockupPhone>
                            <span class="text-sm">"Small"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <MockupPhone>
                                <div class="artboard phone-2 bg-base-200 flex items-center justify-center">
                                    <span class="text-sm">"phone-2"</span>
                                </div>
                            </MockupPhone>
                            <span class="text-sm">"Medium"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <MockupPhone>
                                <div class="artboard phone-3 bg-base-200 flex items-center justify-center">
                                    <span class="text-sm">"phone-3"</span>
                                </div>
                            </MockupPhone>
                            <span class="text-sm">"Large"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

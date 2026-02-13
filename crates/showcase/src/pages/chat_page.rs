use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ChatPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Chat"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Chat components for building messaging interfaces with avatars, bubbles, and metadata."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "chat", type_label: "base", description: "Container for a single chat message" },
                    ClassEntry { name: "chat-start", type_label: "modifier", description: "Aligns chat message to the start (left)" },
                    ClassEntry { name: "chat-end", type_label: "modifier", description: "Aligns chat message to the end (right)" },
                    ClassEntry { name: "chat-image", type_label: "base", description: "Container for the chat avatar image" },
                    ClassEntry { name: "chat-header", type_label: "base", description: "Header text above the chat bubble" },
                    ClassEntry { name: "chat-bubble", type_label: "base", description: "The message bubble container" },
                    ClassEntry { name: "chat-footer", type_label: "base", description: "Footer text below the chat bubble" },
                    ClassEntry { name: "chat-bubble-primary", type_label: "color", description: "Chat bubble with primary color" },
                    ClassEntry { name: "chat-bubble-secondary", type_label: "color", description: "Chat bubble with secondary color" },
                    ClassEntry { name: "chat-bubble-accent", type_label: "color", description: "Chat bubble with accent color" },
                    ClassEntry { name: "chat-bubble-info", type_label: "color", description: "Chat bubble with info color" },
                    ClassEntry { name: "chat-bubble-success", type_label: "color", description: "Chat bubble with success color" },
                    ClassEntry { name: "chat-bubble-warning", type_label: "color", description: "Chat bubble with warning color" },
                    ClassEntry { name: "chat-bubble-error", type_label: "color", description: "Chat bubble with error color" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic Chat"
                    code=r#"<Chat position={ChatPosition::Start}>
    <ChatImage>
        <Avatar><img src="avatar1.png" alt="User" /></Avatar>
    </ChatImage>
    <ChatBubble>"Hello! How can I help you today?"</ChatBubble>
</Chat>
<Chat position={ChatPosition::End}>
    <ChatImage>
        <Avatar><img src="avatar2.png" alt="User" /></Avatar>
    </ChatImage>
    <ChatBubble>"Hi! I have a question."</ChatBubble>
</Chat>"#
                >
                    <div class="space-y-4">
                        <Chat position={ChatPosition::Start}>
                            <ChatImage>
                                <Avatar>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=User1" alt="User" />
                                </Avatar>
                            </ChatImage>
                            <ChatBubble>"Hello! How can I help you today?"</ChatBubble>
                        </Chat>
                        <Chat position={ChatPosition::End}>
                            <ChatImage>
                                <Avatar>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=User2" alt="User" />
                                </Avatar>
                            </ChatImage>
                            <ChatBubble>"Hi! I have a question about your services."</ChatBubble>
                        </Chat>
                        <Chat position={ChatPosition::Start}>
                            <ChatImage>
                                <Avatar>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=User1" alt="User" />
                                </Avatar>
                            </ChatImage>
                            <ChatBubble>"Sure, I'd be happy to help. What would you like to know?"</ChatBubble>
                        </Chat>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Chat with Headers and Footers"
                    code=r#"<Chat position={ChatPosition::Start}>
    <ChatImage>
        <Avatar><img src="support.png" alt="Support" /></Avatar>
    </ChatImage>
    <ChatHeader>"Support Agent"</ChatHeader>
    <ChatBubble>"Welcome to our support chat!"</ChatBubble>
    <ChatFooter>"Delivered"</ChatFooter>
</Chat>
<Chat position={ChatPosition::End}>
    <ChatImage>
        <Avatar><img src="customer.png" alt="Customer" /></Avatar>
    </ChatImage>
    <ChatHeader>"You"</ChatHeader>
    <ChatBubble>"Thanks! I need help with my order."</ChatBubble>
    <ChatFooter>"Seen at 2:30 PM"</ChatFooter>
</Chat>"#
                >
                    <div class="space-y-4">
                        <Chat position={ChatPosition::Start}>
                            <ChatImage>
                                <Avatar>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Support" alt="Support" />
                                </Avatar>
                            </ChatImage>
                            <ChatHeader>"Support Agent"</ChatHeader>
                            <ChatBubble>"Welcome to our support chat!"</ChatBubble>
                            <ChatFooter>"Delivered"</ChatFooter>
                        </Chat>
                        <Chat position={ChatPosition::End}>
                            <ChatImage>
                                <Avatar>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Customer" alt="Customer" />
                                </Avatar>
                            </ChatImage>
                            <ChatHeader>"You"</ChatHeader>
                            <ChatBubble>"Thanks! I need help with my order."</ChatBubble>
                            <ChatFooter>"Seen at 2:30 PM"</ChatFooter>
                        </Chat>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Chat Bubble Colors"
                    code=r#"<Chat position={ChatPosition::Start}>
    <ChatBubble color={Color::Primary}>"Primary bubble"</ChatBubble>
</Chat>
<Chat position={ChatPosition::Start}>
    <ChatBubble color={Color::Secondary}>"Secondary bubble"</ChatBubble>
</Chat>
<Chat position={ChatPosition::Start}>
    <ChatBubble color={Color::Accent}>"Accent bubble"</ChatBubble>
</Chat>
<Chat position={ChatPosition::Start}>
    <ChatBubble color={Color::Info}>"Info bubble"</ChatBubble>
</Chat>
<Chat position={ChatPosition::Start}>
    <ChatBubble color={Color::Success}>"Success bubble"</ChatBubble>
</Chat>
<Chat position={ChatPosition::Start}>
    <ChatBubble color={Color::Warning}>"Warning bubble"</ChatBubble>
</Chat>
<Chat position={ChatPosition::Start}>
    <ChatBubble color={Color::Error}>"Error bubble"</ChatBubble>
</Chat>"#
                >
                    <div class="space-y-4">
                        <Chat position={ChatPosition::Start}>
                            <ChatBubble color={Color::Primary}>"Primary bubble color"</ChatBubble>
                        </Chat>
                        <Chat position={ChatPosition::Start}>
                            <ChatBubble color={Color::Secondary}>"Secondary bubble color"</ChatBubble>
                        </Chat>
                        <Chat position={ChatPosition::Start}>
                            <ChatBubble color={Color::Accent}>"Accent bubble color"</ChatBubble>
                        </Chat>
                        <Chat position={ChatPosition::Start}>
                            <ChatBubble color={Color::Info}>"Info bubble color"</ChatBubble>
                        </Chat>
                        <Chat position={ChatPosition::Start}>
                            <ChatBubble color={Color::Success}>"Success bubble color"</ChatBubble>
                        </Chat>
                        <Chat position={ChatPosition::Start}>
                            <ChatBubble color={Color::Warning}>"Warning bubble color"</ChatBubble>
                        </Chat>
                        <Chat position={ChatPosition::Start}>
                            <ChatBubble color={Color::Error}>"Error bubble color"</ChatBubble>
                        </Chat>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Chat Conversation Example"
                    code=r#"<Chat position={ChatPosition::Start}>
    <ChatImage>
        <Avatar><img src="alice.png" alt="Alice" /></Avatar>
    </ChatImage>
    <ChatHeader>"Alice"</ChatHeader>
    <ChatBubble>"Hey! Are we still on for lunch tomorrow?"</ChatBubble>
    <ChatFooter>"10:30 AM"</ChatFooter>
</Chat>
<Chat position={ChatPosition::End}>
    <ChatImage>
        <Avatar><img src="bob.png" alt="Bob" /></Avatar>
    </ChatImage>
    <ChatHeader>"Bob"</ChatHeader>
    <ChatBubble>"Yes, definitely! How about 12:30?"</ChatBubble>
    <ChatFooter>"10:32 AM"</ChatFooter>
</Chat>"#
                >
                    <div class="bg-base-200 p-4 rounded-lg space-y-4">
                        <Chat position={ChatPosition::Start}>
                            <ChatImage>
                                <Avatar>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Alice" alt="Alice" />
                                </Avatar>
                            </ChatImage>
                            <ChatHeader>"Alice"</ChatHeader>
                            <ChatBubble>"Hey! Are we still on for lunch tomorrow?"</ChatBubble>
                            <ChatFooter>"10:30 AM"</ChatFooter>
                        </Chat>
                        <Chat position={ChatPosition::End}>
                            <ChatImage>
                                <Avatar>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Bob" alt="Bob" />
                                </Avatar>
                            </ChatImage>
                            <ChatHeader>"Bob"</ChatHeader>
                            <ChatBubble>"Yes, definitely! How about 12:30 at the usual place?"</ChatBubble>
                            <ChatFooter>"10:32 AM"</ChatFooter>
                        </Chat>
                        <Chat position={ChatPosition::Start}>
                            <ChatImage>
                                <Avatar>
                                    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Alice" alt="Alice" />
                                </Avatar>
                            </ChatImage>
                            <ChatHeader>"Alice"</ChatHeader>
                            <ChatBubble>"Perfect! See you then."</ChatBubble>
                            <ChatFooter>"10:33 AM"</ChatFooter>
                        </Chat>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}

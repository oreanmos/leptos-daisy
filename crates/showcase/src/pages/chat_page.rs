use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ChatPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Chat"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Chat"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Chat with Headers and Footers"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Chat Bubble Colors"</h2>
                <div class="space-y-4">
                    <Chat position={ChatPosition::Start}>
                        <ChatBubble color={ChatBubbleColor::Primary}>"Primary bubble color"</ChatBubble>
                    </Chat>
                    <Chat position={ChatPosition::Start}>
                        <ChatBubble color={ChatBubbleColor::Secondary}>"Secondary bubble color"</ChatBubble>
                    </Chat>
                    <Chat position={ChatPosition::Start}>
                        <ChatBubble color={ChatBubbleColor::Accent}>"Accent bubble color"</ChatBubble>
                    </Chat>
                    <Chat position={ChatPosition::Start}>
                        <ChatBubble color={ChatBubbleColor::Info}>"Info bubble color"</ChatBubble>
                    </Chat>
                    <Chat position={ChatPosition::Start}>
                        <ChatBubble color={ChatBubbleColor::Success}>"Success bubble color"</ChatBubble>
                    </Chat>
                    <Chat position={ChatPosition::Start}>
                        <ChatBubble color={ChatBubbleColor::Warning}>"Warning bubble color"</ChatBubble>
                    </Chat>
                    <Chat position={ChatPosition::Start}>
                        <ChatBubble color={ChatBubbleColor::Error}>"Error bubble color"</ChatBubble>
                    </Chat>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Chat Conversation Example"</h2>
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
            </section>
        </div>
    }
}

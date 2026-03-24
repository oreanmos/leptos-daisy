use leptos::prelude::*;
use std::fmt::Write;

use crate::components::CodeBlock;

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
enum ContainerKind {
    FlexRow,
    FlexCol,
    Grid2,
    Grid3,
}

impl ContainerKind {
    fn label(self) -> &'static str {
        match self {
            Self::FlexRow => "Flex Row",
            Self::FlexCol => "Flex Col",
            Self::Grid2 => "Grid 2-Col",
            Self::Grid3 => "Grid 3-Col",
        }
    }

    fn css_class(self, gap: &str, padding: &str) -> String {
        let layout = match self {
            Self::FlexRow => "flex flex-row",
            Self::FlexCol => "flex flex-col",
            Self::Grid2 => "grid grid-cols-2",
            Self::Grid3 => "grid grid-cols-3",
        };
        format!("{} {} {}", layout, gap, padding)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ComponentKind {
    // Actions
    Button,
    Dropdown,
    Modal,
    // Data Display
    Alert,
    Avatar,
    Badge,
    Card,
    Kbd,
    Progress,
    Skeleton,
    Stat,
    Table,
    Timeline,
    // Data Input
    Checkbox,
    FileInput,
    Input,
    Radio,
    Range,
    Select,
    Textarea,
    Toggle,
    // Feedback
    Loading,
    Toast,
    // Layout
    Divider,
    Hero,
    // Navigation
    Breadcrumbs,
    Menu,
    Navbar,
    Pagination,
    Steps,
    Tab,
}

impl ComponentKind {
    fn label(self) -> &'static str {
        match self {
            Self::Button => "Button",
            Self::Dropdown => "Dropdown",
            Self::Modal => "Modal",
            Self::Alert => "Alert",
            Self::Avatar => "Avatar",
            Self::Badge => "Badge",
            Self::Card => "Card",
            Self::Kbd => "Kbd",
            Self::Progress => "Progress",
            Self::Skeleton => "Skeleton",
            Self::Stat => "Stat",
            Self::Table => "Table",
            Self::Timeline => "Timeline",
            Self::Checkbox => "Checkbox",
            Self::FileInput => "File Input",
            Self::Input => "Input",
            Self::Radio => "Radio",
            Self::Range => "Range",
            Self::Select => "Select",
            Self::Textarea => "Textarea",
            Self::Toggle => "Toggle",
            Self::Loading => "Loading",
            Self::Toast => "Toast",
            Self::Divider => "Divider",
            Self::Hero => "Hero",
            Self::Breadcrumbs => "Breadcrumbs",
            Self::Menu => "Menu",
            Self::Navbar => "Navbar",
            Self::Pagination => "Pagination",
            Self::Steps => "Steps",
            Self::Tab => "Tab",
        }
    }

    fn default_text(self) -> &'static str {
        match self {
            Self::Button => "Click me",
            Self::Dropdown => "Dropdown",
            Self::Modal => "Open modal",
            Self::Alert => "Alert message",
            Self::Avatar => "A",
            Self::Badge => "New",
            Self::Card => "Card Title",
            Self::Kbd => "Ctrl",
            Self::Progress => "",
            Self::Skeleton => "",
            Self::Stat => "31,000",
            Self::Table => "Row",
            Self::Timeline => "Timeline event",
            Self::Checkbox => "",
            Self::FileInput => "",
            Self::Input => "Type here...",
            Self::Radio => "",
            Self::Range => "",
            Self::Select => "Pick one",
            Self::Textarea => "Type here...",
            Self::Toggle => "",
            Self::Loading => "",
            Self::Toast => "Saved successfully",
            Self::Divider => "OR",
            Self::Hero => "Hero heading",
            Self::Breadcrumbs => "Page",
            Self::Menu => "Menu Item",
            Self::Navbar => "Brand",
            Self::Pagination => "",
            Self::Steps => "Step 2",
            Self::Tab => "Tab",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CanvasNode {
    Component {
        id: usize,
        kind: ComponentKind,
        text: String,
        color: String,
        size: String,
    },
    Container {
        id: usize,
        kind: ContainerKind,
        children: Vec<CanvasNode>,
        gap: String,
        padding: String,
    },
}

impl CanvasNode {
    fn id(&self) -> usize {
        match self {
            CanvasNode::Component { id, .. } | CanvasNode::Container { id, .. } => *id,
        }
    }

    fn is_container(&self) -> bool {
        matches!(self, CanvasNode::Container { .. })
    }
}

// ---------------------------------------------------------------------------
// Tree manipulation helpers
// ---------------------------------------------------------------------------

fn find_node(nodes: &[CanvasNode], target_id: usize) -> Option<&CanvasNode> {
    for node in nodes {
        if node.id() == target_id {
            return Some(node);
        }
        if let CanvasNode::Container { children, .. } = node
            && let Some(found) = find_node(children, target_id)
        {
            return Some(found);
        }
    }
    None
}

fn try_add_to_parent(nodes: &mut [CanvasNode], parent_id: usize, new_node: &CanvasNode) -> bool {
    for node in nodes.iter_mut() {
        if let CanvasNode::Container { id, children, .. } = node {
            if *id == parent_id {
                children.push(new_node.clone());
                return true;
            }
            if try_add_to_parent(children, parent_id, new_node) {
                return true;
            }
        }
    }
    false
}

fn remove_node(nodes: &mut Vec<CanvasNode>, target_id: usize) -> bool {
    if let Some(pos) = nodes.iter().position(|n| n.id() == target_id) {
        nodes.remove(pos);
        return true;
    }
    for node in nodes.iter_mut() {
        if let CanvasNode::Container { children, .. } = node
            && remove_node(children, target_id)
        {
            return true;
        }
    }
    false
}

fn update_node_in_tree(
    nodes: &mut [CanvasNode],
    target_id: usize,
    f: &dyn Fn(&mut CanvasNode),
) -> bool {
    for node in nodes.iter_mut() {
        if node.id() == target_id {
            f(node);
            return true;
        }
        if let CanvasNode::Container { children, .. } = node
            && update_node_in_tree(children, target_id, f)
        {
            return true;
        }
    }
    false
}

/// Collect tree items into a flat list: (id, depth, label, is_container).
fn collect_tree_items(
    nodes: &[CanvasNode],
    depth: usize,
    items: &mut Vec<(usize, usize, String, bool)>,
) {
    for node in nodes {
        let label = match node {
            CanvasNode::Component { kind, .. } => kind.label().to_string(),
            CanvasNode::Container { kind, .. } => kind.label().to_string(),
        };
        items.push((node.id(), depth, label, node.is_container()));
        if let CanvasNode::Container { children, .. } = node {
            collect_tree_items(children, depth + 1, items);
        }
    }
}

// ---------------------------------------------------------------------------
// Canvas rendering (recursive)
// ---------------------------------------------------------------------------

fn render_canvas_node(node: &CanvasNode, selected_id: RwSignal<Option<usize>>) -> AnyView {
    match node {
        CanvasNode::Component {
            id,
            kind,
            text,
            color,
            size,
        } => {
            let id = *id;
            let kind = *kind;
            let text = text.clone();
            let color = color.clone();
            let size = size.clone();
            let is_sel = move || selected_id.get() == Some(id);

            let preview = render_component_preview(kind, &text, &color, &size);

            view! {
                <div
                    class="cursor-pointer rounded-box p-1 transition-all"
                    class:ring-2=is_sel
                    class:ring-primary=is_sel
                    on:click=move |ev| {
                        ev.stop_propagation();
                        selected_id.set(Some(id));
                    }
                >
                    {preview}
                </div>
            }
            .into_any()
        }
        CanvasNode::Container {
            id,
            kind,
            children,
            gap,
            padding,
        } => {
            let id = *id;
            let kind = *kind;
            let gap = gap.clone();
            let padding = padding.clone();
            let is_sel = move || selected_id.get() == Some(id);
            let layout_class = kind.css_class(&gap, &padding);
            let label = kind.label();
            let is_empty = children.is_empty();

            let child_views: Vec<AnyView> = children
                .iter()
                .map(|child| render_canvas_node(child, selected_id))
                .collect();

            view! {
                <div
                    class="border-2 border-dashed border-base-300 rounded-box p-2 min-h-[4rem] cursor-pointer transition-all"
                    class:ring-2=is_sel
                    class:ring-primary=is_sel
                    on:click=move |ev| {
                        ev.stop_propagation();
                        selected_id.set(Some(id));
                    }
                >
                    <div class="text-xs text-base-content/40 mb-1 font-mono">{label}</div>
                    <div class={layout_class}>
                        {child_views.into_iter().collect_view()}
                        {is_empty.then(|| view! {
                            <div class="text-base-content/30 text-sm italic p-4 text-center w-full">
                                "Drop components here"
                            </div>
                        })}
                    </div>
                </div>
            }
            .into_any()
        }
    }
}

fn render_component_preview(kind: ComponentKind, text: &str, color: &str, size: &str) -> AnyView {
    let text = text.to_string();
    let color = color.to_string();
    let size = size.to_string();

    match kind {
        ComponentKind::Button => {
            let cls = if color.is_empty() {
                format!("btn btn-{}", size)
            } else {
                format!("btn btn-{} btn-{}", color, size)
            };
            view! { <button class={cls}>{text}</button> }.into_any()
        }
        ComponentKind::Dropdown => view! {
            <div class="dropdown">
                <div tabindex="0" role="button" class="btn btn-sm">{text.clone()}</div>
                <ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-10 w-40 p-2 shadow">
                    <li><a>"Item 1"</a></li>
                    <li><a>"Item 2"</a></li>
                </ul>
            </div>
        }
        .into_any(),
        ComponentKind::Modal => view! {
            <div class="card bg-base-200 p-4 max-w-sm">
                <p class="font-semibold mb-2">"Modal preview"</p>
                <button class="btn btn-sm btn-primary">{text}</button>
            </div>
        }
        .into_any(),
        ComponentKind::Alert => {
            let cls = if color.is_empty() {
                "alert".to_string()
            } else {
                format!("alert alert-{}", color)
            };
            view! { <div class={cls}><span>{text}</span></div> }.into_any()
        }
        ComponentKind::Avatar => view! {
            <div class="avatar placeholder">
                <div class="bg-neutral text-neutral-content w-12 rounded-full">
                    <span>"A"</span>
                </div>
            </div>
        }
        .into_any(),
        ComponentKind::Badge => {
            let cls = if color.is_empty() {
                format!("badge badge-{}", size)
            } else {
                format!("badge badge-{} badge-{}", color, size)
            };
            view! { <span class={cls}>{text}</span> }.into_any()
        }
        ComponentKind::Card => view! {
            <div class="card bg-base-100 shadow-sm">
                <div class="card-body">
                    <h2 class="card-title">{text}</h2>
                </div>
            </div>
        }
        .into_any(),
        ComponentKind::Kbd => {
            let cls = format!("kbd kbd-{}", size);
            view! { <kbd class={cls}>{text}</kbd> }.into_any()
        }
        ComponentKind::Progress => {
            let cls = if color.is_empty() {
                "progress w-full".to_string()
            } else {
                format!("progress progress-{} w-full", color)
            };
            view! { <progress class={cls} value="70" max="100"></progress> }.into_any()
        }
        ComponentKind::Skeleton => view! {
            <div class="flex flex-col gap-2 w-48">
                <div class="skeleton h-4 w-24"></div>
                <div class="skeleton h-4 w-full"></div>
                <div class="skeleton h-8 w-full"></div>
            </div>
        }
        .into_any(),
        ComponentKind::Stat => {
            let val_cls = if color.is_empty() {
                "stat-value".to_string()
            } else {
                format!("stat-value text-{}", color)
            };
            view! {
                <div class="stats shadow">
                    <div class="stat">
                        <div class="stat-title">"Total"</div>
                        <div class={val_cls}>{text}</div>
                    </div>
                </div>
            }
            .into_any()
        }
        ComponentKind::Table => view! {
            <div class="overflow-x-auto">
                <table class="table table-zebra table-sm">
                    <thead>
                        <tr>
                            <th>"#"</th>
                            <th>"Name"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <th>"1"</th>
                            <td>{text}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        }
        .into_any(),
        ComponentKind::Timeline => view! {
            <ul class="timeline timeline-vertical">
                <li>
                    <div class="timeline-start">"09:00"</div>
                    <div class="timeline-middle">"●"</div>
                    <div class="timeline-end timeline-box">{text}</div>
                </li>
            </ul>
        }
        .into_any(),
        ComponentKind::Checkbox => {
            let cls = if color.is_empty() {
                "checkbox".to_string()
            } else {
                format!("checkbox checkbox-{}", color)
            };
            view! { <input type="checkbox" class={cls} checked /> }.into_any()
        }
        ComponentKind::FileInput => {
            view! { <input type="file" class="file-input file-input-bordered file-input-sm w-full max-w-xs" /> }
                .into_any()
        }
        ComponentKind::Input => {
            let cls = format!("input input-bordered input-{}", size);
            view! { <input type="text" placeholder={text} class={cls} /> }.into_any()
        }
        ComponentKind::Radio => {
            let cls = if color.is_empty() {
                "radio".to_string()
            } else {
                format!("radio radio-{}", color)
            };
            view! { <input type="radio" class={cls} checked /> }.into_any()
        }
        ComponentKind::Range => {
            let cls = if color.is_empty() {
                format!("range range-{}", size)
            } else {
                format!("range range-{} range-{}", color, size)
            };
            view! { <input type="range" class={cls} min="0" max="100" value="40" /> }.into_any()
        }
        ComponentKind::Select => {
            let cls = format!("select select-bordered select-{}", size);
            view! {
                <select class={cls}>
                    <option>{text}</option>
                </select>
            }
            .into_any()
        }
        ComponentKind::Textarea => view! {
            <textarea class="textarea textarea-bordered" placeholder={text}></textarea>
        }
        .into_any(),
        ComponentKind::Toggle => {
            let cls = if color.is_empty() {
                "toggle".to_string()
            } else {
                format!("toggle toggle-{}", color)
            };
            view! { <input type="checkbox" class={cls} checked /> }.into_any()
        }
        ComponentKind::Loading => {
            let cls = format!("loading loading-spinner loading-{}", size);
            view! { <span class={cls}></span> }.into_any()
        }
        ComponentKind::Toast => view! {
            <div class="toast toast-end toast-top static">
                <div class="alert alert-success py-2">
                    <span>{text}</span>
                </div>
            </div>
        }
        .into_any(),
        ComponentKind::Divider => view! { <div class="divider">{text}</div> }.into_any(),
        ComponentKind::Hero => view! {
            <div class="hero bg-base-200 rounded-box min-h-44">
                <div class="hero-content text-center">
                    <div class="max-w-md">
                        <h2 class="text-2xl font-bold">{text}</h2>
                        <p class="py-2 text-sm">"Build pages quickly with reusable blocks."</p>
                    </div>
                </div>
            </div>
        }
        .into_any(),
        ComponentKind::Breadcrumbs => view! {
            <div class="breadcrumbs text-sm">
                <ul>
                    <li><a>"Home"</a></li>
                    <li>{text}</li>
                </ul>
            </div>
        }
        .into_any(),
        ComponentKind::Menu => view! {
            <ul class="menu bg-base-200 rounded-box w-56">
                <li><a>{text}</a></li>
            </ul>
        }
        .into_any(),
        ComponentKind::Navbar => view! {
            <div class="navbar bg-base-200 rounded-box px-3">
                <div class="navbar-start font-bold">{text}</div>
                <div class="navbar-end">
                    <button class="btn btn-ghost btn-sm">"Menu"</button>
                </div>
            </div>
        }
        .into_any(),
        ComponentKind::Pagination => view! {
            <div class="join">
                <button class="join-item btn">"1"</button>
                <button class="join-item btn btn-active">"2"</button>
                <button class="join-item btn">"3"</button>
            </div>
        }
        .into_any(),
        ComponentKind::Steps => view! {
            <ul class="steps">
                <li class="step step-primary">"Step 1"</li>
                <li class="step">{text}</li>
            </ul>
        }
        .into_any(),
        ComponentKind::Tab => view! {
            <div role="tablist" class="tabs tabs-lift">
                <a role="tab" class="tab tab-active">{text}</a>
            </div>
        }
        .into_any(),
    }
}

// ---------------------------------------------------------------------------
// Code generation
// ---------------------------------------------------------------------------

fn generate_code(nodes: &[CanvasNode], indent: usize, buf: &mut String) {
    let prefix = "    ".repeat(indent);
    for (i, node) in nodes.iter().enumerate() {
        if i > 0 {
            buf.push('\n');
        }
        match node {
            CanvasNode::Container {
                kind,
                children,
                gap,
                padding,
                ..
            } => {
                let class = kind.css_class(gap, padding);
                buf.push_str(&prefix);
                let _ = write!(buf, "<div class=\"{}\">", class);

                if !children.is_empty() {
                    buf.push('\n');
                    generate_code(children, indent + 1, buf);
                }

                buf.push('\n');
                buf.push_str(&prefix);
                buf.push_str("</div>");
            }
            CanvasNode::Component {
                kind,
                text,
                color,
                size,
                ..
            } => {
                buf.push_str(&prefix);
                generate_component_code(buf, *kind, text, color, size);
            }
        }
    }
}

fn generate_component_code(
    buf: &mut String,
    kind: ComponentKind,
    text: &str,
    color: &str,
    size: &str,
) {
    let write_color_prop = |buf: &mut String, color: &str| {
        if !color.is_empty() {
            buf.push_str(" color=Color::");
            write_capitalized(buf, color);
        }
    };
    let write_size_prop = |buf: &mut String, size: &str| {
        if !size.is_empty() {
            buf.push_str(" size=Size::");
            write_capitalized(buf, size);
        }
    };

    match kind {
        ComponentKind::Button => {
            buf.push_str("<Button");
            write_color_prop(buf, color);
            write_size_prop(buf, size);
            buf.push('>');
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</Button>");
        }
        ComponentKind::Dropdown => {
            buf.push_str("<Dropdown><DropdownTrigger><Button>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</Button></DropdownTrigger><DropdownContent><DropdownItem>\"Item 1\"</DropdownItem></DropdownContent></Dropdown>");
        }
        ComponentKind::Modal => {
            buf.push_str("<Button color=Color::Primary>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</Button>");
        }
        ComponentKind::Alert => {
            buf.push_str("<Alert");
            write_color_prop(buf, color);
            buf.push_str("><span>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</span></Alert>");
        }
        ComponentKind::Avatar => {
            buf.push_str("<Avatar placeholder=true />");
        }
        ComponentKind::Badge => {
            buf.push_str("<Badge");
            write_color_prop(buf, color);
            write_size_prop(buf, size);
            buf.push('>');
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</Badge>");
        }
        ComponentKind::Card => {
            buf.push_str("<Card><CardBody><CardTitle>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</CardTitle></CardBody></Card>");
        }
        ComponentKind::Kbd => {
            buf.push_str("<Kbd");
            write_size_prop(buf, size);
            buf.push('>');
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</Kbd>");
        }
        ComponentKind::Progress => {
            buf.push_str("<Progress");
            write_color_prop(buf, color);
            buf.push_str(" value=70 max=100 />");
        }
        ComponentKind::Skeleton => {
            buf.push_str("<Skeleton class=\"h-8 w-full\" />");
        }
        ComponentKind::Stat => {
            buf.push_str("<Stat><StatTitle>\"Total\"</StatTitle><StatValue");
            write_color_prop(buf, color);
            buf.push('>');
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</StatValue></Stat>");
        }
        ComponentKind::Table => {
            buf.push_str("<Table><thead><tr><th>\"#\"</th><th>\"Name\"</th></tr></thead><tbody><tr><th>\"1\"</th><td>\"Item\"</td></tr></tbody></Table>");
        }
        ComponentKind::Timeline => {
            buf.push_str("<Timeline><TimelineItem><TimelineStart>\"09:00\"</TimelineStart><TimelineMiddle>\"●\"</TimelineMiddle><TimelineEnd>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</TimelineEnd></TimelineItem></Timeline>");
        }
        ComponentKind::Checkbox => {
            buf.push_str("<Checkbox");
            write_color_prop(buf, color);
            buf.push_str(" />");
        }
        ComponentKind::FileInput => {
            buf.push_str("<FileInput />");
        }
        ComponentKind::Input => {
            buf.push_str("<Input placeholder=");
            let _ = write!(buf, "{:?}", text);
            write_size_prop(buf, size);
            buf.push_str(" />");
        }
        ComponentKind::Radio => {
            buf.push_str("<Radio");
            write_color_prop(buf, color);
            buf.push_str(" />");
        }
        ComponentKind::Range => {
            buf.push_str("<Range");
            write_color_prop(buf, color);
            write_size_prop(buf, size);
            buf.push_str(" />");
        }
        ComponentKind::Select => {
            buf.push_str("<Select");
            write_size_prop(buf, size);
            buf.push_str("><option>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</option></Select>");
        }
        ComponentKind::Textarea => {
            buf.push_str("<Textarea placeholder=");
            let _ = write!(buf, "{:?}", text);
            buf.push_str(" />");
        }
        ComponentKind::Toggle => {
            buf.push_str("<Toggle");
            write_color_prop(buf, color);
            buf.push_str(" />");
        }
        ComponentKind::Loading => {
            buf.push_str("<Loading");
            write_size_prop(buf, size);
            buf.push_str(" />");
        }
        ComponentKind::Toast => {
            buf.push_str("<Toast><Alert color=Color::Success>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</Alert></Toast>");
        }
        ComponentKind::Divider => {
            buf.push_str("<Divider>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</Divider>");
        }
        ComponentKind::Hero => {
            buf.push_str("<Hero><HeroContent><h1>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</h1></HeroContent></Hero>");
        }
        ComponentKind::Breadcrumbs => {
            buf.push_str("<Breadcrumbs><li><a>\"Home\"</a></li><li>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</li></Breadcrumbs>");
        }
        ComponentKind::Menu => {
            buf.push_str("<Menu><li><a>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</a></li></Menu>");
        }
        ComponentKind::Navbar => {
            buf.push_str("<Navbar><NavbarStart>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</NavbarStart></Navbar>");
        }
        ComponentKind::Pagination => {
            buf.push_str("<Join><button class=\"join-item btn\">\"1\"</button><button class=\"join-item btn btn-active\">\"2\"</button><button class=\"join-item btn\">\"3\"</button></Join>");
        }
        ComponentKind::Steps => {
            buf.push_str("<Steps><Step color=Color::Primary>\"Step 1\"</Step><Step>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</Step></Steps>");
        }
        ComponentKind::Tab => {
            buf.push_str("<Tabs><Tab active=true>");
            let _ = write!(buf, "{:?}", text);
            buf.push_str("</Tab></Tabs>");
        }
    }
}

fn write_capitalized(buf: &mut String, s: &str) {
    let mut c = s.chars();
    if let Some(f) = c.next() {
        for char in f.to_uppercase() {
            buf.push(char);
        }
        buf.push_str(c.as_str());
    }
}

// ---------------------------------------------------------------------------
// Palette section data
// ---------------------------------------------------------------------------

const CONTAINER_ITEMS: &[(ContainerKind, &str)] = &[
    (ContainerKind::FlexRow, "Flex Row"),
    (ContainerKind::FlexCol, "Flex Col"),
    (ContainerKind::Grid2, "Grid 2-Col"),
    (ContainerKind::Grid3, "Grid 3-Col"),
];

const ACTION_ITEMS: &[(ComponentKind, &str)] = &[
    (ComponentKind::Button, "Button"),
    (ComponentKind::Dropdown, "Dropdown"),
    (ComponentKind::Modal, "Modal"),
];

const DATA_DISPLAY_ITEMS: &[(ComponentKind, &str)] = &[
    (ComponentKind::Alert, "Alert"),
    (ComponentKind::Avatar, "Avatar"),
    (ComponentKind::Badge, "Badge"),
    (ComponentKind::Card, "Card"),
    (ComponentKind::Kbd, "Kbd"),
    (ComponentKind::Progress, "Progress"),
    (ComponentKind::Skeleton, "Skeleton"),
    (ComponentKind::Stat, "Stat"),
    (ComponentKind::Table, "Table"),
    (ComponentKind::Timeline, "Timeline"),
];

const DATA_INPUT_ITEMS: &[(ComponentKind, &str)] = &[
    (ComponentKind::Checkbox, "Checkbox"),
    (ComponentKind::FileInput, "File Input"),
    (ComponentKind::Input, "Input"),
    (ComponentKind::Radio, "Radio"),
    (ComponentKind::Range, "Range"),
    (ComponentKind::Select, "Select"),
    (ComponentKind::Textarea, "Textarea"),
    (ComponentKind::Toggle, "Toggle"),
];

const FEEDBACK_ITEMS: &[(ComponentKind, &str)] = &[
    (ComponentKind::Loading, "Loading"),
    (ComponentKind::Toast, "Toast"),
];

const LAYOUT_ITEMS: &[(ComponentKind, &str)] = &[
    (ComponentKind::Divider, "Divider"),
    (ComponentKind::Hero, "Hero"),
];

const NAVIGATION_ITEMS: &[(ComponentKind, &str)] = &[
    (ComponentKind::Breadcrumbs, "Breadcrumbs"),
    (ComponentKind::Menu, "Menu"),
    (ComponentKind::Navbar, "Navbar"),
    (ComponentKind::Pagination, "Pagination"),
    (ComponentKind::Steps, "Steps"),
    (ComponentKind::Tab, "Tab"),
];

// ---------------------------------------------------------------------------
// Main component
// ---------------------------------------------------------------------------

#[component]
pub fn PlaygroundPage() -> impl IntoView {
    let canvas = RwSignal::new(Vec::<CanvasNode>::new());
    let selected_id = RwSignal::new(Option::<usize>::None);
    let next_id = RwSignal::new(1usize);

    // Add a node to the canvas, inserting into the selected container when applicable.
    let add_node = move |new_node: CanvasNode| {
        let node_id = new_node.id();
        let sel = selected_id.get_untracked();
        canvas.update(|nodes| {
            let target_is_container =
                sel.is_some_and(|sid| find_node(nodes, sid).is_some_and(|n| n.is_container()));
            if target_is_container {
                let sid = sel.unwrap();
                if !try_add_to_parent(nodes, sid, &new_node) {
                    nodes.push(new_node);
                }
            } else {
                nodes.push(new_node);
            }
        });
        selected_id.set(Some(node_id));
    };

    let alloc_id = move || {
        let id = next_id.get_untracked();
        next_id.set(id + 1);
        id
    };

    // Handlers for adding containers
    let add_container = move |kind: ContainerKind| {
        let id = alloc_id();
        add_node(CanvasNode::Container {
            id,
            kind,
            children: Vec::new(),
            gap: "gap-4".to_string(),
            padding: "p-4".to_string(),
        });
    };

    // Handler for adding components
    let add_component = move |kind: ComponentKind| {
        let id = alloc_id();
        add_node(CanvasNode::Component {
            id,
            kind,
            text: kind.default_text().to_string(),
            color: "primary".to_string(),
            size: "md".to_string(),
        });
    };

    let delete_selected = move || {
        if let Some(id) = selected_id.get_untracked() {
            canvas.update(|nodes| {
                remove_node(nodes, id);
            });
            selected_id.set(None);
        }
    };

    // -- generated code (derived) --
    let generated_code = Signal::derive(move || {
        let nodes = canvas.get();
        if nodes.is_empty() {
            "// Add components from the palette to generate code".to_string()
        } else {
            let mut s = String::from("view! {\n");
            generate_code(&nodes, 1, &mut s);
            s.push('\n');
            s.push('}');
            s
        }
    });

    let load_dashboard_template = move || {
        canvas.set(vec![
            CanvasNode::Component {
                id: 1,
                kind: ComponentKind::Navbar,
                text: "Dashboard".to_string(),
                color: "primary".to_string(),
                size: "md".to_string(),
            },
            CanvasNode::Container {
                id: 2,
                kind: ContainerKind::Grid3,
                gap: "gap-4".to_string(),
                padding: "p-4".to_string(),
                children: vec![
                    CanvasNode::Component {
                        id: 3,
                        kind: ComponentKind::Stat,
                        text: "31,000".to_string(),
                        color: "primary".to_string(),
                        size: "md".to_string(),
                    },
                    CanvasNode::Component {
                        id: 4,
                        kind: ComponentKind::Stat,
                        text: "8,420".to_string(),
                        color: "success".to_string(),
                        size: "md".to_string(),
                    },
                    CanvasNode::Component {
                        id: 5,
                        kind: ComponentKind::Stat,
                        text: "1,209".to_string(),
                        color: "warning".to_string(),
                        size: "md".to_string(),
                    },
                ],
            },
            CanvasNode::Component {
                id: 6,
                kind: ComponentKind::Table,
                text: "Metrics".to_string(),
                color: "".to_string(),
                size: "md".to_string(),
            },
        ]);
        selected_id.set(Some(1));
        next_id.set(7);
    };

    let load_marketing_template = move || {
        canvas.set(vec![
            CanvasNode::Component {
                id: 1,
                kind: ComponentKind::Navbar,
                text: "Brand".to_string(),
                color: "primary".to_string(),
                size: "md".to_string(),
            },
            CanvasNode::Component {
                id: 2,
                kind: ComponentKind::Hero,
                text: "Build faster with leptos-daisyui".to_string(),
                color: "primary".to_string(),
                size: "md".to_string(),
            },
            CanvasNode::Container {
                id: 3,
                kind: ContainerKind::Grid3,
                gap: "gap-4".to_string(),
                padding: "p-2".to_string(),
                children: vec![
                    CanvasNode::Component {
                        id: 4,
                        kind: ComponentKind::Card,
                        text: "Fast setup".to_string(),
                        color: "primary".to_string(),
                        size: "md".to_string(),
                    },
                    CanvasNode::Component {
                        id: 5,
                        kind: ComponentKind::Card,
                        text: "Type-safe wrappers".to_string(),
                        color: "secondary".to_string(),
                        size: "md".to_string(),
                    },
                    CanvasNode::Component {
                        id: 6,
                        kind: ComponentKind::Card,
                        text: "Theme-ready".to_string(),
                        color: "accent".to_string(),
                        size: "md".to_string(),
                    },
                ],
            },
        ]);
        selected_id.set(Some(1));
        next_id.set(7);
    };

    let load_form_template = move || {
        canvas.set(vec![
            CanvasNode::Component {
                id: 1,
                kind: ComponentKind::Navbar,
                text: "Settings".to_string(),
                color: "primary".to_string(),
                size: "md".to_string(),
            },
            CanvasNode::Container {
                id: 2,
                kind: ContainerKind::FlexCol,
                gap: "gap-3".to_string(),
                padding: "p-4".to_string(),
                children: vec![
                    CanvasNode::Component {
                        id: 3,
                        kind: ComponentKind::Input,
                        text: "Name".to_string(),
                        color: "".to_string(),
                        size: "md".to_string(),
                    },
                    CanvasNode::Component {
                        id: 4,
                        kind: ComponentKind::Select,
                        text: "Choose role".to_string(),
                        color: "".to_string(),
                        size: "md".to_string(),
                    },
                    CanvasNode::Component {
                        id: 5,
                        kind: ComponentKind::Textarea,
                        text: "Notes".to_string(),
                        color: "".to_string(),
                        size: "md".to_string(),
                    },
                    CanvasNode::Component {
                        id: 6,
                        kind: ComponentKind::Button,
                        text: "Save".to_string(),
                        color: "primary".to_string(),
                        size: "md".to_string(),
                    },
                ],
            },
        ]);
        selected_id.set(Some(1));
        next_id.set(7);
    };

    view! {
        <div class="flex flex-col gap-4">
            // Header
            <div class="flex items-center justify-between flex-wrap gap-2">
                <div>
                    <h1 class="text-2xl font-bold">"Playground"</h1>
                    <p class="text-base-content/60 text-sm">"Design layouts with containers and components"</p>
                </div>
                <button
                    class="btn btn-sm btn-error btn-outline"
                    on:click=move |_| {
                        canvas.set(Vec::new());
                        selected_id.set(None);
                    }
                >
                    "Clear All"
                </button>
            </div>

            <div class="flex flex-wrap items-center gap-2">
                <span class="text-xs uppercase tracking-wider text-base-content/50">"Templates"</span>
                <button class="btn btn-xs btn-outline" on:click=move |_| load_dashboard_template()>"Dashboard"</button>
                <button class="btn btn-xs btn-outline" on:click=move |_| load_marketing_template()>"Landing"</button>
                <button class="btn btn-xs btn-outline" on:click=move |_| load_form_template()>"Form"</button>
            </div>

            // Three-panel layout
            <div class="flex flex-row h-[calc(100vh-12rem)] border border-base-300 rounded-box overflow-hidden">
                // ---- Left panel: Palette + Tree ----
                <div class="w-64 shrink-0 overflow-y-auto bg-base-200 border-r border-base-300">
                    <div class="p-3">
                        <h3 class="font-bold text-xs uppercase tracking-wider text-base-content/50 mb-2">"Palette"</h3>

                        // Containers
                        <ul class="menu menu-xs w-full">
                            <li class="menu-title">"Containers"</li>
                            {CONTAINER_ITEMS.iter().map(|(kind, label)| {
                                let kind = *kind;
                                let label = *label;
                                view! {
                                    <li>
                                        <button on:click=move |_| add_container(kind)>
                                            {label}
                                        </button>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>

                        // Actions
                        <ul class="menu menu-xs w-full">
                            <li class="menu-title">"Actions"</li>
                            {ACTION_ITEMS.iter().map(|(kind, label)| {
                                let kind = *kind;
                                let label = *label;
                                view! {
                                    <li>
                                        <button on:click=move |_| add_component(kind)>
                                            {label}
                                        </button>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>

                        // Data Display
                        <ul class="menu menu-xs w-full">
                            <li class="menu-title">"Data Display"</li>
                            {DATA_DISPLAY_ITEMS.iter().map(|(kind, label)| {
                                let kind = *kind;
                                let label = *label;
                                view! {
                                    <li>
                                        <button on:click=move |_| add_component(kind)>
                                            {label}
                                        </button>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>

                        // Data Input
                        <ul class="menu menu-xs w-full">
                            <li class="menu-title">"Data Input"</li>
                            {DATA_INPUT_ITEMS.iter().map(|(kind, label)| {
                                let kind = *kind;
                                let label = *label;
                                view! {
                                    <li>
                                        <button on:click=move |_| add_component(kind)>
                                            {label}
                                        </button>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>

                        // Feedback
                        <ul class="menu menu-xs w-full">
                            <li class="menu-title">"Feedback"</li>
                            {FEEDBACK_ITEMS.iter().map(|(kind, label)| {
                                let kind = *kind;
                                let label = *label;
                                view! {
                                    <li>
                                        <button on:click=move |_| add_component(kind)>
                                            {label}
                                        </button>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>

                        // Layout
                        <ul class="menu menu-xs w-full">
                            <li class="menu-title">"Layout"</li>
                            {LAYOUT_ITEMS.iter().map(|(kind, label)| {
                                let kind = *kind;
                                let label = *label;
                                view! {
                                    <li>
                                        <button on:click=move |_| add_component(kind)>
                                            {label}
                                        </button>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>

                        // Navigation
                        <ul class="menu menu-xs w-full">
                            <li class="menu-title">"Navigation"</li>
                            {NAVIGATION_ITEMS.iter().map(|(kind, label)| {
                                let kind = *kind;
                                let label = *label;
                                view! {
                                    <li>
                                        <button on:click=move |_| add_component(kind)>
                                            {label}
                                        </button>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>

                        // Component Tree
                        <div class="divider text-xs uppercase tracking-wider">"Tree"</div>
                        <div class="space-y-0.5">
                            {move || {
                                let nodes = canvas.get();
                                let mut items = Vec::new();
                                collect_tree_items(&nodes, 0, &mut items);
                                if items.is_empty() {
                                    view! {
                                        <p class="text-base-content/40 text-xs italic px-2">"Empty canvas"</p>
                                    }.into_any()
                                } else {
                                    items.into_iter().map(|(id, depth, label, is_container)| {
                                        let is_sel = move || selected_id.get() == Some(id);
                                        let pl = format!("padding-left: {}rem", depth as f32 * 0.75);
                                        view! {
                                            <button
                                                class="flex items-center gap-1 w-full text-left px-2 py-0.5 rounded text-xs hover:bg-base-300 transition-colors"
                                                class:bg-primary=is_sel
                                                class:text-primary-content=is_sel
                                                style={pl}
                                                on:click=move |_| selected_id.set(Some(id))
                                            >
                                                <span class="opacity-40 font-mono">{if is_container { "+" } else { "-" }}</span>
                                                <span>{label}</span>
                                                <span class="opacity-30 ml-auto font-mono">{"#"}{id}</span>
                                            </button>
                                        }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>

                // ---- Center panel: Canvas ----
                <div class="flex-1 bg-base-100 overflow-auto p-4">
                    {move || {
                        let nodes = canvas.get();
                        if nodes.is_empty() {
                            view! {
                                <div class="flex items-center justify-center h-full text-base-content/30">
                                    <div class="text-center">
                                        <p class="text-lg mb-2">"Start by adding components"</p>
                                        <p class="text-sm">"Click items in the palette to add them"</p>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="space-y-3"
                                    on:click=move |_| selected_id.set(None)
                                >
                                    {nodes.iter().map(|node| {
                                        render_canvas_node(node, selected_id)
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }}
                </div>

                // ---- Right panel: Properties ----
                <div class="w-72 shrink-0 overflow-y-auto bg-base-200 border-l border-base-300">
                    <div class="p-3">
                        <h3 class="font-bold text-xs uppercase tracking-wider text-base-content/50 mb-3">"Properties"</h3>
                        {move || {
                            let sel = selected_id.get();
                            let nodes = canvas.get();
                            match sel.and_then(|id| find_node(&nodes, id).cloned()) {
                                None => view! {
                                    <p class="text-sm text-base-content/40 italic">
                                        "Select a component or container to edit its properties"
                                    </p>
                                }.into_any(),
                                Some(CanvasNode::Component { id, kind, text, color, size }) => {
                                    let current_text = text;
                                    let current_color = color;
                                    let current_size = size;
                                    view! {
                                        <div class="space-y-4">
                                            <div>
                                                <span class="text-xs text-base-content/50">"Component"</span>
                                                <p class="font-bold text-sm">{kind.label()} " #" {id}</p>
                                            </div>

                                            // Text
                                            <div class="form-control">
                                                <label class="label py-0.5">
                                                    <span class="label-text text-xs">"Text"</span>
                                                </label>
                                                <input
                                                    type="text"
                                                    class="input input-bordered input-sm"
                                                    value={current_text}
                                                    on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        canvas.update(|nodes| {
                                                            update_node_in_tree(nodes, id, &|node| {
                                                                if let CanvasNode::Component { text, .. } = node {
                                                                    *text = val.clone();
                                                                }
                                                            });
                                                        });
                                                    }
                                                />
                                            </div>

                                            // Color
                                            <div class="form-control">
                                                <label class="label py-0.5">
                                                    <span class="label-text text-xs">"Color"</span>
                                                </label>
                                                <select
                                                    class="select select-bordered select-sm"
                                                    on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        canvas.update(|nodes| {
                                                            update_node_in_tree(nodes, id, &|node| {
                                                                if let CanvasNode::Component { color, .. } = node {
                                                                    *color = val.clone();
                                                                }
                                                            });
                                                        });
                                                    }
                                                >
                                                    <option value="" selected={current_color.is_empty()}>"None"</option>
                                                    {["primary", "secondary", "accent", "info", "success", "warning", "error"].iter().map(|c| {
                                                        let c = *c;
                                                        let sel = c == current_color;
                                                        view! { <option value={c} selected=sel>{c}</option> }
                                                    }).collect_view()}
                                                </select>
                                            </div>

                                            // Size
                                            <div class="form-control">
                                                <label class="label py-0.5">
                                                    <span class="label-text text-xs">"Size"</span>
                                                </label>
                                                <select
                                                    class="select select-bordered select-sm"
                                                    on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        canvas.update(|nodes| {
                                                            update_node_in_tree(nodes, id, &|node| {
                                                                if let CanvasNode::Component { size, .. } = node {
                                                                    *size = val.clone();
                                                                }
                                                            });
                                                        });
                                                    }
                                                >
                                                    {["xs", "sm", "md", "lg"].iter().map(|s| {
                                                        let s = *s;
                                                        let sel = s == current_size;
                                                        view! { <option value={s} selected=sel>{s}</option> }
                                                    }).collect_view()}
                                                </select>
                                            </div>

                                            // Delete
                                            <div class="pt-2">
                                                <button
                                                    class="btn btn-sm btn-error btn-outline w-full"
                                                    on:click=move |_| delete_selected()
                                                >
                                                    "Delete"
                                                </button>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                                Some(CanvasNode::Container { id, kind, gap, padding, .. }) => {
                                    let current_gap = gap;
                                    let current_padding = padding;
                                    view! {
                                        <div class="space-y-4">
                                            <div>
                                                <span class="text-xs text-base-content/50">"Container"</span>
                                                <p class="font-bold text-sm">{kind.label()} " #" {id}</p>
                                            </div>

                                            // Container Kind
                                            <div class="form-control">
                                                <label class="label py-0.5">
                                                    <span class="label-text text-xs">"Kind"</span>
                                                </label>
                                                <select
                                                    class="select select-bordered select-sm"
                                                    on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        let new_kind = match val.as_str() {
                                                            "Flex Row" => ContainerKind::FlexRow,
                                                            "Flex Col" => ContainerKind::FlexCol,
                                                            "Grid 2-Col" => ContainerKind::Grid2,
                                                            "Grid 3-Col" => ContainerKind::Grid3,
                                                            _ => return,
                                                        };
                                                        canvas.update(|nodes| {
                                                            update_node_in_tree(nodes, id, &|node| {
                                                                if let CanvasNode::Container { kind: k, .. } = node {
                                                                    *k = new_kind;
                                                                }
                                                            });
                                                        });
                                                    }
                                                >
                                                    {[ContainerKind::FlexRow, ContainerKind::FlexCol, ContainerKind::Grid2, ContainerKind::Grid3].iter().map(|k| {
                                                        let k = *k;
                                                        let sel = k == kind;
                                                        view! { <option value={k.label()} selected=sel>{k.label()}</option> }
                                                    }).collect_view()}
                                                </select>
                                            </div>

                                            // Gap
                                            <div class="form-control">
                                                <label class="label py-0.5">
                                                    <span class="label-text text-xs">"Gap"</span>
                                                </label>
                                                <select
                                                    class="select select-bordered select-sm"
                                                    on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        canvas.update(|nodes| {
                                                            update_node_in_tree(nodes, id, &|node| {
                                                                if let CanvasNode::Container { gap, .. } = node {
                                                                    *gap = val.clone();
                                                                }
                                                            });
                                                        });
                                                    }
                                                >
                                                    {["gap-1", "gap-2", "gap-3", "gap-4", "gap-5", "gap-6", "gap-7", "gap-8"].iter().map(|g| {
                                                        let g = *g;
                                                        let sel = g == current_gap;
                                                        view! { <option value={g} selected=sel>{g}</option> }
                                                    }).collect_view()}
                                                </select>
                                            </div>

                                            // Padding
                                            <div class="form-control">
                                                <label class="label py-0.5">
                                                    <span class="label-text text-xs">"Padding"</span>
                                                </label>
                                                <select
                                                    class="select select-bordered select-sm"
                                                    on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        canvas.update(|nodes| {
                                                            update_node_in_tree(nodes, id, &|node| {
                                                                if let CanvasNode::Container { padding, .. } = node {
                                                                    *padding = val.clone();
                                                                }
                                                            });
                                                        });
                                                    }
                                                >
                                                    {["p-0", "p-1", "p-2", "p-3", "p-4", "p-5", "p-6", "p-7", "p-8"].iter().map(|p| {
                                                        let p = *p;
                                                        let sel = p == current_padding;
                                                        view! { <option value={p} selected=sel>{p}</option> }
                                                    }).collect_view()}
                                                </select>
                                            </div>

                                            // Delete
                                            <div class="pt-2">
                                                <button
                                                    class="btn btn-sm btn-error btn-outline w-full"
                                                    on:click=move |_| delete_selected()
                                                >
                                                    "Delete"
                                                </button>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }
                        }}
                    </div>
                </div>
            </div>

            // Bottom panel: Generated Code
            <div>
                {move || {
                    let code = generated_code.get();
                    view! {
                        <CodeBlock code={code} title="Generated Leptos Code".to_string() />
                    }
                }}
            </div>
        </div>
    }
}

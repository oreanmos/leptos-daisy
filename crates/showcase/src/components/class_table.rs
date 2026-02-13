use leptos::prelude::*;

/// A single entry in the class reference table.
#[derive(Clone, Debug)]
pub struct ClassEntry {
    pub name: &'static str,
    pub type_label: &'static str,
    pub description: &'static str,
}

/// A table showing DaisyUI class names, their types, and descriptions.
/// Used at the top of each component showcase page as an API reference.
#[component]
pub fn ClassTable(#[prop(into)] entries: Vec<ClassEntry>) -> impl IntoView {
    view! {
        <div class="overflow-x-auto">
            <table class="table table-zebra table-sm">
                <thead>
                    <tr>
                        <th class="font-mono">"Class name"</th>
                        <th>"Type"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    {entries.into_iter().map(|entry| {
                        let type_badge_class = match entry.type_label {
                            "base" => "badge badge-neutral badge-sm",
                            "modifier" => "badge badge-primary badge-sm",
                            "color" => "badge badge-secondary badge-sm",
                            "size" => "badge badge-accent badge-sm",
                            "style" => "badge badge-info badge-sm",
                            "state" => "badge badge-warning badge-sm",
                            "responsive" => "badge badge-success badge-sm",
                            _ => "badge badge-ghost badge-sm",
                        };
                        view! {
                            <tr>
                                <td><code class="text-sm bg-base-200 px-1.5 py-0.5 rounded">{entry.name}</code></td>
                                <td><span class=type_badge_class>{entry.type_label}</span></td>
                                <td class="text-base-content/80">{entry.description}</td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
}

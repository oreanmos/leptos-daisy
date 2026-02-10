use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TablePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Table"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Table"</h2>
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>"Name"</TableHead>
                            <TableHead>"Job"</TableHead>
                            <TableHead>"Company"</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Cy Ganderton"</TableCell>
                            <TableCell>"Quality Control Specialist"</TableCell>
                            <TableCell>"Littel, Schaden and Vandervort"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Hart Hagerty"</TableCell>
                            <TableCell>"Desktop Support Technician"</TableCell>
                            <TableCell>"Zemlak, Daniel and Leannon"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Brice Swyre"</TableCell>
                            <TableCell>"Tax Accountant"</TableCell>
                            <TableCell>"Carroll Group"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Extra Small"</span>
                        <Table size={TableSize::ExtraSmall}>
                            <TableHeader>
                                <TableRow>
                                    <TableHead>"Name"</TableHead>
                                    <TableHead>"Role"</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <TableRow>
                                    <TableCell>"John"</TableCell>
                                    <TableCell>"Admin"</TableCell>
                                </TableRow>
                            </TableBody>
                        </Table>
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Small"</span>
                        <Table size={TableSize::Small}>
                            <TableHeader>
                                <TableRow>
                                    <TableHead>"Name"</TableHead>
                                    <TableHead>"Role"</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <TableRow>
                                    <TableCell>"John"</TableCell>
                                    <TableCell>"Admin"</TableCell>
                                </TableRow>
                            </TableBody>
                        </Table>
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Large"</span>
                        <Table size={TableSize::Large}>
                            <TableHeader>
                                <TableRow>
                                    <TableHead>"Name"</TableHead>
                                    <TableHead>"Role"</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <TableRow>
                                    <TableCell>"John"</TableCell>
                                    <TableCell>"Admin"</TableCell>
                                </TableRow>
                            </TableBody>
                        </Table>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Zebra Striped"</h2>
                <Table zebra=true>
                    <TableHeader>
                        <TableRow>
                            <TableHead>"Name"</TableHead>
                            <TableHead>"Job"</TableHead>
                            <TableHead>"Favorite Color"</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Cy Ganderton"</TableCell>
                            <TableCell>"Quality Control Specialist"</TableCell>
                            <TableCell>"Blue"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Hart Hagerty"</TableCell>
                            <TableCell>"Desktop Support Technician"</TableCell>
                            <TableCell>"Purple"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Brice Swyre"</TableCell>
                            <TableCell>"Tax Accountant"</TableCell>
                            <TableCell>"Red"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Marjy Ferencz"</TableCell>
                            <TableCell>"Office Assistant I"</TableCell>
                            <TableCell>"Crimson"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Footer"</h2>
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>"Item"</TableHead>
                            <TableHead>"Quantity"</TableHead>
                            <TableHead class="text-right">"Price"</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Product A"</TableCell>
                            <TableCell>"2"</TableCell>
                            <TableCell class="text-right">"$20.00"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Product B"</TableCell>
                            <TableCell>"1"</TableCell>
                            <TableCell class="text-right">"$35.00"</TableCell>
                        </TableRow>
                    </TableBody>
                    <TableFooter>
                        <TableRow>
                            <TableCell>"Total"</TableCell>
                            <TableCell></TableCell>
                            <TableCell class="text-right font-bold">"$55.00"</TableCell>
                        </TableRow>
                    </TableFooter>
                </Table>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Pinned Rows"</h2>
                <div class="overflow-x-auto h-48">
                    <Table pin_rows=true>
                        <TableHeader>
                            <TableRow>
                                <TableHead>"Name"</TableHead>
                                <TableHead>"Position"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow><TableCell>"Row 1"</TableCell><TableCell>"Data"</TableCell></TableRow>
                            <TableRow><TableCell>"Row 2"</TableCell><TableCell>"Data"</TableCell></TableRow>
                            <TableRow><TableCell>"Row 3"</TableCell><TableCell>"Data"</TableCell></TableRow>
                            <TableRow><TableCell>"Row 4"</TableCell><TableCell>"Data"</TableCell></TableRow>
                            <TableRow><TableCell>"Row 5"</TableCell><TableCell>"Data"</TableCell></TableRow>
                            <TableRow><TableCell>"Row 6"</TableCell><TableCell>"Data"</TableCell></TableRow>
                            <TableRow><TableCell>"Row 7"</TableCell><TableCell>"Data"</TableCell></TableRow>
                            <TableRow><TableCell>"Row 8"</TableCell><TableCell>"Data"</TableCell></TableRow>
                        </TableBody>
                    </Table>
                </div>
            </section>
        </div>
    }
}

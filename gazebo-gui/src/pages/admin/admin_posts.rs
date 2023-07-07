// Admin all posts/entries page

use yew::prelude::*;

use crate::components::{
    admin_bar::AdminBar, admin_menu::AdminMenu, button_add_new_entry::ButtonAddNewEntry,
    table_entries::EntriesTable,
};

#[function_component(AdminPosts)]
pub fn admin_posts() -> Html {
    html! {
        <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <div class={"gb-admin-panel"}>
                <AdminMenu />

                <div class={"gb-admin-main"}>
                    <h1>{"Posts"}</h1>
                    <ButtonAddNewEntry />
                    <EntriesTable />
                </div>
            </div>

        </main>
    }
}

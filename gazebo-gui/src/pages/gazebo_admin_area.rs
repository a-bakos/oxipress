// This will be the admin (management) area

use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::admin_menu::AdminMenu;

#[function_component(GazeboAdminArea)]
pub fn gazebo_admin_area() -> Html {
    html! {
        <main id={"gb-gui-admin-area"}>
            <AdminBar />

            <div class={"gb-admin-panel"}>
                <AdminMenu />

                <div class={"gb-admin-main"}>
                   <h1>{"Dashboard"}</h1>
                </div>
            </div>

        </main>
    }
}

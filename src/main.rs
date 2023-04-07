extern crate core;

mod allocator;
mod app;
mod consts;
mod database;
mod dates;
mod error;
mod http;
mod posts;
mod url;
mod users;

use crate::database::db;
use crate::posts::post::OX_Post;
use posts::functions as post_functions;
fn main() {
    // Start the App
    let mut app = app::App::init(
        "Rusty CMS Experiment App".to_string(),
        consts::VERSION.to_string(),
    );

    // Let's create a new user and insert it
    let test_user = users::user::User::new(
        "First".to_string(),
        "Last".to_string(),
        "testuser".to_string(),
        "test@test.com".to_string(),
        users::roles::UserRole::Admin,
        "123456".to_string(),
    );
    let _is_user_inserted: bool = users::user::User::insert(&mut app, test_user);

    // Mimic a user login request
    users::user::User::login(&mut app, "test@test.com");
    //dbg!(&app.users);

    // Imitate editing a new posts - Eg. User clicks a "new posts" button
    let mut post = OX_Post::draft(&mut app, posts::entry_type::EntryType::Post);
    // User adds a title to the posts (permalink auto-generated)
    post.add_title(
        "This is the ultimate post of an item @ new #posts & something of value".to_string(),
        true,
    );

    // Imitate editing a second new posts - Eg. User clicks a "new posts" button
    let mut post_2 = OX_Post::draft(&mut app, posts::entry_type::EntryType::Post);
    // User adds a title to the posts (permalink auto-generated)
    post_2.add_title("This is a second posts".to_string(), true);

    // Check the App state
    dbg!(&app.resources);

    // The storage methods will be part of the OX_Post routine
    let to_store: Vec<&OX_Post> = vec![&post, &post_2];
    #[allow(clippy::let_unit_value)]
    let _store = db::store_post(to_store);
    #[allow(clippy::let_unit_value)]
    let _get_post: Option<OX_Post> = post_functions::get_post_by_id(1).unwrap();
    // dbg!(&_get_post);

    // dbg!(crate::database::columns::get_columns());

    dbg!(&app.start);

    #[allow(clippy::let_unit_value)]
    let _all_posts = post_functions::get_all_posts();
    // dbg!(_all_posts);
}

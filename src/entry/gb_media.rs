use crate::entry::entry_id::EntryID;
use crate::users::user::AccountID;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
struct GB_Media {
    id: EntryID,
    uploader: AccountID,
    attached_to: Vec<EntryID>,
    date_publish: String,
    date_modified: String,
    slug: String,
    title: String,
    description: Option<String>,
    alt_text: Option<String>,
}
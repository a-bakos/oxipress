// Mock CSV database
use crate::consts;
use crate::posts::entry_type::EntryType;
use crate::posts::post::{EntryID, OX_Post};
use crate::users::user::UserID;
use csv::{Reader, ReaderBuilder, StringRecord, WriterBuilder};
use std::error::Error;

pub struct Database {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub charset: String,
    pub table_prefix: String,
}

impl Database {
    pub fn new(
        name: String,
        user: String,
        password: String,
        host: String,
        charset: String,
        table_prefix: String,
    ) -> Self {
        Self {
            name,
            user,
            password,
            host,
            charset,
            table_prefix,
        }
    }

    pub fn get_row(id: u32) {
        todo!()
    }
}

pub fn get_post_by_id(post_id: u32) -> Result<Option<OX_Post>, Box<dyn Error>> {
    let csv_db = parse_csv(consts::FILE_PATH)?;
    let mut found_post;
    let mut post = None;
    for row in csv_db.iter() {
        if let Some(id) = row.get(0) {
            if id == post_id.to_string() {
                found_post = row;

                // Turn into OX_Post
                post = Some(OX_Post {
                    id: EntryID(found_post.get(0).unwrap().parse::<u32>().unwrap()),
                    id_author: UserID(found_post.get(1).unwrap().parse::<u32>().unwrap()),
                    id_parent: None,
                    date_publish: found_post.get(3).unwrap().to_string(),
                    date_modified: found_post.get(4).unwrap().to_string(),
                    slug: Some(found_post.get(5).unwrap().to_string()),
                    the_type: EntryType::Post,
                    title: Some(found_post.get(7).unwrap().to_string()),
                    excerpt: None,
                    content: None,
                    password: None,
                });
                break;
            }
        }
    }

    Ok(post)
}

pub fn parse_csv(path: &str) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    println!("Parsing CSV: {:?}", path);
    let mut csv_result: Vec<StringRecord> = Vec::new();
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    for row in reader.records() {
        let record = row?;
        csv_result.push(record);
    }
    Ok(csv_result)
}

pub fn write_to_csv(path: &str, posts: Vec<&OX_Post>) -> Result<(), Box<dyn Error>> {
    println!("Writing CSV: {:?}", path);
    let mut writer = WriterBuilder::new().from_path(path)?;
    for single_post in posts.iter() {
        writer.write_record([
            single_post.id.to_string(),
            single_post.id_author.to_string(),
            single_post.id_parent.unwrap_or_default().to_string(),
            single_post.date_publish.to_string(),
            single_post.date_modified.to_string(),
            single_post.slug.clone().unwrap_or_default().to_string(),
            single_post.the_type.to_string(),
            single_post.title.clone().unwrap_or_default().to_string(),
            single_post.excerpt.clone().unwrap_or_default().to_string(),
            single_post.content.clone().unwrap_or_default().to_string(),
            single_post.password.clone().unwrap_or_default().to_string(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

pub fn store(posts: Vec<&OX_Post>) {
    println!("Storing posts: {:?}", posts);
    let _write = write_to_csv(consts::FILE_PATH, posts);
}

pub fn add(post: &OX_Post) {}
pub fn update(post: &OX_Post) {}
pub fn delete(post: &OX_Post) {}
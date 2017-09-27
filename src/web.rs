#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate shiftrss;

use rocket::http::ContentType;
use rocket::response::Response;
use rocket::response::content::Content;
use shiftrss::*;

#[get("/")]
// fn hello() -> rocket::Response<'static> {
fn hello() -> rocket::response::Content<String> {
    let item_filter = ItemFilter { include_exclude: IncludeExclude::Include,
                                   item_field: ItemField::ItemTitle,
                                   item_contains: ItemContains::ItemDoesContain,
                                   filter_string: "Alternatives".to_owned() };
    let channel = open_rss_uri("http://localhost:8000/rss.xml", item_filter);
    // let content_type = ContentType::XML;    
    // let items_string = format!("{:?}", items);
    // let response = Response::build().header(content_type).finalize();
    // response.set_streamed_body(channel.to_string().as_bytes());
    // response
    Content(ContentType::XML, channel.to_string())
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}

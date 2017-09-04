extern crate rss;

use std::fs::File;
use std::io::BufReader;
use rss::Channel;
use rss::Item;

pub fn open_rss_file(filename: String, item_filter: ItemFilter) {
    let file = File::open(filename).unwrap();
    let mut channel = Channel::read_from(BufReader::new(file)).unwrap();
    filter_feed(&mut channel, item_filter);
    println!("{:?}", channel.items);
}

pub struct ItemFilter {
    pub include_exclude: IncludeExclude,
    pub item_field: ItemField,
    pub item_contains: ItemContains,
    pub filter_string: String,
}

pub enum IncludeExclude {
    Exclude,
    Include,
}

pub enum ItemField {
    ItemTitle,
    ItemDescription,
    ItemLink,
}

pub enum ItemContains {
    ItemDoesNotContain,
    ItemDoesContain,
}

pub fn filter_feed(mut channel: &mut Channel, item_filter: ItemFilter) -> &mut Channel {
    let incl_neg = |x: bool| {
        match item_filter.include_exclude {
            IncludeExclude::Exclude => !x,
            IncludeExclude::Include => x,
        }
    };
    let contain_neg = |x: bool| {
        match item_filter.item_contains {
            ItemContains::ItemDoesNotContain => !x,
            ItemContains::ItemDoesContain => x,
        }
    };
    let filter_func = |item: &Item| {
        let ic = item.clone();
        match item_filter.item_field {
            ItemField::ItemTitle => {
                let val_string = ic.title.unwrap_or("".to_string());
                let contains_val = val_string.contains(&item_filter.filter_string);
                incl_neg(contain_neg(contains_val))
            },
            ItemField::ItemDescription => {
                let val_string = ic.description.unwrap_or("".to_string());
                let contains_val = val_string.contains(&item_filter.filter_string);
                incl_neg(contain_neg(contains_val))
            },
            ItemField::ItemLink => {
                let val_string = ic.link.unwrap_or("".to_string());
                let contains_val = val_string.contains(&item_filter.filter_string);
                incl_neg(contain_neg(contains_val))
            },
        }
    };
    filter_items(&mut channel, filter_func);
    channel
}

pub fn it_can_read_an_rss_file() {
    // let file = File::open("tests/data/bitemyapp_rss_small.xml").unwrap();
    // let mut channel = Channel::read_from(BufReader::new(file)).unwrap();
    // let file = File::open("tests/data/bitemyapp_rss.xml").unwrap();
    // println!("{:?}", channel);
    // println!("{:?}", channel.items);
    // println!("{:?}", channel.items[0]);
    // println!("{:?}", channel.items[1]);
    // one_item(&mut channel);
    // filter_items(&mut channel,
    //              |item| {
    //                  let ic = item.clone();
    //                  ic.title.unwrap_or("".to_string()).contains("Study")
    //              }
    // );
    let item_filter = ItemFilter {
        include_exclude: IncludeExclude::Include,
        // include_exclude: IncludeExclude::Exclude,
        item_field: ItemField::ItemDescription,
        item_contains: ItemContains::ItemDoesContain,
        // item_contains: ItemContains::ItemDoesNotContain,
        filter_string: "This article is".to_string(),
    };
    open_rss_file("tests/data/bitemyapp_rss_small.xml".to_string(), item_filter);
    // filter_feed(&mut channel, item_filter);
    // println!("{:?}", channel.items);
}

pub fn filter_items<F>(channel: &mut Channel, fltr: F) -> &mut Channel
    where
    F: FnMut(&rss::Item) -> bool {
    channel.items.retain(fltr);
    channel
}

pub fn one_item(channel: &mut Channel) -> &mut Channel {
    let first_item = channel.items[0].clone();
    let mut new_items = Vec::new();
    new_items.push(first_item);
    channel.items = new_items;
    channel
}

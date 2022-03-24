#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod web;
mod data;
mod database;

fn main() {
    //连接数据
    database::connect::main();

    //初始化web
    web::main();
}
// `forbid(dead_code)` 在 `--test` 构建下与测试脚手架注入的 allow 冲突（E0453），
// 因此仅在非测试构建时启用。
#![cfg_attr(not(test), forbid(dead_code))]

pub mod clients;
pub mod config;
pub mod contracts;
pub mod database;
pub mod entity;
pub mod error;
pub mod http;
pub mod realtime;
pub mod repositories;
pub mod routes;
pub mod runtime_events;
pub mod services;
pub mod state;
pub mod time;

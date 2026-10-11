pub mod filters;
pub mod rest;
pub mod retry;
pub mod ws;

// 只重导出对外的入口类型；其余符号请按真实模块路径引用
// （clients::binance::retry::xxx / filters::xxx / ws::xxx）。
pub use rest::BinanceFuturesAdapter;

/**
 * IP地址搜索性能基准测试模块
 * 
 * 该模块提供了对IP地址搜索相关功能的性能测试，包括：
 * - IP地址搜索性能
 * - 缓存块获取性能
 * - 全量缓存获取性能
 * - 向量索引缓存获取性能
 * 
 * 基准测试组配置
 * --------
 * 配置所有基准测试函数，包括：
 * - IP地址搜索测试
 * - 缓存块获取测试
 * - 全量缓存获取测试
 * - 向量索引缓存获取测试
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand;
use xdb::searcher::{
    get_block_by_size, get_full_cache, get_vector_index_cache, search_by_ip, searcher_init,
};

/**
 * IP地址搜索性能测试
 * 
 * 测试随机IP地址的搜索性能，包括：
 * - 初始化搜索器
 * - 随机生成IP地址
 * - 执行搜索操作
 */
fn search_by_ip_bench(c: &mut Criterion) {
    c.bench_function("search_by_ip_bench", |b| {
        searcher_init(None);
        b.iter(|| {
            search_by_ip(rand::random::<u32>()).unwrap();
        })
    });
}

/**
 * 缓存块获取性能测试
 * 
 * 测试根据大小获取缓存块的性能，包括：
 * - 获取全量缓存
 * - 随机生成块大小
 * - 获取指定大小的缓存块
 */
fn get_block_by_size_bench(c: &mut Criterion) {
    c.bench_function("get_block_by_size_bench", |b| {
        b.iter(|| {
            black_box(get_block_by_size(
                get_full_cache(),
                rand::random::<u16>() as usize,
                4,
            ));
        })
    });
}

/**
 * 全量缓存获取性能测试
 * 
 * 测试获取全量缓存的性能。
 */
fn get_full_cache_bench(c: &mut Criterion) {
    c.bench_function("get_full_cache_bench", |b| {
        b.iter(|| {
            black_box(get_full_cache());
        })
    });
}

/**
 * 向量索引缓存获取性能测试
 * 
 * 测试获取向量索引缓存的性能。
 */
fn get_vec_index_cache_bench(c: &mut Criterion) {
    c.bench_function("get_vec_index_cache_bench", |b| {
        b.iter(|| {
            black_box(get_vector_index_cache());
        })
    });
}

criterion_group!(
    benches,
    search_by_ip_bench,
    get_block_by_size_bench,
    get_full_cache_bench,
    get_vec_index_cache_bench,
);
criterion_main!(benches);

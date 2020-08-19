use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simple_list::LinkedList;

fn new_list() -> LinkedList<i32> {
    let list = LinkedList::<i32>::new();
    list
}

fn add_list(n: i32, list: &mut LinkedList<i32>) {
    for i in 0..n {
        list.add(i);
    }
}

fn list_add(n: i32) {
    let mut list = new_list();

    add_list(n, &mut list);
}

fn list_get(n: i32) -> i32{
    let mut list = new_list();
    let mut x = 0;

    add_list(n, &mut list);

    for i in 0..n {
        x += list.get(i as u32).unwrap();
    }

    x
}

fn list_pop (n: i32) -> i32 {
    let mut list = new_list();
    let mut x = 0;

    add_list(n, &mut list);

    for _ in 0..n {
        x += list.pop().unwrap();
    }

    x
}

fn list_del(n: i32) {
    let mut list = new_list();

    add_list(n, &mut list);

    for i in 0..n {
        list.del(i as u32);
    }
}

fn list_size(n: i32) -> i32 {
    let mut list = new_list();
    let mut x = 0;

    add_list(n, &mut list);

    for _ in 0..n {
        x += list.size() as i32;
    }

    x
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("list_add 20", |b| b.iter(|| list_add(black_box(20))));
    c.bench_function("list_get 20", |b| b.iter(|| list_get(black_box(20))));
    c.bench_function("list_pop 20", |b| b.iter(|| list_pop(black_box(20))));
    c.bench_function("list_del 20", |b| b.iter(|| list_del(black_box(20))));
    c.bench_function("list_size 20", |b| b.iter(|| list_size(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
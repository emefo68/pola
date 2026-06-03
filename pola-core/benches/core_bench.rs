use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box; // <-- New standard library import

// Import the public function from your library
use pola_core::parser::get_word_count;

fn bench_get_word_count(c: &mut Criterion) {
    // We will use the same string from your test as the benchmarking payload
    let text = "Java Server Pages (JSP) are a variant of servlets
     • When creating a servlet, we write Java-code, and use Java-
     statements to write HTML into a stream
     • When using JSP we write HTML, and add embedded Java-
     statements";

    c.bench_function("word_count_short_text", |b| {
        // black_box prevents the compiler from optimizing away the function call
        // by pretending the input could change at any moment
        b.iter(|| get_word_count(black_box(text)))
    });
}

criterion_group!(benches, bench_get_word_count);
criterion_main!(benches);

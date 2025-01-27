use criterion::{criterion_group, criterion_main, Criterion};
use sudoku_solver_and_generator::logic::game::Game;
use sudoku_solver_and_generator::logic::solver::{generate, Solver}; // Adjust the path as necessary

fn benchmark_solve(c: &mut Criterion) {
    let g = Game::new(9, 3, 3);
    let puzzle = generate(g).unwrap();
    let mut solver = Solver::new(puzzle.clone(), true);
    c.bench_function("solve randomly", |b| b.iter(|| solver.solve()));
}

fn benchmark_solve_deterministic(c: &mut Criterion) {
    let g = Game::new(9, 3, 3);
    let puzzle = generate(g).unwrap();
    let mut solver = Solver::new(puzzle.clone(), false);
    c.bench_function("solve deterministically", |b| b.iter(|| solver.solve()));
}

criterion_group!(benches, benchmark_solve, benchmark_solve_deterministic);
criterion_main!(benches);
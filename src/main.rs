use std::fmt::Display;

use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    let args = Args::parse(&mut std::env::args().map(|a| a.to_string()));
    match args {
        Args::Help => println!("{}", Args::help()),
        Args::Run(run_args) => run(&run_args),
    }
}

fn run(run_args: &RunArgs) {
    let mut session = RunSession::default();
    let mut rng = run_args.rng::<ChaCha8Rng>();
    while !session.is_finished(run_args) {
        let choice = session.rng_gen(&mut rng);
        let correct = session.rng_gen(&mut rng);
        session.step(choice, correct);
    }
    println!("{}", &session);
}

#[derive(Debug, Default)]
struct RunSession {
    i: usize,
    correct_if_changed: usize,
    correct_if_not_changed: usize,
}

impl RunSession {
    fn is_finished(&self, run_args: &RunArgs) -> bool {
        self.i >= run_args.num_iter
    }
    fn rng_gen(&mut self, rng: &mut dyn RngCore) -> usize {
        rng.gen_range(0..=2)
    }
    fn step(&mut self, choice: usize, correct: usize) {
        self.i += 1;
        if choice == correct {
            self.correct_if_not_changed += 1;
        } else {
            self.correct_if_changed += 1;
        }
    }
}

impl Display for RunSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_digits = self.i.to_string().len();
        let accuracy_if_changed = 100.0 * self.correct_if_changed as f32 / self.i as f32;
        let accuracy_if_not_changed = 100.0 * self.correct_if_not_changed as f32 / self.i as f32;

        write!(
            f,
            r"Iteration:   {}
[Correct]
changed=Yes: {:num_digits$} ({:>7.3}%)
changed=No : {:num_digits$} ({:>7.3}%)
",
            self.i,
            self.correct_if_changed,
            accuracy_if_changed,
            self.correct_if_not_changed,
            accuracy_if_not_changed,
        )
    }
}

#[derive(Debug)]
struct RunArgs {
    seed: Option<u64>,
    num_iter: usize,
}

impl RunArgs {
    fn rng<R: SeedableRng>(&self) -> R {
        self.seed.map_or_else(R::from_entropy, R::seed_from_u64)
    }
}

#[derive(Debug)]
enum Args {
    Run(RunArgs),
    Help,
}

impl Args {
    fn parse(args: &mut dyn Iterator<Item = String>) -> Self {
        let mut seed = None;
        let mut num_iter = 1;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => return Self::Help,
                "-s" | "--seed" => {
                    let s = args.next().expect("no seed given");
                    let s = s
                        .parse::<u64>()
                        .unwrap_or_else(|e| panic!("invalid seed: {s} ({e})"));
                    seed = Some(s);
                }
                "-n" => {
                    let n = args
                        .next()
                        .unwrap_or_else(|| panic!("no num iteration given"));
                    let n = n
                        .parse::<usize>()
                        .unwrap_or_else(|e| panic!("invalid num iteration: {n} ({e})"));
                    num_iter = n;
                }
                _ => {}
            }
        }
        Self::Run(RunArgs { seed, num_iter })
    }

    fn help() -> String {
        [
            "-h,--help         show help",
            "-s,--seed SEED    set seed",
            "-n NUM_ITER       set num iterations",
        ]
        .join("\n")
    }
}

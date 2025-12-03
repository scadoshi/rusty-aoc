use std::time::Instant;

pub trait TimeRun<I, R> {
    fn time_run(self, input: I)
    where
        I: Clone,
        R: std::fmt::Debug;
}

impl<I, R> TimeRun<I, R> for Vec<(String, Box<dyn Fn(I) -> R>)>
where
    I: Clone,
    R: std::fmt::Debug,
{
    fn time_run(self, input: I) {
        println!("\n# Results");
        self.into_iter().for_each(|(name, fun)| {
            let start = Instant::now();
            let result = fun(input.clone());
            println!(" - {}: {:?} | runtime: {:?}", name, result, start.elapsed());
        });
        println!("");
    }
}

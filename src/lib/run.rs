use std::{fmt::Debug, time::Instant};

pub trait Run<I, R> {
    fn run(self, input: I)
    where
        I: Clone,
        R: Debug;
}

impl<I, R> Run<I, R> for Vec<(String, Box<dyn Fn(I) -> R>)>
where
    I: Clone,
    R: Debug,
{
    fn run(self, input: I) {
        println!("---\n\n# Results");
        self.into_iter().for_each(|(name, fun)| {
            let start = Instant::now();
            let result = fun(input.clone());
            println!(" - {}: {:?} | runtime: {:?}", name, result, start.elapsed());
        });
        println!("\n---");
    }
}

pub fn funbox<I, R, F>(name: &str, fun: F) -> (String, Box<dyn Fn(I) -> R>)
where
    F: Fn(I) -> R + 'static,
{
    (name.to_string(), Box::new(fun))
}

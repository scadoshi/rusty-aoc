pub trait Run<In, R> {
    fn run(self, input: In)
    where
        In: Clone,
        R: std::fmt::Debug;
}

impl<I, In, R> Run<In, R> for I
where
    I: IntoIterator<Item = (&'static str, Box<dyn Fn(In) -> R>)>,
    In: Clone,
    R: std::fmt::Debug,
{
    fn run(self, input: In) {
        println!("# Functions");
        self.into_iter().for_each(|(name, fun)| {
            let start = std::time::Instant::now();
            let result = fun(input.clone());
            println!(" - {}: {:?} ({:?})", name, result, start.elapsed());
        });
    }
}

pub fn funbox<I, R, F>(name: &'static str, fun: F) -> (&'static str, Box<dyn Fn(I) -> R>)
where
    F: Fn(I) -> R + 'static,
{
    (name, Box::new(fun))
}

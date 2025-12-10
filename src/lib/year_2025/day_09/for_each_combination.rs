pub trait ForEachCombination {
    type Item;
    fn for_each_combination<F>(&self, f: F)
    where
        F: FnMut(&Self::Item, &Self::Item);
}

impl<T> ForEachCombination for [T] {
    type Item = T;

    fn for_each_combination<F>(&self, mut f: F)
    where
        F: FnMut(&T, &T),
    {
        self.iter().enumerate().for_each(|(i, v1)| {
            self.iter().skip(i + 1).for_each(|v2| {
                f(v1, v2);
            })
        });
    }
}

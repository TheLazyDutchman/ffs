pub mod tokens;

pub struct Group<T, I> {
	region: T,
	item: I
}

pub struct List<I, S> {
	items: Vec<(I, Option<S>)>
}
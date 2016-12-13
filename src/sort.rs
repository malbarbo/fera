use props::PropGet;

pub trait SortByProp<K> {
    fn sort_by_prop<P>(&mut self, p: &P)
        where P: PropGet<K>,
              P::Output: Ord;
}

impl<K: Copy> SortByProp<K> for [K] {
    fn sort_by_prop<P>(&mut self, p: &P)
        where P: PropGet<K>,
              P::Output: Ord
    {
        self.sort_by_key(|&k| p.get(k));
    }
}

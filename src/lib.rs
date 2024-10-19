

pub struct AllCombinations<'a,T> {
    base: &'a[T],         // it is a dictionary
    data: Vec<usize>,     // current combinations
    base_max_id: usize,   // base.len()-1
    n: usize,             // current length
    none: bool,           //
}

impl<'a,T> AllCombinations<'a,T> {
    /// create new brute force iterator
    pub fn new_fixed_size(base: &'a[T], size: usize) -> Self {
        Self {
            base: base,
            base_max_id: base.len()-1,
            data: vec![0;size],
            n: size,
            none: false,
        }
    }
}

impl<'a,T:Clone> Iterator for AllCombinations<'a,T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> { 
        if self.none { return None; }

        let mut v = Vec::with_capacity(self.data.len());
        for i in &self.data { v.push(self.base[*i].clone()) }

        self.none = true;
        for i in (0..=self.n-1).rev() {
            if self.data[i]==self.base_max_id { self.data[i]=0; } else { self.data[i]+=1; self.none = false; break; }
        }
        
        Some(v)
    }
}


#[test]
fn t1() {
    let base = b"01234";
    let mut all = AllCombinations::new_fixed_size(base, 2);
    assert_eq!(all.next(), Some(b"00".to_vec()));
    assert_eq!(all.next(), Some(b"01".to_vec()));
    assert_eq!(all.next(), Some(b"02".to_vec()));
    assert_eq!(all.next(), Some(b"03".to_vec()));
    assert_eq!(all.next(), Some(b"04".to_vec()));
    assert_eq!(all.next(), Some(b"10".to_vec()));
    assert_eq!(all.next(), Some(b"11".to_vec()));
    assert_eq!(all.next(), Some(b"12".to_vec()));
    assert_eq!(all.next(), Some(b"13".to_vec()));
    assert_eq!(all.next(), Some(b"14".to_vec()));
    assert_eq!(all.next(), Some(b"20".to_vec()));
    assert_eq!(all.next(), Some(b"21".to_vec()));
    assert_eq!(all.next(), Some(b"22".to_vec()));
    assert_eq!(all.next(), Some(b"23".to_vec()));
    assert_eq!(all.next(), Some(b"24".to_vec()));
    assert_eq!(all.next(), Some(b"30".to_vec()));
    assert_eq!(all.next(), Some(b"31".to_vec()));
    assert_eq!(all.next(), Some(b"32".to_vec()));
    assert_eq!(all.next(), Some(b"33".to_vec()));
    assert_eq!(all.next(), Some(b"34".to_vec()));
    assert_eq!(all.next(), Some(b"40".to_vec()));
    assert_eq!(all.next(), Some(b"41".to_vec()));
    assert_eq!(all.next(), Some(b"42".to_vec()));
    assert_eq!(all.next(), Some(b"43".to_vec()));
    assert_eq!(all.next(), Some(b"44".to_vec()));
    assert_eq!(all.next(), None);
    assert_eq!(all.next(), None);
}
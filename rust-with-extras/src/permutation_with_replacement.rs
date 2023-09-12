// Credit: https://rosettacode.org/wiki/Permutations_with_repetitions

pub fn iter<T>(universe: &[T], size: usize) -> PermutationWithReplacementIterator<T> {
    PermutationWithReplacementIterator {
        universe,
        size,
        prev: None,
    }
}

pub struct PermutationWithReplacementIterator<'a, T: 'a> {
    universe: &'a [T],
    size: usize,
    prev: Option<Vec<usize>>,
}

impl<'a, T> Iterator for PermutationWithReplacementIterator<'a, T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let n = self.universe.len();

        if n == 0 {
            return None;
        }

        match self.prev {
            None => {
                let zeroes: Vec<usize> = std::iter::repeat(0).take(self.size).collect();
                let result = Some(map(self.universe, &zeroes[..]));
                self.prev = Some(zeroes);
                result
            }
            Some(ref mut indexes) => match indexes.iter().position(|&i| i + 1 < n) {
                None => None,
                Some(position) => {
                    for index in indexes.iter_mut().take(position) {
                        *index = 0;
                    }
                    indexes[position] += 1;
                    Some(map(self.universe, &indexes[..]))
                }
            },
        }
    }
}

fn map<T>(values: &[T], ixs: &[usize]) -> Vec<T>
where
    T: Clone,
{
    ixs.iter().map(|&i| values[i].clone()).collect()
}

use std::{cmp, iter::FromIterator, ops};

// ref: https://codereview.stackexchange.com/a/103869
// Modified to use rust's range

type Range = ops::RangeInclusive<usize>;

pub struct RangeStack {
    pub ranges: Vec<Range>,
}

fn ranges_overlap(first: &Range, second: &Range) -> bool {
    // assumes first.start <= second.start
    // | ----- first ------- |
    //                 | ------ start ------ |
    first.end() > second.start()
}

fn merge_ranges(first: &Range, second: &Range) -> Range {
    // assumes first.start <= second.start
    Range::new(*first.start(), cmp::max(*first.end(), *second.end()))
}

impl RangeStack {
    fn add(&mut self, range: &Range) {
        if let Some(last) = self.ranges.last_mut() {
            if ranges_overlap(last, range) {
                let _ = std::mem::replace(last, merge_ranges(last, range));
                return;
            }
        }

        self.ranges.push(range.clone());
    }
}

impl FromIterator<Range> for RangeStack {
    fn from_iter<I>(iterator: I) -> Self
    where
        I: IntoIterator<Item = Range>,
    {
        let mut raw_ranges: Vec<_> = iterator.into_iter().collect();
        raw_ranges.sort_by(|a, b| a.start().cmp(b.start()));

        let mut range_stack = RangeStack { ranges: Vec::new() };

        for range in &raw_ranges {
            range_stack.add(range);
        }

        range_stack
    }
}

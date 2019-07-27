/// Range of input source in lines an columns.
/// It contains two points: the From position and the To position,
/// each of them measured as (line, column)
pub struct InputRange {
    line_from: usize,
    column_from: usize,
    line_to: usize,
    column_to: usize
}

enum RangePosition {
    From,
    To,
}


impl InputRange {
    /// Builds a new InputRange
    fn build(line1: usize, column1: usize, line2: usize, column2: usize) -> InputRange {
        InputRange {
            line_from: line1,
            column_from: column1,
            line_to: line2,
            column_to: column2
        }
    }

    /// Check if the passed InputRange is less than the current InputRange
    /// for the From or To positions.
    fn less_than(&self, other: &InputRange, position: RangePosition) -> bool {
        let (line1, line2) = match position {
            RangePosition::From => (self.line_from, other.line_from),
            RangePosition::To => (self.line_to, other.line_to)
        };
        let (column1, column2) = match position {
            RangePosition::From => (self.column_from, other.column_from),
            RangePosition::To => (self.column_to, other.column_to)
        };

        line1 < line2 || (line1 == line2 && column1 < column2)
    }

    /// Combines two InputRanges into a range containing both input ranges
    pub fn extent(&self, other: &InputRange) -> InputRange {
        let first = if self.less_than(other, RangePosition::From) { self  } else { other };
        let last = if self.less_than(other, RangePosition::To) { other  } else { self };
        InputRange {
            line_from: first.line_from,
            line_to: last.line_to,
            column_from: first.column_from,
            column_to: last.column_to
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InputRange;

    #[test]
    fn extent_range_before_after() {
        let r1 = InputRange::build(1,1, 10, 1);
        let r2 = InputRange::build(2,1, 20, 100);
        let r3 = r1.extent(&r2);
        assert_eq!(r3.line_from, 1);
        assert_eq!(r3.column_from, 1);
        assert_eq!(r3.line_to, 20);
        assert_eq!(r3.column_to, 100);
    }
}

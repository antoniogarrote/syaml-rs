/// Range of input source in lines an columns.
/// It contains two points: the From position and the To position,
/// each of them measured as (line, column)
pub struct InputRange {
    pub line_from: isize,
    pub column_from: isize,
    pub line_to: isize,
    pub column_to: isize
}

enum RangePosition {
    From,
    To,
}


impl InputRange {

    const MIN_LINE: isize = 1;
    const MIN_COLUM: isize = 0;
    const MAX_LINE: isize = std::isize::MAX;
    const MAX_COLUMN: isize = std::isize::MAX;

    /// Builds a new InputRange
    pub fn build(line1: isize, column1: isize, line2: isize, column2: isize) -> InputRange {
        if line1 <= InputRange::MIN_LINE &&
           column1 <= InputRange::MIN_COLUM &&
           line2 <= InputRange::MIN_LINE &&
           column2 <= InputRange::MIN_COLUM {
           InputRange::zero()
        } else if line1 <= InputRange::MIN_LINE &&
            column1 <= InputRange::MIN_COLUM &&
            line2 == InputRange::MAX_LINE &&
            column2 == InputRange::MAX_COLUMN {
            InputRange::all()
        } else {
            InputRange {
                line_from: line1,
                column_from: column1,
                line_to: line2,
                column_to: column2
            }
        }
    }

    pub fn zero() -> InputRange {
        InputRange {
            line_from: InputRange::MIN_LINE,
            column_from: InputRange::MIN_COLUM,
            line_to: InputRange::MIN_LINE,
            column_to: InputRange::MIN_COLUM
        }
    }

    pub fn all() -> InputRange {
        InputRange {
            line_from: InputRange::MIN_LINE,
            column_from: InputRange::MIN_COLUM,
            line_to: InputRange::MAX_LINE,
            column_to: InputRange::MAX_COLUMN
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

    #[test]
    fn extent_range_included() {
        let r1 = InputRange::build(1,1, 100, 1);
        let r2 = InputRange::build(2,1, 20, 100);
        let r3 = r1.extent(&r2);
        assert_eq!(r3.line_from, 1);
        assert_eq!(r3.column_from, 1);
        assert_eq!(r3.line_to, 100);
        assert_eq!(r3.column_to, 1);
    }

    #[test]
    fn extent_range_same_line() {
        let r1 = InputRange::build(1,1, 1, 90);
        let r2 = InputRange::build(1,1, 1, 100);
        let r3 = r1.extent(&r2);
        assert_eq!(r3.line_from, 1);
        assert_eq!(r3.column_from, 1);
        assert_eq!(r3.line_to, 1);
        assert_eq!(r3.column_to, 100);
    }

    #[test]
    fn create_min() {
        let c = InputRange::build(InputRange::MIN_LINE, InputRange::MIN_COLUM, InputRange::MIN_LINE, InputRange::MIN_COLUM);
        let z = InputRange::zero();
        assert_eq!(c.line_from, z.line_from);
        assert_eq!(c.column_from, z.column_from);
        assert_eq!(c.line_to, z.line_to);
        assert_eq!(c.column_to, z.column_to);
    }

    #[test]
    fn create_max() {
        let c = InputRange::build(InputRange::MIN_LINE, InputRange::MIN_COLUM, InputRange::MAX_LINE, InputRange::MAX_COLUMN);
        let a = InputRange::all();
        assert_eq!(c.line_from, a.line_from);
        assert_eq!(c.column_from, a.column_from);
        assert_eq!(c.line_to, a.line_to);
        assert_eq!(c.column_to, a.column_to);
    }

}

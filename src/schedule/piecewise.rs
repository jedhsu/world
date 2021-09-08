//! A piecewise linear schedule.

    PLSchedule(cst)


    PLSchedule(xs, ys)

//! Return a piecewise linear schedule such that:
//   - For all `i`, `(xs<i>, ys<i>)` belongs to the schedule's graph.
//   - Before `xs<1>`, the schedule has value `ys<1>`.
//   - After `xs<end>`, the schedule has value `ys<end>`.

pub struct PLSchedule<T> {
  xs: Vec<u64>,
  ys: Vec<R>,
}



impl<T> PLSchedule<T> {
    fn new(xs: Vec<u64>, Vec<R>) -> PLSchedule<T> {
        assert!(~xs.isempty());
        assert!length(xs) == length(ys);
        T::new(xs, ys)
    }
    
    // PLSchedule(xs, ys) = PLSchedule{eltype(ys)}(xs, ys)
    
    /// Return a schedule with a constant value `cst`.
    fn from_cst(cst) -> PLShedule<T> {
        PLSchedule(<0>, <cst>)
    }

    fn index(&self, step: i32) {
        let ptidx = findlast(x -> x <= i, s.xs);

        if ptidx.is_none() {
            // We are before the first poi32
            return s.ys<1>;
        } else if  ptidx == length(s.xs) {
            # We are past the last point
            return s.ys<end>
        }
      else
        # We are between two points

        x0, y0 = s.xs<ptidx>, s.ys<ptidx>
        x1, y1 = s.xs<ptidx+1>, s.ys<ptidx+1>
        y = y0 + (y1 - y0) / (x1 - x0) * (i - x0)

        R <: Integer && (y = ceil(Int, y))
        return y
        }
    }
}

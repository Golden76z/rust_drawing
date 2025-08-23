mod lib;
struct Line {
    start: i32,
    end: i32,
}

impl Line {
    pub fn random(w,h) {
        Line{
            start: Point::random(w,h)
            end: Point::random(w, h)
        }
    }
}
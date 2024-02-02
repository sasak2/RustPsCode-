
type I = i128;
type P = [I; 2];
type L = [P; 2];

fn scale(s: I, a: P) -> P { a.map(|x| x * s) }
fn add(a: P, b: P) -> P { [a[0] + b[0], a[1] + b[1]] }
fn sub(a: P, b: P) -> P { [a[0] - b[0], a[1] - b[1]] }
fn dot(a: P, b: P) -> I { a[0] * b[0] + a[1] * b[1] }
fn cross(a: P, b: P) -> I { a[0] * b[1] - a[1] * b[0] }
fn ccw(a: P, b: P, c: P) -> I { cross(sub(b, a), sub(c, b)) }
//https://bamgoesn.github.io/rust-ps-md/geometry/2dgeometrybase.html 밤고수 템플릿 뺏어쓰기 heheThe code below only checks if two lines meet, without calculating where they do.

//https://bamgoesn.github.io/rust-ps-md/geometry/line_intersection.html
/// Checks if line segments `p` and `q` intersect.
/// Returns `true` if they intersect at any point, `false` otherwise.
fn meets(p: L, q: L) -> bool {
    let u = cross(sub(p[1], p[0]), sub(q[1], q[0]));
    let sn = cross(sub(q[0], p[0]), sub(q[1], q[0]));
    let tn = cross(sub(q[0], p[0]), sub(p[1], p[0]));
    if u != 0 {
        let int = if u >= 0 { 0..=u } else { u..=0 };
        int.contains(&sn) && int.contains(&tn)
    } else {
        if sn != 0 || tn != 0 {
            return false;
        }
        let (a0, a1) = (p[0].min(p[1]), p[0].max(p[1]));
        let (b0, b1) = (q[0].min(q[1]), q[0].max(q[1]));
        let (l, r) = (a0.max(b0), a1.min(b1));
        l <= r
    }
}

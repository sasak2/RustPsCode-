
type I = i128;
type P = [I; 2];
type L = [P; 2];

fn scale(s: I, a: P) -> P { a.map(|x| x * s) }
fn add(a: P, b: P) -> P { [a[0] + b[0], a[1] + b[1]] }
fn sub(a: P, b: P) -> P { [a[0] - b[0], a[1] - b[1]] }
fn dot(a: P, b: P) -> I { a[0] * b[0] + a[1] * b[1] }
fn cross(a: P, b: P) -> I { a[0] * b[1] - a[1] * b[0] }
fn ccw(a: P, b: P, c: P) -> I { cross(sub(b, a), sub(c, b)) }
//https://bamgoesn.github.io/rust-ps-md/geometry/2dgeometrybase.html 밤고수 템플릿 뺏어쓰기 hehe
//0,0 처리 못하니까 나중에 알아서 잘 처리해라 미래의 나
fn is_GyoCha(l1:L,l2:L)->bool {
//https://killerwhale0917.tistory.com/6
    let mut p1=l1[0];
    let mut p2=l1[1];
    let mut p3=l2[0];
    let mut p4=l2[1];
    let temp:i64=0;

    let p1p2 = ccw(p1, p2, p3) * ccw(p1, p2, p4);
    let p3p4 = ccw(p3, p4, p1) * ccw(p3, p4, p2); 
    

    if p1==p3||p2==p4||p1==p4||p2==p3 {
        return true;
    }
    if p1p2 == 0 && p3p4 == 0 {
    	if p1 > p2 { 
            let temp=p1;
             p1=p2;
             p2=temp;
        }
        if p3 > p4 {
            let temp=p3;
             p3=p4;
             p4=temp;
        }
        
        return p3 <= p2 && p1 <= p4;
    }
    
    return p1p2 <= 0 && p3p4 <= 0;
}

struct S<'a>{
    x: &'a i32,
    y: &'a i32,
}

// Both r and s have same life time assiged with same params
fn f<'a>(r: &'a i32, s: &'a i32) -> &'a i32 {r}

// We are giving un-nessary lifetime param 'b which we are not using 
fn g<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 {r}

fn main() {
    let x = 10;
    let y = 20;
    let r;
    {
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }
    println!("{}", r);
}
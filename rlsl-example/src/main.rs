#![feature(custom_attribute, attr_literals)]
#![feature(platform_intrinsics)]

//use std::ops::Add;
////#[spirv(Vec2)]
////#[repr(C)]
////#[derive(Copy, Clone)]
//pub struct Vec2<T: Copy> {
//    x: T,
//    y: T,
//}
////impl<T: Copy> Add for Vec2<T>
////where
////    T: Add<Output = T>,
////{
////    type Output = Vec2<T>;
////    fn add(self, other: Vec2<T>) -> Vec2<T> {
////        Vec2 {
////            x: self.x + other.x,
////            y: self.y + other.y,
////        }
////    }
////}
//impl Add for Vec2<f32>
//{
//    type Output = Vec2<f32>;
//    fn add(self, other: Vec2<f32>) -> Vec2<f32> {
//        Vec2 {
//            x: self.x + other.x,
//            y: self.y + other.y,
//        }
//    }
//}
//
//impl Vec2<f32>{
//    #[spirv(dot)]
//    pub fn dot(self, other: Vec2<f32>) -> f32{
//        self.x * other.x + self.y * other.y
//    }
//}



//#[cfg(spirv)]

//struct Foo {
//    f: f32,
//}
//struct Bar {
//    x: f32,
//    y: f32,
//}
//
//#[spirv(Vec2)]
//type Bar1 = Bar;
//
//trait A {}
//impl A for Bar {}
//
//struct Foo1;

#[spirv(ty(Vec2))]
struct Test{
    x: f32,
    y: f32
}
#[spirv(vertex)]
fn vert() {
    let t = Test{
        x: 1.0,
        y: 2.0
    };
    let f = 1.0f32.sqrt();
    //let f = unsafe { sqrt(1.0) };
    //let f = Foo1{};
    //    let b = if 1.0f32 > 1.0 {
    //        Test::B(1, 2)
    //    } else {
    //        Test::C
    //    };

    //    let f = if let Test::A(f) = t1 {
    //        if let Some(f) = f {
    //            f
    //        }
    //        else{
    //            1.0
    //        }
    //    }
    //    else{
    //        3.0
    //    };
    //    if 1.0f32 > 1.0{
    //        let i = 4.0f32;
    //    }
    //let i = Some(1.0f32);
    //    let v = Vec2 { x: 1.0f32, y: 2.0 };
    //    let v1 = v;
    //    let t = Test{x: 1.0, y: 2};
    //    let f1 = v.x;
    //    //let f1 = v.dot(v1);
    //    let mut b = Bar{t: Test{v: v}};
    //    let f2 = b.t.v.y;
    //    b.t.v.x = 1.0;
    //let f4: f32 = if f1 > 1.0 { 1.0 } else { 2.0 };
    //    let f3 = b.t.v.y;
    //for i in (0u32 .. 100){}
}
fn main() {}

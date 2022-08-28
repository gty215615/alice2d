


// pub struct Unique<T> {
//     pub ptr:*const T,                            // 保持可变性
//     _marker: std::marker::PhantomData<T>     // drop 检查
// }


// unsafe impl<T:Send> Send for Unique<T> {}
// unsafe impl<T:Sync> Sync for Unique<T> {}

// impl <T> Unique<T> {
//     pub fn new(ptr:*mut T) -> Self {
//         Unique {
//             ptr,
//             _marker: std::marker::PhantomData
//         }
//     }

//     pub fn as_ptr(&self) -> *mut T {
//         self.ptr as *mut T
//     }
    
// }
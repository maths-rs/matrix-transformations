impl<T> arrOps<T> for [T]
    where
    T: Copy,
    
{
    fn diff<const N: usize>(&mut self, vector: [T; N]){
        println!("{:}", vector.len() );
    }

}
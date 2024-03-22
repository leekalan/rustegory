pub mod wrapper;
pub mod composer;
pub mod injectable;
pub mod embedable;
pub mod vec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("hello");
    }
}

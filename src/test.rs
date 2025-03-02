#[cfg(test)]
mod tests {
    use crate::module::metadata::Metadata;

    #[test]
    fn test() {
        let _ = dbg!(Metadata::new("..modprobe/mod.ko"));
    }
}

#[derive(derive_bug::MyDerive)]
struct Foo<#[cfg(FALSE)] T> {
    #[cfg(FALSE)] field: T,
    val: String,
    array: [u8; {
        #[cfg(FALSE)] struct Bar;
        0
    }]
}

fn main() {}

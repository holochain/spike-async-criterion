pub async fn my_async_fn_that_does_work(input: u64) -> Result<u64, ()> {
    // simulate doing some cpu work
    std::thread::sleep(std::time::Duration::from_millis(10));
    Ok(input + 1)
}

#[cfg(test)]
mod tests {
    #[tokio::test(threaded_scheduler)]
    async fn it_works() {
        assert_eq!(43, super::my_async_fn_that_does_work(42).await.unwrap());
    }
}

use std::pin::Pin;

pub trait AsyncAndThen<TOk, Err, UOk, F>
where
    F: FnOnce(TOk) -> Pin<Box<dyn Future<Output = Result<UOk, Err>> + Send>> + Send,
{
    type Output;
    async fn and_then_async(self, map: F) -> Self::Output;
}

impl<TOk, TErr, UOk, F> AsyncAndThen<TOk, TErr, UOk, F> for Result<TOk, TErr>
where
    TOk: Send,
    UOk: Send,
    F: 'static + FnOnce(TOk) -> Pin<Box<dyn Future<Output = Result<UOk, TErr>> + Send>> + Send
{
    type Output = Result<UOk, TErr>;

    async fn and_then_async(self, map: F) -> Self::Output {
        match self {
            Ok(val) => {
                let x = map(val).await;
                x
            },
            Err(err) => Err(err)
        }
    }
}

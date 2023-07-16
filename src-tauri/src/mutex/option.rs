macro_rules! unwrap {
    ( $self: ident, $body: expr ) => {
        if let Some($self) = &mut *$self.lock().await {
            $body
        }
    }
}

pub(crate) use unwrap;

#[tokio::test]
async fn unwrap_mac() {
    let mutex_option = tokio::sync::Mutex::new(Some(55));

    unwrap!(mutex_option, {
        assert_eq!(mutex_option, &mut 55);
    });
}
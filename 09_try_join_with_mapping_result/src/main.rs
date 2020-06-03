use futures::{
    future::TryFutureExt,
    try_join,
    executor::block_on,
};

struct Book {}
struct Music {}

async fn get_book() -> Result<Book, ()> { /* ... */ Ok(Book {}) }
async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music {}) }

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book().map_err(|()| "Unable to get book".to_string());
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}

fn main() {
    let future = get_book_and_music();
    block_on(future).unwrap();
}

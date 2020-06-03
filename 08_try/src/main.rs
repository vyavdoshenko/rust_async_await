use futures::join;
use futures::executor::block_on;

struct Book {}
struct Music {}

async fn get_book() -> Book {
    Book {}
}

async fn get_music() -> Music {
    Music {}
}

async fn get_book_and_music() -> (Book, Music) {
    let book_fut = get_book();
    let music_fut = get_music();
    join!(book_fut, music_fut)
}

fn main() {
    let future = get_book_and_music();
    let (_, _) = block_on(future);
}

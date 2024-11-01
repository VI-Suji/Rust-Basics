#[derive(Debug)]
enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
}

impl Media {
    fn description(&self) -> String{

    }
}

fn print_media(media: Media){
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook{
        title: String::from("audiobook"),
    };

    let good_movie = Media::Movie{
        title: String::from("good_movie"),
        director: String::from("good_director")
    };

    let bad_book = Media::Book{
        title: String::from("bad_book"),
        author: String::from("bad_author"),
    };

    print_media(audiobook);
    print_media(good_movie);
    print_media(bad_book);
}

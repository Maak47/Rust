struct Book {
    pages: i32,
    rating: i32,

}

fn display_page_count(book: &Book) { // & is used to tell the function that its borrowed
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book){ // & is used to tell the function that its borrowed
    println!("rating = {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 5,
        rating: 9,
    };

    // display_page_count(book); is moving the ownership to the function
    // display_rating(book); cannot be used because the display_page_count now have the ownership over the variable, 
                          // which in-turn have cleared the variable memory.
    display_page_count(&book); // ampersand (&) is used to tell rust, that the variable is being borrowed(referrenced)
    display_rating(&book); // can access it now because the ownership is to the main function because it was borrowed.

}
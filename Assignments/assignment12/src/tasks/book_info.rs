use log::*;

/// Structure Library contains the data about Library.
///
/// #Fields
/// title - Title of the book stored in the type Vec<String>.
/// author - Author name of the book stored in the type Vec<String>.
/// access_num - Access number of the book stored in the type Vec<i32>.
/// flag - Flag value representing the issue status.

pub struct Library {
    pub title: Vec<String>,
    pub author: Vec<String>,
    pub access_num: Vec<i32>,
    pub flag: i32,
}
/// Implementation of method on Library
impl Library {
    /// add_book: It is a function to add Library details in the existing data.
    ///
    /// #Argument
    /// title - Title of the book of type String.
    /// author - Author name of the type String.
    /// access_num - Accession number of the book  type String.
    /// flag - Flag value representing the issue status of type i32
    ///
    /// #Return
    /// Panic if the book already exist else add the book
    pub fn add_book(&mut self, title: String, author: String, number: i32, flag: i32) {
        if self.access_num.contains(&number) {
            panic!("Hey already exist")
        } else {
            self.author.push(author);
            self.title.push(title);
            self.access_num.push(number);
            self.flag = flag
        }
    }
    /// book_display :It is a function to display the data of the Library are present.
    ///
    /// #Argument
    /// &mut self - Self type parameter
    ///
    /// #Return
    /// Result of Ok value if the data is present else error message.
    pub fn book_display(&mut self) -> Result<String, String> {
        if self.access_num.is_empty() {
            return Err("Empty".to_string());
        }
        for index in 0..self.access_num.len() {
            log::info!("{},{},{}",self.access_num[index],self.author[index],self.title[index]);
        }
        Ok("Data showed".to_string())
    }
    /// total_book:It is a funcntion which is used to display the total number of book.
    ///
    /// #Arguments
    ///
    /// &mut self - Self type parameter
    ///
    /// #Return
    ///
    /// Returns Result type object which signify that total number of books in library.

    pub fn total_book(&self) -> Result<i32, i32> {
        if self.title.is_empty() {
            error!("No book present their");
            Err(0)
        } else {
            Ok(self.title.len() as i32)
        }
    }
    /// title_name: It is a function which gives the information of book based on title.
    ///
    /// #Argument
    /// &self - A referenced Self type parameter
    /// reference - The title given of type String
    ///
    /// #Return
    /// The Ok type value if the book with title is present in data else error message.
    pub fn title_name(&self, reference: String) -> Result<String, String> {
        if !self.title.contains(&reference) {
            return Err("Hey book is not present".to_string());
        }
        for index in 0..self.title.len() {
            if reference == self.title[index] {
                log::info!("{},{},{},{}",self.title[index],self.flag,self.access_num[index],self.author[index]);
            }
        }
        Ok("The book is present withe given title".to_string())
    }
    /// author_name:It is a function gives the information of book based on author.
    ///
    /// #Argument
    /// &self - A referenced Self type parameter
    /// reference - The author given of type String
    ///
    /// #Return
    /// The Ok type value if the book with author present in data else error message.
    pub fn author_name(&self, reference: String) -> Result<String, String> {
        if !self.author.contains(&reference) {
            return Err("Hey book is not present".to_string());
        }
        for index in 0..self.author.len() {
            if reference == self.author[index] {
                log::info!("{},{},{},{}",self.title[index],self.flag,self.access_num[index],self.author[index]);
            }
        }
        Ok("The book is present with given author".to_string())
    }
    /// issue_library: It is a function to issue the book and update the data.
    ///
    /// #Argument
    /// &self - A referenced Self type parameter
    /// title - The title  given of type String
    ///
    /// #Return
    /// The Ok type value if the book with title issued in data else error message.
    pub fn issue_library(&mut self, title: String) -> Result<String, String> {
        if !self.title.contains(&title) {
            Err("Hey this is not present".to_string())
        } else {
            for i in 0..self.title.len() - 1 {
                if title == self.title[i] {
                    self.title.remove(i);
                    self.flag = 1;
                    self.author.remove(i);
                    self.access_num.remove(i);
                }
            }
            Ok("Book issued".to_string())
        }
    }
}
//! **sessionstoragemod.rs - for debugging texts accessible everywhere**  

//region: use
use crate::logmod;

//endregion

///add to begin of debug text
pub fn add_to_begin_of_debug_text(text: &str) {
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.session_storage()));
    let mut sss = format!("{}: {}\n{}", 
    logmod::now_string(),
    text, 
    get_debug_text()
    );
    utf8_truncate(&mut sss,500);
    let _x = ls.set_item("debug_text", sss.as_str());
}

///utf8 truncate
fn utf8_truncate(input : &mut String, maxsize: usize) {
  let mut utf8_maxsize = input.len();
  if utf8_maxsize >= maxsize {
    { let mut char_iter = input.char_indices();
    while utf8_maxsize >= maxsize {
      utf8_maxsize = match char_iter.next_back() {
        Some((index, _)) => index,
        _ => 0
      };
    } } // Extra {} wrap to limit the immutable borrow of char_indices()
    input.truncate(utf8_maxsize);
  }
}

///get debug text from session storage
pub fn get_debug_text() -> String {
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.session_storage()));
    let empty1 = "".to_string();
    //return nickname
    unwrap!(ls.get_item("debug_text")).unwrap_or(empty1)
}
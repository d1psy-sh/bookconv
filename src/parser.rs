use crate::reader::Bookmarks;

fn folder(folder_name: &str) -> String {
    let mut s = String::new();
    if folder_name == "Bookmarks" {
        s.push_str(r#"<DT><H3 ADD_DATE="0" LAST_MODIFIED="0" PERSONAL_TOOLBAR_FOLDER="true" "#);
    } else {
        s.push_str(r#"<DT><H3 ADD_DATE="0" LAST_MODIFIED="0""#);
    }
    s.push_str(r#">"#);
    s.push_str(folder_name);
    s.push_str(r#"<\H3>"#);
    s.push_str("\n");
    s.push_str(r#"<DL><p>"#);
    s.push_str("\n");
    s
    // here goes the item
}

fn folder_close() -> String {
    let mut s = String::new();
    s.push_str(r#"<\DL><p>"#);
    s.push_str("\n");
    s
}

fn item(item_name: &str, item_url: &str) -> String {
    let mut s = String::new();
    s.push_str(r#"<DT><A HREF=""#);
    s.push_str(item_url);
    s.push_str(r#"" ADD_DATE="0" LAST_MODIFIED="0">"#);
    s.push_str(item_name);
    s.push_str(r#"<\A>"#);
    s.push_str("\n");
    s
}

pub(crate) fn parse_file(input: &Bookmarks) -> String {
    // parse the input Bookmarks to netscape html Bookmark format
    let mut s = String::new();
    resolve_child(&input.roots.bookmark_bar.children, &mut s);
    s.push_str(&folder_close());
    s
}

fn resolve_child(input: &Vec<crate::reader::Children>, s: &mut String) {
    for child in input {
        match &child.url {
            Some(url) => s.push_str(&item(&child.name, &url)),
            None => {
                s.push_str(&folder(&child.name));
                resolve_child(&child.children, s);
                s.push_str(&folder_close());
            }
        }
    }
}

mod textfile;
mod textpad;
use crate::textpad::textpad::TextPad;

fn main() {
    let mut tf = TextPad::new(String::from("test_path/"));
    tf.file_obj.add_content_to_row(1, String::from("content1"));

    let res: Result<String, &'static str> = tf.file_obj.clone().get_row(1);
    match res {
        Ok(x) => println!("ok: {}", x),
        Err(x) => println!("not ok: {}", x)
    }
}

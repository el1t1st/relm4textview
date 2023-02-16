use gtk::prelude::{BoxExt, GtkWindowExt, OrientableExt, TextBufferExt, TextViewExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
//
// The Goal:
// - Initialize a text in a TextView by reading a string
// - Color the read in text in grey
// - Use the keyboard to overwrite each character
// - For each overwritten character check if the typed character is the same as the character you are
// overwriting.
// -- If it is the same character change the color to green
// -- If it is not the same character change the color to red
// - Allow the input cursor to move back changing the color of the green or red characters to grey
// again
// - Save the time between each keypress to a variable
// - Save the correct coefficient in percentage (#keypresses/#correct keypresses * 100)
//
//

// The initial text to be read
const TEXT: &str =
    "Rust is a programming language which \n allows even new programmers to write robust code \n ";

#[derive(Debug, Default)]
struct AppModel {
    work_text: gtk::TextBuffer,
}

#[derive(Debug)]
enum AppMsg {
    CheckChar,
    MoveOneBack,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    // Associated types used inside the implementation
    // Todo:: I thought the Init was the initialization of the model with empty or filled values?
    // In this case i want to show the text
    type Init = AppModel;
    type Input = AppMsg;
    type Output = (); // There are no output messages to other components

    view! {
        gtk::Window {
            set_title: Some("Retype a Text in TextView"),
            set_default_width: 500,
            set_default_height: 400,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                // Add a watcher to change it on update
                gtk::TextView {
                    #[watch]
                    set_buffer: Some(&model.work_text),
                    set_editable: true,
                    // The wrapmode can also be WordChar ?
                    set_wrap_mode: gtk::WrapMode::Word,
                }
            },
        }
    }

    fn init(
        _app_state: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            work_text: gtk::TextBuffer::new(),
        };
        // There is a buffer.set_text(TEXT) method we can call to populate the text inside the
        // TextView?
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    // fn update_view() {}
    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::CheckChar => {
                // Get the current location and keyboard input
                // Check the value against the location in versus the value in the existing text:w
                println!("Process the current index of the string versus the keyboard value")
            }
            AppMsg::MoveOneBack => {
                // We need to get the current location and change the color of the current char to
                // default
                println!("Move one position back and change the color to default color")
            }
        }
    }
}

// Function that takes in 2 &str's, compares them and returns a bool
//
// Function that takes in a location of a &str and the color of a single char/&str
//
fn main() {
    let app = RelmApp::new("relm4.textviewexperiment");
    // What am i doing wrong here?
    app.run::<AppModel>(work_text: gtk::TextBuffer::new().set_text(TEXT));
}

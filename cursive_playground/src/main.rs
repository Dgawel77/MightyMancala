use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView};
use cursive::traits::*;
use cursive::views::TextView;
use cursive_tabs::TabPanel;
use cursive::view::Nameable;
use cursive::event::{Event, EventResult, Key};
use cursive::views::{Canvas};
use unicode_width::UnicodeWidthStr;

fn main(){
    let state = String::new();
    let canvas = Canvas::new(state)
        .with_draw(|text: &String, printer| {
            // Simply print our string
            printer.print((0, 0), text);
        })
        .with_on_event(|text: &mut String, event| match event {
            Event::Char(c) => {
                text.push(c);
                EventResult::Consumed(None)
            }
            Event::Key(Key::Enter) => {
                let text = text.clone();
                EventResult::with_cb(move |s| {
                    s.add_layer(Dialog::info(&text));
                })
            }
            _ => EventResult::Ignored,
        })
        .with_required_size(|text, _constraints| (text.width(), 1).into());
}

fn example() {
    let mut siv = cursive::default();

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5));
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title("Select a profile"));

    siv.run();
}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("name")
            .fixed_width(10))
        .title("Enter a new name")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", Cursive::quit));
}
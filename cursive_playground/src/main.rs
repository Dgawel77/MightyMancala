use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView, HideableView};
use cursive::traits::*;
use cursive::views::TextView;
use cursive_tabs::TabPanel;
use cursive::view::Nameable;
use cursive::event::{Event, EventResult, Key};
use cursive::views::{Canvas};

use cursive::views::{EnableableView, Checkbox};

fn main(){
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::around(
        LinearLayout::vertical()
        .child(HideableView::new(
            TextView::new("Hello")
        ).with_name("main"))
        .child(EnableableView::new(Checkbox::new()).with_name("my_view"))
        .child(Button::new("Toggle", |s| {
            s.call_on_name("main", |v: &mut HideableView<TextView>| {
                // This will disable (or re-enable) the checkbox, preventing the user from
                // interacting with it.
                v.set_visible(!v.is_visible());
            });
        })))
    );

    siv.run();
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
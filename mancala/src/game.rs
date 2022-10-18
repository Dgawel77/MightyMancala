use cursive::{views::Dialog, Cursive};

pub fn run(){
    let mut siv = cursive::default();
    
    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(Dialog::text("Hey!"));
    
    siv.run();
}
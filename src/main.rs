mod Widgets;

fn main() {

    let main_block = Widgets::widget_block::setup_termina().unwrap();
    let widget_bar_chart = Widgets::widget_bar_chart::widget_bar();
}
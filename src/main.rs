

            horizontal-alignment: center;
            text: "Hello, Slint!";
            vertical-alignment: center;
        Text {
        height: 600px;
        width: 400px;
        }
    export component MainWindow inherits Window {
    let ui = MainWindow::new().unwrap();
    ui.run().unwrap();
    }
fn main() {
slint! {
use slint::slint;
}
}

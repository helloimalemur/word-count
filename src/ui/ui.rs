slint::slint!{
    import { SpinBox, Button, CheckBox, Slider, LineEdit, ScrollView, ListView, HorizontalBox, VerticalBox, GridBox } from "std-widgets.slint";
    export component WordCount inherits Window {
        in-out property <int> counter: 0;
        callback button-pressed <=> button.clicked;
        VerticalBox {
            button := Button {
                text: "Button, pressed " + root.counter + " times";
            }
        }
    }
}

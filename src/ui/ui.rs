slint::slint!{
    import { SpinBox, Button, CheckBox, StandardTableView, Slider, LineEdit, ScrollView, ListView, HorizontalBox, VerticalBox, GridBox } from "std-widgets.slint";
    export component WordCount inherits Window {
        title: "Word Count";

        height: 400px;
        width: 600px;
        in-out property <int> counter: 0;
        callback button-pressed <=> button.clicked;

        in-out property<[{text: string}]> list-of-structs: [{text: "hello"}, {text: "world"}, {text: "asdf"}, {text: "asdf"},];

        VerticalBox {
            height: 200px;
            width: 400px;
            button := Button {
                text: "Button, pressed " + root.counter + " times";
            }

            VerticalLayout {
                for data in root.list-of-structs: my-repeated-text := Text {
                    text: data.text;
                }
            }
        }
    }

    export global DataAdapter {
        in property <string> title: "test";
    }

}

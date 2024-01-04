slint::slint! {
    import { SpinBox, Button, CheckBox, StandardTableView, Slider, LineEdit, ScrollView, ListView, HorizontalBox, VerticalBox, GridBox } from "std-widgets.slint";

    export component WordCount inherits Window {
        title: "Word Count";

        height: 600px;
        width: 800px;
        in-out property <int> counter: 0;
        callback open-file-pressed <=> open-file.clicked;
        callback re-calc-pressed <=> re-calc.clicked;

        in-out property<[{text: string}]> list-of-structs: [{text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, ];

        VerticalBox {
            height: 600px;
            width: 800px;
            Rectangle {
                padding: 50px;
                open-file := Button {
                    height: 50px;
                    width: 200px;
                    text: "Open File (files open:" + root.counter +")";
                }
            }
            Rectangle {
                padding: 50px;
                re-calc := Button {
                    height: 50px;
                    width: 200px;
                    text: "Recalculate";
                }
            }


            VerticalLayout {
                for data in root.list-of-structs: my-repeated-text := Text {
                    text: data.text;
                }
            }
        }
    }
}

slint::slint! {
    import { SpinBox, Button, CheckBox, StandardTableView, Slider, LineEdit, ScrollView, ListView, HorizontalBox, VerticalBox, GridBox } from "std-widgets.slint";

    export component WordCount inherits Window {
        title: "Word Count";

        height: 400px;
        width: 600px;
        in-out property <int> counter: 0;
        callback open-file-pressed <=> open-file.clicked;

        in-out property<[{text: string}]> list-of-structs: [{text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, ];

        VerticalBox {
            height: 400px;
            width: 600px;
            Rectangle {
                open-file := Button {
                    height: 50px;
                    width: 80px;
                    text: "Open File (files open:" + root.counter +")";
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

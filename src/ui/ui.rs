slint::slint! {
    import { SpinBox, Button, CheckBox, StandardTableView, Slider, LineEdit, ScrollView, ListView, HorizontalBox, VerticalBox, GridBox } from "std-widgets.slint";

    export component WordCount inherits Window {
        title: "Word Count";

        height: 500px;
        width: 600px;
        in-out property <int> counter: 0;
        callback open-file-pressed <=> open-file.clicked;
        callback re-calc-pressed <=> re-calc.clicked;

        in-out property<[{text: string}]> list-of-structs: [{text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, {text: ""}, ];

        VerticalLayout {
            HorizontalBox {
                height: 100px;
                width: 600px;
                Rectangle {
                    open-file := Button {
                        text: "Open File (files open:" + root.counter +")";
                    }
                }
                Rectangle {
                    re-calc := Button {
                        text: "Recalculate";
                    }
                }
                Rectangle {
                    clear := Button {
                        text: "Clear";
                    }
                }
            }
            HorizontalBox {
                width: 600px;
                height: 400px;
                Rectangle {
                    VerticalLayout {
                        for data in root.list-of-structs: my-repeated-text := Text {
                            text: data.text;
                        }
                    }
                }
            }
        }
    }
}

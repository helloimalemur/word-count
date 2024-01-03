slint::slint!{
    import { SpinBox, Button, CheckBox, StandardTableView, Slider, LineEdit, ScrollView, ListView, HorizontalBox, VerticalBox, GridBox } from "std-widgets.slint";
    export component WordCount inherits Window {
        height: 400px;
        width: 600px;
        in-out property <int> counter: 0;
        callback button-pressed <=> button.clicked;
        VerticalBox {
            height: 200px;
            width: 400px;
            button := Button {
                text: "Button, pressed " + root.counter + " times";
            }
            StandardTableView {
        width: 200px;
        height: 200px;
            columns: [
                { title: "Header 1" },
                { title: "Header 2" },
            ];
            rows: [
                [
                    { text: "Item 1" }, { text: "Item 2" },
                ],
                [
                    { text: "Item 1" }, { text: "Item 2" },
                ],
                [
                    { text: "Item 1" }, { text: "Item 2" },
                ]
        ];
    }
        }
    }
}

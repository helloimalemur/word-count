slint::slint!{
    import { SpinBox, Button, CheckBox, StandardTableView, Slider, LineEdit, ScrollView, ListView, HorizontalBox, VerticalBox, GridBox } from "std-widgets.slint";
    export component WordCount inherits Window {
        callback getdata;

        height: 400px;
        width: 600px;
        in-out property <int> counter: 0;
        in-out property <string> astring: "";
        in-out property <string> bstring: "atest";
        in-out property <[{text: string, text: string}]> listof: [];
        callback button-pressed <=> button.clicked;

        in-out property<[{a: string, b: string}]> list-of-structs: [{ a: "asf", b: "hello" }, {text: "dd", text: "world"}];

        VerticalBox {
            height: 200px;
            width: 400px;
            button := Button {
                text: "Button, pressed " + root.counter + " times";
            }
            table := StandardTableView {
                width: 200px;
                height: 200px;
                columns: [
                    { title: "Header 1" },
                    { title: "Header 2" },
                ];
                rows: [
                    [
                        { text: root.astring }, { text: "Item 2" },
                    ],
                    [
                        { text: root.bstring }, { text: "Item 2" },
                    ],
                    // [
                    //     root.rowa
                    // ],
                ];
            }
        }
    }

    export global DataAdapter {
        in property <string> title: "test";
    }

}

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

        in-out property<[{text: string}]> list-of-structs: [{text: "world"}];

        VerticalBox {
            height: 200px;
            width: 400px;
            button := Button {
                text: "Button, pressed " + root.counter + " times";
            }

            VerticalLayout {
                for data[ind] in root.list-of-structs: my-repeated-text := Text {
                    text: data.text;
                }
            }
        }
    }

    export global DataAdapter {
        in property <string> title: "test";
    }

}

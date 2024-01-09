slint::slint! {
    import { SpinBox, Button, CheckBox, StandardTableView, Slider, LineEdit, ScrollView, ListView, HorizontalBox, VerticalBox, GridBox } from "std-widgets.slint";

    export component WordCount inherits Window {
        title: "Word Count";

        height: 500px;
        width: 1000px;
        in-out property <int> counter: 0;
        callback open-file-pressed <=> open-file.clicked;
        callback re-calc-pressed <=> re-calc.clicked;
        callback clear-pressed <=> clear.clicked;

        in-out property<[{text: string, text2: string, text3: string, text4: string, text5: string}]> list-of-structs: [{text: "", text2: "", text3: "", text4: "", text5: ""}, {text: "", text2: "", text3: "", text4: "", text5: ""}, {text: "", text2: "", text3: "", text4: "", text5: ""}, {text: "", text2: "", text3: "", text4: "", text5: ""}, {text: "", text2: "", text3: "", text4: "", text5: ""}, {text: "", text2: "", text3: "", text4: "", text5: ""}, {text: "", text2: "", text3: "", text4: "", text5: ""}, {text: "", text2: "", text3: "", text4: "", text5: ""}, {text: "", text2: "", text3: "", text4: "", text5: ""}, {text: "", text2: "", text3: "", text4: "", text5: ""}, ];

        VerticalLayout {
            HorizontalBox {
                height: 100px;
                width: 1000px;
                Rectangle {
                    open-file := Button {
                        text: "Open File (files open:" + root.counter +")";
                    }
                }
                Rectangle {
                    open-folder := Button {
                        text: "Open Folder (files open:" + root.counter +")";
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
                width: 1000px;
                height: 400px;
                Rectangle {
                    VerticalLayout {
                        for data in root.list-of-structs: my-repeated-text := HorizontalBox {
                            Text { text: data.text;}
                            Text {text: data.text2;}
                            Text {text: data.text3;}
                            Text {text: data.text4;}
                            Text {text: data.text5;}
                        }
                    }
                }
            }
        }
    }
}

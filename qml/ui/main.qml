import QtQuick 2.2
import QtQuick.Controls 1.0
import QtQuick.Layouts 1.1
import QtQuick.Window 2.1

ApplicationWindow {
    id: application
    x: 400
    y: 200
    width: 800
    height: 600
    title: "typr"
    Component.onCompleted: {
        visible = true
        //register signals
        WordModel.update.connect(onUpdate);
        WordModel.emptyInput.connect(emptyInput);

        WordModel.guiReady();
    }

    Rectangle {
        width: application.width - 40
        height: 100
        anchors.top: parent.top
        anchors.topMargin: 20

        anchors.left: parent.left
        anchors.leftMargin: 20
        anchors.right: parent.right
        anchors.rightMargin: 20
        color: '#CCF'

        border.color: "#aaa"
        border.width: 1
        radius: 4

        Text {
            id: contextText
            anchors.fill: parent
            text: WordModel.current_context
            font.pixelSize: 40
            wrapMode: Text.WordWrap
        }
    }

    Rectangle {
        width: 300
        height: 50
        anchors.centerIn: parent
        color: "#FFF"

        border.color: "#aaa"
        border.width: 1
        radius: 4

        TextInput {
            id: textInput
            anchors.fill: parent
            font.pixelSize: 40
            Component.onCompleted: textInput.forceActiveFocus()
            onTextChanged: {
                WordModel.current_word = textInput.text
                WordModel.validate()
            }
        }
    }

    function onUpdate() {
        console.log("qml is getting updated");
        contextText.text = WordModel.current_context
    }

    function emptyInput() {
        console.log("empty input!");
        textInput.text = "";
    }

}

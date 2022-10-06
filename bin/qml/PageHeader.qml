import QtQuick 2.0
import QtQuick.Controls 2.15
import "UI.js" as UI

ToolBar {
    property alias text: label.text
    Item {
        anchors.fill: parent
        Label {
            id: label
            anchors.verticalCenter: parent.verticalCenter
            anchors.left: parent.left
            anchors.leftMargin: UI.ITEM_MARGIN
            anchors.right: parent.right
            anchors.rightMargin: UI.ITEM_MARGIN
            font.pixelSize: UI.TEXT_SIZE_LARGE
        }
    }
}

import QtQuick 2.0
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import "UI.js" as UI

Page {
    id: container
    header: PageHeader {
        text: qsTr("SKDeck")
    }

    ListView {
        id: menuView
        anchors.fill: parent
        highlightMoveDuration: UI.HIGHLIGHT_MOVE_DURATION
        ScrollIndicator.vertical: ScrollIndicator {}
        model: menuModel
        focus: true
        Keys.forwardTo: menuView.currentItem

        delegate: ItemDelegate {
            width: parent.width
            onClicked: model.action()
            Keys.onReturnPressed: model.action()

            Label {
                id: nameLabel
                anchors.verticalCenter: parent.verticalCenter
                anchors.left: parent.left
                anchors.leftMargin: UI.ITEM_MARGIN
                anchors.right: parent.right
                anchors.rightMargin: UI.ITEM_MARGIN
                font.pixelSize: UI.TEXT_SIZE_LARGE
                text: qsTr(model.name)
            }
        }

        highlight: Rectangle {
            color: Material.listHighlightColor
            visible: menuView.focus
        }
    }

    ListModel {
        id: menuModel

        ListElement {
            name: QT_TR_NOOP("Crusader Kings III")
            action: function () {
                stackView.push(ck3Page)
            }
        }
        ListElement {
            name: QT_TR_NOOP("Quit")
            action: function () {
                Qt.quit()
            }
        }
    }

    Component {
        id: ck3Page
        Ck3Page {}
    }
}

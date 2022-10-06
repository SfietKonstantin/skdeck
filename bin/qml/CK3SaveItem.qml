import QtQuick 2.0
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import "UI.js" as UI

ItemDelegate {
    property alias name: nameLabel.text
    property date createdAt
    property bool current
    height: nameLabel.height + createdAtLabel.height + 2 * UI.ITEM_MARGIN + UI.TEXT_MARGIN

    Label {
        id: nameLabel
        anchors.top: parent.top
        anchors.topMargin: UI.ITEM_MARGIN
        anchors.left: parent.left
        anchors.leftMargin: UI.ITEM_MARGIN
        anchors.right: parent.right
        anchors.rightMargin: UI.ITEM_MARGIN
        font.pixelSize: UI.TEXT_SIZE_LARGE
        color: current ? Material.accentColor : Material.primaryTextColor
    }

    Label {
        id: createdAtLabel
        anchors.top: nameLabel.bottom
        anchors.topMargin: UI.TEXT_MARGIN
        anchors.left: parent.left
        anchors.leftMargin: UI.ITEM_MARGIN
        anchors.right: parent.right
        anchors.rightMargin: UI.ITEM_MARGIN
        font.pixelSize: UI.TEXT_SIZE_NORMAL
        font.italic: true
        color: Material.secondaryTextColor
        text: createdAt.toLocaleString(null, Locale.ShortFormat)
    }
}

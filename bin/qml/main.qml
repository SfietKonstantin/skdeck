import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import QtQuick.Layouts 1.15

ApplicationWindow {
    title: "SKDeck"
    width: 1280
    height: 800
    visible: true

    Material.theme: Material.Dark
    Material.accent: Material.Orange

    StackView {
        id: stackView
        anchors.fill: parent
        focus: true
        initialItem: MenuPage {}
    }
}

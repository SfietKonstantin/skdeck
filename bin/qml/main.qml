import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import QtQuick.Layouts 1.15
import QtGamepad 1.0

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

    Gamepad {
        id: gamepad
        deviceId: GamepadManager.connectedGamepads.length > 0 // Check connected gamepad
                  ? GamepadManager.connectedGamepads[0] : -1
    }

    Connections {
        target: GamepadManager
        function onGamepadConnected(deviceId) {
            gamepad1.deviceId = deviceId
        }
    }

    GamepadKeyNavigation {
        id: gamepadKeyNavigation
        gamepad: gamepad
        active: true
        buttonAKey: Qt.Key_Return
        buttonBKey: Qt.Key_Escape
    }
}

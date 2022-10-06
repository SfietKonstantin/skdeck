import QtQuick 2.0
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import QtQuick.Layouts 1.15
import skdeck 1.0
import "UI.js" as UI

Page {
    id: container

    header: PageHeader {
        text: qsTr("Crusader Kings III")
    }
    footer: ToolBar {
        RowLayout {
            ToolButton {
                text: qsTr("Back")
                icon.name: "go-previous"
                onClicked: stackView.pop()
            }

            ToolButton {
                text: qsTr("Reload")
                icon.name: "view-refresh"
                enabled: !saveLoader.loading
                onClicked: {
                    backupLoader.save = ""
                    saveLoader.reload()
                }
            }
        }
    }
    Keys.onEscapePressed: {
        if (backupView.focus) {
            saveView.focus = true
        } else {
            stackView.pop()
        }
    }

    Keys.onPressed: {
        if (!saveView.focus && !backupView.focus) {
            saveView.focus = true
        }
    }

    Ck3Model {
        id: saveModel
        loader: Ck3SaveLoader {
            id: saveLoader
        }
    }

    Ck3Model {
        id: backupModel
        loader: Ck3BackupLoader {
            id: backupLoader
        }
    }

    SplitView {
        anchors.fill: parent
        orientation: Qt.Horizontal

        ListView {
            id: saveView
            implicitWidth: container.width / 2
            highlightMoveDuration: UI.HIGHLIGHT_MOVE_DURATION
            ScrollIndicator.vertical: ScrollIndicator {}
            model: saveModel
            focus: true
            Keys.forwardTo: saveView.currentItem

            delegate: CK3SaveItem {
                width: saveView.width
                name: model.data.name
                createdAt: model.data.createdAt
                onClicked: backupView.select(model.data.name)
                Keys.onReturnPressed: backupView.select(model.data.name)
            }

            highlight: Rectangle {
                color: Material.listHighlightColor
                visible: saveView.focus
            }
        }
        ListView {
            id: backupView

            function select(name) {
                backupLoader.save = name
                backupView.focus = true
            }

            implicitWidth: container.width / 2
            ScrollIndicator.vertical: ScrollIndicator {}

            model: backupModel
            delegate: CK3SaveItem {
                function getName(model) {
                    if (model.data.kind === Ck3Save.SaveKind) {
                        return qsTr("Backup %1").arg(model.data.name)
                    } else if (model.data.kind === Ck3Save.CurrentBackupKind) {
                        return model.data.name
                    } else {
                        return qsTr("Restore %1").arg(model.data.name)
                    }
                }

                function handle() {
                    if (model.data.kind === Ck3Save.SaveKind) {
                        backupLoader.backup()
                    } else if (model.data.kind === Ck3Save.BackupKind) {
                        backupLoader.restore(model.data.name)
                    }
                }

                width: saveView.width
                name: getName(model)
                createdAt: model.data.createdAt
                current: model.data.kind === Ck3Save.CurrentBackupKind
                onClicked: handle()
                Keys.onReturnPressed: handle()
            }
            highlight: Rectangle {
                color: Material.listHighlightColor
                visible: backupView.focus
            }

            BusyIndicator {
                anchors.centerIn: parent
                visible: backupLoader.loading
            }
        }
    }
}

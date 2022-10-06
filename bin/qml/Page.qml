import QtQuick 2.0

FocusScope {
    id: container

    signal onReload
    function reload() {
        container.onReload()
    }

    Component.onCompleted: {
        container.reload()
        container.focus = true
    }

    onVisibleChanged: {
        if (visible) {
            container.reload()
            container.focus = true
        }
    }
}

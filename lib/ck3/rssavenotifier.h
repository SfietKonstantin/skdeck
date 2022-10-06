#pragma once

#include <QObject>

#include <ck3/save.h>

#include "rust/bindings.h"

namespace ck3 {

class RsSaveNotifier : public QObject {
    Q_OBJECT
public:
    explicit RsSaveNotifier(QObject *parent = nullptr);
    template <class F> void invoke(F function) {
        function(this, loadCallback);
    }

signals:
    void finished(QList<Save> saves);

private:
    static void loadCallback(void *ctx, const RsCk3Saves *saves);
};

} // namespace ck3

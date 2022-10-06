#pragma once

#include <QObject>

#include "save.h"

namespace ck3 {

class Loader : public QObject {
    Q_OBJECT
    Q_PROPERTY(bool loading READ loading NOTIFY loadingChanged)
public:
    bool loading() const;

signals:
    void loadingChanged();
    void started();
    void finished(QList<Save> saves);

protected:
    explicit Loader(QObject *parent = nullptr);
    void setLoading(bool loading);

private:
    bool m_loading{false};
};

} // namespace ck3

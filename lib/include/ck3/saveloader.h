#pragma once

#include "loader.h"

#include <QtQml/QQmlParserStatus>

namespace ck3 {

class SaveLoader : public Loader, public QQmlParserStatus {
    Q_OBJECT
    Q_INTERFACES(QQmlParserStatus)
public:
    explicit SaveLoader(QObject *parent = nullptr);
    void classBegin() override;
    void componentComplete() override;

public slots:
    void reload();

private:
    void setFinished(QList<Save> saves);
};

} // namespace ck3

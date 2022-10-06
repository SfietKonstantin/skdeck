#pragma once

#include "loader.h"

namespace ck3 {

class BackupLoader : public Loader {
    Q_OBJECT
    Q_PROPERTY(QString save READ save WRITE setSave NOTIFY saveChanged)
public:
    explicit BackupLoader(QObject *parent = nullptr);
    QString save() const;

public slots:
    void setSave(QString save);
    void backup();
    void restore(QString backup);

signals:
    void saveChanged();

private:
    void setFinished(QList<Save> saves);
    void reload();
    QString m_save;
};

} // namespace ck3

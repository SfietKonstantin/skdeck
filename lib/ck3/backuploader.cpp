#include <ck3/backuploader.h>

#include <QPointer>

#include "rssavenotifier.h"
#include "rust/bindings.h"

namespace ck3 {

BackupLoader::BackupLoader(QObject *parent)
    : Loader(parent) //
{
}

QString BackupLoader::save() const {
    return m_save;
}

void BackupLoader::setSave(QString save) {
    if (m_save != save) {
        m_save = save;
        reload();

        emit saveChanged();
    }
}

void BackupLoader::backup() {
    setLoading(true);
    emit started();

    auto bytes = m_save.toUtf8();
    auto notifier = new RsSaveNotifier();
    connect(notifier, &RsSaveNotifier::finished, this, &BackupLoader::setFinished, Qt::QueuedConnection);
    notifier->invoke([&bytes](auto ctx, auto callback) { ck3_backup(ctx, bytes.data(), callback); });
}

void BackupLoader::restore(QString backup) {
    setLoading(true);
    emit started();

    auto saveBytes = m_save.toUtf8();
    auto backupBytes = backup.toUtf8();
    auto notifier = new RsSaveNotifier();
    connect(notifier, &RsSaveNotifier::finished, this, &BackupLoader::setFinished, Qt::QueuedConnection);
    notifier->invoke([&saveBytes, &backupBytes](auto ctx, auto callback) {
        ck3_restore(ctx, saveBytes.data(), backupBytes.data(), callback);
    });
}

void BackupLoader::setFinished(QList<Save> saves) {
    emit finished(std::move(saves));
    setLoading(false);
}

void BackupLoader::reload() {
    setLoading(true);
    emit started();

    if (m_save.isEmpty()) {
        setFinished(QList<Save>());
    }

    auto bytes = m_save.toUtf8();
    auto notifier = new RsSaveNotifier();
    connect(notifier, &RsSaveNotifier::finished, this, &BackupLoader::setFinished, Qt::QueuedConnection);
    notifier->invoke([&bytes](auto ctx, auto callback) { ck3_list_backups(ctx, bytes.data(), callback); });
}

} // namespace ck3

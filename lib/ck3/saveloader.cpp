#include <ck3/saveloader.h>

#include "rssavenotifier.h"

namespace ck3 {

SaveLoader::SaveLoader(QObject *parent)
    : Loader(parent) //
{
}

void SaveLoader::classBegin() {
}

void SaveLoader::componentComplete() {
    reload();
}

void SaveLoader::reload() {
    emit started();

    auto notifier = new RsSaveNotifier();
    connect(notifier, &RsSaveNotifier::finished, this, &SaveLoader::setFinished, Qt::QueuedConnection);
    notifier->invoke(ck3_list_saves);
}

void SaveLoader::setFinished(QList<Save> saves) {
    emit finished(std::move(saves));
    setLoading(false);
}

} // namespace ck3

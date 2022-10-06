#include <ck3/model.h>

namespace ck3 {

Model::Model(QObject *parent)
    : QAbstractListModel(parent) //
{
}

QHash<int, QByteArray> Model::roleNames() const {
    return {{DataRole, "data"}};
}

int Model::rowCount(const QModelIndex &parent) const {
    Q_UNUSED(parent)
    return m_saves.size();
}

QVariant Model::data(const QModelIndex &index, int role) const {
    auto row = index.row();
    if (row < 0 || row >= m_saves.size()) {
        return QVariant();
    }

    auto save = m_saves.at(row);
    switch (role) {
    case DataRole:
        return QVariant::fromValue(save);
    default:
        return QVariant();
    }
}

int Model::count() const {
    return rowCount();
}

Loader *Model::loader() const {
    return m_loader;
}

void Model::setLoader(Loader *loader) {
    if (m_loader != loader) {
        // Disconnect previous loader
        if (m_loader != nullptr) {
            disconnect(m_loader, &Loader::started, this, &Model::onStarted);
            disconnect(m_loader, &Loader::finished, this, &Model::onFinished);
        }

        m_loader = loader;

        connect(m_loader, &Loader::started, this, &Model::onStarted, Qt::QueuedConnection);
        connect(m_loader, &Loader::finished, this, &Model::onFinished, Qt::QueuedConnection);

        emit loaderChanged();
    }
}

void Model::onStarted() {
    if (m_saves.isEmpty()) {
        return;
    }

    beginRemoveRows(QModelIndex(), 0, m_saves.size() - 1);
    m_saves.clear();
    emit countChanged();
    endRemoveRows();
}

void Model::onFinished(QList<Save> saves) {
    if (!m_saves.isEmpty()) {
        return;
    }

    beginInsertRows(QModelIndex(), 0, saves.size() - 1);
    m_saves = std::move(saves);
    emit countChanged();
    endInsertRows();
}

} // namespace ck3

//#include "model.moc"

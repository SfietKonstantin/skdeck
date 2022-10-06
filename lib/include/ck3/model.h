#pragma once

#include <QAbstractListModel>

#include <ck3/loader.h>
#include <ck3/save.h>

namespace ck3 {

// class SaveModelLoader;
class Model : public QAbstractListModel {
    Q_OBJECT
    Q_PROPERTY(int count READ count NOTIFY countChanged)
    Q_PROPERTY(Loader *loader READ loader WRITE setLoader NOTIFY loaderChanged)
public:
    enum Roles {
        DataRole = Qt::UserRole + 1,
    };
    explicit Model(QObject *parent = nullptr);
    QHash<int, QByteArray> roleNames() const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role) const override;
    int count() const;
    Loader *loader() const;
public slots:
    void setLoader(Loader *loader);
signals:
    void countChanged();
    void loadingChanged();
    void loaderChanged();

private:
    void onStarted();
    void onFinished(QList<Save> saves);
    Loader *m_loader{nullptr};
    QList<Save> m_saves;
};

} // namespace ck3

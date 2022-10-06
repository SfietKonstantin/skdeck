#pragma once

#include <QDateTime>
#include <QObject>

namespace ck3 {

class Save {
    Q_GADGET
    Q_PROPERTY(Kind kind READ kind CONSTANT)
    Q_PROPERTY(QString name READ name CONSTANT)
    Q_PROPERTY(QDateTime createdAt READ createdAt CONSTANT)
    Q_PROPERTY(bool backup READ isBackup CONSTANT)
    Q_PROPERTY(bool current READ isCurrent CONSTANT)
public:
    enum class Kind {
        SaveKind,
        CurrentBackupKind,
        BackupKind
    };
    Q_ENUM(Kind)
    explicit Save() = default;
    explicit Save(Kind kind, QString name, QDateTime createdAt, bool backup, bool current);
    Kind kind() const;
    const QString &name() const;
    const QDateTime &createdAt() const;
    bool isBackup() const;
    bool isCurrent() const;

private:
    Kind m_kind;
    QString m_name;
    QDateTime m_createdAt;
    bool m_backup{false};
    bool m_current{false};
};

} // namespace ck3

Q_DECLARE_METATYPE(ck3::Save)

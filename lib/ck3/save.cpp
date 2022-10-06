#include <ck3/save.h>

namespace ck3 {

Save::Save(Kind kind, QString name, QDateTime createdAt, bool backup, bool current)
    : m_kind(kind)
    , m_name(std::move(name))
    , m_createdAt(std::move(createdAt))
    , m_backup(backup)
    , m_current(current) //
{
}

Save::Kind Save::kind() const {
    return m_kind;
}

const QString &Save::name() const {
    return m_name;
}

const QDateTime &Save::createdAt() const {
    return m_createdAt;
}

bool Save::isBackup() const {
    return m_backup;
}

bool Save::isCurrent() const {
    return m_current;
}

} // namespace ck3

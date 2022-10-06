#include "rssavenotifier.h"

namespace ck3 {

namespace {

Save::Kind convert(RsCk3SaveKind kind) {
    switch (kind) {
    case RsCk3SaveKind::CurrentBackup:
        return Save::Kind::CurrentBackupKind;
    case RsCk3SaveKind::Backup:
        return Save::Kind::BackupKind;
    default:
        return Save::Kind::SaveKind;
    }
}

QList<Save> convert(const RsCk3Saves *saves) {
    auto returned = QList<ck3::Save>();
    for (uintptr_t i = 0; i < ck3_saves_size(saves); ++i) {
        auto *save = ck3_saves_at(saves, i);
        auto kind = ck3_save_kind(save);
        auto name = QString::fromUtf8(ck3_save_name(save));
        auto createdAt = QDateTime::fromMSecsSinceEpoch(ck3_save_created_at(save));
        auto backup = false;
        auto current = false;
        // auto backup = ck3_save_is_backup(save);
        // auto current = ck3_save_is_current(save);
        returned.append(ck3::Save(convert(kind), std::move(name), std::move(createdAt), backup, current));
    }
    return returned;
}

} // namespace

RsSaveNotifier::RsSaveNotifier(QObject *parent)
    : QObject(parent) //
{
}

void RsSaveNotifier::loadCallback(void *ctx, const RsCk3Saves *saves) {
    auto *self = static_cast<RsSaveNotifier *>(ctx);
    emit self->finished(convert(saves));
    self->deleteLater();
}

} // namespace ck3

#include <ck3/loader.h>

namespace ck3 {

bool Loader::loading() const {
    return m_loading;
}

Loader::Loader(QObject *parent)
    : QObject(parent) //
{
}

void Loader::setLoading(bool loading) {
    if (m_loading != loading) {
        m_loading = loading;
        emit loadingChanged();
    }
}

} // namespace ck3

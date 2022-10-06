#include <QGuiApplication>
#include <QLocale>
#include <QQmlApplicationEngine>
#include <QQuickStyle>
#include <QTranslator>

#include <ck3/backuploader.h>
#include <ck3/model.h>
#include <ck3/saveloader.h>

namespace {

void registerTypes() {
    qRegisterMetaType<ck3::Save>();

    qmlRegisterUncreatableType<ck3::Save>("skdeck", 1, 0, "Ck3Save", "Uncreatable");
    qmlRegisterUncreatableType<ck3::Loader>("skdeck", 1, 0, "Ck3Loader", "Uncreatable");
    qmlRegisterType<ck3::Model>("skdeck", 1, 0, "Ck3Model");
    qmlRegisterType<ck3::SaveLoader>("skdeck", 1, 0, "Ck3SaveLoader");
    qmlRegisterType<ck3::BackupLoader>("skdeck", 1, 0, "Ck3BackupLoader");
}

std::unique_ptr<QTranslator> createTranslator() {
    auto translator = std::make_unique<QTranslator>();
    auto uiLanguages = QLocale::system().uiLanguages();
    for (const auto &locale : uiLanguages) {
        const QString baseName = "skdeck_" + QLocale(locale).name();
        if (translator->load(":/i18n/" + baseName)) {
            return translator;
        }
    }

    return {};
}

} // namespace

int main(int argc, char *argv[]) {
    registerTypes();

    QGuiApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QGuiApplication app(argc, argv);
    //    QQuickStyle::setStyle("org.kde.desktop");
    QQuickStyle::setStyle("Material");

    auto translator = createTranslator();
    if (translator) {
        app.installTranslator(translator.get());
    }

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/qml/main.qml")));
    return app.exec();
}

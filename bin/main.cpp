#include "mainwindow.h"

#include <QApplication>
#include <QLocale>
#include <QTranslator>

namespace {

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
    QApplication app(argc, argv);

    auto translator = createTranslator();
    if (translator) {
        app.installTranslator(translator.get());
    }

    auto window = MainWindow();
    window.show();

    return app.exec();
}

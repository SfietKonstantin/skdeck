#pragma once

#include <QMainWindow>

namespace Ui {
class MainWindow;
} // namespace Ui

class MainWindow : public QMainWindow {
    Q_OBJECT
public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow() override;

private:
    std::unique_ptr<Ui::MainWindow> m_ui;
};

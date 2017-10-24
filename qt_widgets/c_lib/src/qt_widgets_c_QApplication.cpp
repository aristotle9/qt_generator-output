#include "qt_widgets_c_QApplication.h"

QApplication* qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QCoreApplication(QCoreApplication* ptr) {
  return static_cast<QApplication*>(ptr);
}

QApplication* qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QGuiApplication(QGuiApplication* ptr) {
  return static_cast<QApplication*>(ptr);
}

QApplication* qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QObject(QObject* ptr) {
  return static_cast<QApplication*>(ptr);
}

QCoreApplication* qt_widgets_c_QApplication_G_static_cast_QCoreApplication_ptr(QApplication* ptr) {
  return static_cast<QCoreApplication*>(ptr);
}

QGuiApplication* qt_widgets_c_QApplication_G_static_cast_QGuiApplication_ptr(QApplication* ptr) {
  return static_cast<QGuiApplication*>(ptr);
}

QObject* qt_widgets_c_QApplication_G_static_cast_QObject_ptr(QApplication* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QApplication_aboutQt() {
  QApplication::aboutQt();
}

QWidget* qt_widgets_c_QApplication_activeModalWidget() {
  return QApplication::activeModalWidget();
}

QWidget* qt_widgets_c_QApplication_activePopupWidget() {
  return QApplication::activePopupWidget();
}

QWidget* qt_widgets_c_QApplication_activeWindow() {
  return QApplication::activeWindow();
}

void qt_widgets_c_QApplication_alert_widget(QWidget* widget) {
  QApplication::alert(widget);
}

void qt_widgets_c_QApplication_alert_widget_duration(QWidget* widget, int duration) {
  QApplication::alert(widget, duration);
}

void qt_widgets_c_QApplication_allWidgets_to_output(QList< QWidget* >* output) {
  new(output) QList< QWidget* >(QApplication::allWidgets());
}

bool qt_widgets_c_QApplication_autoSipEnabled(const QApplication* this_ptr) {
  return this_ptr->autoSipEnabled();
}

void qt_widgets_c_QApplication_beep() {
  QApplication::beep();
}

void qt_widgets_c_QApplication_closeAllWindows() {
  QApplication::closeAllWindows();
}

int qt_widgets_c_QApplication_colorSpec() {
  return QApplication::colorSpec();
}

int qt_widgets_c_QApplication_cursorFlashTime() {
  return QApplication::cursorFlashTime();
}

void qt_widgets_c_QApplication_delete(QApplication* this_ptr) {
  delete this_ptr;
}

QDesktopWidget* qt_widgets_c_QApplication_desktop() {
  return QApplication::desktop();
}

int qt_widgets_c_QApplication_doubleClickInterval() {
  return QApplication::doubleClickInterval();
}

int qt_widgets_c_QApplication_exec() {
  return QApplication::exec();
}

QWidget* qt_widgets_c_QApplication_focusWidget() {
  return QApplication::focusWidget();
}

void qt_widgets_c_QApplication_fontMetrics_to_output(QFontMetrics* output) {
  new(output) QFontMetrics(QApplication::fontMetrics());
}

void qt_widgets_c_QApplication_font_to_output_arg1(const QWidget* arg1, QFont* output) {
  new(output) QFont(QApplication::font(arg1));
}

void qt_widgets_c_QApplication_font_to_output_className(const char* className, QFont* output) {
  new(output) QFont(QApplication::font(className));
}

void qt_widgets_c_QApplication_font_to_output_no_args(QFont* output) {
  new(output) QFont(QApplication::font());
}

void qt_widgets_c_QApplication_globalStrut_to_output(QSize* output) {
  new(output) QSize(QApplication::globalStrut());
}

bool qt_widgets_c_QApplication_isEffectEnabled(const Qt::UIEffect* arg1) {
  return QApplication::isEffectEnabled(*arg1);
}

int qt_widgets_c_QApplication_keyboardInputInterval() {
  return QApplication::keyboardInputInterval();
}

const QMetaObject* qt_widgets_c_QApplication_metaObject(const QApplication* this_ptr) {
  return this_ptr->metaObject();
}

QApplication* qt_widgets_c_QApplication_new_argc_argv(int* argc, char** argv) {
  return new QApplication(*argc, argv);
}

QApplication* qt_widgets_c_QApplication_new_argc_argv_arg3(int* argc, char** argv, int arg3) {
  return new QApplication(*argc, argv, arg3);
}

bool qt_widgets_c_QApplication_notify(QApplication* this_ptr, QObject* arg1, QEvent* arg2) {
  return this_ptr->notify(arg1, arg2);
}

void qt_widgets_c_QApplication_palette_to_output_arg1(const QWidget* arg1, QPalette* output) {
  new(output) QPalette(QApplication::palette(arg1));
}

void qt_widgets_c_QApplication_palette_to_output_className(const char* className, QPalette* output) {
  new(output) QPalette(QApplication::palette(className));
}

int qt_widgets_c_QApplication_qt_metacall(QApplication* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QApplication_qt_metacast(QApplication* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QApplication_setActiveWindow(QWidget* act) {
  QApplication::setActiveWindow(act);
}

void qt_widgets_c_QApplication_setAutoSipEnabled(QApplication* this_ptr, const bool enabled) {
  this_ptr->setAutoSipEnabled(enabled);
}

void qt_widgets_c_QApplication_setColorSpec(int arg1) {
  QApplication::setColorSpec(arg1);
}

void qt_widgets_c_QApplication_setCursorFlashTime(int arg1) {
  QApplication::setCursorFlashTime(arg1);
}

void qt_widgets_c_QApplication_setDoubleClickInterval(int arg1) {
  QApplication::setDoubleClickInterval(arg1);
}

void qt_widgets_c_QApplication_setEffectEnabled_arg1(const Qt::UIEffect* arg1) {
  QApplication::setEffectEnabled(*arg1);
}

void qt_widgets_c_QApplication_setEffectEnabled_arg1_enable(const Qt::UIEffect* arg1, bool enable) {
  QApplication::setEffectEnabled(*arg1, enable);
}

void qt_widgets_c_QApplication_setFont_arg1(const QFont* arg1) {
  QApplication::setFont(*arg1);
}

void qt_widgets_c_QApplication_setFont_arg1_className(const QFont* arg1, const char* className) {
  QApplication::setFont(*arg1, className);
}

void qt_widgets_c_QApplication_setGlobalStrut(const QSize* arg1) {
  QApplication::setGlobalStrut(*arg1);
}

void qt_widgets_c_QApplication_setKeyboardInputInterval(int arg1) {
  QApplication::setKeyboardInputInterval(arg1);
}

void qt_widgets_c_QApplication_setPalette_arg1(const QPalette* arg1) {
  QApplication::setPalette(*arg1);
}

void qt_widgets_c_QApplication_setPalette_arg1_className(const QPalette* arg1, const char* className) {
  QApplication::setPalette(*arg1, className);
}

void qt_widgets_c_QApplication_setStartDragDistance(int l) {
  QApplication::setStartDragDistance(l);
}

void qt_widgets_c_QApplication_setStartDragTime(int ms) {
  QApplication::setStartDragTime(ms);
}

void qt_widgets_c_QApplication_setStyleSheet(QApplication* this_ptr, const QString* sheet) {
  this_ptr->setStyleSheet(*sheet);
}

QStyle* qt_widgets_c_QApplication_setStyle_QString(const QString* arg1) {
  return QApplication::setStyle(*arg1);
}

void qt_widgets_c_QApplication_setStyle_QStyle(QStyle* arg1) {
  QApplication::setStyle(arg1);
}

void qt_widgets_c_QApplication_setWheelScrollLines(int arg1) {
  QApplication::setWheelScrollLines(arg1);
}

void qt_widgets_c_QApplication_setWindowIcon(const QIcon* icon) {
  QApplication::setWindowIcon(*icon);
}

int qt_widgets_c_QApplication_startDragDistance() {
  return QApplication::startDragDistance();
}

int qt_widgets_c_QApplication_startDragTime() {
  return QApplication::startDragTime();
}

QStyle* qt_widgets_c_QApplication_style() {
  return QApplication::style();
}

void qt_widgets_c_QApplication_styleSheet_to_output(const QApplication* this_ptr, QString* output) {
  new(output) QString(this_ptr->styleSheet());
}

QWidget* qt_widgets_c_QApplication_topLevelAt_p(const QPoint* p) {
  return QApplication::topLevelAt(*p);
}

QWidget* qt_widgets_c_QApplication_topLevelAt_x_y(int x, int y) {
  return QApplication::topLevelAt(x, y);
}

void qt_widgets_c_QApplication_topLevelWidgets_to_output(QList< QWidget* >* output) {
  new(output) QList< QWidget* >(QApplication::topLevelWidgets());
}

void qt_widgets_c_QApplication_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QApplication::trUtf8(s, c, n));
}

void qt_widgets_c_QApplication_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QApplication::tr(s, c, n));
}

int qt_widgets_c_QApplication_wheelScrollLines() {
  return QApplication::wheelScrollLines();
}

QWidget* qt_widgets_c_QApplication_widgetAt_p(const QPoint* p) {
  return QApplication::widgetAt(*p);
}

QWidget* qt_widgets_c_QApplication_widgetAt_x_y(int x, int y) {
  return QApplication::widgetAt(x, y);
}

void qt_widgets_c_QApplication_windowIcon_to_output(QIcon* output) {
  new(output) QIcon(QApplication::windowIcon());
}


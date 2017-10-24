#include "qt_gui_c_QStyleHints.h"

QObject* qt_gui_c_QStyleHints_G_static_cast_QObject_ptr(QStyleHints* ptr) {
  return static_cast<QObject*>(ptr);
}

QStyleHints* qt_gui_c_QStyleHints_G_static_cast_QStyleHints_ptr(QObject* ptr) {
  return static_cast<QStyleHints*>(ptr);
}

int qt_gui_c_QStyleHints_cursorFlashTime(const QStyleHints* this_ptr) {
  return this_ptr->cursorFlashTime();
}

void qt_gui_c_QStyleHints_delete(QStyleHints* this_ptr) {
  delete this_ptr;
}

double qt_gui_c_QStyleHints_fontSmoothingGamma(const QStyleHints* this_ptr) {
  return this_ptr->fontSmoothingGamma();
}

int qt_gui_c_QStyleHints_keyboardAutoRepeatRate(const QStyleHints* this_ptr) {
  return this_ptr->keyboardAutoRepeatRate();
}

int qt_gui_c_QStyleHints_keyboardInputInterval(const QStyleHints* this_ptr) {
  return this_ptr->keyboardInputInterval();
}

const QMetaObject* qt_gui_c_QStyleHints_metaObject(const QStyleHints* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QStyleHints_mouseDoubleClickInterval(const QStyleHints* this_ptr) {
  return this_ptr->mouseDoubleClickInterval();
}

int qt_gui_c_QStyleHints_mousePressAndHoldInterval(const QStyleHints* this_ptr) {
  return this_ptr->mousePressAndHoldInterval();
}

void qt_gui_c_QStyleHints_passwordMaskCharacter_to_output(const QStyleHints* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->passwordMaskCharacter());
}

int qt_gui_c_QStyleHints_passwordMaskDelay(const QStyleHints* this_ptr) {
  return this_ptr->passwordMaskDelay();
}

int qt_gui_c_QStyleHints_qt_metacall(QStyleHints* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QStyleHints_qt_metacast(QStyleHints* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QStyleHints_setCursorFlashTime(QStyleHints* this_ptr, int cursorFlashTime) {
  this_ptr->setCursorFlashTime(cursorFlashTime);
}

bool qt_gui_c_QStyleHints_setFocusOnTouchRelease(const QStyleHints* this_ptr) {
  return this_ptr->setFocusOnTouchRelease();
}

void qt_gui_c_QStyleHints_setKeyboardInputInterval(QStyleHints* this_ptr, int keyboardInputInterval) {
  this_ptr->setKeyboardInputInterval(keyboardInputInterval);
}

void qt_gui_c_QStyleHints_setMouseDoubleClickInterval(QStyleHints* this_ptr, int mouseDoubleClickInterval) {
  this_ptr->setMouseDoubleClickInterval(mouseDoubleClickInterval);
}

void qt_gui_c_QStyleHints_setMousePressAndHoldInterval(QStyleHints* this_ptr, int mousePressAndHoldInterval) {
  this_ptr->setMousePressAndHoldInterval(mousePressAndHoldInterval);
}

void qt_gui_c_QStyleHints_setStartDragDistance(QStyleHints* this_ptr, int startDragDistance) {
  this_ptr->setStartDragDistance(startDragDistance);
}

void qt_gui_c_QStyleHints_setStartDragTime(QStyleHints* this_ptr, int startDragTime) {
  this_ptr->setStartDragTime(startDragTime);
}

void qt_gui_c_QStyleHints_setTabFocusBehavior(QStyleHints* this_ptr, const Qt::TabFocusBehavior* tabFocusBehavior) {
  this_ptr->setTabFocusBehavior(*tabFocusBehavior);
}

void qt_gui_c_QStyleHints_setUseHoverEffects(QStyleHints* this_ptr, bool useHoverEffects) {
  this_ptr->setUseHoverEffects(useHoverEffects);
}

void qt_gui_c_QStyleHints_setWheelScrollLines(QStyleHints* this_ptr, int scrollLines) {
  this_ptr->setWheelScrollLines(scrollLines);
}

bool qt_gui_c_QStyleHints_showIsFullScreen(const QStyleHints* this_ptr) {
  return this_ptr->showIsFullScreen();
}

bool qt_gui_c_QStyleHints_showIsMaximized(const QStyleHints* this_ptr) {
  return this_ptr->showIsMaximized();
}

bool qt_gui_c_QStyleHints_singleClickActivation(const QStyleHints* this_ptr) {
  return this_ptr->singleClickActivation();
}

int qt_gui_c_QStyleHints_startDragDistance(const QStyleHints* this_ptr) {
  return this_ptr->startDragDistance();
}

int qt_gui_c_QStyleHints_startDragTime(const QStyleHints* this_ptr) {
  return this_ptr->startDragTime();
}

int qt_gui_c_QStyleHints_startDragVelocity(const QStyleHints* this_ptr) {
  return this_ptr->startDragVelocity();
}

void qt_gui_c_QStyleHints_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStyleHints::trUtf8(s, c, n));
}

void qt_gui_c_QStyleHints_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStyleHints::tr(s, c, n));
}

bool qt_gui_c_QStyleHints_useHoverEffects(const QStyleHints* this_ptr) {
  return this_ptr->useHoverEffects();
}

bool qt_gui_c_QStyleHints_useRtlExtensions(const QStyleHints* this_ptr) {
  return this_ptr->useRtlExtensions();
}

int qt_gui_c_QStyleHints_wheelScrollLines(const QStyleHints* this_ptr) {
  return this_ptr->wheelScrollLines();
}


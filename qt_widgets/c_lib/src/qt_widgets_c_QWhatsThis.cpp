#include "qt_widgets_c_QWhatsThis.h"

QAction* qt_widgets_c_QWhatsThis_createAction_no_args() {
  return QWhatsThis::createAction();
}

QAction* qt_widgets_c_QWhatsThis_createAction_parent(QObject* parent) {
  return QWhatsThis::createAction(parent);
}

void qt_widgets_c_QWhatsThis_delete(QWhatsThis* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QWhatsThis_enterWhatsThisMode() {
  QWhatsThis::enterWhatsThisMode();
}

void qt_widgets_c_QWhatsThis_hideText() {
  QWhatsThis::hideText();
}

bool qt_widgets_c_QWhatsThis_inWhatsThisMode() {
  return QWhatsThis::inWhatsThisMode();
}

void qt_widgets_c_QWhatsThis_leaveWhatsThisMode() {
  QWhatsThis::leaveWhatsThisMode();
}

void qt_widgets_c_QWhatsThis_showText_pos_text(const QPoint* pos, const QString* text) {
  QWhatsThis::showText(*pos, *text);
}

void qt_widgets_c_QWhatsThis_showText_pos_text_w(const QPoint* pos, const QString* text, QWidget* w) {
  QWhatsThis::showText(*pos, *text, w);
}


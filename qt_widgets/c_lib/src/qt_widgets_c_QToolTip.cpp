#include "qt_widgets_c_QToolTip.h"

void qt_widgets_c_QToolTip_delete(QToolTip* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QToolTip_font_to_output(QFont* output) {
  new(output) QFont(QToolTip::font());
}

void qt_widgets_c_QToolTip_hideText() {
  QToolTip::hideText();
}

bool qt_widgets_c_QToolTip_isVisible() {
  return QToolTip::isVisible();
}

void qt_widgets_c_QToolTip_palette_to_output(QPalette* output) {
  new(output) QPalette(QToolTip::palette());
}

void qt_widgets_c_QToolTip_setFont(const QFont* arg1) {
  QToolTip::setFont(*arg1);
}

void qt_widgets_c_QToolTip_setPalette(const QPalette* arg1) {
  QToolTip::setPalette(*arg1);
}

void qt_widgets_c_QToolTip_showText_pos_text(const QPoint* pos, const QString* text) {
  QToolTip::showText(*pos, *text);
}

void qt_widgets_c_QToolTip_showText_pos_text_w(const QPoint* pos, const QString* text, QWidget* w) {
  QToolTip::showText(*pos, *text, w);
}

void qt_widgets_c_QToolTip_showText_pos_text_w_rect(const QPoint* pos, const QString* text, QWidget* w, const QRect* rect) {
  QToolTip::showText(*pos, *text, w, *rect);
}

void qt_widgets_c_QToolTip_showText_pos_text_w_rect_msecShowTime(const QPoint* pos, const QString* text, QWidget* w, const QRect* rect, int msecShowTime) {
  QToolTip::showText(*pos, *text, w, *rect, msecShowTime);
}

void qt_widgets_c_QToolTip_text_to_output(QString* output) {
  new(output) QString(QToolTip::text());
}


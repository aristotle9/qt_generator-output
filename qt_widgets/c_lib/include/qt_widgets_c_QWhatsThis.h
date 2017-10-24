#ifndef QT_WIDGETS_C_QWHATSTHIS_H
#define QT_WIDGETS_C_QWHATSTHIS_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QWhatsThis_createAction_no_args();
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QWhatsThis_createAction_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWhatsThis_delete(QWhatsThis* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWhatsThis_enterWhatsThisMode();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWhatsThis_hideText();
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWhatsThis_inWhatsThisMode();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWhatsThis_leaveWhatsThisMode();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWhatsThis_showText_pos_text(const QPoint* pos, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWhatsThis_showText_pos_text_w(const QPoint* pos, const QString* text, QWidget* w);

} // extern "C"

#endif // QT_WIDGETS_C_QWHATSTHIS_H

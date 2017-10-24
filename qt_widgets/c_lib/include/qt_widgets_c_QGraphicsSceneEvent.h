#ifndef QT_WIDGETS_C_QGRAPHICSSCENEEVENT_H
#define QT_WIDGETS_C_QGRAPHICSSCENEEVENT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QEvent* qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QEvent_ptr(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QGraphicsSceneEvent_ptr(QEvent* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneEvent_delete(QGraphicsSceneEvent* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneEvent_new(QEvent::Type type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneEvent_setWidget(QGraphicsSceneEvent* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QGraphicsSceneEvent_widget(const QGraphicsSceneEvent* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSSCENEEVENT_H

#ifndef QT_WIDGETS_C_QGRAPHICSSCENEDRAGDROPEVENT_H
#define QT_WIDGETS_C_QGRAPHICSSCENEDRAGDROPEVENT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_dynamic_cast_QGraphicsSceneDragDropEvent_ptr(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT QEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QEvent_ptr(QGraphicsSceneDragDropEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneDragDropEvent_ptr_QEvent(QEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneDragDropEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneDragDropEvent* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_acceptProposedAction(QGraphicsSceneDragDropEvent* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_delete(QGraphicsSceneDragDropEvent* this_ptr);
QT_WIDGETS_C_EXPORT const QMimeData* qt_widgets_c_QGraphicsSceneDragDropEvent_mimeData(const QGraphicsSceneDragDropEvent* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_new_type(QEvent::Type type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_pos_to_output(const QGraphicsSceneDragDropEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_scenePos_to_output(const QGraphicsSceneDragDropEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_screenPos_to_output(const QGraphicsSceneDragDropEvent* this_ptr, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_setDropAction(QGraphicsSceneDragDropEvent* this_ptr, const Qt::DropAction* action);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_setMimeData(QGraphicsSceneDragDropEvent* this_ptr, const QMimeData* data);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_setPos(QGraphicsSceneDragDropEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_setProposedAction(QGraphicsSceneDragDropEvent* this_ptr, const Qt::DropAction* action);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_setScenePos(QGraphicsSceneDragDropEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_setScreenPos(QGraphicsSceneDragDropEvent* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneDragDropEvent_setSource(QGraphicsSceneDragDropEvent* this_ptr, QWidget* source);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QGraphicsSceneDragDropEvent_source(const QGraphicsSceneDragDropEvent* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSSCENEDRAGDROPEVENT_H

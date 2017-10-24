#ifndef QT_WIDGETS_C_QSTYLEOPTIONRUBBERBAND_H
#define QT_WIDGETS_C_QSTYLEOPTIONRUBBERBAND_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyleOptionRubberBand* qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOptionRubberBand_ptr(QStyleOption* ptr);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOption_ptr(QStyleOptionRubberBand* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionRubberBand_delete(QStyleOptionRubberBand* this_ptr);
QT_WIDGETS_C_EXPORT QStyleOptionRubberBand* qt_widgets_c_QStyleOptionRubberBand_new_no_args();
QT_WIDGETS_C_EXPORT QStyleOptionRubberBand* qt_widgets_c_QStyleOptionRubberBand_new_other(const QStyleOptionRubberBand* other);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStyleOptionRubberBand_opaque(const QStyleOptionRubberBand* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionRubberBand_set_opaque(QStyleOptionRubberBand* this_ptr, bool value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionRubberBand_set_shape(QStyleOptionRubberBand* this_ptr, const QRubberBand::Shape* value);
QT_WIDGETS_C_EXPORT const QRubberBand::Shape* qt_widgets_c_QStyleOptionRubberBand_shape(const QStyleOptionRubberBand* this_ptr);
QT_WIDGETS_C_EXPORT QRubberBand::Shape* qt_widgets_c_QStyleOptionRubberBand_shape_mut(QStyleOptionRubberBand* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEOPTIONRUBBERBAND_H
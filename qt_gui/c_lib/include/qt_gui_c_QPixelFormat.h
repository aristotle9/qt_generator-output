#ifndef QT_GUI_C_QPIXELFORMAT_H
#define QT_GUI_C_QPIXELFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_QtPrivate_QPixelFormat_createYUV_to_output(const QPixelFormat::YUVLayout* yuvLayout, unsigned char alphaSize, const QPixelFormat::AlphaUsage* alphaUsage, const QPixelFormat::AlphaPosition* alphaPosition, const QPixelFormat::AlphaPremultiplied* premultiplied, const QPixelFormat::TypeInterpretation* typeInterpretation, const QPixelFormat::ByteOrder* byteOrder, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatAlpha_to_output_channelSize(unsigned char channelSize, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatAlpha_to_output_channelSize_typeInt(unsigned char channelSize, const QPixelFormat::TypeInterpretation* typeInt, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize(unsigned char channelSize, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize_alfa(unsigned char channelSize, unsigned char alfa, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize_alfa_usage(unsigned char channelSize, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize_alfa_usage_position(unsigned char channelSize, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize_alfa_usage_position_typeInt(unsigned char channelSize, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, const QPixelFormat::TypeInterpretation* typeInt, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatGrayscale_to_output_channelSize(unsigned char channelSize, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatGrayscale_to_output_channelSize_typeInt(unsigned char channelSize, const QPixelFormat::TypeInterpretation* typeInt, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize(unsigned char channelSize, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize_alfa(unsigned char channelSize, unsigned char alfa, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize_alfa_usage(unsigned char channelSize, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize_alfa_usage_position(unsigned char channelSize, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize_alfa_usage_position_typeInt(unsigned char channelSize, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, const QPixelFormat::TypeInterpretation* typeInt, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize(unsigned char channelSize, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize_alfa(unsigned char channelSize, unsigned char alfa, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize_alfa_usage(unsigned char channelSize, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize_alfa_usage_position(unsigned char channelSize, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize_alfa_usage_position_typeInt(unsigned char channelSize, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, const QPixelFormat::TypeInterpretation* typeInt, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatRgba_to_output_red_green_blue_alfa_usage_position(unsigned char red, unsigned char green, unsigned char blue, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatRgba_to_output_red_green_blue_alfa_usage_position_pmul(unsigned char red, unsigned char green, unsigned char blue, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, const QPixelFormat::AlphaPremultiplied* pmul, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatRgba_to_output_red_green_blue_alfa_usage_position_pmul_typeInt(unsigned char red, unsigned char green, unsigned char blue, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, const QPixelFormat::AlphaPremultiplied* pmul, const QPixelFormat::TypeInterpretation* typeInt, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout(const QPixelFormat::YUVLayout* layout, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa(const QPixelFormat::YUVLayout* layout, unsigned char alfa, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage(const QPixelFormat::YUVLayout* layout, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage_position(const QPixelFormat::YUVLayout* layout, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage_position_p_mul(const QPixelFormat::YUVLayout* layout, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, const QPixelFormat::AlphaPremultiplied* p_mul, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage_position_p_mul_typeInt(const QPixelFormat::YUVLayout* layout, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, const QPixelFormat::AlphaPremultiplied* p_mul, const QPixelFormat::TypeInterpretation* typeInt, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage_position_p_mul_typeInt_b_order(const QPixelFormat::YUVLayout* layout, unsigned char alfa, const QPixelFormat::AlphaUsage* usage, const QPixelFormat::AlphaPosition* position, const QPixelFormat::AlphaPremultiplied* p_mul, const QPixelFormat::TypeInterpretation* typeInt, const QPixelFormat::ByteOrder* b_order, QPixelFormat* output);
QT_GUI_C_EXPORT QPixelFormat::AlphaPosition qt_gui_c_QPixelFormat_alphaPosition(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_alphaSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT QPixelFormat::AlphaUsage qt_gui_c_QPixelFormat_alphaUsage(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_bitsPerPixel(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_blackSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_blueSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_brightnessSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT QPixelFormat::ByteOrder qt_gui_c_QPixelFormat_byteOrder(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_channelCount(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT QPixelFormat::ColorModel qt_gui_c_QPixelFormat_colorModel(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_constructor_colorModel_firstSize_secondSize_thirdSize_fourthSize_fifthSize_alphaSize_alphaUsage_alphaPosition_premultiplied_typeInterpretation(QPixelFormat::ColorModel colorModel, unsigned char firstSize, unsigned char secondSize, unsigned char thirdSize, unsigned char fourthSize, unsigned char fifthSize, unsigned char alphaSize, QPixelFormat::AlphaUsage alphaUsage, QPixelFormat::AlphaPosition alphaPosition, QPixelFormat::AlphaPremultiplied premultiplied, QPixelFormat::TypeInterpretation typeInterpretation, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_constructor_colorModel_firstSize_secondSize_thirdSize_fourthSize_fifthSize_alphaSize_alphaUsage_alphaPosition_premultiplied_typeInterpretation_byteOrder(QPixelFormat::ColorModel colorModel, unsigned char firstSize, unsigned char secondSize, unsigned char thirdSize, unsigned char fourthSize, unsigned char fifthSize, unsigned char alphaSize, QPixelFormat::AlphaUsage alphaUsage, QPixelFormat::AlphaPosition alphaPosition, QPixelFormat::AlphaPremultiplied premultiplied, QPixelFormat::TypeInterpretation typeInterpretation, QPixelFormat::ByteOrder byteOrder, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_constructor_colorModel_firstSize_secondSize_thirdSize_fourthSize_fifthSize_alphaSize_alphaUsage_alphaPosition_premultiplied_typeInterpretation_byteOrder_subEnum(QPixelFormat::ColorModel colorModel, unsigned char firstSize, unsigned char secondSize, unsigned char thirdSize, unsigned char fourthSize, unsigned char fifthSize, unsigned char alphaSize, QPixelFormat::AlphaUsage alphaUsage, QPixelFormat::AlphaPosition alphaPosition, QPixelFormat::AlphaPremultiplied premultiplied, QPixelFormat::TypeInterpretation typeInterpretation, QPixelFormat::ByteOrder byteOrder, unsigned char subEnum, QPixelFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_constructor_no_args(QPixelFormat* output);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_cyanSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPixelFormat_destructor(QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_greenSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_hueSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_lightnessSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_magentaSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT QPixelFormat::AlphaPremultiplied qt_gui_c_QPixelFormat_premultiplied(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_redSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_saturationSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_subEnum(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT QPixelFormat::TypeInterpretation qt_gui_c_QPixelFormat_typeInterpretation(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QPixelFormat_yellowSize(const QPixelFormat* this_ptr);
QT_GUI_C_EXPORT QPixelFormat::YUVLayout qt_gui_c_QPixelFormat_yuvLayout(const QPixelFormat* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPIXELFORMAT_H

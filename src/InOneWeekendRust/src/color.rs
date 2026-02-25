use crate::interval::Interval;
use glam::Vec3;

/// 線形輝度値をガンマ補正済みの値に変換する（section 9.5）。
/// ガンマ値 2 を仮定しているので、変換は sqrt（2乗の逆操作）になる。
/// 負の値はそのまま黒にする。
fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

/// 線形輝度の Vec3 をガンマ補正して 8bit RGB ピクセルに変換する。
/// - ガンマ補正で暗部の潰れを修正する
/// - [0.0, 0.999] にクランプして u8 変換時のオーバーフローを防ぐ
pub fn to_rgb(pixel_color: Vec3) -> image::Rgb<u8> {
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    // 0〜255 の整数値に写像する。256 倍して切り捨てることで [0, 255] を均等に使う。
    let intensity = Interval::new(0.000, 0.999);
    let r_byte = (256.0 * intensity.clamp(r)) as u8;
    let g_byte = (256.0 * intensity.clamp(g)) as u8;
    let b_byte = (256.0 * intensity.clamp(b)) as u8;

    image::Rgb([r_byte, g_byte, b_byte])
}

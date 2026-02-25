/// 実数区間 [min, max] を表す型。
/// レイのヒット判定で「有効な t の範囲」として使う。
#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    /// 空区間: min > max なので何も含まない
    pub const EMPTY: Self = Interval {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };
    /// 全区間: あらゆる実数を含む
    pub const UNIVERSE: Self = Interval {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };

    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    /// 区間の幅
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    /// x が区間に含まれるか（端点を含む）
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    /// x が区間の内側にあるか（端点を含まない）
    /// ヒット判定では端点ちょうどを除外したいため surrounds を使う
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    /// x を [min, max] に収める
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

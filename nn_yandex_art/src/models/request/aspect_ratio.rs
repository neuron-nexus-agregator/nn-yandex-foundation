use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AspectRatio{
    width_ratio: i64,
    height_ratio: i64,
}

/// Builder for AspectRatio
#[derive(Debug)]
pub struct AspectRatioBuilder{
    width_ratio: i64,
    height_ratio: i64,
}

/// Builder for AspectRatio
impl AspectRatioBuilder{
    pub fn new() -> Self{
        AspectRatioBuilder{
            width_ratio: 1,
            height_ratio: 1
        }
    }

    pub fn width_ratio(mut self, width_ratio: i64) -> Self{
        self.width_ratio = width_ratio;
        self
    }

    pub fn height_ratio(mut self, height_ratio: i64) -> Self{
        self.height_ratio = height_ratio;
        self
    }

    pub fn build(self) -> AspectRatio{
        AspectRatio{
            width_ratio: self.width_ratio,
            height_ratio: self.height_ratio
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_ratio_builder_defaults() {
        let ar = AspectRatioBuilder::new().build();
        assert_eq!(ar.width_ratio, 1);
        assert_eq!(ar.height_ratio, 1);
    }

    #[test]
    fn test_aspect_ratio_builder_custom_values() {
        let ar = AspectRatioBuilder::new()
            .width_ratio(16)
            .height_ratio(9)
            .build();

        assert_eq!(ar.width_ratio, 16);
        assert_eq!(ar.height_ratio, 9);
    }
}
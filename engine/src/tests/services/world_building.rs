#[cfg(test)]
pub mod surface_type_heights_inteval_tests {
    use crate::services::world_building::SurfaceTypeHeightsInteval;

    #[test]
    #[rustfmt::skip]
    pub fn contains__contains_height__is_true() {
        let interval =
            SurfaceTypeHeightsInteval::new(0.3, 0.75);
        
        assert!(interval.contains(0.4));
    }

    #[test]
    #[rustfmt::skip]
    pub fn contains__height_less_than_min__is_false() {
        let interval =
            SurfaceTypeHeightsInteval::new(0.3, 0.75);
        
        assert!(!interval.contains(0.2));        
    }

    #[test]
    #[rustfmt::skip]
    pub fn contains__height_greater_than_max__is_false() {
        let interval =
            SurfaceTypeHeightsInteval::new(0.3, 0.75);
        
        assert!(!interval.contains(0.9));        
    }

    #[test]
    #[rustfmt::skip]
    pub fn contains__height_equals_min__is_true() {
        let interval =
            SurfaceTypeHeightsInteval::new(0.3, 0.75);
        
        assert!(interval.contains(0.3));        
    }

    #[test]
    #[rustfmt::skip]
    pub fn contains__height_equals_max__is_true() {
        let interval =
            SurfaceTypeHeightsInteval::new(0.3, 0.75);
        
        assert!(interval.contains(0.75));        
    }
}

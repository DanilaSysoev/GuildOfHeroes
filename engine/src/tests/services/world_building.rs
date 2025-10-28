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

pub mod map_builder_from_heights_tests {
    use crate::{
        services::world_building::{MapBuilder, MapBuilderFromHeights},
        world::entities::global::SurfaceType,
    };

    static DEFAULT_SURFACE_TYPE: SurfaceType = SurfaceType::Swamp;

    fn get_data() -> &'static [&'static [f64]] {
        &[
            &[0.0, 0.05, 0.1],
            &[0.1, 0.15, 0.2],
            &[0.2, 0.25, 0.3],
            &[0.3, 0.35, 0.4],
            &[0.4, 0.45, 0.5],
            &[0.5, 0.55, 0.6],
            &[0.6, 0.65, 0.7],
        ]
    }

    fn get_builder() -> MapBuilderFromHeights {
        MapBuilderFromHeights::new(get_data())
            .with_default_surface_type(DEFAULT_SURFACE_TYPE)
            .with_surface_type_interval(SurfaceType::Forest, 0.0, 0.1)
            .with_surface_type_interval(SurfaceType::Ground, 0.1, 0.2)
            .with_surface_type_interval(SurfaceType::Hill, 0.2, 0.3)
            .with_surface_type_interval(SurfaceType::Mountain, 0.3, 0.4)
            .with_surface_type_interval(SurfaceType::Swamp, 0.4, 0.5)
            .with_surface_type_interval(SurfaceType::Water, 0.5, 0.6)
    }

    #[test]
    #[rustfmt::skip]
    fn build__heights_inside_intervals__surface_type_is_correct() {
        let map = get_builder().build().unwrap();
        assert!(*map.get_tile((0, 1)).unwrap().surface_type() == SurfaceType::Forest);
        assert!(*map.get_tile((1, 1)).unwrap().surface_type() == SurfaceType::Ground);
        assert!(*map.get_tile((2, 1)).unwrap().surface_type() == SurfaceType::Hill);
        assert!(*map.get_tile((3, 1)).unwrap().surface_type() == SurfaceType::Mountain);
        assert!(*map.get_tile((4, 1)).unwrap().surface_type() == SurfaceType::Swamp);
        assert!(*map.get_tile((5, 1)).unwrap().surface_type() == SurfaceType::Water);
    }

    #[test]
    #[rustfmt::skip]
    fn build__height_outside_all_intervals__surface_type_is_default() {
        let map = get_builder().build().unwrap();
        assert!(*map.get_tile((6, 1)).unwrap().surface_type() == DEFAULT_SURFACE_TYPE);
    }

    #[test]
    #[rustfmt::skip]
    fn build__heights_on_bound__surface_type_is_one_of_acceptable() {
        let map = get_builder().build().unwrap();
        assert!(*map.get_tile((0, 0)).unwrap().surface_type() == SurfaceType::Forest);

        assert!(*map.get_tile((1, 0)).unwrap().surface_type() == SurfaceType::Ground ||
                *map.get_tile((1, 0)).unwrap().surface_type() == SurfaceType::Forest);

        assert!(*map.get_tile((2, 0)).unwrap().surface_type() == SurfaceType::Hill ||
                *map.get_tile((2, 0)).unwrap().surface_type() == SurfaceType::Ground);

        assert!(*map.get_tile((3, 0)).unwrap().surface_type() == SurfaceType::Mountain ||
                *map.get_tile((3, 0)).unwrap().surface_type() == SurfaceType::Hill);

        assert!(*map.get_tile((4, 0)).unwrap().surface_type() == SurfaceType::Swamp ||
                *map.get_tile((4, 0)).unwrap().surface_type() == SurfaceType::Mountain);

        assert!(*map.get_tile((5, 0)).unwrap().surface_type() == SurfaceType::Water ||
                *map.get_tile((5, 0)).unwrap().surface_type() == SurfaceType::Swamp);
            
        assert!(*map.get_tile((6, 0)).unwrap().surface_type() == SurfaceType::Water)
    }
}

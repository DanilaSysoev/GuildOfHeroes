mod camera_tests {
    use crate::drawing::{Camera, Direction};

    const BASE_WIDTH: u32 = 64;
    const BASE_HEIGHT: u32 = 36;
    const MIN_WIDTH: u32 = 16;
    const MAX_WIDTH: u32 = 128;
    const ZOOM_STEP: u32 = 2;

    fn get_camera() -> Camera {
        Camera::new(BASE_WIDTH, MIN_WIDTH, MAX_WIDTH, BASE_HEIGHT, ZOOM_STEP)
            .unwrap()
    }

    #[test]
    #[rustfmt::skip]
    pub fn shift__one_left__position_change_correctly() {
        let mut camera = get_camera();
        let (left, top) = (camera.left(), camera.top());

        camera.shift(Direction::Left);

        assert_eq!(camera.left(), left - 1);
        assert_eq!(camera.top(), top);
    }

    #[test]
    #[rustfmt::skip]
    pub fn shift__one_left_then_return__position_change_correctly() {
        let mut camera = get_camera();
        let (left, top) = (camera.left(), camera.top());

        camera.shift(Direction::Left);
        camera.shift(Direction::Right);

        assert_eq!(camera.left(), left);
        assert_eq!(camera.top(), top);
    }

    #[test]
    #[rustfmt::skip]
    pub fn shift__one_right__position_change_correctly() {
        let mut camera = get_camera();
        let (left, top) = (camera.left(), camera.top());

        camera.shift(Direction::Right);

        assert_eq!(camera.left(), left + 1);
        assert_eq!(camera.top(), top);
    }

    #[test]
    #[rustfmt::skip]
    pub fn shift__one_right_then_return__position_change_correctly() {
        let mut camera = get_camera();
        let (left, top) = (camera.left(), camera.top());

        camera.shift(Direction::Right);
        camera.shift(Direction::Left);

        assert_eq!(camera.left(), left);
        assert_eq!(camera.top(), top);
    }

    #[test]
    #[rustfmt::skip]
    pub fn shift__one_up__position_change_correctly() {
        let mut camera = get_camera();
        let (left, top) = (camera.left(), camera.top());

        camera.shift(Direction::Up);

        assert_eq!(camera.left(), left);
        assert_eq!(camera.top(), top - 1);
    }

    #[test]
    #[rustfmt::skip]
    pub fn shift__one_up_then_return__position_change_correctly() {
        let mut camera = get_camera();
        let (left, top) = (camera.left(), camera.top());

        camera.shift(Direction::Up);
        camera.shift(Direction::Down);

        assert_eq!(camera.left(), left);
        assert_eq!(camera.top(), top);
    }

    #[test]
    #[rustfmt::skip]
    pub fn shift__one_down__position_change_correctly() {
        let mut camera = get_camera();
        let (left, top) = (camera.left(), camera.top());

        camera.shift(Direction::Down);

        assert_eq!(camera.left(), left);
        assert_eq!(camera.top(), top + 1);
    }

    #[test]
    #[rustfmt::skip]
    pub fn shift__one_down_then_return__position_change_correctly() {
        let mut camera = get_camera();
        let (left, top) = (camera.left(), camera.top());

        camera.shift(Direction::Down);
        camera.shift(Direction::Up);

        assert_eq!(camera.left(), left);
        assert_eq!(camera.top(), top);
    }

    #[test]
    #[rustfmt::skip]
    pub fn right__some_pos_and_width__right_is_correct() {
        let camera = get_camera().with_left(20);

        assert_eq!(camera.right(), BASE_WIDTH as i32 + 20 - 1);
    }

    #[test]
    #[rustfmt::skip]
    pub fn bottom__some_pos_and_height__bottom_is_correct() {
        let camera = get_camera().with_top(20);

        assert_eq!(camera.bottom(), BASE_HEIGHT as i32 + 20 - 1);
    }

    #[test]
    pub fn to_screen_line__some_pos__result_is_correct() {
        let camera = get_camera().with_top(20);
        assert_eq!(camera.line_to_screen(25), 5)
    }

    #[test]
    pub fn to_screen_column__some_pos__result_is_correct() {
        let camera = get_camera().with_left(20);
        assert_eq!(camera.column_to_screen(25), 5)
    }

    #[test]
    pub fn to_world_line__some_pos__result_is_correct() {
        let camera = get_camera().with_top(20);
        assert_eq!(camera.line_to_world(5), 25)
    }

    #[test]
    pub fn to_world_column__some_pos__result_is_correct() {
        let camera = get_camera().with_left(20);
        assert_eq!(camera.column_to_world(5), 25)
    }

    #[test]
    pub fn zoom_in__increase_width__height_is_correct() {
        let mut camera = get_camera().with_zoom_step(10);
        camera.zoom_in();

        assert_eq!(camera.width(), BASE_WIDTH - 10);
        assert_eq!(camera.height(), 31);
    }

    #[test]
    pub fn zoom_out__decrease_width__height_is_correct() {
        let mut camera = get_camera().with_zoom_step(10);
        camera.zoom_out();

        assert_eq!(camera.width(), BASE_WIDTH + 10);
        assert_eq!(camera.height(), 42);
    }

    #[test]
    pub fn zoom_reset__zoom_out_twice_then_reset__size_is_correct() {
        let mut camera = get_camera().with_zoom_step(10);
        camera.zoom_out();
        camera.zoom_out();
        camera.zoom_reset();

        assert_eq!(camera.width(), BASE_WIDTH);
        assert_eq!(camera.height(), BASE_HEIGHT);
    }

    #[test]
    pub fn zoom_reset__zoom_in_twice_then_reset__size_is_correct() {
        let mut camera = get_camera().with_zoom_step(10);
        camera.zoom_in();
        camera.zoom_in();
        camera.zoom_reset();

        assert_eq!(camera.width(), BASE_WIDTH);
        assert_eq!(camera.height(), BASE_HEIGHT);
    }

    #[test]
    pub fn zoom_reset__zoom_out_then_reset__size_is_correct() {
        let mut camera = get_camera().with_zoom_step(10);
        camera.zoom_out();
        camera.zoom_reset();

        assert_eq!(camera.width(), BASE_WIDTH);
        assert_eq!(camera.height(), BASE_HEIGHT);
    }

    #[test]
    pub fn zoom_reset__zoom_in_then_reset__size_is_correct() {
        let mut camera = get_camera().with_zoom_step(10);
        camera.zoom_in();
        camera.zoom_reset();

        assert_eq!(camera.width(), BASE_WIDTH);
        assert_eq!(camera.height(), BASE_HEIGHT);
    }
}

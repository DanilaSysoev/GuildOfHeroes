mod camera_tests {
    use crate::drawing::{Camera, Direction};

    fn get_camera() -> Camera {
        Camera::new(80, 50)
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

        assert_eq!(camera.right(), 99);
    }

    #[test]
    #[rustfmt::skip]
    pub fn bottom__some_pos_and_height__bottom_is_correct() {
        let camera = get_camera().with_top(20);

        assert_eq!(camera.bottom(), 69);
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
}

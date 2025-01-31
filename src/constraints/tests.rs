use super::*;
use modcholesky::ModCholeskySE99;
use rand;

#[test]
fn t_zero_set() {
    let zero = Zero::new();
    let mut x = [1.0, 2.0, 3.0];
    let x_projection = [0.0; 3];
    zero.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(
        &x_projection,
        &x,
        1e-12,
        1e-12,
        "wrong projection on zero set",
    );
}

#[test]
fn t_hyperplane() {
    let normal_vector = [1.0, 2.0, 3.0];
    let offset = 1.0;
    let hyperplane = Hyperplane::new(&normal_vector, offset);
    let mut x = [-1., 3., 5.];
    let x_proj_expected = [
        -2.357_142_857_142_857,
        0.285_714_285_714_286,
        0.928_571_428_571_429,
    ];
    hyperplane.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(
        &x,
        &x_proj_expected,
        1e-8,
        1e-14,
        "halfspace projection failed",
    );
}

#[test]
fn t_halfspace_project_inside() {
    let normal_vector = [1., 2.];
    let offset = 5.0;
    let halfspace = Halfspace::new(&normal_vector, offset);
    let mut x = [-1., 3.];
    let x_expected = [-1., 3.];
    halfspace.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(
        &x,
        &x_expected,
        1e-10,
        1e-14,
        "halfspace projection failed (inside)",
    );
}

#[test]
fn t_halfspace_project_outside() {
    let normal_vector = [1., 2.];
    let offset = 1.0;
    let halfspace = Halfspace::new(&normal_vector, offset);
    let mut x = [-1., 3.];
    let x_expected = [-1.8, 1.4];
    halfspace.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(
        &x,
        &x_expected,
        1e-8,
        1e-14,
        "halfspace projection failed (outside)",
    );
}

#[test]
#[should_panic]
fn t_finite_set_inconsistent_dimensions() {
    let x1 = vec![1.0; 2];
    let x2 = vec![0.0; 3];
    let data: &[&[f64]] = &[&x1, &x2];
    let _f = FiniteSet::new(data);
}

#[test]
#[should_panic]
fn t_finite_set_empty_data() {
    let mut _data: &[&[f64]] = &[];
    let _f = FiniteSet::new(_data);
}

#[test]
fn t_finite_set() {
    let data: &[&[f64]] = &[&[0.0, 0.0], &[1.0, 1.0], &[0.0, 1.0], &[1.0, 0.0]];
    let finite_set = FiniteSet::new(data);
    let mut x = [0.7, 0.6];
    finite_set.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(
        &[1.0, 1.0],
        &x,
        1e-10,
        1e-10,
        "projection is wrong (should be [1,1])",
    );
    x = [-0.1, 0.2];
    finite_set.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(
        &[0.0, 0.0],
        &x,
        1e-10,
        1e-10,
        "projection is wrong (should be [0,0])",
    );
    x = [0.48, 0.501];
    finite_set.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(
        &[0.0, 1.0],
        &x,
        1e-10,
        1e-10,
        "projection is wrong (should be [0,1])",
    );
    x = [0.7, 0.2];
    finite_set.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(
        &[1.0, 0.0],
        &x,
        1e-10,
        1e-10,
        "projection is wrong (should be [1,0])",
    );
}

#[test]
fn t_rectangle_bounded() {
    let xmin = vec![2.0; 5];
    let xmax = vec![4.5; 5];
    let rectangle = Rectangle::new(Some(&xmin[..]), Some(&xmax[..]));
    let mut x = [1.0, 2.0, 3.0, 4.0, 5.0];

    rectangle.project(&mut x);

    unit_test_utils::assert_nearly_equal_array(
        &[2.0, 2.0, 3.0, 4.0, 4.5],
        &x,
        1e-8,
        1e-8,
        "projection on bounded rectangle",
    );
}

#[test]
fn t_rectangle_infinite_bounds() {
    let xmin = [-1.0, 2.0, std::f64::NEG_INFINITY];
    let xmax = [1.0, std::f64::INFINITY, 5.0];
    let rectangle = Rectangle::new(Some(&xmin[..]), Some(&xmax[..]));
    let mut x = [-2.0, 3.0, 1.0];

    rectangle.project(&mut x);

    unit_test_utils::assert_nearly_equal_array(
        &[-1.0, 3.0, 1.0],
        &x,
        1e-8,
        1e-8,
        "projection on unbounded rectangle",
    );
}

#[test]
#[should_panic]
fn t_rectangle_incompatible_dims() {
    let xmin = vec![1.0; 5];
    let xmax = vec![2.0; 4];
    let _rectangle = Rectangle::new(Some(&xmin[..]), Some(&xmax[..]));
}

#[test]
fn t_rectangle_bounded_negative_entries() {
    let xmin = [-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let xmax = [-1.0, -2.0, -1.0, 2.0, 1.0, 0.0, 4.0, 6.0, 9.0, 100.0, 500.0];
    let rectangle = Rectangle::new(Some(&xmin[..]), Some(&xmax[..]));
    let mut x = [-6.0, -3.0, 0.0, 3.0, -5.0, 1.0, 2.0, 3.0, -1.0, 0.0, 0.0];

    rectangle.project(&mut x);

    unit_test_utils::assert_nearly_equal_array(
        &[-5.0, -3.0, -1.0, 2.0, -1.0, 0.0, 2.0, 3.0, 3.0, 4.0, 5.0],
        &x,
        1e-8,
        1e-8,
        "projection on bounded rectangle v2",
    );
}

#[test]
fn t_rectangle_only_xmin() {
    let xmin = vec![2.0; 5];
    let rectangle = Rectangle::new(Some(&xmin[..]), None);
    let mut x = [1.0, 2.0, 3.0, 4.0, 5.0];

    rectangle.project(&mut x);

    unit_test_utils::assert_nearly_equal_array(
        &[2.0, 2.0, 3.0, 4.0, 5.0],
        &x,
        1e-8,
        1e-8,
        "projection on halfspace (xmin)",
    );
}

#[test]
fn t_rectangle_only_xmax() {
    let xmax = vec![-3.0; 5];
    let rectangle = Rectangle::new(None, Some(&xmax[..]));
    let mut x = [-10.0, -20.0, 0.0, 5.0, 3.0];

    rectangle.project(&mut x);

    unit_test_utils::assert_nearly_equal_array(
        &[-10.0, -20.0, -3.0, -3.0, -3.0],
        &x,
        1e-8,
        1e-8,
        "projection",
    );
}

#[test]
fn t_ball2_at_origin() {
    let radius = 1.0;
    let mut x = [1.0, 1.0];
    let ball = Ball2::new(None, radius);

    ball.project(&mut x);

    unit_test_utils::assert_nearly_equal_array(
        &[
            std::f64::consts::FRAC_1_SQRT_2,
            std::f64::consts::FRAC_1_SQRT_2,
        ],
        &x,
        1e-8,
        1e-8,
        "projection on ball centered at origin",
    );
}

#[test]
fn t_ball2_at_origin_different_radius_outside() {
    let radius = 0.8;
    let mut x = [1.0, 1.0];
    let ball = Ball2::new(None, radius);
    ball.project(&mut x);
    let norm_proj_x = crate::matrix_operations::norm2(&x);
    unit_test_utils::assert_nearly_equal(radius, norm_proj_x, 1e-10, 1e-12, "wrong norm");
}

#[test]
fn t_ball2_at_origin_different_radius_inside() {
    let radius = 0.8;
    let mut x = [-0.2, 0.15];
    let ball = Ball2::new(None, radius);
    ball.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&x, &[-0.2, 0.15], 1e-10, 1e-12, "wrong");
}

#[test]
fn t_ball2_at_center_different_radius_outside() {
    let radius = 1.2;
    let mut x = [1.0, 1.0];
    let center = [-0.8, -1.1];
    let ball = Ball2::new(Some(&center), radius);
    ball.project(&mut x);
    let norm_x_minus_c = crate::matrix_operations::norm2_squared_diff(&x, &center).sqrt();
    unit_test_utils::assert_nearly_equal(radius, norm_x_minus_c, 1e-10, 1e-12, "wrong norm");
}

#[test]
fn t_ball2_at_center_different_radius_inside() {
    let radius = 1.2;
    let mut x = [-0.9, -0.85];
    let center = [-0.8, -1.1];
    let ball = Ball2::new(Some(&center), radius);
    ball.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&[-0.9, -0.85], &x, 1e-10, 1e-12, "wrong result");
}

#[test]
fn t_ball2_elsewhere() {
    let radius = 1.0;
    let center = [1.0, 1.0];
    let mut x = [2.0, 2.0];
    let ball = Ball2::new(Some(&center[..]), radius);

    ball.project(&mut x);

    let expected_proj_element = std::f64::consts::FRAC_1_SQRT_2 + 1.;
    unit_test_utils::assert_nearly_equal_array(
        &[expected_proj_element, expected_proj_element],
        &x,
        1e-8,
        1e-8,
        "projection on ball centered at [1, 1]",
    );
}

#[test]
fn t_no_constraints() {
    let mut x = [1.0, 2.0, 3.0];
    let whole_space = NoConstraints::new();

    whole_space.project(&mut x);

    unit_test_utils::assert_nearly_equal_array(&[1., 2., 3.], &x, 1e-10, 1e-15, "x is wrong");
}

#[test]
#[should_panic]
fn t_cartesian_product_constraints_incoherent_indices() {
    let ball1 = Ball2::new(None, 1.0);
    let ball2 = Ball2::new(None, 0.5);
    let _cart_prod = CartesianProduct::new()
        .add_constraint(3, ball1)
        .add_constraint(2, ball2);
}

#[test]
#[should_panic]
fn t_cartesian_product_constraints_wrong_vector_dim() {
    let ball1 = Ball2::new(None, 1.0);
    let ball2 = Ball2::new(None, 0.5);
    let cart_prod = CartesianProduct::new()
        .add_constraint(3, ball1)
        .add_constraint(10, ball2);
    let mut x = [0.0; 30];
    cart_prod.project(&mut x);
}

#[test]
fn t_cartesian_product_constraints() {
    let radius1 = 1.0;
    let radius2 = 0.5;
    let idx1 = 3;
    let idx2 = 5;
    let ball1 = Ball2::new(None, radius1);
    let ball2 = Ball2::new(None, radius2);
    let cart_prod = CartesianProduct::new()
        .add_constraint(idx1, ball1)
        .add_constraint(idx2, ball2);
    let mut x = [3.0, 4.0, 5.0, 2.0, 1.0];
    cart_prod.project(&mut x);
    let r1 = crate::matrix_operations::norm2(&x[0..idx1]);
    let r2 = crate::matrix_operations::norm2(&x[idx1..idx2]);
    unit_test_utils::assert_nearly_equal(r1, radius1, 1e-8, 1e-12, "r1 is wrong");
    unit_test_utils::assert_nearly_equal(r2, radius2, 1e-8, 1e-12, "r2 is wrong");
}

#[test]
fn t_cartesian_product_ball_and_rectangle() {
    /* Rectangle 1 */
    let xmin1 = vec![-1.0; 3];
    let xmax1 = vec![1.0; 3];
    let rectangle1 = Rectangle::new(Some(&xmin1), Some(&xmax1));

    /* Ball */
    let radius = 1.0;
    let ball = Ball2::new(None, radius);

    /* Rectangle 2 */
    let xmin2 = vec![-0.5; 2];
    let xmax2 = vec![0.5; 2];
    let rectangle2 = Rectangle::new(Some(&xmin2), Some(&xmax2));

    /* Cartesian product */
    let cart_prod = CartesianProduct::new()
        .add_constraint(3, rectangle1)
        .add_constraint(7, ball)
        .add_constraint(9, rectangle2);

    /* Projection */
    let mut x = [-10.0, 0.5, 10.0, 0.01, -0.01, 0.1, 10.0, -1.0, 1.0];
    cart_prod.project(&mut x);

    unit_test_utils::assert_nearly_equal_array(
        &x[0..3],
        &[-1.0, 0.5, 1.0],
        1e-8,
        1e-10,
        "wrong projection on rectangle 1",
    );

    let r = crate::matrix_operations::norm2(&x[3..7]);
    unit_test_utils::assert_nearly_equal(r, radius, 1e-8, 1e-12, "r is wrong");

    unit_test_utils::assert_nearly_equal_array(
        &x[7..9],
        &[-0.5, 0.5],
        1e-8,
        1e-10,
        "wrong projection on rectangle 2",
    );
}

#[test]
fn t_second_order_cone_case_i() {
    let soc = SecondOrderCone::new(1.0);
    let mut x = vec![1.0, 1.0, 1.42];
    let x_copy = x.clone();
    soc.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&x, &x_copy, 1e-10, 1e-12, "x has been modified");
}

#[test]
fn t_second_order_cone_case_ii() {
    let alpha = 0.5;
    let soc = SecondOrderCone::new(alpha);
    let mut x = vec![1.0, 1.0, -0.71];
    soc.project(&mut x);
    let expected = vec![0.0; 3];
    unit_test_utils::assert_nearly_equal_array(
        &x,
        &expected,
        1e-10,
        1e-12,
        "wrong result (should be zero)",
    );
}

#[test]
fn t_second_order_cone_case_iii() {
    let alpha = 1.5;
    let soc = SecondOrderCone::new(alpha);
    let mut x = vec![1.0, 1.0, 0.1];
    soc.project(&mut x);
    // make sure the new `x` is in the cone
    let norm_z = crate::matrix_operations::norm2(&x[..=1]);
    assert!(norm_z <= alpha * x[2]);
    // in fact the projection should be on the boundary of the cone
    assert!((norm_z - alpha * x[2]).abs() <= 1e-7);
}

#[test]
#[should_panic]
fn t_second_order_cone_illegal_alpha_i() {
    let alpha = 0.0;
    let _soc = SecondOrderCone::new(alpha);
}

#[test]
#[should_panic]
fn t_second_order_cone_illegal_alpha_ii() {
    let alpha = -1.0;
    let _soc = SecondOrderCone::new(alpha);
}

#[test]
#[should_panic]
fn t_second_order_cone_short_vector() {
    let alpha = 1.0;
    let soc = SecondOrderCone::new(alpha);
    let mut _x = vec![1.0];
    soc.project(&mut _x);
}

#[test]
fn t_cartesian_product_dimension() {
    let data: &[&[f64]] = &[&[0.0, 0.0], &[1.0, 1.0]];
    let finite_set = FiniteSet::new(data);
    let finite_set_2 = finite_set;
    let ball = Ball2::new(None, 1.0);
    let no_constraints = NoConstraints::new();
    let cartesian = CartesianProduct::new_with_capacity(4)
        .add_constraint(2, finite_set)
        .add_constraint(4, finite_set_2)
        .add_constraint(7, no_constraints)
        .add_constraint(10, ball);
    assert!(10 == cartesian.dimension());

    // let's do a projection to make sure this works
    // Note: we've used the same set (finite_set), twice
    let mut x = [-0.5, 1.1, 0.45, 0.55, 10.0, 10.0, -500.0, 1.0, 1.0, 1.0];
    cartesian.project(&mut x);
    println!("X = {:#?}", x);
    let sqrt_3_over_3 = 3.0_f64.sqrt() / 3.;
    unit_test_utils::assert_nearly_equal_array(
        &x,
        &[
            0.0,
            0.0,
            1.0,
            1.0,
            10.0,
            10.0,
            -500.0,
            sqrt_3_over_3,
            sqrt_3_over_3,
            sqrt_3_over_3,
        ],
        1e-10,
        1e-12,
        "wrong projection on cartesian product",
    );
}

#[test]
fn t_cartesian_ball_no_constraint() {
    let xc = [1., 0., 0.];
    let radius = 1.0;
    let ball2 = Ball2::new(Some(&xc), radius);
    let no_constraints = NoConstraints::new();
    let cartesian = CartesianProduct::new_with_capacity(4)
        .add_constraint(2, no_constraints)
        .add_constraint(5, ball2)
        .add_constraint(8, no_constraints)
        .add_constraint(9, no_constraints);
    assert_eq!(9, cartesian.dimension());
    let mut x = [100., -200., 0.5, 1.5, 3.5, 1000., 5., -500., 2_000_000.];
    cartesian.project(&mut x);
    let x_proj_ball = [0.869811089019176, 0.390566732942472, 0.911322376865767];
    unit_test_utils::assert_nearly_equal_array(
        &x[0..=1],
        &[100., -200.],
        1e-10,
        1e-15,
        "projection on no constraints is wrong",
    );
    unit_test_utils::assert_nearly_equal_array(&x[2..=4], &x_proj_ball, 1e-8, 1e-15, "haha");
    unit_test_utils::assert_nearly_equal_array(
        &x[5..=8],
        &[1000., 5., -500., 2_000_000.],
        1e-10,
        1e-5,
        "projection on no constraints is wrong",
    );
}

#[test]
fn t_ball_inf_origin() {
    let ball_inf = BallInf::new(None, 1.0);
    let mut x = [0.0, -0.5, 0.5, 1.5, 3.5, 0.8, 1.1, -5.0, -10.0];
    let x_correct = [0.0, -0.5, 0.5, 1.0, 1.0, 0.8, 1.0, -1.0, -1.0];
    ball_inf.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(
        &x_correct,
        &x,
        1e-10,
        1e-12,
        "projection on ball inf",
    );
    println!("x = {:#?}", x);
}

#[test]
fn t_ball_inf_center() {
    let xc = [5.0, -6.0];
    let ball_inf = BallInf::new(Some(&xc), 1.5);
    let mut x = [11.0, -0.5];
    ball_inf.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&[6.5, -4.5], &x, 1e-10, 1e-12, "upper right");

    let mut x = [3.0, -7.0];
    ball_inf.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&[3.5, -7.0], &x, 1e-10, 1e-12, "left");

    let mut x = [800.0, -5.0];
    ball_inf.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&[6.5, -5.0], &x, 1e-10, 1e-12, "right");

    let mut x = [9.0, -10.0];
    ball_inf.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&[6.5, -7.5], &x, 1e-10, 1e-12, "down right");

    let mut x = [3.0, 0.0];
    ball_inf.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&[3.5, -4.5], &x, 1e-10, 1e-12, "top left");

    let mut x = [6.0, -5.0];
    ball_inf.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&[6.0, -5.0], &x, 1e-10, 1e-12, "inside");

    let mut x = [5.0, -6.0];
    ball_inf.project(&mut x);
    unit_test_utils::assert_nearly_equal_array(&[5.0, -6.0], &x, 1e-10, 1e-12, "centre");
}

#[test]
fn t_is_convex_ball_inf() {
    let ball_inf = BallInf::new(None, 1.5);
    assert!(ball_inf.is_convex());
}

#[test]
fn t_is_convex_ball2() {
    let ball_2 = Ball2::new(None, 1.0);
    assert!(ball_2.is_convex());
}

#[test]
fn t_is_convex_finite_set() {
    let finite = FiniteSet::new(&[&[1.0, 2.0, 3.0]]);
    assert!(finite.is_convex());

    let finite_noncvx = FiniteSet::new(&[&[1.0, 2.0], &[3.0, 4.0]]);
    assert!(!finite_noncvx.is_convex());
}

#[test]
fn t_is_convex_soc() {
    let soc = SecondOrderCone::new(2.0);
    assert!(soc.is_convex());
}

#[test]
fn t_is_convex_zero() {
    let zero = Zero::new();
    assert!(zero.is_convex());
}

#[test]
fn t_is_convex_halfspace() {
    let normal_vector = vec![1.0, 2.0, 4.0];
    let offset = 1.0;
    let halfspace = Halfspace::new(&normal_vector, offset);
    assert!(halfspace.is_convex());
}

#[test]
fn t_is_convex_cartesian_product() {
    let ball_2 = Ball2::new(None, 1.0);
    let ball_inf = BallInf::new(None, 1.5);
    let finite = FiniteSet::new(&[&[1.0, 2.0, 3.0]]);
    let cartesian_product = CartesianProduct::new()
        .add_constraint(4, ball_2)
        .add_constraint(6, ball_inf)
        .add_constraint(9, finite);
    assert!(cartesian_product.is_convex());

    let finite_noncvx = FiniteSet::new(&[&[1.0, 2.0], &[3.0, 4.0]]);
    let cartesian_product = cartesian_product.add_constraint(10, finite_noncvx);
    assert!(!cartesian_product.is_convex());
}

#[test]
fn t_hyperplane_is_convex() {
    let normal_vector = [1.0, 2.0, 3.0];
    let offset = 1.0;
    let hyperplane = Hyperplane::new(&normal_vector, offset);
    assert!(hyperplane.is_convex());
}

#[test]
fn t_simplex_projection() {
    let mut x = [1.0, 2.0, 3.0];
    let alpha = 3.0;
    let my_simplex = Simplex::new(alpha);
    my_simplex.project(&mut x);
    unit_test_utils::assert_nearly_equal(
        crate::matrix_operations::sum(&x),
        alpha,
        1e-8,
        1e-10,
        "sum of projected vector not equal to alpha",
    );
}

#[test]
fn t_simplex_projection_random_spam() {
    let n = 10;
    let n_trials = 1000;
    for _ in 0..n_trials {
        let mut x = vec![0.0; n];
        let scale = 10.;
        x.iter_mut()
            .for_each(|xi| *xi = scale * (2. * rand::random::<f64>() - 1.));
        let alpha_scale = 20.;
        let alpha = alpha_scale * rand::random::<f64>();
        let simplex = Simplex::new(alpha);
        simplex.project(&mut x);
        println!("x = {:?}", x);
        assert!(x.iter().all(|&xi| xi >= -1e-12));
        unit_test_utils::assert_nearly_equal(
            crate::matrix_operations::sum(&x),
            alpha,
            1e-8,
            1e-10,
            "sum of projected vector not equal to alpha",
        );
    }
}

#[test]
fn t_simplex_projection_random_optimality() {
    for n in (10..=60).step_by(10) {
        for _ in 0..10 * n {
            let mut z = vec![0.0; n];
            let scale = 1000.;
            z.iter_mut()
                .for_each(|xi| *xi = scale * (2. * rand::random::<f64>() - 1.));
            let alpha_scale = 100.;
            let alpha = alpha_scale * rand::random::<f64>();
            let simplex = Simplex::new(alpha);
            let y = z.clone();
            simplex.project(&mut z);
            for j in 0..n {
                let w = alpha * (y[j] - z[j]) - crate::matrix_operations::inner_product(&z, &y)
                    + crate::matrix_operations::norm2_squared(&z);
                let norm_z = crate::matrix_operations::norm_inf(&z);
                let norm_diff_y_z = crate::matrix_operations::norm_inf_diff(&y, &z);
                assert!(
                    w <= 1e-9 * (1. + f64::max(norm_z, norm_diff_y_z)),
                    "optimality conditions failed for simplex"
                );
            }
        }
    }
}

#[test]
#[should_panic]
fn t_simplex_alpha_zero() {
    let _ = Simplex::new(0.);
}

#[test]
#[should_panic]
fn t_simplex_alpha_negative() {
    let _ = Simplex::new(-1.);
}

#[test]
fn t_ball1_random_optimality_conditions() {
    for n in (10..=60).step_by(10) {
        let n_trials = 1000;
        for _ in 0..n_trials {
            let mut x = vec![0.0; n];
            let mut x_star = vec![0.0; n];
            let scale = 20.;
            x_star
                .iter_mut()
                .for_each(|xi| *xi = scale * (2. * rand::random::<f64>() - 1.));
            x.copy_from_slice(&x_star);
            let radius = 5. * rand::random::<f64>();
            let ball1 = Ball1::new(None, radius);
            ball1.project(&mut x_star);
            // make sure |x|_1 <= radius
            assert!(
                crate::matrix_operations::norm1(&x_star) <= radius * (1. + 1e-9),
                "norm(x, 1) > radius"
            );
            // check the optimality conditions
            for j in 0..n {
                let w = radius * (x[j] - x_star[j])
                    - crate::matrix_operations::inner_product(&x, &x_star)
                    + crate::matrix_operations::norm2_squared(&x_star);
                let norm_x_star = crate::matrix_operations::norm_inf(&x_star);
                let norm_diff_x_x_star = crate::matrix_operations::norm_inf_diff(&x, &x_star);
                assert!(
                    w <= 1e-10 * (1. + f64::max(norm_x_star, norm_diff_x_x_star)),
                    "optimality conditions failed for ball1"
                );
            }
            // and of course...
            for j in 0..n {
                let w = -radius * (x[j] - x_star[j])
                    - crate::matrix_operations::inner_product(&x, &x_star)
                    + crate::matrix_operations::norm2_squared(&x_star);
                let norm_x_star = crate::matrix_operations::norm_inf(&x_star);
                let norm_diff_x_x_star = crate::matrix_operations::norm_inf_diff(&x, &x_star);
                assert!(
                    w <= 1e-10 * (1. + f64::max(norm_x_star, norm_diff_x_x_star)),
                    "optimality conditions failed for ball1"
                );
            }
        }
    }
}

#[test]
fn t_ball1_random_optimality_conditions_centered() {
    for n in (10..=60).step_by(10) {
        let n_trials = 1000;
        for _ in 0..n_trials {
            let mut x = vec![0.0; n];
            let mut xc = vec![0.0; n];
            let scale = 50.;
            let scale_xc = 10.;
            x.iter_mut()
                .for_each(|xi| *xi = scale * (2. * rand::random::<f64>() - 1.));
            xc.iter_mut()
                .for_each(|xi| *xi = scale_xc * (2. * rand::random::<f64>() - 1.));
            let radius = 5. * rand::random::<f64>();
            let ball1 = Ball1::new(Some(&xc), radius);
            ball1.project(&mut x);
            // x = x - xc
            x.iter_mut()
                .zip(xc.iter())
                .for_each(|(xi, &xci)| *xi -= xci);
            assert!(
                crate::matrix_operations::norm1(&x) <= radius * (1. + 1e-9),
                "norm(x - xc, 1) > radius"
            );
        }
    }
}

#[test]
fn t_sphere2_no_center() {
    let radius = 0.9;
    let mut x_out = [1.0, 1.0];
    let mut x_in = [-0.3, -0.2];
    let unit_sphere = Sphere2::new(None, radius);
    unit_sphere.project(&mut x_out);
    unit_sphere.project(&mut x_in);
    let norm_out = crate::matrix_operations::norm2(&x_out);
    let norm_in = crate::matrix_operations::norm2(&x_in);
    unit_test_utils::assert_nearly_equal(radius, norm_out, 1e-10, 1e-12, "norm_out is not 1.0");
    unit_test_utils::assert_nearly_equal(radius, norm_in, 1e-10, 1e-12, "norm_in is not 1.0");
}

#[test]
fn t_sphere2_no_center_projection_of_zero() {
    let radius = 0.9;
    let mut x = [0.0, 0.0];
    let unit_sphere = Sphere2::new(None, radius);
    unit_sphere.project(&mut x);
    let norm_result = crate::matrix_operations::norm2(&x);
    unit_test_utils::assert_nearly_equal(radius, norm_result, 1e-10, 1e-12, "norm_out is not 1.0");
}

#[test]
fn t_sphere2_center() {
    let radius = 1.3;
    let center = [-3.0, 5.0];
    let mut x = [1.0, 1.0];
    let unit_sphere = Sphere2::new(Some(&center), radius);

    unit_sphere.project(&mut x);
    let mut x_minus_c = [0.0; 2];
    x.iter()
        .zip(center.iter())
        .zip(x_minus_c.iter_mut())
        .for_each(|((a, b), c)| {
            *c = a - b;
        });

    let norm_out = crate::matrix_operations::norm2(&x_minus_c);
    unit_test_utils::assert_nearly_equal(radius, norm_out, 1e-10, 1e-12, "norm_out is not 1.0");
}

#[test]
fn t_sphere2_center_projection_of_center() {
    let radius = 1.3;
    let center = [-3.0, 5.0];
    let mut x = [-3.0, 5.0];
    let unit_sphere = Sphere2::new(Some(&center), radius);

    unit_sphere.project(&mut x);
    let mut x_minus_c = [0.0; 2];
    x.iter()
        .zip(center.iter())
        .zip(x_minus_c.iter_mut())
        .for_each(|((a, b), c)| {
            *c = a - b;
        });

    let norm_out = crate::matrix_operations::norm2(&x_minus_c);
    unit_test_utils::assert_nearly_equal(radius, norm_out, 1e-10, 1e-12, "norm_out is not 1.0");
}

#[test]
#[should_panic]
fn t_ball1_alpha_negative() {
    let _ = Ball1::new(None, -1.);
}

#[test]
fn t_affine_space() {
    let a = vec![
        0.5, 0.1, 0.2, -0.3, -0.6, 0.3, 0., 0.5, 1.0, 0.1, -1.0, -0.4,
    ];
    let b = vec![1., 2., -0.5];
    let affine_set = AffineSpace::new(a, b);
    let mut x = [1., -2., -0.3, 0.5];
    affine_set.project(&mut x);
    let x_correct = [
        1.888564346697095,
        5.629857182200888,
        1.796204902230790,
        2.888362906715977,
    ];
    unit_test_utils::assert_nearly_equal_array(
        &x_correct,
        &x,
        1e-10,
        1e-12,
        "projection on affine set is wrong",
    );
}

#[test]
fn t_affine_space_larger() {
    let a = vec![
        1.0f64, 1., 1., 0., 0., 0., 1., 1., 1., 0., 0., 0., 1., 1., 1., -1., 4., -1., 0., 2.,
    ];
    let b = vec![1., -2., 3., 4.];
    let affine_set = AffineSpace::new(a, b);
    let mut x = [10., 11., -9., 4., 5.];
    affine_set.project(&mut x);
    let x_correct = [
        9.238095238095237,
        -0.714285714285714,
        -7.523809523809524,
        6.238095238095238,
        4.285714285714288,
    ];
    unit_test_utils::assert_nearly_equal_array(
        &x_correct,
        &x,
        1e-10,
        1e-12,
        "projection on affine set is wrong",
    );
}

#[test]
fn t_affine_space_single_row() {
    let a = vec![1., 1., 1., 1.];
    let b = vec![1.];
    let affine_set = AffineSpace::new(a, b);
    let mut x = [5., 6., 10., 25.];
    affine_set.project(&mut x);
    let s = x.iter().sum();
    unit_test_utils::assert_nearly_equal(1., s, 1e-12, 1e-14, "wrong sum");
}

#[test]
#[should_panic]
fn t_affine_space_wrong_dimensions() {
    let a = vec![0.5, 0.1, 0.2, -0.3, -0.6, 0.3, 0., 0.5, 1.0, 0.1, -1.0];
    let b = vec![1., 2., -0.5];
    let _ = AffineSpace::new(a, b);
}

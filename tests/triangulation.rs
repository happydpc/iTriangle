pub mod triangulation {
    use i_float::fix_vec::FixVec;
    use i_shape::fix_path::FixPath;
    use i_shape::fix_shape::FixShape;
    use i_triangle::delaunay::convex::{ConvexPath, ConvexSide};

    pub struct Test {
        pub shape: FixShape,
        pub points: FixPath,
        pub indices: Vec<usize>,
        pub polygons: Vec<ConvexPath>,
    }

    pub struct Tests;

    impl Tests {
        pub fn test_at_index(index: usize) -> Test {
            match index {
                0 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-20, -20),
                            FixVec::new_i64(-20, 20),
                            FixVec::new_i64(20, 20),
                            FixVec::new_i64(20, -20)]),
                        points: vec![
                            FixVec::new_i64(-20, -20),
                            FixVec::new_i64(-20, 20),
                            FixVec::new_i64(20, 20),
                            FixVec::new_i64(20, -20)],
                        indices: vec![0, 1, 3, 1, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, 20),
                                    FixVec::new_i64(20, -20),
                                    FixVec::new_i64(-20, -20),
                                    FixVec::new_i64(-20, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                1 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(0, -15)]),
                        points: vec![
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(0, -15),
                            FixVec::new_i64(-15, 0)],
                        indices: vec![3, 0, 2, 2, 0, 1],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(0, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                2 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-25, 0),
                            FixVec::new_i64(-15, 15),
                            FixVec::new_i64(15, 15),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(15, -15)]),
                        points: vec![
                            FixVec::new_i64(-15, 15),
                            FixVec::new_i64(15, 15),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(15, -15),
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-25, 0)],
                        indices: vec![5, 0, 4, 4, 0, 3, 0, 1, 3, 3, 1, 2],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(15, -15),
                                    FixVec::new_i64(-15, -15),
                                    FixVec::new_i64(-25, 0),
                                    FixVec::new_i64(-15, 15),
                                    FixVec::new_i64(15, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                3 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-5, -15),
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(10, 5),
                            FixVec::new_i64(5, -10)]),
                        points: vec![
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(10, 5),
                            FixVec::new_i64(5, -10),
                            FixVec::new_i64(-5, -15),
                            FixVec::new_i64(-10, 0)],
                        indices: vec![1, 4, 0, 4, 2, 3, 1, 2, 4],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, -15),
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(0, 15),
                                    FixVec::new_i64(10, 5),
                                    FixVec::new_i64(5, -10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                4 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-20, -20),
                                FixVec::new_i64(-20, 20),
                                FixVec::new_i64(20, 20),
                                FixVec::new_i64(20, -20)],
                            vec![
                                FixVec::new_i64(-10, -10),
                                FixVec::new_i64(10, -10),
                                FixVec::new_i64(10, 10),
                                FixVec::new_i64(-10, 10)]]),
                        points: vec![
                            FixVec::new_i64(-20, -20),
                            FixVec::new_i64(-20, 20),
                            FixVec::new_i64(20, 20),
                            FixVec::new_i64(20, -20),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-10, -10)],
                        indices: vec![0, 1, 7, 7, 1, 6, 6, 1, 5, 1, 2, 5, 7, 4, 0, 0, 4, 3, 4, 5, 3, 5, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-20, -20),
                                    FixVec::new_i64(-20, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, 20),
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-20, 20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, -20),
                                    FixVec::new_i64(-20, -20),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(10, -10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, 20),
                                    FixVec::new_i64(20, -20),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                5 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-15, 0),
                                FixVec::new_i64(0, 15),
                                FixVec::new_i64(15, 0),
                                FixVec::new_i64(0, -15)],
                            vec![
                                FixVec::new_i64(-5, 0),
                                FixVec::new_i64(0, -5),
                                FixVec::new_i64(5, 0),
                                FixVec::new_i64(0, 5)]]),
                        points: vec![
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(0, -15),
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-5, 0),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(0, 5)],
                        indices: vec![4, 3, 7, 3, 0, 7, 7, 0, 6, 0, 1, 6, 3, 4, 2, 4, 5, 2, 5, 6, 2, 2, 6, 1],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 15),
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-15, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(0, 15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-5, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                6 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 0)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 0)],
                        indices: vec![0, 1, 3, 2, 3, 1],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(10, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                7 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(10, -10)]),
                        points: vec![
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, 0)],
                        indices: vec![3, 0, 1, 3, 1, 2],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                8 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(-20, 20),
                            FixVec::new_i64(-5, 20),
                            FixVec::new_i64(0, 10),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(20, 20),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(20, -20),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(0, -10),
                            FixVec::new_i64(-5, -20),
                            FixVec::new_i64(-20, -20)]),
                        points: vec![
                            FixVec::new_i64(-20, -20),
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(-20, 20),
                            FixVec::new_i64(-5, 20),
                            FixVec::new_i64(0, 10),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(20, 20),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(20, -20),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(0, -10),
                            FixVec::new_i64(-5, -20)],
                        indices: vec![0, 1, 11, 2, 3, 1, 4, 1, 3, 1, 10, 11, 4, 10, 1, 10, 4, 7, 4, 5, 7, 10, 7, 9, 5, 6, 7, 9, 7, 8],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(0, -10),
                                    FixVec::new_i64(-5, -20),
                                    FixVec::new_i64(-20, -20),
                                    FixVec::new_i64(-10, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(-20, 20),
                                    FixVec::new_i64(-5, 20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, -20),
                                    FixVec::new_i64(5, -20),
                                    FixVec::new_i64(0, -10),
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(10, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, 20),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(5, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                9 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(10, -10)],
                        indices: vec![0, 1, 3, 1, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(0, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                10 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(15, -15)]),
                        points: vec![
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(15, -15)],
                        indices: vec![1, 2, 0, 0, 2, 4, 2, 3, 4],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, -15),
                                    FixVec::new_i64(-15, -15),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, -15),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(0, 15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                11 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-1, 20),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(15, -15)]),
                        points: vec![
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-1, 20),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(15, -15)],
                        indices: vec![1, 2, 3, 1, 3, 0, 0, 3, 4],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-15, -15),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-1, 20),
                                    FixVec::new_i64(0, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, -15),
                                    FixVec::new_i64(-15, -15),
                                    FixVec::new_i64(0, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                12 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(10, -20),
                            FixVec::new_i64(-10, -20)]),
                        points: vec![
                            FixVec::new_i64(-10, -20),
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(10, -20)],
                        indices: vec![0, 1, 3, 1, 2, 3, 0, 3, 4],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -20),
                                    FixVec::new_i64(-10, -20),
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                13 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(-15, 15),
                            FixVec::new_i64(15, 15),
                            FixVec::new_i64(15, -15)]),
                        points: vec![
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(-15, 15),
                            FixVec::new_i64(15, 15),
                            FixVec::new_i64(15, -15)],
                        indices: vec![0, 1, 4, 1, 3, 4, 2, 3, 1],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 15),
                                    FixVec::new_i64(15, -15),
                                    FixVec::new_i64(-15, -15),
                                    FixVec::new_i64(-10, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(-15, 15),
                                    FixVec::new_i64(15, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                14 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-15, -5),
                            FixVec::new_i64(-20, 15),
                            FixVec::new_i64(15, 15),
                            FixVec::new_i64(15, -15)]),
                        points: vec![
                            FixVec::new_i64(15, 15),
                            FixVec::new_i64(15, -15),
                            FixVec::new_i64(-15, -15),
                            FixVec::new_i64(-15, -5),
                            FixVec::new_i64(-20, 15)],
                        indices: vec![4, 0, 3, 2, 3, 1, 3, 0, 1],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, -15),
                                    FixVec::new_i64(-15, -5),
                                    FixVec::new_i64(-20, 15),
                                    FixVec::new_i64(15, 15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, -15),
                                    FixVec::new_i64(-15, -15),
                                    FixVec::new_i64(-15, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                15 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-5, 10),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-5, -10)]),
                        points: vec![
                            FixVec::new_i64(-5, 10),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-5, -10),
                            FixVec::new_i64(-15, 0)],
                        indices: vec![7, 0, 6, 5, 6, 4, 6, 0, 3, 1, 2, 0, 0, 2, 3, 6, 3, 4],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 20),
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(5, -20),
                                    FixVec::new_i64(-5, -10),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-5, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, -20),
                                    FixVec::new_i64(-10, -15),
                                    FixVec::new_i64(-5, -10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, 10),
                                    FixVec::new_i64(-10, 15),
                                    FixVec::new_i64(5, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                16 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-15, 10),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-5, -10)]),
                        points: vec![
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-15, 10),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-5, -10)],
                        indices: vec![1, 2, 0, 0, 2, 7, 2, 3, 4, 2, 4, 7, 4, 5, 7, 6, 7, 5],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 20),
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(5, -20),
                                    FixVec::new_i64(-5, -10),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-15, 10),
                                    FixVec::new_i64(-10, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, -20),
                                    FixVec::new_i64(-10, -15),
                                    FixVec::new_i64(-5, -10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                17 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(-10, 5),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-10, -5)]),
                        points: vec![
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-10, -5),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(-10, 5),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(5, -20)],
                        indices: vec![0, 1, 7, 1, 2, 7, 2, 5, 6, 2, 6, 7, 3, 4, 2, 4, 5, 2],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(5, -20),
                                    FixVec::new_i64(-10, -15),
                                    FixVec::new_i64(-10, -5),
                                    FixVec::new_i64(5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, 5),
                                    FixVec::new_i64(-10, 15),
                                    FixVec::new_i64(5, 20),
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                18 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-10, 5),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(10, 15),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(10, -15),
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-10, -5)]),
                        points: vec![
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-10, -5),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-10, 5),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(10, 15),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(10, -15)],
                        indices: vec![1, 2, 0, 0, 2, 7, 2, 6, 7, 3, 4, 6, 3, 6, 2, 4, 5, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -15),
                                    FixVec::new_i64(-10, -15),
                                    FixVec::new_i64(-10, -5),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -15),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(5, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-10, 5),
                                    FixVec::new_i64(-10, 15),
                                    FixVec::new_i64(5, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 10),
                                    FixVec::new_i64(-10, 15),
                                    FixVec::new_i64(10, 15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                19 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-5, 10),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-5, -10)]),
                        points: vec![
                            FixVec::new_i64(-5, 10),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-5, -10),
                            FixVec::new_i64(-15, 0)],
                        indices: vec![7, 0, 3, 7, 3, 6, 5, 6, 4, 6, 3, 4, 1, 2, 0, 0, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, -20),
                                    FixVec::new_i64(-5, -10),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-5, 10),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, -20),
                                    FixVec::new_i64(-10, -15),
                                    FixVec::new_i64(-5, -10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, 10),
                                    FixVec::new_i64(-10, 15),
                                    FixVec::new_i64(5, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-5, 10),
                                    FixVec::new_i64(5, 20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                20 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15360, 0),
                            FixVec::new_i64(-5120, 10240),
                            FixVec::new_i64(-10240, 15360),
                            FixVec::new_i64(-2560, 20480),
                            FixVec::new_i64(5120, 20480),
                            FixVec::new_i64(2560, 10240),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(2560, -10240),
                            FixVec::new_i64(5120, -20480),
                            FixVec::new_i64(-2560, -20480),
                            FixVec::new_i64(-10240, -15360),
                            FixVec::new_i64(-5120, -10240)]),
                        points: vec![
                            FixVec::new_i64(-5120, 10240),
                            FixVec::new_i64(-10240, 15360),
                            FixVec::new_i64(-2560, 20480),
                            FixVec::new_i64(5120, 20480),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5120, -20480),
                            FixVec::new_i64(-2560, -20480),
                            FixVec::new_i64(-10240, -15360),
                            FixVec::new_i64(-5120, -10240),
                            FixVec::new_i64(-15360, 0)],
                        indices: vec![4, 9, 0, 7, 8, 6, 4, 8, 9, 3, 0, 2, 8, 4, 5, 3, 4, 0, 1, 2, 0, 8, 5, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5120, -10240),
                                    FixVec::new_i64(-15360, 0),
                                    FixVec::new_i64(-5120, 10240),
                                    FixVec::new_i64(5120, 20480),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, -20480),
                                    FixVec::new_i64(-2560, -20480),
                                    FixVec::new_i64(-10240, -15360),
                                    FixVec::new_i64(-5120, -10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10240, 15360),
                                    FixVec::new_i64(-2560, 20480),
                                    FixVec::new_i64(5120, 20480),
                                    FixVec::new_i64(-5120, 10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, -20480),
                                    FixVec::new_i64(-5120, -10240),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                21 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-15360, 0),
                            FixVec::new_i64(-5120, 10240),
                            FixVec::new_i64(-10240, 15360),
                            FixVec::new_i64(-2560, 20480),
                            FixVec::new_i64(5120, 20480),
                            FixVec::new_i64(-2560, -15360),
                            FixVec::new_i64(5120, -20480),
                            FixVec::new_i64(-2560, -20480),
                            FixVec::new_i64(-10240, -15360),
                            FixVec::new_i64(-5120, -10240)]),
                        points: vec![
                            FixVec::new_i64(-5120, 10240),
                            FixVec::new_i64(-10240, 15360),
                            FixVec::new_i64(-2560, 20480),
                            FixVec::new_i64(5120, 20480),
                            FixVec::new_i64(-2560, -15360),
                            FixVec::new_i64(5120, -20480),
                            FixVec::new_i64(-2560, -20480),
                            FixVec::new_i64(-10240, -15360),
                            FixVec::new_i64(-5120, -10240),
                            FixVec::new_i64(-15360, 0)],
                        indices: vec![9, 0, 8, 7, 8, 4, 8, 0, 3, 7, 4, 6, 6, 4, 5, 1, 2, 0, 0, 2, 3, 8, 3, 4],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, 20480),
                                    FixVec::new_i64(-5120, -10240),
                                    FixVec::new_i64(-15360, 0),
                                    FixVec::new_i64(-5120, 10240)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-2560, -20480),
                                    FixVec::new_i64(-10240, -15360),
                                    FixVec::new_i64(-5120, -10240),
                                    FixVec::new_i64(-2560, -15360)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, -20480),
                                    FixVec::new_i64(-2560, -20480),
                                    FixVec::new_i64(-2560, -15360)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, 20480),
                                    FixVec::new_i64(-5120, 10240),
                                    FixVec::new_i64(-10240, 15360),
                                    FixVec::new_i64(-2560, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-2560, -15360),
                                    FixVec::new_i64(-5120, -10240),
                                    FixVec::new_i64(5120, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                22 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 5),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(20, 20),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, -5),
                            FixVec::new_i64(10, -5),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 5),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, -5),
                            FixVec::new_i64(10, -5),
                            FixVec::new_i64(10, -10)],
                        indices: vec![1, 2, 0, 0, 2, 6, 0, 6, 7, 2, 3, 6, 6, 3, 5, 3, 4, 5],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 20),
                                    FixVec::new_i64(10, -5),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-5, 5)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(25, 20),
                                    FixVec::new_i64(25, -5),
                                    FixVec::new_i64(10, -5),
                                    FixVec::new_i64(10, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                23 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 15),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(20, 20),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, -5),
                            FixVec::new_i64(10, -5),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 15),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, -5),
                            FixVec::new_i64(10, -5),
                            FixVec::new_i64(10, -10)],
                        indices: vec![6, 1, 2, 0, 1, 6, 0, 6, 7, 2, 3, 6, 6, 3, 5, 3, 4, 5],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(25, 20),
                                    FixVec::new_i64(25, -5),
                                    FixVec::new_i64(10, -5),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-5, 15),
                                    FixVec::new_i64(10, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(10, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                24 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 5),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(15, 10),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 5),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(15, 10),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(10, -10)],
                        indices: vec![1, 2, 0, 0, 2, 8, 8, 2, 7, 2, 3, 4, 2, 4, 7, 7, 4, 6, 4, 5, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(-5, 5),
                                    FixVec::new_i64(10, 20),
                                    FixVec::new_i64(15, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(25, 20),
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(15, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                25 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(15, 10),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(15, 10),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(10, -10)],
                        indices: vec![1, 2, 0, 0, 2, 8, 8, 2, 7, 2, 3, 7, 3, 4, 7, 7, 4, 6, 4, 5, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-5, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-5, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 10),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(10, 20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(25, 20),
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(15, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                26 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(15, 10),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(15, 10),
                            FixVec::new_i64(25, 20),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(10, -10)],
                        indices: vec![0, 1, 7, 0, 7, 8, 1, 2, 7, 2, 3, 4, 2, 4, 7, 7, 4, 6, 4, 5, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(10, 20),
                                    FixVec::new_i64(15, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(25, 20),
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(15, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                27 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-35, 5),
                            FixVec::new_i64(-20, 10),
                            FixVec::new_i64(-18, 20),
                            FixVec::new_i64(0, 20),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(15, 5),
                            FixVec::new_i64(20, 10),
                            FixVec::new_i64(35, 0),
                            FixVec::new_i64(25, -10),
                            FixVec::new_i64(10, -4),
                            FixVec::new_i64(-5, -15),
                            FixVec::new_i64(-5, -20),
                            FixVec::new_i64(-15, -25),
                            FixVec::new_i64(-20, -10),
                            FixVec::new_i64(-30, -5)]),
                        points: vec![
                            FixVec::new_i64(-20, 10),
                            FixVec::new_i64(-18, 20),
                            FixVec::new_i64(0, 20),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(15, 5),
                            FixVec::new_i64(20, 10),
                            FixVec::new_i64(35, 0),
                            FixVec::new_i64(25, -10),
                            FixVec::new_i64(10, -4),
                            FixVec::new_i64(-5, -15),
                            FixVec::new_i64(-5, -20),
                            FixVec::new_i64(-15, -25),
                            FixVec::new_i64(-20, -10),
                            FixVec::new_i64(-30, -5),
                            FixVec::new_i64(-35, 5)],
                        indices: vec![13, 14, 0, 13, 0, 12, 2, 0, 1, 12, 0, 9, 12, 9, 11, 11, 9, 10, 2, 3, 0, 0, 3, 9, 9, 3, 8, 3, 4, 8, 4, 5, 7, 4, 7, 8, 5, 6, 7],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 5),
                                    FixVec::new_i64(10, -4),
                                    FixVec::new_i64(-5, -15),
                                    FixVec::new_i64(-20, -10),
                                    FixVec::new_i64(-30, -5),
                                    FixVec::new_i64(-35, 5),
                                    FixVec::new_i64(-20, 10),
                                    FixVec::new_i64(5, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 10),
                                    FixVec::new_i64(-20, 10),
                                    FixVec::new_i64(-18, 20),
                                    FixVec::new_i64(0, 20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, -20),
                                    FixVec::new_i64(-15, -25),
                                    FixVec::new_i64(-20, -10),
                                    FixVec::new_i64(-5, -15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(35, 0),
                                    FixVec::new_i64(25, -10),
                                    FixVec::new_i64(10, -4),
                                    FixVec::new_i64(15, 5),
                                    FixVec::new_i64(20, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                28 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-35, 5),
                            FixVec::new_i64(-20, 10),
                            FixVec::new_i64(-10, 20),
                            FixVec::new_i64(0, 20),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(15, 5),
                            FixVec::new_i64(20, 10),
                            FixVec::new_i64(35, 0),
                            FixVec::new_i64(25, -10),
                            FixVec::new_i64(10, -4),
                            FixVec::new_i64(-5, -15),
                            FixVec::new_i64(-5, -20),
                            FixVec::new_i64(-15, -25),
                            FixVec::new_i64(-20, -10),
                            FixVec::new_i64(-30, -5)]),
                        points: vec![
                            FixVec::new_i64(-20, 10),
                            FixVec::new_i64(-10, 20),
                            FixVec::new_i64(0, 20),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(15, 5),
                            FixVec::new_i64(20, 10),
                            FixVec::new_i64(35, 0),
                            FixVec::new_i64(25, -10),
                            FixVec::new_i64(10, -4),
                            FixVec::new_i64(-5, -15),
                            FixVec::new_i64(-5, -20),
                            FixVec::new_i64(-15, -25),
                            FixVec::new_i64(-20, -10),
                            FixVec::new_i64(-30, -5),
                            FixVec::new_i64(-35, 5)],
                        indices: vec![13, 14, 0, 13, 0, 12, 0, 1, 3, 9, 12, 0, 9, 11, 12, 11, 9, 10, 1, 2, 3, 0, 3, 9, 9, 3, 8, 3, 4, 8, 4, 5, 7, 4, 7, 8, 5, 6, 7],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 5),
                                    FixVec::new_i64(10, -4),
                                    FixVec::new_i64(-5, -15),
                                    FixVec::new_i64(-20, -10),
                                    FixVec::new_i64(-30, -5),
                                    FixVec::new_i64(-35, 5),
                                    FixVec::new_i64(-20, 10),
                                    FixVec::new_i64(5, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 20),
                                    FixVec::new_i64(5, 10),
                                    FixVec::new_i64(-20, 10),
                                    FixVec::new_i64(-10, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, -20),
                                    FixVec::new_i64(-15, -25),
                                    FixVec::new_i64(-20, -10),
                                    FixVec::new_i64(-5, -15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(35, 0),
                                    FixVec::new_i64(25, -10),
                                    FixVec::new_i64(10, -4),
                                    FixVec::new_i64(15, 5),
                                    FixVec::new_i64(20, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                29 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10240, -10240),
                            FixVec::new_i64(-10240, -5120),
                            FixVec::new_i64(-10240, 0),
                            FixVec::new_i64(-10240, 5120),
                            FixVec::new_i64(-10240, 10240),
                            FixVec::new_i64(10240, 10240),
                            FixVec::new_i64(10240, 5120),
                            FixVec::new_i64(10240, 0),
                            FixVec::new_i64(10240, -5120),
                            FixVec::new_i64(10240, -10240)]),
                        points: vec![
                            FixVec::new_i64(-10240, -10240),
                            FixVec::new_i64(-10240, 10240),
                            FixVec::new_i64(10240, 10240),
                            FixVec::new_i64(10240, -10240)],
                        indices: vec![0, 1, 3, 1, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10240, 10240),
                                    FixVec::new_i64(10240, -10240),
                                    FixVec::new_i64(-10240, -10240),
                                    FixVec::new_i64(-10240, 10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                30 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-20, 0),
                            FixVec::new_i64(-15, 15),
                            FixVec::new_i64(-10, 20),
                            FixVec::new_i64(-5, 15),
                            FixVec::new_i64(0, 20),
                            FixVec::new_i64(5, 15),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(15, 15),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(20, -15),
                            FixVec::new_i64(15, -20),
                            FixVec::new_i64(10, -15),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(0, -15),
                            FixVec::new_i64(-5, -20),
                            FixVec::new_i64(-10, -15)]),
                        points: vec![
                            FixVec::new_i64(-20, 0),
                            FixVec::new_i64(-15, 15),
                            FixVec::new_i64(-10, 20),
                            FixVec::new_i64(-5, 15),
                            FixVec::new_i64(0, 20),
                            FixVec::new_i64(5, 15),
                            FixVec::new_i64(10, 20),
                            FixVec::new_i64(15, 15),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(20, -15),
                            FixVec::new_i64(15, -20),
                            FixVec::new_i64(10, -15),
                            FixVec::new_i64(5, -20),
                            FixVec::new_i64(0, -15),
                            FixVec::new_i64(-5, -20),
                            FixVec::new_i64(-10, -15)],
                        indices: vec![0, 1, 3, 3, 1, 2, 13, 0, 3, 13, 15, 0, 13, 14, 15, 5, 3, 4, 5, 13, 3, 11, 13, 5, 11, 12, 13, 7, 5, 6, 8, 5, 7, 8, 11, 5, 9, 10, 11, 11, 8, 9],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 15),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(-5, -20),
                                    FixVec::new_i64(-10, -15),
                                    FixVec::new_i64(-20, 0),
                                    FixVec::new_i64(-15, 15),
                                    FixVec::new_i64(-5, 15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, 20),
                                    FixVec::new_i64(-5, 15),
                                    FixVec::new_i64(-15, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 20),
                                    FixVec::new_i64(5, 15),
                                    FixVec::new_i64(-5, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 20),
                                    FixVec::new_i64(15, 15),
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(20, -15),
                                    FixVec::new_i64(10, -15),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(5, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(10, -15),
                                    FixVec::new_i64(5, -20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -15),
                                    FixVec::new_i64(20, -15),
                                    FixVec::new_i64(15, -20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                31 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-20, 5),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 20),
                            FixVec::new_i64(0, 25),
                            FixVec::new_i64(5, 15),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(15, 5),
                            FixVec::new_i64(20, -5),
                            FixVec::new_i64(15, -15),
                            FixVec::new_i64(5, -25),
                            FixVec::new_i64(0, -15),
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-15, -5)]),
                        points: vec![
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 20),
                            FixVec::new_i64(0, 25),
                            FixVec::new_i64(5, 15),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(15, 5),
                            FixVec::new_i64(20, -5),
                            FixVec::new_i64(15, -15),
                            FixVec::new_i64(5, -25),
                            FixVec::new_i64(0, -15),
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-15, -5),
                            FixVec::new_i64(-20, 5)],
                        indices: vec![11, 12, 0, 11, 0, 10, 0, 1, 3, 4, 10, 0, 3, 1, 2, 4, 0, 3, 4, 9, 10, 9, 4, 7, 9, 7, 8, 4, 5, 6, 4, 6, 7],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, -5),
                                    FixVec::new_i64(15, -15),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-15, -5),
                                    FixVec::new_i64(-20, 5),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(10, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 25),
                                    FixVec::new_i64(5, 15),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-5, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, -25),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(15, -15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, -5),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(15, 5)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                32 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-350, 50),
                            FixVec::new_i64(-135, 80),
                            FixVec::new_i64(-95, 200),
                            FixVec::new_i64(30, 200),
                            FixVec::new_i64(85, 110),
                            FixVec::new_i64(150, 50),
                            FixVec::new_i64(32, 145),
                            FixVec::new_i64(350, 0),
                            FixVec::new_i64(250, -100),
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(-5, -125),
                            FixVec::new_i64(-50, -200),
                            FixVec::new_i64(-75, 25),
                            FixVec::new_i64(-310, -40)]),
                        points: vec![
                            FixVec::new_i64(-135, 80),
                            FixVec::new_i64(-95, 200),
                            FixVec::new_i64(30, 200),
                            FixVec::new_i64(75, 126),
                            FixVec::new_i64(32, 145),
                            FixVec::new_i64(150, 50),
                            FixVec::new_i64(85, 110),
                            FixVec::new_i64(75, 126),
                            FixVec::new_i64(350, 0),
                            FixVec::new_i64(250, -100),
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(-5, -125),
                            FixVec::new_i64(-50, -200),
                            FixVec::new_i64(-75, 25),
                            FixVec::new_i64(-310, -40),
                            FixVec::new_i64(-350, 50)],
                        indices: vec![15, 0, 14, 14, 0, 13, 4, 0, 1, 12, 13, 11, 4, 13, 0, 13, 10, 11, 1, 2, 4, 13, 4, 10, 4, 5, 10, 10, 5, 9, 2, 3, 4, 6, 7, 5, 7, 8, 5, 5, 8, 9],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-75, 25),
                                    FixVec::new_i64(-310, -40),
                                    FixVec::new_i64(-350, 50),
                                    FixVec::new_i64(-135, 80)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 15),
                                    FixVec::new_i64(-75, 25),
                                    FixVec::new_i64(-135, 80),
                                    FixVec::new_i64(-95, 200),
                                    FixVec::new_i64(30, 200),
                                    FixVec::new_i64(32, 145)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 15),
                                    FixVec::new_i64(-5, -125),
                                    FixVec::new_i64(-50, -200),
                                    FixVec::new_i64(-75, 25)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(250, -100),
                                    FixVec::new_i64(0, 15),
                                    FixVec::new_i64(32, 145),
                                    FixVec::new_i64(150, 50)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(32, 145),
                                    FixVec::new_i64(30, 200),
                                    FixVec::new_i64(75, 126)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(350, 0),
                                    FixVec::new_i64(150, 50),
                                    FixVec::new_i64(85, 110),
                                    FixVec::new_i64(75, 126)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(250, -100),
                                    FixVec::new_i64(150, 50),
                                    FixVec::new_i64(350, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                33 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 5),
                            FixVec::new_i64(-5, 5),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5, 5),
                            FixVec::new_i64(10, 5),
                            FixVec::new_i64(10, -5),
                            FixVec::new_i64(5, -5),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(-10, -5)]),
                        points: vec![
                            FixVec::new_i64(-10, -5),
                            FixVec::new_i64(-10, 5),
                            FixVec::new_i64(-5, 5),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(5, 5),
                            FixVec::new_i64(10, 5),
                            FixVec::new_i64(10, -5),
                            FixVec::new_i64(5, -5),
                            FixVec::new_i64(0, 0)],
                        indices: vec![0, 1, 4, 1, 2, 4, 4, 2, 3, 9, 5, 8, 8, 5, 7, 5, 6, 7],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(-10, -5),
                                    FixVec::new_i64(-10, 5),
                                    FixVec::new_i64(-5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 5),
                                    FixVec::new_i64(10, -5),
                                    FixVec::new_i64(5, -5),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                34 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-512, -5120),
                            FixVec::new_i64(-10240, 0),
                            FixVec::new_i64(-10240, 10240),
                            FixVec::new_i64(20480, 10240),
                            FixVec::new_i64(10240, 5120),
                            FixVec::new_i64(10240, 0),
                            FixVec::new_i64(5120, 5120),
                            FixVec::new_i64(0, -15360)]),
                        points: vec![
                            FixVec::new_i64(-10240, 0),
                            FixVec::new_i64(-10240, 10240),
                            FixVec::new_i64(20480, 10240),
                            FixVec::new_i64(10240, 5120),
                            FixVec::new_i64(10240, 0),
                            FixVec::new_i64(5120, 5120),
                            FixVec::new_i64(0, -15360),
                            FixVec::new_i64(-512, -5120)],
                        indices: vec![0, 1, 5, 6, 7, 5, 0, 5, 7, 4, 5, 3, 5, 1, 2, 5, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-512, -5120),
                                    FixVec::new_i64(-10240, 0),
                                    FixVec::new_i64(-10240, 10240),
                                    FixVec::new_i64(5120, 5120)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, 5120),
                                    FixVec::new_i64(0, -15360),
                                    FixVec::new_i64(-512, -5120)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10240, 5120),
                                    FixVec::new_i64(10240, 0),
                                    FixVec::new_i64(5120, 5120)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10240, 5120),
                                    FixVec::new_i64(5120, 5120),
                                    FixVec::new_i64(-10240, 10240),
                                    FixVec::new_i64(20480, 10240)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                35 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(-20, 10),
                            FixVec::new_i64(20, 10),
                            FixVec::new_i64(15, -5),
                            FixVec::new_i64(5, 5),
                            FixVec::new_i64(0, -15)]),
                        points: vec![
                            FixVec::new_i64(20, 10),
                            FixVec::new_i64(15, -5),
                            FixVec::new_i64(5, 5),
                            FixVec::new_i64(0, -15),
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(-20, 10)],
                        indices: vec![3, 4, 2, 4, 5, 2, 5, 0, 2, 2, 0, 1],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 5),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(-5, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 5),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(-20, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 5),
                                    FixVec::new_i64(-20, 10),
                                    FixVec::new_i64(20, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, -5),
                                    FixVec::new_i64(5, 5),
                                    FixVec::new_i64(20, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                36 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-150, 150),
                            FixVec::new_i64(30, 150),
                            FixVec::new_i64(50, 50),
                            FixVec::new_i64(100, -40),
                            FixVec::new_i64(170, -80),
                            FixVec::new_i64(90, -100),
                            FixVec::new_i64(85, -70),
                            FixVec::new_i64(-50, 20)]),
                        points: vec![
                            FixVec::new_i64(30, 150),
                            FixVec::new_i64(50, 50),
                            FixVec::new_i64(100, -40),
                            FixVec::new_i64(170, -80),
                            FixVec::new_i64(90, -100),
                            FixVec::new_i64(85, -70),
                            FixVec::new_i64(-50, 20),
                            FixVec::new_i64(-150, 150)],
                        indices: vec![7, 0, 6, 0, 1, 6, 6, 1, 5, 1, 2, 5, 5, 2, 3, 5, 3, 4],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(50, 50),
                                    FixVec::new_i64(-50, 20),
                                    FixVec::new_i64(-150, 150),
                                    FixVec::new_i64(30, 150)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(100, -40),
                                    FixVec::new_i64(85, -70),
                                    FixVec::new_i64(-50, 20),
                                    FixVec::new_i64(50, 50)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(90, -100),
                                    FixVec::new_i64(85, -70),
                                    FixVec::new_i64(100, -40),
                                    FixVec::new_i64(170, -80)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                37 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-40, 40),
                                FixVec::new_i64(40, 40),
                                FixVec::new_i64(40, -40),
                                FixVec::new_i64(-40, -40)],
                            vec![
                                FixVec::new_i64(5, 0),
                                FixVec::new_i64(10, -10),
                                FixVec::new_i64(25, 0),
                                FixVec::new_i64(15, 5)],
                            vec![
                                FixVec::new_i64(-15, -25),
                                FixVec::new_i64(5, -15),
                                FixVec::new_i64(-5, -5),
                                FixVec::new_i64(5, 10),
                                FixVec::new_i64(5, 20),
                                FixVec::new_i64(-15, 20)]]),
                        points: vec![
                            FixVec::new_i64(-40, -40),
                            FixVec::new_i64(-40, 40),
                            FixVec::new_i64(40, 40),
                            FixVec::new_i64(40, -40),
                            FixVec::new_i64(5, -15),
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(5, 20),
                            FixVec::new_i64(-15, 20),
                            FixVec::new_i64(-15, -25),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(15, 5)],
                        indices: vec![0, 1, 8, 0, 8, 9, 8, 1, 7, 13, 7, 2, 1, 2, 7, 13, 2, 12, 9, 4, 3, 10, 11, 4, 3, 11, 12, 9, 3, 0, 3, 4, 11, 12, 2, 3, 4, 5, 10, 5, 6, 10, 13, 6, 7, 13, 10, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-15, -25),
                                    FixVec::new_i64(-40, -40),
                                    FixVec::new_i64(-40, 40),
                                    FixVec::new_i64(-15, 20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(40, 40),
                                    FixVec::new_i64(5, 20),
                                    FixVec::new_i64(-15, 20),
                                    FixVec::new_i64(-40, 40)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 10),
                                    FixVec::new_i64(5, 20),
                                    FixVec::new_i64(40, 40),
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(15, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-40, -40),
                                    FixVec::new_i64(-15, -25),
                                    FixVec::new_i64(5, -15),
                                    FixVec::new_i64(40, -40)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(5, -15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, -15),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(40, -40)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(40, -40),
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(40, 40)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 5),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(5, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                38 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-5, 0),
                                FixVec::new_i64(-25, 15),
                                FixVec::new_i64(20, 15),
                                FixVec::new_i64(20, -15),
                                FixVec::new_i64(-25, -15)],
                            vec![
                                FixVec::new_i64(-15, -10),
                                FixVec::new_i64(10, -10),
                                FixVec::new_i64(10, 10),
                                FixVec::new_i64(-10, 10),
                                FixVec::new_i64(0, 0)]]),
                        points: vec![
                            FixVec::new_i64(-25, -15),
                            FixVec::new_i64(-5, 0),
                            FixVec::new_i64(-25, 15),
                            FixVec::new_i64(20, 15),
                            FixVec::new_i64(20, -15),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-15, -10)],
                        indices: vec![0, 1, 9, 9, 1, 8, 7, 2, 3, 7, 3, 6, 9, 5, 0, 0, 5, 4, 5, 6, 4, 6, 3, 4, 2, 7, 1, 7, 8, 1],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-15, -10),
                                    FixVec::new_i64(-25, -15),
                                    FixVec::new_i64(-5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-25, 15),
                                    FixVec::new_i64(20, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, -15),
                                    FixVec::new_i64(-25, -15),
                                    FixVec::new_i64(-15, -10),
                                    FixVec::new_i64(10, -10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20, 15),
                                    FixVec::new_i64(20, -15),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-25, 15),
                                    FixVec::new_i64(-10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                39 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-25, 5),
                                FixVec::new_i64(-30, 20),
                                FixVec::new_i64(-25, 30),
                                FixVec::new_i64(-10, 25),
                                FixVec::new_i64(0, 30),
                                FixVec::new_i64(15, 15),
                                FixVec::new_i64(30, 15),
                                FixVec::new_i64(35, 5),
                                FixVec::new_i64(30, -10),
                                FixVec::new_i64(25, -10),
                                FixVec::new_i64(15, -20),
                                FixVec::new_i64(15, -30),
                                FixVec::new_i64(-5, -35),
                                FixVec::new_i64(-15, -20),
                                FixVec::new_i64(-40, -25),
                                FixVec::new_i64(-35, -5)],
                            vec![
                                FixVec::new_i64(5, 0),
                                FixVec::new_i64(10, -10),
                                FixVec::new_i64(25, 0),
                                FixVec::new_i64(15, 5)],
                            vec![
                                FixVec::new_i64(-15, 0),
                                FixVec::new_i64(-25, -5),
                                FixVec::new_i64(-30, -15),
                                FixVec::new_i64(-15, -10),
                                FixVec::new_i64(-5, -15),
                                FixVec::new_i64(0, -25),
                                FixVec::new_i64(5, -15),
                                FixVec::new_i64(-5, -5),
                                FixVec::new_i64(-5, 5),
                                FixVec::new_i64(5, 10),
                                FixVec::new_i64(0, 20),
                                FixVec::new_i64(-5, 15),
                                FixVec::new_i64(-10, 15),
                                FixVec::new_i64(-15, 20),
                                FixVec::new_i64(-20, 10),
                                FixVec::new_i64(-15, 5)]]),
                        points: vec![
                            FixVec::new_i64(-40, -25),
                            FixVec::new_i64(-35, -5),
                            FixVec::new_i64(-25, 5),
                            FixVec::new_i64(-30, 20),
                            FixVec::new_i64(-25, 30),
                            FixVec::new_i64(-10, 25),
                            FixVec::new_i64(0, 30),
                            FixVec::new_i64(15, 15),
                            FixVec::new_i64(30, 15),
                            FixVec::new_i64(35, 5),
                            FixVec::new_i64(30, -10),
                            FixVec::new_i64(25, -10),
                            FixVec::new_i64(15, -20),
                            FixVec::new_i64(15, -30),
                            FixVec::new_i64(-5, -35),
                            FixVec::new_i64(-15, -20),
                            FixVec::new_i64(-15, -10),
                            FixVec::new_i64(-5, -15),
                            FixVec::new_i64(0, -25),
                            FixVec::new_i64(5, -15),
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(-5, 5),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(0, 20),
                            FixVec::new_i64(-5, 15),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(-15, 20),
                            FixVec::new_i64(-20, 10),
                            FixVec::new_i64(-15, 5),
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-25, -5),
                            FixVec::new_i64(-30, -15),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(25, 0),
                            FixVec::new_i64(15, 5)],
                        indices: vec![1, 31, 0, 0, 31, 15, 31, 16, 15, 15, 16, 17, 18, 15, 17, 18, 14, 15, 12, 19, 33, 32, 33, 19, 12, 18, 19, 13, 18, 12, 13, 14, 18, 12, 33, 11, 33, 34, 11, 11, 34, 10, 31, 1, 30, 1, 2, 30, 27, 2, 3, 2, 27, 28, 26, 27, 3, 26, 4, 5, 26, 5, 25, 25, 5, 24, 24, 5, 23, 5, 6, 23, 7, 23, 6, 22, 23, 7, 22, 7, 35, 35, 7, 8, 35, 8, 34, 34, 8, 9, 34, 9, 10, 26, 3, 4, 29, 30, 2, 2, 28, 29, 20, 21, 32, 20, 32, 19, 21, 22, 32, 32, 22, 35],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-25, 5),
                                    FixVec::new_i64(-25, -5),
                                    FixVec::new_i64(-30, -15),
                                    FixVec::new_i64(-40, -25),
                                    FixVec::new_i64(-35, -5)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, -15),
                                    FixVec::new_i64(-15, -20),
                                    FixVec::new_i64(-40, -25),
                                    FixVec::new_i64(-30, -15),
                                    FixVec::new_i64(-15, -10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, -35),
                                    FixVec::new_i64(-15, -20),
                                    FixVec::new_i64(-5, -15),
                                    FixVec::new_i64(0, -25)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -25),
                                    FixVec::new_i64(5, -15),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(25, -10),
                                    FixVec::new_i64(15, -20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, 5),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(5, -15),
                                    FixVec::new_i64(-5, -5)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, -35),
                                    FixVec::new_i64(0, -25),
                                    FixVec::new_i64(15, -20),
                                    FixVec::new_i64(15, -30)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, 15),
                                    FixVec::new_i64(35, 5),
                                    FixVec::new_i64(30, -10),
                                    FixVec::new_i64(25, -10),
                                    FixVec::new_i64(25, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-25, 30),
                                    FixVec::new_i64(-15, 20),
                                    FixVec::new_i64(-20, 10),
                                    FixVec::new_i64(-25, 5),
                                    FixVec::new_i64(-30, 20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-25, -5),
                                    FixVec::new_i64(-25, 5),
                                    FixVec::new_i64(-20, 10),
                                    FixVec::new_i64(-15, 5),
                                    FixVec::new_i64(-15, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 20),
                                    FixVec::new_i64(-5, 15),
                                    FixVec::new_i64(-10, 15),
                                    FixVec::new_i64(-15, 20),
                                    FixVec::new_i64(-25, 30),
                                    FixVec::new_i64(-10, 25)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 15),
                                    FixVec::new_i64(0, 20),
                                    FixVec::new_i64(-10, 25),
                                    FixVec::new_i64(0, 30)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 5),
                                    FixVec::new_i64(5, 10),
                                    FixVec::new_i64(0, 20),
                                    FixVec::new_i64(15, 15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(25, 0),
                                    FixVec::new_i64(15, 5),
                                    FixVec::new_i64(15, 15),
                                    FixVec::new_i64(30, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 5),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(-5, 5),
                                    FixVec::new_i64(5, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                40 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-5, 10),
                                FixVec::new_i64(5, 10),
                                FixVec::new_i64(10, 5),
                                FixVec::new_i64(10, -5),
                                FixVec::new_i64(5, -10),
                                FixVec::new_i64(-5, -10),
                                FixVec::new_i64(-10, -5),
                                FixVec::new_i64(-10, 5)],
                            vec![
                                FixVec::new_i64(-5, 0),
                                FixVec::new_i64(0, -5),
                                FixVec::new_i64(5, 0),
                                FixVec::new_i64(0, 5)]]),
                        points: vec![
                            FixVec::new_i64(-10, -5),
                            FixVec::new_i64(-10, 5),
                            FixVec::new_i64(-5, 10),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(10, 5),
                            FixVec::new_i64(10, -5),
                            FixVec::new_i64(5, -10),
                            FixVec::new_i64(-5, -10),
                            FixVec::new_i64(-5, 0),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(0, 5)],
                        indices: vec![0, 1, 8, 0, 8, 7, 1, 2, 8, 8, 2, 11, 11, 2, 3, 11, 3, 10, 8, 9, 7, 7, 9, 6, 9, 10, 6, 4, 10, 3, 10, 5, 6, 4, 5, 10],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, -10),
                                    FixVec::new_i64(-5, -10),
                                    FixVec::new_i64(-10, -5),
                                    FixVec::new_i64(-10, 5),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(0, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 10),
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-10, 5),
                                    FixVec::new_i64(-5, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -5),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(5, 10),
                                    FixVec::new_i64(10, 5)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -5),
                                    FixVec::new_i64(5, -10),
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                41 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-10, -10),
                                FixVec::new_i64(-10, 10),
                                FixVec::new_i64(10, 10),
                                FixVec::new_i64(10, -10)],
                            vec![
                                FixVec::new_i64(0, -5),
                                FixVec::new_i64(5, -5),
                                FixVec::new_i64(5, 5),
                                FixVec::new_i64(0, 5)],
                            vec![
                                FixVec::new_i64(-5, -5),
                                FixVec::new_i64(0, -5),
                                FixVec::new_i64(0, 5),
                                FixVec::new_i64(-5, 5)]]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(5, -5),
                            FixVec::new_i64(5, 5),
                            FixVec::new_i64(-5, 5),
                            FixVec::new_i64(-5, -5)],
                        indices: vec![0, 1, 7, 7, 1, 6, 6, 1, 5, 1, 2, 5, 7, 4, 0, 0, 4, 3, 4, 5, 3, 5, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, 5),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(5, 5),
                                    FixVec::new_i64(-5, 5),
                                    FixVec::new_i64(-10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(5, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(5, -5),
                                    FixVec::new_i64(5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                42 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-5120, 5120),
                            FixVec::new_i64(-5120, 10240),
                            FixVec::new_i64(5120, 10240),
                            FixVec::new_i64(5120, 5120),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5120, -5120),
                            FixVec::new_i64(5120, -10240),
                            FixVec::new_i64(-5120, -10240),
                            FixVec::new_i64(-5120, -5120)]),
                        points: vec![
                            FixVec::new_i64(-5120, -10240),
                            FixVec::new_i64(-5120, -5120),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5120, -5120),
                            FixVec::new_i64(5120, -10240),
                            FixVec::new_i64(-5120, 5120),
                            FixVec::new_i64(-5120, 10240),
                            FixVec::new_i64(5120, 10240),
                            FixVec::new_i64(5120, 5120),
                            FixVec::new_i64(0, 0)],
                        indices: vec![3, 1, 2, 1, 4, 0, 3, 4, 1, 5, 6, 8, 5, 8, 9, 6, 7, 8],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5120, -10240),
                                    FixVec::new_i64(-5120, -5120),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(5120, -5120),
                                    FixVec::new_i64(5120, -10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, 10240),
                                    FixVec::new_i64(5120, 5120),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-5120, 5120),
                                    FixVec::new_i64(-5120, 10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                43 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-15, -10),
                                FixVec::new_i64(-15, 10),
                                FixVec::new_i64(15, 10),
                                FixVec::new_i64(15, -10)],
                            vec![
                                FixVec::new_i64(-10, 0),
                                FixVec::new_i64(-5, -5),
                                FixVec::new_i64(0, 0),
                                FixVec::new_i64(-5, 5)],
                            vec![
                                FixVec::new_i64(0, 0),
                                FixVec::new_i64(5, -5),
                                FixVec::new_i64(10, 0),
                                FixVec::new_i64(5, 5)]]),
                        points: vec![
                            FixVec::new_i64(-15, -10),
                            FixVec::new_i64(-15, 10),
                            FixVec::new_i64(15, 10),
                            FixVec::new_i64(15, -10),
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5, -5),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(5, 5),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-5, 5)],
                        indices: vec![0, 1, 4, 4, 1, 11, 10, 11, 9, 11, 1, 9, 1, 2, 9, 9, 2, 8, 4, 5, 0, 6, 7, 5, 5, 7, 0, 0, 7, 3, 7, 8, 3, 8, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(-15, -10),
                                    FixVec::new_i64(-15, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, 5),
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(-15, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 5),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 10),
                                    FixVec::new_i64(5, 5),
                                    FixVec::new_i64(-5, 5),
                                    FixVec::new_i64(-15, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(5, 5),
                                    FixVec::new_i64(15, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-15, -10),
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(-5, -5)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(5, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, -10),
                                    FixVec::new_i64(-15, -10),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(5, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, -10),
                                    FixVec::new_i64(5, -5),
                                    FixVec::new_i64(10, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, -10),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(15, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                44 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-10, 15),
                                FixVec::new_i64(10, 15),
                                FixVec::new_i64(10, -15),
                                FixVec::new_i64(-10, -15)],
                            vec![
                                FixVec::new_i64(-5, 5),
                                FixVec::new_i64(0, 0),
                                FixVec::new_i64(5, 5),
                                FixVec::new_i64(0, 10)],
                            vec![
                                FixVec::new_i64(-5, -5),
                                FixVec::new_i64(0, -10),
                                FixVec::new_i64(5, -5),
                                FixVec::new_i64(0, 0)]]),
                        points: vec![
                            FixVec::new_i64(-10, -15),
                            FixVec::new_i64(-10, 15),
                            FixVec::new_i64(10, 15),
                            FixVec::new_i64(10, -15),
                            FixVec::new_i64(-5, -5),
                            FixVec::new_i64(0, -10),
                            FixVec::new_i64(5, -5),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5, 5),
                            FixVec::new_i64(0, 10),
                            FixVec::new_i64(-5, 5),
                            FixVec::new_i64(0, 0)],
                        indices: vec![0, 1, 4, 4, 1, 10, 10, 1, 9, 1, 2, 9, 9, 2, 8, 4, 5, 0, 0, 5, 3, 5, 6, 3, 6, 8, 3, 8, 2, 3, 4, 10, 11, 7, 8, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, 5),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(-10, -15),
                                    FixVec::new_i64(-10, 15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(-5, 5),
                                    FixVec::new_i64(-10, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(-10, 15),
                                    FixVec::new_i64(10, 15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 5),
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(10, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, -15),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(0, -10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -15),
                                    FixVec::new_i64(-10, -15),
                                    FixVec::new_i64(0, -10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -15),
                                    FixVec::new_i64(0, -10),
                                    FixVec::new_i64(5, -5)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 15),
                                    FixVec::new_i64(10, -15),
                                    FixVec::new_i64(5, -5),
                                    FixVec::new_i64(5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-5, -5),
                                    FixVec::new_i64(-5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, -5),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(5, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                45 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-10, 10),
                                FixVec::new_i64(-5, 10),
                                FixVec::new_i64(0, 5),
                                FixVec::new_i64(5, 10),
                                FixVec::new_i64(10, 10),
                                FixVec::new_i64(10, -10),
                                FixVec::new_i64(-10, -10)],
                            vec![
                                FixVec::new_i64(-5, 0),
                                FixVec::new_i64(0, -5),
                                FixVec::new_i64(5, 0),
                                FixVec::new_i64(0, 5)]]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 10),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(-5, 0),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10)],
                        indices: vec![0, 1, 4, 1, 2, 4, 4, 2, 3, 4, 5, 0, 0, 5, 10, 5, 6, 10, 6, 8, 9, 6, 9, 10, 7, 8, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, 10),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-5, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(0, -5)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(0, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(5, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(5, 10),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(5, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                46 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-10, 10),
                                FixVec::new_i64(10, 10),
                                FixVec::new_i64(10, -10),
                                FixVec::new_i64(5, -10),
                                FixVec::new_i64(0, -5),
                                FixVec::new_i64(-5, -10),
                                FixVec::new_i64(-10, -10)],
                            vec![
                                FixVec::new_i64(-5, 0),
                                FixVec::new_i64(0, -5),
                                FixVec::new_i64(5, 0),
                                FixVec::new_i64(0, 5)]]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(5, -10),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(-5, 0),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(-5, -10)],
                        indices: vec![0, 1, 8, 0, 8, 10, 8, 1, 7, 1, 2, 7, 7, 2, 6, 10, 8, 9, 5, 6, 4, 4, 6, 3, 6, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-10, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(-5, -10),
                                    FixVec::new_i64(-5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(5, -10),
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                47 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-10, 10),
                                FixVec::new_i64(-5, 10),
                                FixVec::new_i64(0, 5),
                                FixVec::new_i64(5, 10),
                                FixVec::new_i64(10, 10),
                                FixVec::new_i64(10, -10),
                                FixVec::new_i64(5, -10),
                                FixVec::new_i64(0, -5),
                                FixVec::new_i64(-5, -10),
                                FixVec::new_i64(-10, -10)],
                            vec![
                                FixVec::new_i64(-5, 0),
                                FixVec::new_i64(0, -5),
                                FixVec::new_i64(5, 0),
                                FixVec::new_i64(0, 5)]]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 10),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(-5, 0),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(-5, -10),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(5, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(5, -10),
                            FixVec::new_i64(0, -5)],
                        indices: vec![0, 1, 4, 0, 4, 6, 1, 2, 4, 4, 2, 3, 6, 4, 5, 13, 7, 12, 10, 7, 9, 7, 11, 12, 10, 11, 7, 8, 9, 7],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, 10),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-5, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-5, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(-5, -10),
                                    FixVec::new_i64(-5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(5, -10),
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(5, 10),
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(10, -10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                48 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10)],
                        indices: vec![0, 1, 3, 1, 2, 3, 3, 4, 0, 0, 4, 6, 4, 5, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                49 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(5, 0),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(10, -10)],
                        indices: vec![0, 1, 5, 5, 1, 4, 1, 2, 4, 4, 2, 3, 0, 5, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(5, 0),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(0, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                50 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(-5, 0),
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(0, -5),
                            FixVec::new_i64(-5, 0)],
                        indices: vec![0, 1, 6, 6, 1, 2, 6, 2, 5, 4, 5, 3, 5, 2, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(0, -5),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(0, -5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                51 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 0),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10),
                            FixVec::new_i64(-10, -10)]),
                        points: vec![
                            FixVec::new_i64(-10, -10),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(-5, 0),
                            FixVec::new_i64(0, 5),
                            FixVec::new_i64(-10, 10),
                            FixVec::new_i64(10, 10),
                            FixVec::new_i64(10, -10)],
                        indices: vec![1, 2, 0, 2, 3, 6, 2, 6, 0, 3, 5, 6, 4, 5, 3],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(-5, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10, 10),
                                    FixVec::new_i64(10, -10),
                                    FixVec::new_i64(-10, -10),
                                    FixVec::new_i64(-5, 0),
                                    FixVec::new_i64(0, 5)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 5),
                                    FixVec::new_i64(-10, 10),
                                    FixVec::new_i64(10, 10)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                52 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(10240, 10240),
                            FixVec::new_i64(10240, -5120),
                            FixVec::new_i64(1024, 0),
                            FixVec::new_i64(5120, -5120),
                            FixVec::new_i64(3072, -5120),
                            FixVec::new_i64(5120, -10240),
                            FixVec::new_i64(-10240, -10240)]),
                        points: vec![
                            FixVec::new_i64(10240, 10240),
                            FixVec::new_i64(10240, -5120),
                            FixVec::new_i64(1024, 0),
                            FixVec::new_i64(5120, -5120),
                            FixVec::new_i64(3072, -5120),
                            FixVec::new_i64(5120, -10240),
                            FixVec::new_i64(-10240, -10240)],
                        indices: vec![6, 0, 2, 2, 0, 1, 2, 4, 6, 6, 4, 5, 2, 3, 4],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(1024, 0),
                                    FixVec::new_i64(-10240, -10240),
                                    FixVec::new_i64(10240, 10240)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10240, -5120),
                                    FixVec::new_i64(1024, 0),
                                    FixVec::new_i64(10240, 10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, -10240),
                                    FixVec::new_i64(-10240, -10240),
                                    FixVec::new_i64(1024, 0),
                                    FixVec::new_i64(3072, -5120)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(3072, -5120),
                                    FixVec::new_i64(1024, 0),
                                    FixVec::new_i64(5120, -5120)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                53 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(10240, 8192),
                            FixVec::new_i64(15360, -1024),
                            FixVec::new_i64(6144, 3072),
                            FixVec::new_i64(10240, 0),
                            FixVec::new_i64(8192, 0),
                            FixVec::new_i64(9216, -2048),
                            FixVec::new_i64(-5120, -2048)]),
                        points: vec![
                            FixVec::new_i64(10240, 8192),
                            FixVec::new_i64(15360, -1024),
                            FixVec::new_i64(6144, 3072),
                            FixVec::new_i64(10240, 0),
                            FixVec::new_i64(8192, 0),
                            FixVec::new_i64(9216, -2048),
                            FixVec::new_i64(-5120, -2048)],
                        indices: vec![6, 0, 2, 2, 0, 1, 2, 4, 6, 6, 4, 5, 2, 3, 4],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(6144, 3072),
                                    FixVec::new_i64(-5120, -2048),
                                    FixVec::new_i64(10240, 8192)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15360, -1024),
                                    FixVec::new_i64(6144, 3072),
                                    FixVec::new_i64(10240, 8192)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(9216, -2048),
                                    FixVec::new_i64(-5120, -2048),
                                    FixVec::new_i64(6144, 3072),
                                    FixVec::new_i64(8192, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(8192, 0),
                                    FixVec::new_i64(6144, 3072),
                                    FixVec::new_i64(10240, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                54 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(10854, 8499),
                            FixVec::new_i64(10035, 5529),
                            FixVec::new_i64(14540, 3481),
                            FixVec::new_i64(13312, 0),
                            FixVec::new_i64(14540, 1228),
                            FixVec::new_i64(15564, -1228),
                            FixVec::new_i64(6348, 2457),
                            FixVec::new_i64(10854, -512),
                            FixVec::new_i64(8089, 1024),
                            FixVec::new_i64(8806, -2048),
                            FixVec::new_i64(4300, -1740),
                            FixVec::new_i64(4300, -5222),
                            FixVec::new_i64(-921, -5734),
                            FixVec::new_i64(0, -3993),
                            FixVec::new_i64(-3686, -4300),
                            FixVec::new_i64(-2150, -1536),
                            FixVec::new_i64(-5120, -1740),
                            FixVec::new_i64(-2867, -307),
                            FixVec::new_i64(-3686, 2048),
                            FixVec::new_i64(-10956, 4300),
                            FixVec::new_i64(-12185, 1024),
                            FixVec::new_i64(-14438, 3993),
                            FixVec::new_i64(-11673, 3481),
                            FixVec::new_i64(-11161, 6758),
                            FixVec::new_i64(-6451, 7475),
                            FixVec::new_i64(-1228, 3788),
                            FixVec::new_i64(8294, 4300)]),
                        points: vec![
                            FixVec::new_i64(-11673, 3481),
                            FixVec::new_i64(-11161, 6758),
                            FixVec::new_i64(-6451, 7475),
                            FixVec::new_i64(-1228, 3788),
                            FixVec::new_i64(8294, 4300),
                            FixVec::new_i64(10854, 8499),
                            FixVec::new_i64(10035, 5529),
                            FixVec::new_i64(14540, 3481),
                            FixVec::new_i64(13312, 0),
                            FixVec::new_i64(14540, 1228),
                            FixVec::new_i64(15564, -1228),
                            FixVec::new_i64(6348, 2457),
                            FixVec::new_i64(10854, -512),
                            FixVec::new_i64(8089, 1024),
                            FixVec::new_i64(8806, -2048),
                            FixVec::new_i64(4300, -1740),
                            FixVec::new_i64(4300, -5222),
                            FixVec::new_i64(-921, -5734),
                            FixVec::new_i64(0, -3993),
                            FixVec::new_i64(-3686, -4300),
                            FixVec::new_i64(-2150, -1536),
                            FixVec::new_i64(-5120, -1740),
                            FixVec::new_i64(-2867, -307),
                            FixVec::new_i64(-3686, 2048),
                            FixVec::new_i64(-10956, 4300),
                            FixVec::new_i64(-12185, 1024),
                            FixVec::new_i64(-14438, 3993)],
                        indices: vec![26, 0, 25, 25, 0, 24, 0, 1, 24, 1, 2, 24, 24, 2, 23, 22, 23, 3, 2, 3, 23, 22, 3, 20, 21, 22, 20, 19, 20, 18, 15, 20, 3, 15, 18, 20, 18, 15, 16, 15, 3, 11, 3, 4, 11, 4, 6, 8, 4, 8, 11, 11, 8, 10, 8, 9, 10, 17, 18, 16, 11, 13, 15, 15, 13, 14, 11, 12, 13, 4, 5, 6, 6, 7, 8],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-12185, 1024),
                                    FixVec::new_i64(-14438, 3993),
                                    FixVec::new_i64(-11673, 3481)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10956, 4300),
                                    FixVec::new_i64(-12185, 1024),
                                    FixVec::new_i64(-11673, 3481)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10956, 4300),
                                    FixVec::new_i64(-11673, 3481),
                                    FixVec::new_i64(-11161, 6758)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-1228, 3788),
                                    FixVec::new_i64(-3686, 2048),
                                    FixVec::new_i64(-10956, 4300),
                                    FixVec::new_i64(-11161, 6758),
                                    FixVec::new_i64(-6451, 7475)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(4300, -5222),
                                    FixVec::new_i64(0, -3993),
                                    FixVec::new_i64(-2150, -1536),
                                    FixVec::new_i64(-2867, -307),
                                    FixVec::new_i64(-3686, 2048),
                                    FixVec::new_i64(-1228, 3788),
                                    FixVec::new_i64(4300, -1740)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-2150, -1536),
                                    FixVec::new_i64(-5120, -1740),
                                    FixVec::new_i64(-2867, -307)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -3993),
                                    FixVec::new_i64(-3686, -4300),
                                    FixVec::new_i64(-2150, -1536)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(8806, -2048),
                                    FixVec::new_i64(4300, -1740),
                                    FixVec::new_i64(-1228, 3788),
                                    FixVec::new_i64(6348, 2457),
                                    FixVec::new_i64(8089, 1024)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(6348, 2457),
                                    FixVec::new_i64(-1228, 3788),
                                    FixVec::new_i64(8294, 4300)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(14540, 3481),
                                    FixVec::new_i64(13312, 0),
                                    FixVec::new_i64(6348, 2457),
                                    FixVec::new_i64(8294, 4300),
                                    FixVec::new_i64(10035, 5529)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15564, -1228),
                                    FixVec::new_i64(6348, 2457),
                                    FixVec::new_i64(13312, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15564, -1228),
                                    FixVec::new_i64(13312, 0),
                                    FixVec::new_i64(14540, 1228)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(4300, -5222),
                                    FixVec::new_i64(-921, -5734),
                                    FixVec::new_i64(0, -3993)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(8089, 1024),
                                    FixVec::new_i64(6348, 2457),
                                    FixVec::new_i64(10854, -512)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10035, 5529),
                                    FixVec::new_i64(8294, 4300),
                                    FixVec::new_i64(10854, 8499)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                55 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-30, 0),
                                FixVec::new_i64(-30, 30),
                                FixVec::new_i64(0, 30),
                                FixVec::new_i64(30, 30),
                                FixVec::new_i64(15, 0),
                                FixVec::new_i64(30, -30),
                                FixVec::new_i64(0, -30),
                                FixVec::new_i64(-30, -30)],
                            vec![
                                FixVec::new_i64(-15, -15),
                                FixVec::new_i64(18, -15),
                                FixVec::new_i64(10, 15),
                                FixVec::new_i64(-15, 15)]]),
                        points: vec![
                            FixVec::new_i64(-30, -30),
                            FixVec::new_i64(-30, 30),
                            FixVec::new_i64(30, 30),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(30, -30),
                            FixVec::new_i64(18, -15),
                            FixVec::new_i64(10, 15),
                            FixVec::new_i64(-15, 15),
                            FixVec::new_i64(-15, -15)],
                        indices: vec![0, 1, 8, 8, 1, 7, 7, 1, 6, 1, 2, 6, 6, 2, 3, 8, 5, 4, 8, 4, 0, 6, 3, 5, 3, 4, 5],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-15, 15),
                                    FixVec::new_i64(-15, -15),
                                    FixVec::new_i64(-30, -30),
                                    FixVec::new_i64(-30, 30)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, 30),
                                    FixVec::new_i64(10, 15),
                                    FixVec::new_i64(-15, 15),
                                    FixVec::new_i64(-30, 30)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(10, 15),
                                    FixVec::new_i64(30, 30)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-30, -30),
                                    FixVec::new_i64(-15, -15),
                                    FixVec::new_i64(18, -15),
                                    FixVec::new_i64(30, -30)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(18, -15),
                                    FixVec::new_i64(10, 15),
                                    FixVec::new_i64(15, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(18, -15),
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(30, -30)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                56 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-15, 0),
                                FixVec::new_i64(-30, 30),
                                FixVec::new_i64(30, 30),
                                FixVec::new_i64(30, -30),
                                FixVec::new_i64(0, -30),
                                FixVec::new_i64(-30, -30)],
                            vec![
                                FixVec::new_i64(10, -8),
                                FixVec::new_i64(-10, -13),
                                FixVec::new_i64(19, -11),
                                FixVec::new_i64(-12, 15),
                                FixVec::new_i64(19, 18),
                                FixVec::new_i64(-20, 18)]]),
                        points: vec![
                            FixVec::new_i64(-30, -30),
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-30, 30),
                            FixVec::new_i64(30, 30),
                            FixVec::new_i64(30, -30),
                            FixVec::new_i64(-20, 18),
                            FixVec::new_i64(10, -8),
                            FixVec::new_i64(-10, -13),
                            FixVec::new_i64(19, -11),
                            FixVec::new_i64(-12, 15),
                            FixVec::new_i64(19, 18)],
                        indices: vec![1, 7, 0, 7, 8, 4, 7, 4, 0, 8, 10, 3, 8, 3, 4, 5, 2, 10, 2, 3, 10, 2, 5, 1, 1, 5, 6, 1, 6, 7, 9, 10, 8],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-30, -30),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-10, -13)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-30, -30),
                                    FixVec::new_i64(-10, -13),
                                    FixVec::new_i64(19, -11),
                                    FixVec::new_i64(30, -30)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, -30),
                                    FixVec::new_i64(19, -11),
                                    FixVec::new_i64(19, 18),
                                    FixVec::new_i64(30, 30)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, 30),
                                    FixVec::new_i64(19, 18),
                                    FixVec::new_i64(-20, 18),
                                    FixVec::new_i64(-30, 30)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-30, 30),
                                    FixVec::new_i64(-20, 18)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, -13),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-20, 18),
                                    FixVec::new_i64(10, -8)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(19, -11),
                                    FixVec::new_i64(-12, 15),
                                    FixVec::new_i64(19, 18)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                57 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-15, 0),
                                FixVec::new_i64(-30, 30),
                                FixVec::new_i64(0, 15),
                                FixVec::new_i64(30, 30),
                                FixVec::new_i64(15, 0),
                                FixVec::new_i64(30, -30),
                                FixVec::new_i64(0, -15),
                                FixVec::new_i64(-30, -30)],
                            vec![
                                FixVec::new_i64(-10, 0),
                                FixVec::new_i64(0, -10),
                                FixVec::new_i64(10, 0),
                                FixVec::new_i64(0, 10)]]),
                        points: vec![
                            FixVec::new_i64(-30, -30),
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-30, 30),
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(30, 30),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(30, -30),
                            FixVec::new_i64(0, -15),
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(0, -10),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(0, 10)],
                        indices: vec![1, 8, 7, 1, 7, 0, 8, 9, 7, 9, 10, 7, 10, 5, 7, 7, 5, 6, 3, 1, 2, 1, 11, 8, 3, 11, 1, 11, 3, 10, 10, 3, 5, 3, 4, 5],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -10),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(-30, -30),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-10, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, -30),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(0, -10),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(15, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-30, 30),
                                    FixVec::new_i64(0, 15),
                                    FixVec::new_i64(0, 10)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, 30),
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(0, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                58 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-15, 0),
                                FixVec::new_i64(-30, 30),
                                FixVec::new_i64(0, 15),
                                FixVec::new_i64(30, 30),
                                FixVec::new_i64(15, 0),
                                FixVec::new_i64(30, -30),
                                FixVec::new_i64(0, -15),
                                FixVec::new_i64(-30, -30)],
                            vec![
                                FixVec::new_i64(-10, 0),
                                FixVec::new_i64(-20, -20),
                                FixVec::new_i64(0, -10),
                                FixVec::new_i64(20, -20),
                                FixVec::new_i64(10, 0),
                                FixVec::new_i64(20, 20),
                                FixVec::new_i64(0, 10),
                                FixVec::new_i64(-20, 20)]]),
                        points: vec![
                            FixVec::new_i64(-30, -30),
                            FixVec::new_i64(-15, 0),
                            FixVec::new_i64(-30, 30),
                            FixVec::new_i64(0, 15),
                            FixVec::new_i64(30, 30),
                            FixVec::new_i64(15, 0),
                            FixVec::new_i64(30, -30),
                            FixVec::new_i64(0, -15),
                            FixVec::new_i64(0, -10),
                            FixVec::new_i64(20, -20),
                            FixVec::new_i64(10, 0),
                            FixVec::new_i64(20, 20),
                            FixVec::new_i64(0, 10),
                            FixVec::new_i64(-20, 20),
                            FixVec::new_i64(-10, 0),
                            FixVec::new_i64(-20, -20)],
                        indices: vec![0, 1, 15, 15, 1, 14, 13, 2, 3, 13, 3, 12, 12, 3, 11, 3, 4, 11, 0, 15, 7, 15, 8, 7, 8, 9, 7, 7, 9, 6, 2, 13, 1, 13, 14, 1, 10, 11, 5, 5, 11, 4, 10, 5, 9, 5, 6, 9],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(-20, -20),
                                    FixVec::new_i64(-30, -30),
                                    FixVec::new_i64(-15, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(-20, 20),
                                    FixVec::new_i64(-30, 30),
                                    FixVec::new_i64(0, 15)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, 30),
                                    FixVec::new_i64(20, 20),
                                    FixVec::new_i64(0, 10),
                                    FixVec::new_i64(0, 15)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -10),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(-30, -30),
                                    FixVec::new_i64(-20, -20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, -30),
                                    FixVec::new_i64(0, -15),
                                    FixVec::new_i64(0, -10),
                                    FixVec::new_i64(20, -20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10, 0),
                                    FixVec::new_i64(-15, 0),
                                    FixVec::new_i64(-30, 30),
                                    FixVec::new_i64(-20, 20)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, 30),
                                    FixVec::new_i64(15, 0),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(20, 20)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30, -30),
                                    FixVec::new_i64(20, -20),
                                    FixVec::new_i64(10, 0),
                                    FixVec::new_i64(15, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                59 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-20480, -20480),
                                FixVec::new_i64(-20480, 20480),
                                FixVec::new_i64(20480, 20480),
                                FixVec::new_i64(20480, -20480)],
                            vec![
                                FixVec::new_i64(0, 20480),
                                FixVec::new_i64(-10240, 10240),
                                FixVec::new_i64(0, 0),
                                FixVec::new_i64(10240, 10240)]]),
                        points: vec![
                            FixVec::new_i64(-20480, -20480),
                            FixVec::new_i64(-20480, 20480),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(-10240, 10240),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(10240, 10240),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(20480, 20480),
                            FixVec::new_i64(20480, -20480)],
                        indices: vec![1, 3, 0, 3, 4, 0, 4, 5, 8, 4, 8, 0, 5, 7, 8, 1, 2, 3, 6, 7, 5],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20480, -20480),
                                    FixVec::new_i64(-20480, -20480),
                                    FixVec::new_i64(-20480, 20480),
                                    FixVec::new_i64(-10240, 10240),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20480, 20480),
                                    FixVec::new_i64(20480, -20480),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(10240, 10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10240, 10240),
                                    FixVec::new_i64(-20480, 20480),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10240, 10240),
                                    FixVec::new_i64(0, 20480),
                                    FixVec::new_i64(20480, 20480)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                60 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-20480, -20480),
                                FixVec::new_i64(-20480, 20480),
                                FixVec::new_i64(20480, 20480),
                                FixVec::new_i64(20480, -20480)],
                            vec![
                                FixVec::new_i64(0, 20480),
                                FixVec::new_i64(-10240, 10240),
                                FixVec::new_i64(10240, -10240),
                                FixVec::new_i64(-10240, -10240),
                                FixVec::new_i64(10240, 10240)]]),
                        points: vec![
                            FixVec::new_i64(-20480, -20480),
                            FixVec::new_i64(-20480, 20480),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(-10240, 10240),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-10240, -10240),
                            FixVec::new_i64(10240, -10240),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(10240, 10240),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(20480, 20480),
                            FixVec::new_i64(20480, -20480)],
                        indices: vec![0, 1, 5, 5, 1, 3, 1, 2, 3, 5, 6, 0, 0, 6, 11, 6, 8, 11, 8, 10, 11, 5, 3, 4, 7, 8, 6, 9, 10, 8],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-10240, -10240),
                                    FixVec::new_i64(-20480, -20480),
                                    FixVec::new_i64(-20480, 20480),
                                    FixVec::new_i64(-10240, 10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10240, 10240),
                                    FixVec::new_i64(-20480, 20480),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20480, -20480),
                                    FixVec::new_i64(-20480, -20480),
                                    FixVec::new_i64(-10240, -10240),
                                    FixVec::new_i64(10240, -10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(10240, 10240),
                                    FixVec::new_i64(20480, 20480),
                                    FixVec::new_i64(20480, -20480),
                                    FixVec::new_i64(10240, -10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10240, 10240),
                                    FixVec::new_i64(0, 20480),
                                    FixVec::new_i64(20480, 20480)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                61 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(-20480, -20480),
                                FixVec::new_i64(-20480, 20480),
                                FixVec::new_i64(20480, 20480),
                                FixVec::new_i64(20480, -20480)],
                            vec![
                                FixVec::new_i64(0, 20480),
                                FixVec::new_i64(-10240, 10240),
                                FixVec::new_i64(10240, 10240),
                                FixVec::new_i64(0, 0)]]),
                        points: vec![
                            FixVec::new_i64(-20480, -20480),
                            FixVec::new_i64(-20480, 20480),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(-10240, 10240),
                            FixVec::new_i64(0, 10240),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(10240, 10240),
                            FixVec::new_i64(0, 10240),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(20480, 20480),
                            FixVec::new_i64(20480, -20480)],
                        indices: vec![1, 3, 0, 3, 5, 0, 5, 6, 10, 5, 10, 0, 6, 9, 10, 1, 2, 3, 3, 4, 5, 7, 8, 6, 8, 9, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20480, -20480),
                                    FixVec::new_i64(-20480, -20480),
                                    FixVec::new_i64(-20480, 20480),
                                    FixVec::new_i64(-10240, 10240),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20480, 20480),
                                    FixVec::new_i64(20480, -20480),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(10240, 10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10240, 10240),
                                    FixVec::new_i64(-20480, 20480),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-10240, 10240),
                                    FixVec::new_i64(0, 10240)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(20480, 20480),
                                    FixVec::new_i64(10240, 10240),
                                    FixVec::new_i64(0, 10240),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                62 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(0, 20480),
                                FixVec::new_i64(20480, -20480),
                                FixVec::new_i64(-20480, -20480)],
                            vec![
                                FixVec::new_i64(0, 0),
                                FixVec::new_i64(10240, 0),
                                FixVec::new_i64(0, -10240),
                                FixVec::new_i64(10240, -10240)]]),
                        points: vec![
                            FixVec::new_i64(-20480, -20480),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(10240, 0),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5120, -5120),
                            FixVec::new_i64(0, -10240),
                            FixVec::new_i64(10240, -10240),
                            FixVec::new_i64(5120, -5120),
                            FixVec::new_i64(10240, 0),
                            FixVec::new_i64(20480, -20480)],
                        indices: vec![5, 0, 3, 0, 1, 3, 3, 1, 2, 5, 6, 9, 5, 9, 0, 6, 8, 9, 5, 3, 4, 7, 8, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, -5120),
                                    FixVec::new_i64(0, -10240),
                                    FixVec::new_i64(-20480, -20480),
                                    FixVec::new_i64(0, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-20480, -20480),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10240, 0),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-20480, -20480),
                                    FixVec::new_i64(0, -10240),
                                    FixVec::new_i64(10240, -10240),
                                    FixVec::new_i64(20480, -20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, -5120),
                                    FixVec::new_i64(10240, 0),
                                    FixVec::new_i64(20480, -20480),
                                    FixVec::new_i64(10240, -10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                63 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(0, 20480),
                                FixVec::new_i64(9216, 1024),
                                FixVec::new_i64(14336, -15360),
                                FixVec::new_i64(-14336, -15360)],
                            vec![
                                FixVec::new_i64(-4096, -5120),
                                FixVec::new_i64(9216, 1024),
                                FixVec::new_i64(2048, -11264),
                                FixVec::new_i64(5120, -9216)]]),
                        points: vec![
                            FixVec::new_i64(-14336, -15360),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(9216, 1024),
                            FixVec::new_i64(-4096, -5120),
                            FixVec::new_i64(3629, -8554),
                            FixVec::new_i64(2048, -11264),
                            FixVec::new_i64(5120, -9216),
                            FixVec::new_i64(3629, -8554),
                            FixVec::new_i64(9216, 1024),
                            FixVec::new_i64(14336, -15360)],
                        indices: vec![0, 1, 3, 3, 1, 2, 3, 5, 0, 0, 5, 9, 5, 6, 9, 6, 8, 9, 3, 4, 5, 7, 8, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-4096, -5120),
                                    FixVec::new_i64(-14336, -15360),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(9216, 1024),
                                    FixVec::new_i64(-4096, -5120),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(3629, -8554),
                                    FixVec::new_i64(2048, -11264),
                                    FixVec::new_i64(-14336, -15360),
                                    FixVec::new_i64(-4096, -5120)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(14336, -15360),
                                    FixVec::new_i64(-14336, -15360),
                                    FixVec::new_i64(2048, -11264)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(14336, -15360),
                                    FixVec::new_i64(2048, -11264),
                                    FixVec::new_i64(5120, -9216)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(14336, -15360),
                                    FixVec::new_i64(5120, -9216),
                                    FixVec::new_i64(9216, 1024)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, -9216),
                                    FixVec::new_i64(3629, -8554),
                                    FixVec::new_i64(9216, 1024)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            }],
                    }
                }
                64 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(0, 20480),
                                FixVec::new_i64(9216, 1024),
                                FixVec::new_i64(26624, -7168),
                                FixVec::new_i64(14336, -15360),
                                FixVec::new_i64(-14336, -15360),
                                FixVec::new_i64(-25600, -7168)],
                            vec![
                                FixVec::new_i64(-4096, -5120),
                                FixVec::new_i64(9216, 1024),
                                FixVec::new_i64(2048, -11264)]]),
                        points: vec![
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(9216, 1024),
                            FixVec::new_i64(-4096, -5120),
                            FixVec::new_i64(2048, -11264),
                            FixVec::new_i64(9216, 1024),
                            FixVec::new_i64(26624, -7168),
                            FixVec::new_i64(14336, -15360),
                            FixVec::new_i64(-14336, -15360),
                            FixVec::new_i64(-25600, -7168)],
                        indices: vec![7, 8, 2, 8, 0, 2, 2, 0, 1, 2, 3, 7, 7, 3, 6, 3, 4, 6, 4, 5, 6],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(2048, -11264),
                                    FixVec::new_i64(-14336, -15360),
                                    FixVec::new_i64(-25600, -7168),
                                    FixVec::new_i64(-4096, -5120)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(9216, 1024),
                                    FixVec::new_i64(-4096, -5120),
                                    FixVec::new_i64(-25600, -7168),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(14336, -15360),
                                    FixVec::new_i64(-14336, -15360),
                                    FixVec::new_i64(2048, -11264)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(26624, -7168),
                                    FixVec::new_i64(14336, -15360),
                                    FixVec::new_i64(2048, -11264),
                                    FixVec::new_i64(9216, 1024)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                65 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(0, 20480),
                                FixVec::new_i64(14336, -15360),
                                FixVec::new_i64(-14336, -15360),
                                FixVec::new_i64(-18432, 0),
                                FixVec::new_i64(-7168, 6144),
                                FixVec::new_i64(-10240, 8192)],
                            vec![
                                FixVec::new_i64(-2048, -2048),
                                FixVec::new_i64(-9216, 10240),
                                FixVec::new_i64(-2048, -9216)]]),
                        points: vec![
                            FixVec::new_i64(-7618, 5900),
                            FixVec::new_i64(-2048, -9216),
                            FixVec::new_i64(-2048, -2048),
                            FixVec::new_i64(-8936, 9758),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(14336, -15360),
                            FixVec::new_i64(-14336, -15360),
                            FixVec::new_i64(-18432, 0),
                            FixVec::new_i64(-10240, 8192),
                            FixVec::new_i64(-9007, 9672),
                            FixVec::new_i64(-7883, 6621),
                            FixVec::new_i64(-8936, 9758),
                            FixVec::new_i64(-9007, 9672),
                            FixVec::new_i64(-9216, 10240),
                            FixVec::new_i64(-7168, 6144),
                            FixVec::new_i64(-7618, 5900),
                            FixVec::new_i64(-7883, 6621)],
                        indices: vec![7, 0, 1, 7, 1, 6, 2, 4, 5, 6, 1, 5, 2, 5, 1, 3, 4, 2, 8, 9, 10, 13, 11, 12, 16, 14, 15],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-14336, -15360),
                                    FixVec::new_i64(-18432, 0),
                                    FixVec::new_i64(-7618, 5900),
                                    FixVec::new_i64(-2048, -9216)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-2048, -9216),
                                    FixVec::new_i64(-2048, -2048),
                                    FixVec::new_i64(0, 20480),
                                    FixVec::new_i64(14336, -15360)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(14336, -15360),
                                    FixVec::new_i64(-14336, -15360),
                                    FixVec::new_i64(-2048, -9216)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-2048, -2048),
                                    FixVec::new_i64(-8936, 9758),
                                    FixVec::new_i64(0, 20480)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-7883, 6621),
                                    FixVec::new_i64(-10240, 8192),
                                    FixVec::new_i64(-9007, 9672)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-9007, 9672),
                                    FixVec::new_i64(-9216, 10240),
                                    FixVec::new_i64(-8936, 9758)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-7618, 5900),
                                    FixVec::new_i64(-7883, 6621),
                                    FixVec::new_i64(-7168, 6144)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                66 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(0, 20480),
                                FixVec::new_i64(8192, 10240),
                                FixVec::new_i64(7168, 6144),
                                FixVec::new_i64(9216, 1024),
                                FixVec::new_i64(13312, -1024),
                                FixVec::new_i64(17408, 1024),
                                FixVec::new_i64(26624, -7168),
                                FixVec::new_i64(14336, -15360),
                                FixVec::new_i64(0, -18432),
                                FixVec::new_i64(-14336, -15360),
                                FixVec::new_i64(-25600, -7168),
                                FixVec::new_i64(-18432, 0),
                                FixVec::new_i64(-16384, -3072),
                                FixVec::new_i64(-13312, -4096),
                                FixVec::new_i64(-8192, -2048),
                                FixVec::new_i64(-6144, 2048),
                                FixVec::new_i64(-7168, 6144),
                                FixVec::new_i64(-10240, 8192)],
                            vec![
                                FixVec::new_i64(2048, 0),
                                FixVec::new_i64(-2048, -2048),
                                FixVec::new_i64(-9216, 10240),
                                FixVec::new_i64(-2048, -9216),
                                FixVec::new_i64(2048, -11264),
                                FixVec::new_i64(5120, -9216),
                                FixVec::new_i64(7168, -5120),
                                FixVec::new_i64(5120, -2048)]]),
                        points: vec![
                            FixVec::new_i64(-25600, -7168),
                            FixVec::new_i64(-18432, 0),
                            FixVec::new_i64(-16384, -3072),
                            FixVec::new_i64(-13312, -4096),
                            FixVec::new_i64(-8192, -2048),
                            FixVec::new_i64(-6176, 1986),
                            FixVec::new_i64(-2048, -9216),
                            FixVec::new_i64(2048, -11264),
                            FixVec::new_i64(5120, -9216),
                            FixVec::new_i64(7168, -5120),
                            FixVec::new_i64(5120, -2048),
                            FixVec::new_i64(2048, 0),
                            FixVec::new_i64(-2048, -2048),
                            FixVec::new_i64(-8936, 9758),
                            FixVec::new_i64(0, 20480),
                            FixVec::new_i64(8192, 10240),
                            FixVec::new_i64(7168, 6144),
                            FixVec::new_i64(9216, 1024),
                            FixVec::new_i64(13312, -1024),
                            FixVec::new_i64(17408, 1024),
                            FixVec::new_i64(26624, -7168),
                            FixVec::new_i64(14336, -15360),
                            FixVec::new_i64(0, -18432),
                            FixVec::new_i64(-14336, -15360),
                            FixVec::new_i64(-10240, 8192),
                            FixVec::new_i64(-9007, 9672),
                            FixVec::new_i64(-7883, 6621),
                            FixVec::new_i64(-8936, 9758),
                            FixVec::new_i64(-9007, 9672),
                            FixVec::new_i64(-9216, 10240),
                            FixVec::new_i64(-7883, 6621),
                            FixVec::new_i64(-7168, 6144),
                            FixVec::new_i64(-6144, 2048),
                            FixVec::new_i64(-6176, 1986)],
                        indices: vec![1, 2, 0, 0, 2, 23, 2, 3, 23, 3, 4, 6, 4, 5, 6, 3, 6, 23, 23, 6, 22, 6, 7, 22, 7, 8, 21, 17, 9, 10, 17, 18, 9, 21, 9, 18, 21, 8, 9, 7, 21, 22, 18, 19, 20, 18, 20, 21, 13, 14, 15, 13, 11, 12, 16, 11, 13, 17, 11, 16, 17, 10, 11, 13, 15, 16, 24, 25, 26, 29, 27, 28, 30, 31, 33, 31, 32, 33],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-14336, -15360),
                                    FixVec::new_i64(-25600, -7168),
                                    FixVec::new_i64(-18432, 0),
                                    FixVec::new_i64(-16384, -3072)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(2048, -11264),
                                    FixVec::new_i64(0, -18432),
                                    FixVec::new_i64(-14336, -15360),
                                    FixVec::new_i64(-16384, -3072),
                                    FixVec::new_i64(-13312, -4096),
                                    FixVec::new_i64(-2048, -9216)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-2048, -9216),
                                    FixVec::new_i64(-13312, -4096),
                                    FixVec::new_i64(-8192, -2048)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-2048, -9216),
                                    FixVec::new_i64(-8192, -2048),
                                    FixVec::new_i64(-6176, 1986)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -18432),
                                    FixVec::new_i64(2048, -11264),
                                    FixVec::new_i64(5120, -9216),
                                    FixVec::new_i64(14336, -15360)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(14336, -15360),
                                    FixVec::new_i64(7168, -5120),
                                    FixVec::new_i64(5120, -2048),
                                    FixVec::new_i64(9216, 1024),
                                    FixVec::new_i64(13312, -1024)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(7168, -5120),
                                    FixVec::new_i64(14336, -15360),
                                    FixVec::new_i64(5120, -9216)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(14336, -15360),
                                    FixVec::new_i64(13312, -1024),
                                    FixVec::new_i64(17408, 1024),
                                    FixVec::new_i64(26624, -7168)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-2048, -2048),
                                    FixVec::new_i64(-8936, 9758),
                                    FixVec::new_i64(0, 20480),
                                    FixVec::new_i64(8192, 10240),
                                    FixVec::new_i64(7168, 6144),
                                    FixVec::new_i64(2048, 0)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, -2048),
                                    FixVec::new_i64(2048, 0),
                                    FixVec::new_i64(7168, 6144),
                                    FixVec::new_i64(9216, 1024)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-7883, 6621),
                                    FixVec::new_i64(-10240, 8192),
                                    FixVec::new_i64(-9007, 9672)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-9007, 9672),
                                    FixVec::new_i64(-9216, 10240),
                                    FixVec::new_i64(-8936, 9758)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-6144, 2048),
                                    FixVec::new_i64(-6176, 1986),
                                    FixVec::new_i64(-7883, 6621),
                                    FixVec::new_i64(-7168, 6144)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                67 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(32768, 0),
                                FixVec::new_i64(21532, -4283),
                                FixVec::new_i64(30273, -12539),
                                FixVec::new_i64(36236, -24212),
                                FixVec::new_i64(23170, -23170),
                                FixVec::new_i64(12197, -18254),
                                FixVec::new_i64(12539, -30273),
                                FixVec::new_i64(8502, -42744),
                                FixVec::new_i64(0, -32768),
                                FixVec::new_i64(-4283, -21532),
                                FixVec::new_i64(-12539, -30273),
                                FixVec::new_i64(-24212, -36236),
                                FixVec::new_i64(-23170, -23170),
                                FixVec::new_i64(-18254, -12197),
                                FixVec::new_i64(-30273, -12539),
                                FixVec::new_i64(-42744, -8502),
                                FixVec::new_i64(-32768, 0),
                                FixVec::new_i64(-21532, 4283),
                                FixVec::new_i64(-30273, 12539),
                                FixVec::new_i64(-36236, 24212),
                                FixVec::new_i64(-23170, 23170),
                                FixVec::new_i64(-12197, 18254),
                                FixVec::new_i64(-12539, 30273),
                                FixVec::new_i64(-8502, 42744),
                                FixVec::new_i64(0, 32768),
                                FixVec::new_i64(4283, 21532),
                                FixVec::new_i64(12539, 30273),
                                FixVec::new_i64(24212, 36236),
                                FixVec::new_i64(23170, 23170),
                                FixVec::new_i64(18254, 12197),
                                FixVec::new_i64(30273, 12539),
                                FixVec::new_i64(42744, 8502)],
                            vec![
                                FixVec::new_i64(21372, 4251),
                                FixVec::new_i64(15136, 6269),
                                FixVec::new_i64(9127, 6098),
                                FixVec::new_i64(11585, 11585),
                                FixVec::new_i64(12106, 18118),
                                FixVec::new_i64(6269, 15136),
                                FixVec::new_i64(2141, 10766),
                                FixVec::new_i64(0, 16384),
                                FixVec::new_i64(-4251, 21372),
                                FixVec::new_i64(-6269, 15136),
                                FixVec::new_i64(-6098, 9127),
                                FixVec::new_i64(-11585, 11585),
                                FixVec::new_i64(-18118, 12106),
                                FixVec::new_i64(-15136, 6269),
                                FixVec::new_i64(-10766, 2141),
                                FixVec::new_i64(-16384, 0),
                                FixVec::new_i64(-21372, -4251),
                                FixVec::new_i64(-15136, -6269),
                                FixVec::new_i64(-9127, -6098),
                                FixVec::new_i64(-11585, -11585),
                                FixVec::new_i64(-12106, -18118),
                                FixVec::new_i64(-6269, -15136),
                                FixVec::new_i64(-2141, -10766),
                                FixVec::new_i64(0, -16384),
                                FixVec::new_i64(4251, -21372),
                                FixVec::new_i64(6269, -15136),
                                FixVec::new_i64(6098, -9127),
                                FixVec::new_i64(11585, -11585),
                                FixVec::new_i64(18118, -12106),
                                FixVec::new_i64(15136, -6269),
                                FixVec::new_i64(10766, -2141),
                                FixVec::new_i64(16384, 0)]]),
                        points: vec![
                            FixVec::new_i64(-42744, -8502),
                            FixVec::new_i64(-32768, 0),
                            FixVec::new_i64(-21532, 4283),
                            FixVec::new_i64(-30273, 12539),
                            FixVec::new_i64(-36236, 24212),
                            FixVec::new_i64(-23170, 23170),
                            FixVec::new_i64(-12197, 18254),
                            FixVec::new_i64(-12539, 30273),
                            FixVec::new_i64(-8502, 42744),
                            FixVec::new_i64(0, 32768),
                            FixVec::new_i64(4283, 21532),
                            FixVec::new_i64(12539, 30273),
                            FixVec::new_i64(24212, 36236),
                            FixVec::new_i64(23170, 23170),
                            FixVec::new_i64(18254, 12197),
                            FixVec::new_i64(30273, 12539),
                            FixVec::new_i64(42744, 8502),
                            FixVec::new_i64(32768, 0),
                            FixVec::new_i64(21532, -4283),
                            FixVec::new_i64(30273, -12539),
                            FixVec::new_i64(36236, -24212),
                            FixVec::new_i64(23170, -23170),
                            FixVec::new_i64(12197, -18254),
                            FixVec::new_i64(12539, -30273),
                            FixVec::new_i64(8502, -42744),
                            FixVec::new_i64(0, -32768),
                            FixVec::new_i64(-4283, -21532),
                            FixVec::new_i64(-12539, -30273),
                            FixVec::new_i64(-24212, -36236),
                            FixVec::new_i64(-23170, -23170),
                            FixVec::new_i64(-18254, -12197),
                            FixVec::new_i64(-30273, -12539),
                            FixVec::new_i64(-15136, -6269),
                            FixVec::new_i64(-9127, -6098),
                            FixVec::new_i64(-11585, -11585),
                            FixVec::new_i64(-12106, -18118),
                            FixVec::new_i64(-6269, -15136),
                            FixVec::new_i64(-2141, -10766),
                            FixVec::new_i64(0, -16384),
                            FixVec::new_i64(4251, -21372),
                            FixVec::new_i64(6269, -15136),
                            FixVec::new_i64(6098, -9127),
                            FixVec::new_i64(11585, -11585),
                            FixVec::new_i64(18118, -12106),
                            FixVec::new_i64(15136, -6269),
                            FixVec::new_i64(10766, -2141),
                            FixVec::new_i64(16384, 0),
                            FixVec::new_i64(21372, 4251),
                            FixVec::new_i64(15136, 6269),
                            FixVec::new_i64(9127, 6098),
                            FixVec::new_i64(11585, 11585),
                            FixVec::new_i64(12106, 18118),
                            FixVec::new_i64(6269, 15136),
                            FixVec::new_i64(2141, 10766),
                            FixVec::new_i64(0, 16384),
                            FixVec::new_i64(-4251, 21372),
                            FixVec::new_i64(-6269, 15136),
                            FixVec::new_i64(-6098, 9127),
                            FixVec::new_i64(-11585, 11585),
                            FixVec::new_i64(-18118, 12106),
                            FixVec::new_i64(-15136, 6269),
                            FixVec::new_i64(-10766, 2141),
                            FixVec::new_i64(-16384, 0),
                            FixVec::new_i64(-21372, -4251)],
                        indices: vec![0, 1, 31, 1, 2, 63, 1, 63, 31, 31, 63, 30, 63, 32, 30, 4, 5, 3, 59, 3, 5, 60, 2, 59, 59, 2, 3, 5, 6, 59, 59, 6, 58, 35, 29, 30, 34, 30, 32, 29, 27, 28, 35, 27, 29, 34, 35, 30, 32, 33, 34, 2, 62, 63, 60, 62, 2, 62, 60, 61, 9, 7, 8, 55, 6, 7, 56, 58, 6, 58, 56, 57, 35, 36, 26, 35, 26, 27, 38, 36, 37, 38, 26, 36, 39, 26, 38, 39, 25, 26, 23, 25, 39, 39, 40, 22, 55, 56, 6, 9, 55, 7, 10, 55, 9, 10, 54, 55, 52, 54, 10, 52, 53, 54, 52, 10, 51, 10, 11, 51, 40, 41, 42, 22, 40, 42, 39, 22, 23, 23, 24, 25, 49, 50, 48, 50, 51, 14, 13, 51, 11, 50, 14, 48, 13, 14, 51, 11, 12, 13, 45, 46, 44, 44, 46, 18, 46, 47, 18, 42, 43, 22, 44, 18, 43, 22, 43, 21, 43, 18, 19, 43, 19, 21, 21, 19, 20, 48, 14, 47, 14, 15, 47, 47, 15, 17, 47, 17, 18, 15, 16, 17],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-18254, -12197),
                                    FixVec::new_i64(-30273, -12539),
                                    FixVec::new_i64(-42744, -8502),
                                    FixVec::new_i64(-32768, 0),
                                    FixVec::new_i64(-21372, -4251)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-15136, 6269),
                                    FixVec::new_i64(-16384, 0),
                                    FixVec::new_i64(-21372, -4251),
                                    FixVec::new_i64(-32768, 0),
                                    FixVec::new_i64(-21532, 4283)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-12106, -18118),
                                    FixVec::new_i64(-18254, -12197),
                                    FixVec::new_i64(-21372, -4251),
                                    FixVec::new_i64(-15136, -6269),
                                    FixVec::new_i64(-11585, -11585)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-21532, 4283),
                                    FixVec::new_i64(-30273, 12539),
                                    FixVec::new_i64(-36236, 24212),
                                    FixVec::new_i64(-23170, 23170),
                                    FixVec::new_i64(-18118, 12106)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-18118, 12106),
                                    FixVec::new_i64(-15136, 6269),
                                    FixVec::new_i64(-21532, 4283)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-6269, 15136),
                                    FixVec::new_i64(-11585, 11585),
                                    FixVec::new_i64(-18118, 12106),
                                    FixVec::new_i64(-23170, 23170),
                                    FixVec::new_i64(-12197, 18254)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-24212, -36236),
                                    FixVec::new_i64(-23170, -23170),
                                    FixVec::new_i64(-18254, -12197),
                                    FixVec::new_i64(-12106, -18118),
                                    FixVec::new_i64(-12539, -30273)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-11585, -11585),
                                    FixVec::new_i64(-15136, -6269),
                                    FixVec::new_i64(-9127, -6098)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10766, 2141),
                                    FixVec::new_i64(-16384, 0),
                                    FixVec::new_i64(-15136, 6269)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-12197, 18254),
                                    FixVec::new_i64(-12539, 30273),
                                    FixVec::new_i64(-8502, 42744),
                                    FixVec::new_i64(0, 32768),
                                    FixVec::new_i64(-4251, 21372)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-6098, 9127),
                                    FixVec::new_i64(-11585, 11585),
                                    FixVec::new_i64(-6269, 15136)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -16384),
                                    FixVec::new_i64(-4283, -21532),
                                    FixVec::new_i64(-12539, -30273),
                                    FixVec::new_i64(-12106, -18118),
                                    FixVec::new_i64(-6269, -15136)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-2141, -10766),
                                    FixVec::new_i64(0, -16384),
                                    FixVec::new_i64(-6269, -15136)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -32768),
                                    FixVec::new_i64(-4283, -21532),
                                    FixVec::new_i64(0, -16384),
                                    FixVec::new_i64(4251, -21372)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(8502, -42744),
                                    FixVec::new_i64(0, -32768),
                                    FixVec::new_i64(4251, -21372),
                                    FixVec::new_i64(12197, -18254),
                                    FixVec::new_i64(12539, -30273)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(18118, -12106),
                                    FixVec::new_i64(12197, -18254),
                                    FixVec::new_i64(4251, -21372),
                                    FixVec::new_i64(6269, -15136),
                                    FixVec::new_i64(11585, -11585)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-12197, 18254),
                                    FixVec::new_i64(-4251, 21372),
                                    FixVec::new_i64(-6269, 15136)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(6269, 15136),
                                    FixVec::new_i64(0, 16384),
                                    FixVec::new_i64(-4251, 21372),
                                    FixVec::new_i64(0, 32768),
                                    FixVec::new_i64(4283, 21532)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, 16384),
                                    FixVec::new_i64(6269, 15136),
                                    FixVec::new_i64(2141, 10766)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(12539, 30273),
                                    FixVec::new_i64(12106, 18118),
                                    FixVec::new_i64(6269, 15136),
                                    FixVec::new_i64(4283, 21532)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(11585, -11585),
                                    FixVec::new_i64(6269, -15136),
                                    FixVec::new_i64(6098, -9127)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(18254, 12197),
                                    FixVec::new_i64(15136, 6269),
                                    FixVec::new_i64(9127, 6098),
                                    FixVec::new_i64(11585, 11585)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(23170, 23170),
                                    FixVec::new_i64(18254, 12197),
                                    FixVec::new_i64(11585, 11585),
                                    FixVec::new_i64(12106, 18118)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(24212, 36236),
                                    FixVec::new_i64(23170, 23170),
                                    FixVec::new_i64(12106, 18118),
                                    FixVec::new_i64(12539, 30273)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(21532, -4283),
                                    FixVec::new_i64(15136, -6269),
                                    FixVec::new_i64(10766, -2141),
                                    FixVec::new_i64(16384, 0)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(32768, 0),
                                    FixVec::new_i64(21532, -4283),
                                    FixVec::new_i64(16384, 0),
                                    FixVec::new_i64(21372, 4251)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30273, -12539),
                                    FixVec::new_i64(18118, -12106),
                                    FixVec::new_i64(15136, -6269),
                                    FixVec::new_i64(21532, -4283)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(36236, -24212),
                                    FixVec::new_i64(23170, -23170),
                                    FixVec::new_i64(12197, -18254),
                                    FixVec::new_i64(18118, -12106),
                                    FixVec::new_i64(30273, -12539)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30273, 12539),
                                    FixVec::new_i64(21372, 4251),
                                    FixVec::new_i64(15136, 6269),
                                    FixVec::new_i64(18254, 12197)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(42744, 8502),
                                    FixVec::new_i64(32768, 0),
                                    FixVec::new_i64(21372, 4251),
                                    FixVec::new_i64(30273, 12539)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                68 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(32768, 0),
                                FixVec::new_i64(17033, -3388),
                                FixVec::new_i64(30273, -12539),
                                FixVec::new_i64(40051, -26761),
                                FixVec::new_i64(23170, -23170),
                                FixVec::new_i64(9648, -14440),
                                FixVec::new_i64(12539, -30273),
                                FixVec::new_i64(9397, -47243),
                                FixVec::new_i64(0, -32768),
                                FixVec::new_i64(-3388, -17033),
                                FixVec::new_i64(-12539, -30273),
                                FixVec::new_i64(-26761, -40051),
                                FixVec::new_i64(-23170, -23170),
                                FixVec::new_i64(-14440, -9648),
                                FixVec::new_i64(-30273, -12539),
                                FixVec::new_i64(-47243, -9397),
                                FixVec::new_i64(-32768, 0),
                                FixVec::new_i64(-17033, 3388),
                                FixVec::new_i64(-30273, 12539),
                                FixVec::new_i64(-40051, 26761),
                                FixVec::new_i64(-23170, 23170),
                                FixVec::new_i64(-9648, 14440),
                                FixVec::new_i64(-12539, 30273),
                                FixVec::new_i64(-9397, 47243),
                                FixVec::new_i64(0, 32768),
                                FixVec::new_i64(3388, 17033),
                                FixVec::new_i64(12539, 30273),
                                FixVec::new_i64(26761, 40050),
                                FixVec::new_i64(23170, 23170),
                                FixVec::new_i64(14440, 9648),
                                FixVec::new_i64(30273, 12539),
                                FixVec::new_i64(47243, 9397)],
                            vec![
                                FixVec::new_i64(23621, 4698),
                                FixVec::new_i64(15136, 6269),
                                FixVec::new_i64(7220, 4824),
                                FixVec::new_i64(11585, 11585),
                                FixVec::new_i64(13380, 20025),
                                FixVec::new_i64(6269, 15136),
                                FixVec::new_i64(1694, 8516),
                                FixVec::new_i64(0, 16384),
                                FixVec::new_i64(-4698, 23621),
                                FixVec::new_i64(-6269, 15136),
                                FixVec::new_i64(-4824, 7220),
                                FixVec::new_i64(-11585, 11585),
                                FixVec::new_i64(-20025, 13380),
                                FixVec::new_i64(-15136, 6269),
                                FixVec::new_i64(-8516, 1694),
                                FixVec::new_i64(-16384, 0),
                                FixVec::new_i64(-23621, -4698),
                                FixVec::new_i64(-15136, -6269),
                                FixVec::new_i64(-7220, -4824),
                                FixVec::new_i64(-11585, -11585),
                                FixVec::new_i64(-13380, -20025),
                                FixVec::new_i64(-6269, -15136),
                                FixVec::new_i64(-1694, -8516),
                                FixVec::new_i64(0, -16384),
                                FixVec::new_i64(4698, -23621),
                                FixVec::new_i64(6269, -15136),
                                FixVec::new_i64(4824, -7220),
                                FixVec::new_i64(11585, -11585),
                                FixVec::new_i64(20025, -13380),
                                FixVec::new_i64(15136, -6269),
                                FixVec::new_i64(8516, -1694),
                                FixVec::new_i64(16384, 0)]]),
                        points: vec![
                            FixVec::new_i64(-47243, -9397),
                            FixVec::new_i64(-32768, 0),
                            FixVec::new_i64(-17033, 3388),
                            FixVec::new_i64(-30273, 12539),
                            FixVec::new_i64(-40051, 26761),
                            FixVec::new_i64(-23170, 23170),
                            FixVec::new_i64(-9648, 14440),
                            FixVec::new_i64(-12539, 30273),
                            FixVec::new_i64(-9397, 47243),
                            FixVec::new_i64(0, 32768),
                            FixVec::new_i64(3388, 17033),
                            FixVec::new_i64(12539, 30273),
                            FixVec::new_i64(26761, 40050),
                            FixVec::new_i64(23170, 23170),
                            FixVec::new_i64(14440, 9648),
                            FixVec::new_i64(30273, 12539),
                            FixVec::new_i64(47243, 9397),
                            FixVec::new_i64(32768, 0),
                            FixVec::new_i64(17033, -3388),
                            FixVec::new_i64(30273, -12539),
                            FixVec::new_i64(40051, -26761),
                            FixVec::new_i64(23170, -23170),
                            FixVec::new_i64(9648, -14440),
                            FixVec::new_i64(12539, -30273),
                            FixVec::new_i64(9397, -47243),
                            FixVec::new_i64(0, -32768),
                            FixVec::new_i64(-3388, -17033),
                            FixVec::new_i64(-12539, -30273),
                            FixVec::new_i64(-26761, -40051),
                            FixVec::new_i64(-23170, -23170),
                            FixVec::new_i64(-14440, -9648),
                            FixVec::new_i64(-30273, -12539),
                            FixVec::new_i64(-15136, -6269),
                            FixVec::new_i64(-7220, -4824),
                            FixVec::new_i64(-11585, -11585),
                            FixVec::new_i64(-13380, -20025),
                            FixVec::new_i64(-6269, -15136),
                            FixVec::new_i64(-1694, -8516),
                            FixVec::new_i64(0, -16384),
                            FixVec::new_i64(4698, -23621),
                            FixVec::new_i64(6269, -15136),
                            FixVec::new_i64(4824, -7220),
                            FixVec::new_i64(11585, -11585),
                            FixVec::new_i64(20025, -13380),
                            FixVec::new_i64(15136, -6269),
                            FixVec::new_i64(8516, -1694),
                            FixVec::new_i64(16384, 0),
                            FixVec::new_i64(23621, 4698),
                            FixVec::new_i64(15136, 6269),
                            FixVec::new_i64(7220, 4824),
                            FixVec::new_i64(11585, 11585),
                            FixVec::new_i64(13380, 20025),
                            FixVec::new_i64(6269, 15136),
                            FixVec::new_i64(1694, 8516),
                            FixVec::new_i64(0, 16384),
                            FixVec::new_i64(-4698, 23621),
                            FixVec::new_i64(-6269, 15136),
                            FixVec::new_i64(-4824, 7220),
                            FixVec::new_i64(-11585, 11585),
                            FixVec::new_i64(-20025, 13380),
                            FixVec::new_i64(-15136, 6269),
                            FixVec::new_i64(-8516, 1694),
                            FixVec::new_i64(-16384, 0),
                            FixVec::new_i64(-23621, -4698)],
                        indices: vec![0, 1, 31, 31, 1, 63, 1, 2, 63, 63, 2, 62, 4, 5, 3, 5, 59, 3, 3, 59, 2, 59, 60, 2, 2, 60, 61, 2, 61, 62, 29, 30, 35, 27, 29, 35, 27, 28, 29, 35, 36, 26, 35, 26, 27, 36, 37, 26, 26, 37, 38, 39, 26, 38, 39, 25, 26, 25, 39, 23, 39, 40, 22, 63, 32, 30, 63, 30, 31, 35, 30, 34, 30, 32, 33, 30, 33, 34, 59, 5, 6, 59, 6, 58, 6, 7, 55, 6, 56, 57, 6, 57, 58, 9, 7, 8, 6, 55, 56, 9, 55, 7, 10, 55, 9, 10, 54, 55, 54, 10, 53, 53, 10, 52, 10, 11, 51, 10, 51, 52, 51, 11, 13, 51, 13, 14, 11, 12, 13, 23, 39, 22, 40, 41, 22, 41, 42, 22, 42, 43, 22, 22, 43, 21, 51, 14, 50, 50, 14, 49, 49, 14, 48, 48, 14, 47, 14, 15, 47, 45, 46, 18, 45, 18, 44, 44, 18, 43, 43, 18, 19, 43, 19, 21, 21, 19, 20, 25, 23, 24, 46, 47, 18, 47, 15, 17, 47, 17, 18, 15, 16, 17],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-14440, -9648),
                                    FixVec::new_i64(-30273, -12539),
                                    FixVec::new_i64(-47243, -9397),
                                    FixVec::new_i64(-32768, 0),
                                    FixVec::new_i64(-23621, -4698)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-16384, 0),
                                    FixVec::new_i64(-23621, -4698),
                                    FixVec::new_i64(-32768, 0),
                                    FixVec::new_i64(-17033, 3388)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-17033, 3388),
                                    FixVec::new_i64(-30273, 12539),
                                    FixVec::new_i64(-40051, 26761),
                                    FixVec::new_i64(-23170, 23170),
                                    FixVec::new_i64(-20025, 13380)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-17033, 3388),
                                    FixVec::new_i64(-20025, 13380),
                                    FixVec::new_i64(-15136, 6269)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-16384, 0),
                                    FixVec::new_i64(-17033, 3388),
                                    FixVec::new_i64(-15136, 6269),
                                    FixVec::new_i64(-8516, 1694)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-26761, -40051),
                                    FixVec::new_i64(-23170, -23170),
                                    FixVec::new_i64(-14440, -9648),
                                    FixVec::new_i64(-13380, -20025),
                                    FixVec::new_i64(-12539, -30273)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-12539, -30273),
                                    FixVec::new_i64(-13380, -20025),
                                    FixVec::new_i64(-6269, -15136),
                                    FixVec::new_i64(-3388, -17033)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -16384),
                                    FixVec::new_i64(-3388, -17033),
                                    FixVec::new_i64(-6269, -15136),
                                    FixVec::new_i64(-1694, -8516)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -32768),
                                    FixVec::new_i64(-3388, -17033),
                                    FixVec::new_i64(0, -16384),
                                    FixVec::new_i64(4698, -23621)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(9648, -14440),
                                    FixVec::new_i64(12539, -30273),
                                    FixVec::new_i64(9397, -47243),
                                    FixVec::new_i64(0, -32768),
                                    FixVec::new_i64(4698, -23621)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(9648, -14440),
                                    FixVec::new_i64(4698, -23621),
                                    FixVec::new_i64(6269, -15136)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-14440, -9648),
                                    FixVec::new_i64(-23621, -4698),
                                    FixVec::new_i64(-15136, -6269)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-11585, -11585),
                                    FixVec::new_i64(-13380, -20025),
                                    FixVec::new_i64(-14440, -9648)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-11585, -11585),
                                    FixVec::new_i64(-14440, -9648),
                                    FixVec::new_i64(-15136, -6269),
                                    FixVec::new_i64(-7220, -4824)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-11585, 11585),
                                    FixVec::new_i64(-20025, 13380),
                                    FixVec::new_i64(-23170, 23170),
                                    FixVec::new_i64(-9648, 14440)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-6269, 15136),
                                    FixVec::new_i64(-9648, 14440),
                                    FixVec::new_i64(-12539, 30273),
                                    FixVec::new_i64(-4698, 23621)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-11585, 11585),
                                    FixVec::new_i64(-9648, 14440),
                                    FixVec::new_i64(-6269, 15136),
                                    FixVec::new_i64(-4824, 7220)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(3388, 17033),
                                    FixVec::new_i64(-4698, 23621),
                                    FixVec::new_i64(-12539, 30273),
                                    FixVec::new_i64(-9397, 47243),
                                    FixVec::new_i64(0, 32768)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-4698, 23621),
                                    FixVec::new_i64(3388, 17033),
                                    FixVec::new_i64(0, 16384)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(6269, 15136),
                                    FixVec::new_i64(1694, 8516),
                                    FixVec::new_i64(0, 16384),
                                    FixVec::new_i64(3388, 17033)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(6269, 15136),
                                    FixVec::new_i64(3388, 17033),
                                    FixVec::new_i64(12539, 30273),
                                    FixVec::new_i64(13380, 20025)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(26761, 40050),
                                    FixVec::new_i64(23170, 23170),
                                    FixVec::new_i64(14440, 9648),
                                    FixVec::new_i64(13380, 20025),
                                    FixVec::new_i64(12539, 30273)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(11585, -11585),
                                    FixVec::new_i64(9648, -14440),
                                    FixVec::new_i64(6269, -15136),
                                    FixVec::new_i64(4824, -7220)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(23170, -23170),
                                    FixVec::new_i64(9648, -14440),
                                    FixVec::new_i64(11585, -11585),
                                    FixVec::new_i64(20025, -13380)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(11585, 11585),
                                    FixVec::new_i64(13380, 20025),
                                    FixVec::new_i64(14440, 9648)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15136, 6269),
                                    FixVec::new_i64(7220, 4824),
                                    FixVec::new_i64(11585, 11585),
                                    FixVec::new_i64(14440, 9648)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30273, 12539),
                                    FixVec::new_i64(23621, 4698),
                                    FixVec::new_i64(15136, 6269),
                                    FixVec::new_i64(14440, 9648)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15136, -6269),
                                    FixVec::new_i64(8516, -1694),
                                    FixVec::new_i64(16384, 0),
                                    FixVec::new_i64(17033, -3388)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30273, -12539),
                                    FixVec::new_i64(20025, -13380),
                                    FixVec::new_i64(15136, -6269),
                                    FixVec::new_i64(17033, -3388)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(40051, -26761),
                                    FixVec::new_i64(23170, -23170),
                                    FixVec::new_i64(20025, -13380),
                                    FixVec::new_i64(30273, -12539)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(32768, 0),
                                    FixVec::new_i64(17033, -3388),
                                    FixVec::new_i64(16384, 0),
                                    FixVec::new_i64(23621, 4698)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(47243, 9397),
                                    FixVec::new_i64(32768, 0),
                                    FixVec::new_i64(23621, 4698),
                                    FixVec::new_i64(30273, 12539)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                69 => {
                    Test {
                        shape: FixShape::new(vec![
                            vec![
                                FixVec::new_i64(0, -18432),
                                FixVec::new_i64(-2867, -20070),
                                FixVec::new_i64(-4915, -20070),
                                FixVec::new_i64(-3686, -18022),
                                FixVec::new_i64(-6553, -18432),
                                FixVec::new_i64(-9420, -17203),
                                FixVec::new_i64(-7782, -15564),
                                FixVec::new_i64(-10649, -15974),
                                FixVec::new_i64(-13107, -13926),
                                FixVec::new_i64(-10649, -13107),
                                FixVec::new_i64(-13926, -12697),
                                FixVec::new_i64(-15564, -11059),
                                FixVec::new_i64(-14336, -10240),
                                FixVec::new_i64(-16384, -9830),
                                FixVec::new_i64(-17612, -7372),
                                FixVec::new_i64(-13107, -6963),
                                FixVec::new_i64(-7372, -4096),
                                FixVec::new_i64(-7782, -1228),
                                FixVec::new_i64(-8601, 819),
                                FixVec::new_i64(-9011, -409),
                                FixVec::new_i64(-10240, -1228),
                                FixVec::new_i64(-10649, 1228),
                                FixVec::new_i64(-11059, 0),
                                FixVec::new_i64(-13107, -1638),
                                FixVec::new_i64(-13107, 1228),
                                FixVec::new_i64(-13926, -409),
                                FixVec::new_i64(-16793, -2048),
                                FixVec::new_i64(-16384, 409),
                                FixVec::new_i64(-18841, -1228),
                                FixVec::new_i64(-18841, 1228),
                                FixVec::new_i64(-21708, -409),
                                FixVec::new_i64(-21299, 1638),
                                FixVec::new_i64(-24166, 409),
                                FixVec::new_i64(-23756, 2867),
                                FixVec::new_i64(-26624, 1638),
                                FixVec::new_i64(-25804, 3686),
                                FixVec::new_i64(-29900, 2457),
                                FixVec::new_i64(-29081, 4505),
                                FixVec::new_i64(-33177, 3686),
                                FixVec::new_i64(-31744, 6553),
                                FixVec::new_i64(-36044, 6144),
                                FixVec::new_i64(-34816, 7372),
                                FixVec::new_i64(-38502, 7782),
                                FixVec::new_i64(-36454, 9420),
                                FixVec::new_i64(-39321, 9011),
                                FixVec::new_i64(-42188, 9420),
                                FixVec::new_i64(-40140, 11059),
                                FixVec::new_i64(-43008, 11059),
                                FixVec::new_i64(-45875, 12288),
                                FixVec::new_i64(-42188, 13926),
                                FixVec::new_i64(-46284, 13926),
                                FixVec::new_i64(-48742, 15564),
                                FixVec::new_i64(-43827, 16793),
                                FixVec::new_i64(-47923, 17612),
                                FixVec::new_i64(-49971, 20070),
                                FixVec::new_i64(-45056, 20070),
                                FixVec::new_i64(-41779, 20480),
                                FixVec::new_i64(-43827, 20889),
                                FixVec::new_i64(-47104, 23961),
                                FixVec::new_i64(-49152, 26214),
                                FixVec::new_i64(-43008, 23756),
                                FixVec::new_i64(-37683, 22528),
                                FixVec::new_i64(-39321, 24166),
                                FixVec::new_i64(-40550, 27033),
                                FixVec::new_i64(-40550, 31129),
                                FixVec::new_i64(-38502, 27033),
                                FixVec::new_i64(-28672, 22937),
                                FixVec::new_i64(-30720, 24985),
                                FixVec::new_i64(-30720, 27443),
                                FixVec::new_i64(-29081, 25395),
                                FixVec::new_i64(-21708, 22528),
                                FixVec::new_i64(-11264, 13926),
                                FixVec::new_i64(-8601, 12697),
                                FixVec::new_i64(-6144, 12697),
                                FixVec::new_i64(-4096, 13107),
                                FixVec::new_i64(-2457, 14336),
                                FixVec::new_i64(0, 14745),
                                FixVec::new_i64(2457, 14336),
                                FixVec::new_i64(4096, 13107),
                                FixVec::new_i64(6144, 12697),
                                FixVec::new_i64(8601, 12697),
                                FixVec::new_i64(11264, 13926),
                                FixVec::new_i64(21708, 22528),
                                FixVec::new_i64(29081, 25395),
                                FixVec::new_i64(30720, 27443),
                                FixVec::new_i64(30720, 24985),
                                FixVec::new_i64(28672, 22937),
                                FixVec::new_i64(38502, 27033),
                                FixVec::new_i64(40550, 31129),
                                FixVec::new_i64(40550, 27033),
                                FixVec::new_i64(39321, 24166),
                                FixVec::new_i64(37683, 22528),
                                FixVec::new_i64(43008, 23756),
                                FixVec::new_i64(49152, 26214),
                                FixVec::new_i64(47104, 23961),
                                FixVec::new_i64(43827, 20889),
                                FixVec::new_i64(41779, 20480),
                                FixVec::new_i64(45056, 20070),
                                FixVec::new_i64(49971, 20070),
                                FixVec::new_i64(47923, 17612),
                                FixVec::new_i64(43827, 16793),
                                FixVec::new_i64(48742, 15564),
                                FixVec::new_i64(46284, 13926),
                                FixVec::new_i64(42188, 13926),
                                FixVec::new_i64(45875, 12288),
                                FixVec::new_i64(43008, 11059),
                                FixVec::new_i64(40140, 11059),
                                FixVec::new_i64(42188, 9420),
                                FixVec::new_i64(39321, 9011),
                                FixVec::new_i64(36454, 9420),
                                FixVec::new_i64(38502, 7782),
                                FixVec::new_i64(34816, 7372),
                                FixVec::new_i64(36044, 6144),
                                FixVec::new_i64(31744, 6553),
                                FixVec::new_i64(33177, 3686),
                                FixVec::new_i64(29081, 4505),
                                FixVec::new_i64(29900, 2457),
                                FixVec::new_i64(25804, 3686),
                                FixVec::new_i64(26624, 1638),
                                FixVec::new_i64(23756, 2867),
                                FixVec::new_i64(24166, 409),
                                FixVec::new_i64(21299, 1638),
                                FixVec::new_i64(21708, -409),
                                FixVec::new_i64(18841, 1228),
                                FixVec::new_i64(18841, -1228),
                                FixVec::new_i64(16384, 409),
                                FixVec::new_i64(16793, -2048),
                                FixVec::new_i64(13926, -409),
                                FixVec::new_i64(13107, 1228),
                                FixVec::new_i64(13107, -1638),
                                FixVec::new_i64(11059, 0),
                                FixVec::new_i64(10649, 1228),
                                FixVec::new_i64(10240, -1228),
                                FixVec::new_i64(9011, -409),
                                FixVec::new_i64(8601, 819),
                                FixVec::new_i64(7782, -1228),
                                FixVec::new_i64(7372, -4096),
                                FixVec::new_i64(13107, -6963),
                                FixVec::new_i64(17612, -7372),
                                FixVec::new_i64(16384, -9830),
                                FixVec::new_i64(14336, -10240),
                                FixVec::new_i64(15564, -11059),
                                FixVec::new_i64(13926, -12697),
                                FixVec::new_i64(10649, -13107),
                                FixVec::new_i64(13107, -13926),
                                FixVec::new_i64(10649, -15974),
                                FixVec::new_i64(7782, -15564),
                                FixVec::new_i64(9420, -17203),
                                FixVec::new_i64(6553, -18432),
                                FixVec::new_i64(3686, -18022),
                                FixVec::new_i64(4915, -20070),
                                FixVec::new_i64(2867, -20070)],
                            vec![
                                FixVec::new_i64(-2457, 9420),
                                FixVec::new_i64(-2457, 8192),
                                FixVec::new_i64(-1638, 7372),
                                FixVec::new_i64(-1228, 8192)],
                            vec![
                                FixVec::new_i64(1228, 8192),
                                FixVec::new_i64(1638, 7372),
                                FixVec::new_i64(2457, 8192),
                                FixVec::new_i64(2457, 9420)],
                            vec![
                                FixVec::new_i64(0, 8192),
                                FixVec::new_i64(-819, 7782),
                                FixVec::new_i64(-1228, 6963),
                                FixVec::new_i64(-1638, 6553),
                                FixVec::new_i64(-3276, 6553),
                                FixVec::new_i64(-1638, 5734),
                                FixVec::new_i64(-819, 4915),
                                FixVec::new_i64(0, 2867),
                                FixVec::new_i64(819, 4915),
                                FixVec::new_i64(1638, 5734),
                                FixVec::new_i64(3276, 6553),
                                FixVec::new_i64(1638, 6553),
                                FixVec::new_i64(1228, 6963),
                                FixVec::new_i64(819, 7782)]]),
                        points: vec![
                            FixVec::new_i64(-45056, 20070),
                            FixVec::new_i64(-41779, 20480),
                            FixVec::new_i64(-43827, 20889),
                            FixVec::new_i64(-47104, 23961),
                            FixVec::new_i64(-49152, 26214),
                            FixVec::new_i64(-43008, 23756),
                            FixVec::new_i64(-37683, 22528),
                            FixVec::new_i64(-39321, 24166),
                            FixVec::new_i64(-40550, 27033),
                            FixVec::new_i64(-40550, 31129),
                            FixVec::new_i64(-38502, 27033),
                            FixVec::new_i64(-28672, 22937),
                            FixVec::new_i64(-30720, 24985),
                            FixVec::new_i64(-30720, 27443),
                            FixVec::new_i64(-29081, 25395),
                            FixVec::new_i64(-21708, 22528),
                            FixVec::new_i64(-11264, 13926),
                            FixVec::new_i64(-8601, 12697),
                            FixVec::new_i64(-6144, 12697),
                            FixVec::new_i64(-4096, 13107),
                            FixVec::new_i64(-2457, 14336),
                            FixVec::new_i64(0, 14745),
                            FixVec::new_i64(2457, 14336),
                            FixVec::new_i64(4096, 13107),
                            FixVec::new_i64(6144, 12697),
                            FixVec::new_i64(8601, 12697),
                            FixVec::new_i64(11264, 13926),
                            FixVec::new_i64(21708, 22528),
                            FixVec::new_i64(29081, 25395),
                            FixVec::new_i64(30720, 27443),
                            FixVec::new_i64(30720, 24985),
                            FixVec::new_i64(28672, 22937),
                            FixVec::new_i64(38502, 27033),
                            FixVec::new_i64(40550, 31129),
                            FixVec::new_i64(40550, 27033),
                            FixVec::new_i64(39321, 24166),
                            FixVec::new_i64(37683, 22528),
                            FixVec::new_i64(43008, 23756),
                            FixVec::new_i64(49152, 26214),
                            FixVec::new_i64(47104, 23961),
                            FixVec::new_i64(43827, 20889),
                            FixVec::new_i64(41779, 20480),
                            FixVec::new_i64(45056, 20070),
                            FixVec::new_i64(49971, 20070),
                            FixVec::new_i64(47923, 17612),
                            FixVec::new_i64(43827, 16793),
                            FixVec::new_i64(48742, 15564),
                            FixVec::new_i64(46284, 13926),
                            FixVec::new_i64(42188, 13926),
                            FixVec::new_i64(45875, 12288),
                            FixVec::new_i64(43008, 11059),
                            FixVec::new_i64(40140, 11059),
                            FixVec::new_i64(42188, 9420),
                            FixVec::new_i64(39321, 9011),
                            FixVec::new_i64(36454, 9420),
                            FixVec::new_i64(38502, 7782),
                            FixVec::new_i64(34816, 7372),
                            FixVec::new_i64(36044, 6144),
                            FixVec::new_i64(31744, 6553),
                            FixVec::new_i64(33177, 3686),
                            FixVec::new_i64(29081, 4505),
                            FixVec::new_i64(29900, 2457),
                            FixVec::new_i64(25804, 3686),
                            FixVec::new_i64(26624, 1638),
                            FixVec::new_i64(23756, 2867),
                            FixVec::new_i64(24166, 409),
                            FixVec::new_i64(21299, 1638),
                            FixVec::new_i64(21708, -409),
                            FixVec::new_i64(18841, 1228),
                            FixVec::new_i64(18841, -1228),
                            FixVec::new_i64(16384, 409),
                            FixVec::new_i64(16793, -2048),
                            FixVec::new_i64(13926, -409),
                            FixVec::new_i64(13107, 1228),
                            FixVec::new_i64(13107, -1638),
                            FixVec::new_i64(11059, 0),
                            FixVec::new_i64(10649, 1228),
                            FixVec::new_i64(10240, -1228),
                            FixVec::new_i64(9011, -409),
                            FixVec::new_i64(8601, 819),
                            FixVec::new_i64(7782, -1228),
                            FixVec::new_i64(7372, -4096),
                            FixVec::new_i64(13107, -6963),
                            FixVec::new_i64(17612, -7372),
                            FixVec::new_i64(16384, -9830),
                            FixVec::new_i64(14336, -10240),
                            FixVec::new_i64(15564, -11059),
                            FixVec::new_i64(13926, -12697),
                            FixVec::new_i64(10649, -13107),
                            FixVec::new_i64(13107, -13926),
                            FixVec::new_i64(10649, -15974),
                            FixVec::new_i64(7782, -15564),
                            FixVec::new_i64(9420, -17203),
                            FixVec::new_i64(6553, -18432),
                            FixVec::new_i64(3686, -18022),
                            FixVec::new_i64(4915, -20070),
                            FixVec::new_i64(2867, -20070),
                            FixVec::new_i64(0, -18432),
                            FixVec::new_i64(-2867, -20070),
                            FixVec::new_i64(-4915, -20070),
                            FixVec::new_i64(-3686, -18022),
                            FixVec::new_i64(-6553, -18432),
                            FixVec::new_i64(-9420, -17203),
                            FixVec::new_i64(-7782, -15564),
                            FixVec::new_i64(-10649, -15974),
                            FixVec::new_i64(-13107, -13926),
                            FixVec::new_i64(-10649, -13107),
                            FixVec::new_i64(-13926, -12697),
                            FixVec::new_i64(-15564, -11059),
                            FixVec::new_i64(-14336, -10240),
                            FixVec::new_i64(-16384, -9830),
                            FixVec::new_i64(-17612, -7372),
                            FixVec::new_i64(-13107, -6963),
                            FixVec::new_i64(-7372, -4096),
                            FixVec::new_i64(-7782, -1228),
                            FixVec::new_i64(-8601, 819),
                            FixVec::new_i64(-9011, -409),
                            FixVec::new_i64(-10240, -1228),
                            FixVec::new_i64(-10649, 1228),
                            FixVec::new_i64(-11059, 0),
                            FixVec::new_i64(-13107, -1638),
                            FixVec::new_i64(-13107, 1228),
                            FixVec::new_i64(-13926, -409),
                            FixVec::new_i64(-16793, -2048),
                            FixVec::new_i64(-16384, 409),
                            FixVec::new_i64(-18841, -1228),
                            FixVec::new_i64(-18841, 1228),
                            FixVec::new_i64(-21708, -409),
                            FixVec::new_i64(-21299, 1638),
                            FixVec::new_i64(-24166, 409),
                            FixVec::new_i64(-23756, 2867),
                            FixVec::new_i64(-26624, 1638),
                            FixVec::new_i64(-25804, 3686),
                            FixVec::new_i64(-29900, 2457),
                            FixVec::new_i64(-29081, 4505),
                            FixVec::new_i64(-33177, 3686),
                            FixVec::new_i64(-31744, 6553),
                            FixVec::new_i64(-36044, 6144),
                            FixVec::new_i64(-34816, 7372),
                            FixVec::new_i64(-38502, 7782),
                            FixVec::new_i64(-36454, 9420),
                            FixVec::new_i64(-39321, 9011),
                            FixVec::new_i64(-42188, 9420),
                            FixVec::new_i64(-40140, 11059),
                            FixVec::new_i64(-43008, 11059),
                            FixVec::new_i64(-45875, 12288),
                            FixVec::new_i64(-42188, 13926),
                            FixVec::new_i64(-46284, 13926),
                            FixVec::new_i64(-48742, 15564),
                            FixVec::new_i64(-43827, 16793),
                            FixVec::new_i64(-47923, 17612),
                            FixVec::new_i64(-49971, 20070),
                            FixVec::new_i64(-3276, 6553),
                            FixVec::new_i64(-1638, 5734),
                            FixVec::new_i64(-819, 4915),
                            FixVec::new_i64(0, 2867),
                            FixVec::new_i64(819, 4915),
                            FixVec::new_i64(1638, 5734),
                            FixVec::new_i64(3276, 6553),
                            FixVec::new_i64(1638, 6553),
                            FixVec::new_i64(1228, 6963),
                            FixVec::new_i64(819, 7782),
                            FixVec::new_i64(0, 8192),
                            FixVec::new_i64(-819, 7782),
                            FixVec::new_i64(-1228, 6963),
                            FixVec::new_i64(-1638, 6553),
                            FixVec::new_i64(-1638, 7372),
                            FixVec::new_i64(-1228, 8192),
                            FixVec::new_i64(-2457, 9420),
                            FixVec::new_i64(-2457, 8192),
                            FixVec::new_i64(1228, 8192),
                            FixVec::new_i64(1638, 7372),
                            FixVec::new_i64(2457, 8192),
                            FixVec::new_i64(2457, 9420)],
                        indices: vec![151, 0, 150, 150, 0, 149, 149, 0, 1, 149, 1, 146, 3, 4, 5, 3, 5, 2, 2, 5, 1, 6, 143, 146, 5, 6, 1, 140, 143, 6, 140, 141, 143, 148, 149, 147, 147, 149, 146, 145, 146, 144, 144, 146, 143, 6, 146, 1, 142, 143, 141, 8, 9, 10, 8, 10, 7, 7, 10, 6, 11, 140, 6, 11, 6, 10, 140, 136, 138, 11, 136, 140, 15, 136, 11, 139, 140, 138, 137, 138, 136, 135, 136, 134, 13, 14, 12, 12, 14, 11, 15, 132, 136, 11, 14, 15, 16, 132, 15, 133, 134, 132, 132, 134, 136, 131, 132, 130, 129, 130, 128, 16, 130, 132, 128, 130, 16, 128, 16, 126, 121, 124, 126, 124, 121, 122, 16, 121, 126, 127, 128, 126, 125, 126, 124, 110, 111, 112, 110, 112, 109, 109, 112, 106, 109, 106, 107, 106, 112, 113, 106, 113, 103, 123, 124, 122, 108, 109, 107, 105, 106, 104, 104, 106, 103, 121, 16, 17, 121, 119, 120, 118, 119, 121, 121, 17, 118, 118, 116, 117, 115, 116, 118, 152, 118, 17, 152, 17, 18, 168, 18, 19, 152, 115, 118, 153, 115, 152, 155, 153, 154, 155, 115, 153, 115, 155, 114, 168, 152, 18, 152, 168, 169, 19, 20, 168, 21, 168, 20, 162, 167, 168, 167, 162, 163, 168, 21, 173, 168, 173, 162, 162, 170, 161, 171, 161, 170, 173, 170, 162, 21, 22, 173, 102, 103, 101, 103, 113, 100, 99, 100, 98, 103, 100, 101, 155, 113, 114, 113, 97, 100, 100, 97, 98, 113, 155, 81, 155, 156, 157, 155, 157, 79, 81, 97, 113, 157, 158, 79, 97, 94, 96, 96, 94, 95, 152, 169, 166, 152, 166, 165, 165, 166, 164, 163, 166, 167, 163, 164, 166, 171, 160, 161, 171, 159, 160, 158, 171, 172, 173, 22, 23, 158, 159, 171, 173, 23, 24, 158, 173, 24, 158, 172, 173, 158, 24, 25, 81, 94, 97, 91, 94, 81, 155, 80, 81, 155, 79, 80, 158, 25, 76, 158, 76, 79, 78, 79, 76, 78, 76, 77, 73, 25, 26, 73, 76, 25, 73, 75, 76, 75, 73, 74, 68, 73, 26, 73, 70, 72, 68, 70, 73, 70, 68, 69, 68, 26, 66, 62, 26, 27, 26, 64, 66, 66, 64, 65, 91, 93, 94, 93, 91, 92, 91, 81, 88, 91, 88, 90, 81, 82, 88, 88, 82, 85, 88, 85, 87, 85, 82, 84, 82, 83, 84, 90, 88, 89, 87, 85, 86, 72, 70, 71, 68, 66, 67, 62, 64, 26, 58, 27, 31, 27, 28, 31, 31, 28, 30, 28, 29, 30, 64, 62, 63, 62, 27, 58, 62, 60, 61, 62, 58, 60, 58, 31, 54, 58, 54, 56, 54, 31, 36, 31, 32, 36, 36, 32, 35, 35, 32, 34, 32, 33, 34, 60, 58, 59, 58, 56, 57, 56, 54, 55, 54, 36, 51, 54, 51, 53, 36, 41, 48, 36, 37, 41, 41, 37, 40, 40, 37, 39, 37, 38, 39, 53, 51, 52, 36, 48, 51, 51, 48, 50, 48, 49, 50, 48, 41, 45, 41, 42, 45, 45, 42, 44, 42, 43, 44, 48, 45, 47, 45, 46, 47],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-43827, 16793),
                                    FixVec::new_i64(-47923, 17612),
                                    FixVec::new_i64(-49971, 20070),
                                    FixVec::new_i64(-45056, 20070)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-42188, 13926),
                                    FixVec::new_i64(-43827, 16793),
                                    FixVec::new_i64(-45056, 20070),
                                    FixVec::new_i64(-41779, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-41779, 20480),
                                    FixVec::new_i64(-43827, 20889),
                                    FixVec::new_i64(-47104, 23961),
                                    FixVec::new_i64(-49152, 26214),
                                    FixVec::new_i64(-43008, 23756)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-28672, 22937),
                                    FixVec::new_i64(-36454, 9420),
                                    FixVec::new_i64(-40140, 11059),
                                    FixVec::new_i64(-42188, 13926),
                                    FixVec::new_i64(-41779, 20480),
                                    FixVec::new_i64(-37683, 22528)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-41779, 20480),
                                    FixVec::new_i64(-43008, 23756),
                                    FixVec::new_i64(-37683, 22528)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-42188, 9420),
                                    FixVec::new_i64(-40140, 11059),
                                    FixVec::new_i64(-36454, 9420),
                                    FixVec::new_i64(-39321, 9011)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-42188, 13926),
                                    FixVec::new_i64(-46284, 13926),
                                    FixVec::new_i64(-48742, 15564),
                                    FixVec::new_i64(-43827, 16793)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-40140, 11059),
                                    FixVec::new_i64(-43008, 11059),
                                    FixVec::new_i64(-45875, 12288),
                                    FixVec::new_i64(-42188, 13926)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-37683, 22528),
                                    FixVec::new_i64(-39321, 24166),
                                    FixVec::new_i64(-40550, 27033),
                                    FixVec::new_i64(-40550, 31129),
                                    FixVec::new_i64(-38502, 27033)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-38502, 27033),
                                    FixVec::new_i64(-28672, 22937),
                                    FixVec::new_i64(-37683, 22528)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-21708, 22528),
                                    FixVec::new_i64(-31744, 6553),
                                    FixVec::new_i64(-34816, 7372),
                                    FixVec::new_i64(-36454, 9420),
                                    FixVec::new_i64(-28672, 22937)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-34816, 7372),
                                    FixVec::new_i64(-38502, 7782),
                                    FixVec::new_i64(-36454, 9420)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-31744, 6553),
                                    FixVec::new_i64(-36044, 6144),
                                    FixVec::new_i64(-34816, 7372)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-29081, 4505),
                                    FixVec::new_i64(-33177, 3686),
                                    FixVec::new_i64(-31744, 6553)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-28672, 22937),
                                    FixVec::new_i64(-30720, 24985),
                                    FixVec::new_i64(-30720, 27443),
                                    FixVec::new_i64(-29081, 25395)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-11264, 13926),
                                    FixVec::new_i64(-25804, 3686),
                                    FixVec::new_i64(-29081, 4505),
                                    FixVec::new_i64(-31744, 6553),
                                    FixVec::new_i64(-21708, 22528)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-21708, 22528),
                                    FixVec::new_i64(-28672, 22937),
                                    FixVec::new_i64(-29081, 25395)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-25804, 3686),
                                    FixVec::new_i64(-29900, 2457),
                                    FixVec::new_i64(-29081, 4505)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-11264, 13926),
                                    FixVec::new_i64(-23756, 2867),
                                    FixVec::new_i64(-26624, 1638),
                                    FixVec::new_i64(-25804, 3686)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-11264, 13926),
                                    FixVec::new_i64(-21299, 1638),
                                    FixVec::new_i64(-24166, 409),
                                    FixVec::new_i64(-23756, 2867)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-21708, -409),
                                    FixVec::new_i64(-21299, 1638),
                                    FixVec::new_i64(-11264, 13926),
                                    FixVec::new_i64(-18841, 1228)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-13926, -409),
                                    FixVec::new_i64(-16384, 409),
                                    FixVec::new_i64(-18841, 1228),
                                    FixVec::new_i64(-11264, 13926),
                                    FixVec::new_i64(-8601, 12697),
                                    FixVec::new_i64(-13107, 1228)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-16384, 409),
                                    FixVec::new_i64(-18841, -1228),
                                    FixVec::new_i64(-18841, 1228)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-14336, -10240),
                                    FixVec::new_i64(-16384, -9830),
                                    FixVec::new_i64(-17612, -7372),
                                    FixVec::new_i64(-13107, -6963)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-7372, -4096),
                                    FixVec::new_i64(-10649, -13107),
                                    FixVec::new_i64(-13926, -12697),
                                    FixVec::new_i64(-14336, -10240),
                                    FixVec::new_i64(-13107, -6963)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10649, -15974),
                                    FixVec::new_i64(-10649, -13107),
                                    FixVec::new_i64(-7372, -4096),
                                    FixVec::new_i64(-7782, -15564)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-13926, -409),
                                    FixVec::new_i64(-16793, -2048),
                                    FixVec::new_i64(-16384, 409)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-13926, -12697),
                                    FixVec::new_i64(-15564, -11059),
                                    FixVec::new_i64(-14336, -10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-10649, -15974),
                                    FixVec::new_i64(-13107, -13926),
                                    FixVec::new_i64(-10649, -13107)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-8601, 12697),
                                    FixVec::new_i64(-10649, 1228),
                                    FixVec::new_i64(-11059, 0),
                                    FixVec::new_i64(-13107, -1638),
                                    FixVec::new_i64(-13107, 1228)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-8601, 819),
                                    FixVec::new_i64(-9011, -409),
                                    FixVec::new_i64(-10240, -1228),
                                    FixVec::new_i64(-10649, 1228)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-8601, 819),
                                    FixVec::new_i64(-10649, 1228),
                                    FixVec::new_i64(-8601, 12697),
                                    FixVec::new_i64(-6144, 12697),
                                    FixVec::new_i64(-2457, 9420),
                                    FixVec::new_i64(-2457, 8192),
                                    FixVec::new_i64(-3276, 6553)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-4096, 13107),
                                    FixVec::new_i64(-2457, 9420),
                                    FixVec::new_i64(-6144, 12697)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-819, 4915),
                                    FixVec::new_i64(0, 2867),
                                    FixVec::new_i64(-7782, -1228),
                                    FixVec::new_i64(-8601, 819),
                                    FixVec::new_i64(-3276, 6553),
                                    FixVec::new_i64(-1638, 5734)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(4096, 13107),
                                    FixVec::new_i64(2457, 9420),
                                    FixVec::new_i64(1228, 8192),
                                    FixVec::new_i64(0, 8192),
                                    FixVec::new_i64(-1228, 8192),
                                    FixVec::new_i64(-2457, 9420),
                                    FixVec::new_i64(-4096, 13107),
                                    FixVec::new_i64(-2457, 14336),
                                    FixVec::new_i64(0, 14745),
                                    FixVec::new_i64(2457, 14336)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-1638, 7372),
                                    FixVec::new_i64(-1228, 8192),
                                    FixVec::new_i64(0, 8192),
                                    FixVec::new_i64(-819, 7782)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(1638, 7372),
                                    FixVec::new_i64(819, 7782),
                                    FixVec::new_i64(0, 8192),
                                    FixVec::new_i64(1228, 8192)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-3686, -18022),
                                    FixVec::new_i64(-6553, -18432),
                                    FixVec::new_i64(-9420, -17203),
                                    FixVec::new_i64(-7782, -15564)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(13107, -6963),
                                    FixVec::new_i64(10649, -13107),
                                    FixVec::new_i64(7782, -15564),
                                    FixVec::new_i64(3686, -18022),
                                    FixVec::new_i64(0, -18432),
                                    FixVec::new_i64(-3686, -18022),
                                    FixVec::new_i64(-7782, -15564),
                                    FixVec::new_i64(-7372, -4096),
                                    FixVec::new_i64(7372, -4096)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(0, -18432),
                                    FixVec::new_i64(-2867, -20070),
                                    FixVec::new_i64(-4915, -20070),
                                    FixVec::new_i64(-3686, -18022)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(7782, -1228),
                                    FixVec::new_i64(7372, -4096),
                                    FixVec::new_i64(-7372, -4096),
                                    FixVec::new_i64(-7782, -1228),
                                    FixVec::new_i64(0, 2867)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(3276, 6553),
                                    FixVec::new_i64(8601, 819),
                                    FixVec::new_i64(7782, -1228),
                                    FixVec::new_i64(0, 2867),
                                    FixVec::new_i64(819, 4915),
                                    FixVec::new_i64(1638, 5734)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(4915, -20070),
                                    FixVec::new_i64(2867, -20070),
                                    FixVec::new_i64(0, -18432),
                                    FixVec::new_i64(3686, -18022)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-1638, 6553),
                                    FixVec::new_i64(-3276, 6553),
                                    FixVec::new_i64(-2457, 8192),
                                    FixVec::new_i64(-1638, 7372)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-819, 7782),
                                    FixVec::new_i64(-1228, 6963),
                                    FixVec::new_i64(-1638, 6553),
                                    FixVec::new_i64(-1638, 7372)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(1638, 6553),
                                    FixVec::new_i64(1228, 6963),
                                    FixVec::new_i64(819, 7782),
                                    FixVec::new_i64(1638, 7372)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(1638, 6553),
                                    FixVec::new_i64(1638, 7372),
                                    FixVec::new_i64(2457, 8192),
                                    FixVec::new_i64(3276, 6553)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(2457, 8192),
                                    FixVec::new_i64(2457, 9420),
                                    FixVec::new_i64(4096, 13107),
                                    FixVec::new_i64(6144, 12697),
                                    FixVec::new_i64(3276, 6553)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(8601, 819),
                                    FixVec::new_i64(3276, 6553),
                                    FixVec::new_i64(6144, 12697),
                                    FixVec::new_i64(8601, 12697),
                                    FixVec::new_i64(10649, 1228)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(10240, -1228),
                                    FixVec::new_i64(9011, -409),
                                    FixVec::new_i64(8601, 819),
                                    FixVec::new_i64(10649, 1228)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(13926, -409),
                                    FixVec::new_i64(13107, 1228),
                                    FixVec::new_i64(8601, 12697),
                                    FixVec::new_i64(11264, 13926),
                                    FixVec::new_i64(18841, 1228),
                                    FixVec::new_i64(16384, 409)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(13107, -1638),
                                    FixVec::new_i64(11059, 0),
                                    FixVec::new_i64(10649, 1228),
                                    FixVec::new_i64(8601, 12697),
                                    FixVec::new_i64(13107, 1228)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(18841, -1228),
                                    FixVec::new_i64(16384, 409),
                                    FixVec::new_i64(18841, 1228)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(21708, -409),
                                    FixVec::new_i64(18841, 1228),
                                    FixVec::new_i64(11264, 13926),
                                    FixVec::new_i64(21299, 1638)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(28672, 22937),
                                    FixVec::new_i64(31744, 6553),
                                    FixVec::new_i64(29081, 4505),
                                    FixVec::new_i64(25804, 3686),
                                    FixVec::new_i64(11264, 13926),
                                    FixVec::new_i64(21708, 22528)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(24166, 409),
                                    FixVec::new_i64(21299, 1638),
                                    FixVec::new_i64(11264, 13926),
                                    FixVec::new_i64(23756, 2867)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(9420, -17203),
                                    FixVec::new_i64(6553, -18432),
                                    FixVec::new_i64(3686, -18022),
                                    FixVec::new_i64(7782, -15564)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(13107, -13926),
                                    FixVec::new_i64(10649, -15974),
                                    FixVec::new_i64(7782, -15564),
                                    FixVec::new_i64(10649, -13107)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(13926, -12697),
                                    FixVec::new_i64(10649, -13107),
                                    FixVec::new_i64(13107, -6963),
                                    FixVec::new_i64(14336, -10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(17612, -7372),
                                    FixVec::new_i64(16384, -9830),
                                    FixVec::new_i64(14336, -10240),
                                    FixVec::new_i64(13107, -6963)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(15564, -11059),
                                    FixVec::new_i64(13926, -12697),
                                    FixVec::new_i64(14336, -10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(16793, -2048),
                                    FixVec::new_i64(13926, -409),
                                    FixVec::new_i64(16384, 409)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(26624, 1638),
                                    FixVec::new_i64(23756, 2867),
                                    FixVec::new_i64(11264, 13926),
                                    FixVec::new_i64(25804, 3686)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30720, 24985),
                                    FixVec::new_i64(28672, 22937),
                                    FixVec::new_i64(21708, 22528),
                                    FixVec::new_i64(29081, 25395)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(30720, 24985),
                                    FixVec::new_i64(29081, 25395),
                                    FixVec::new_i64(30720, 27443)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(29900, 2457),
                                    FixVec::new_i64(25804, 3686),
                                    FixVec::new_i64(29081, 4505)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(37683, 22528),
                                    FixVec::new_i64(36454, 9420),
                                    FixVec::new_i64(34816, 7372),
                                    FixVec::new_i64(31744, 6553),
                                    FixVec::new_i64(28672, 22937)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(40550, 27033),
                                    FixVec::new_i64(39321, 24166),
                                    FixVec::new_i64(37683, 22528),
                                    FixVec::new_i64(28672, 22937),
                                    FixVec::new_i64(38502, 27033)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(40550, 27033),
                                    FixVec::new_i64(38502, 27033),
                                    FixVec::new_i64(40550, 31129)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(33177, 3686),
                                    FixVec::new_i64(29081, 4505),
                                    FixVec::new_i64(31744, 6553)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(36044, 6144),
                                    FixVec::new_i64(31744, 6553),
                                    FixVec::new_i64(34816, 7372)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(38502, 7782),
                                    FixVec::new_i64(34816, 7372),
                                    FixVec::new_i64(36454, 9420)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(39321, 9011),
                                    FixVec::new_i64(36454, 9420),
                                    FixVec::new_i64(37683, 22528),
                                    FixVec::new_i64(40140, 11059)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(43827, 16793),
                                    FixVec::new_i64(42188, 13926),
                                    FixVec::new_i64(40140, 11059),
                                    FixVec::new_i64(37683, 22528),
                                    FixVec::new_i64(41779, 20480)],
                                side: vec![ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(47104, 23961),
                                    FixVec::new_i64(43827, 20889),
                                    FixVec::new_i64(41779, 20480),
                                    FixVec::new_i64(37683, 22528),
                                    FixVec::new_i64(43008, 23756)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(47104, 23961),
                                    FixVec::new_i64(43008, 23756),
                                    FixVec::new_i64(49152, 26214)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(42188, 9420),
                                    FixVec::new_i64(39321, 9011),
                                    FixVec::new_i64(40140, 11059)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(45875, 12288),
                                    FixVec::new_i64(43008, 11059),
                                    FixVec::new_i64(40140, 11059),
                                    FixVec::new_i64(42188, 13926)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(47923, 17612),
                                    FixVec::new_i64(43827, 16793),
                                    FixVec::new_i64(41779, 20480),
                                    FixVec::new_i64(45056, 20070)],
                                side: vec![ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Inner],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(47923, 17612),
                                    FixVec::new_i64(45056, 20070),
                                    FixVec::new_i64(49971, 20070)],
                                side: vec![ConvexSide::Inner, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(48742, 15564),
                                    FixVec::new_i64(46284, 13926),
                                    FixVec::new_i64(42188, 13926),
                                    FixVec::new_i64(43827, 16793)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Inner, ConvexSide::Outer],
                            }],
                    }
                }
                70 => {
                    Test {
                        shape: FixShape::new_with_contour(vec![
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(-5120, 5120),
                            FixVec::new_i64(-5120, 10240),
                            FixVec::new_i64(5120, 10240),
                            FixVec::new_i64(5120, 5120),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5120, -5120),
                            FixVec::new_i64(5120, -10240),
                            FixVec::new_i64(-5120, -10240),
                            FixVec::new_i64(-5120, -5120)]),
                        points: vec![
                            FixVec::new_i64(-5120, -10240),
                            FixVec::new_i64(-5120, -5120),
                            FixVec::new_i64(0, 0),
                            FixVec::new_i64(5120, -5120),
                            FixVec::new_i64(5120, -10240),
                            FixVec::new_i64(-5120, 5120),
                            FixVec::new_i64(-5120, 10240),
                            FixVec::new_i64(5120, 10240),
                            FixVec::new_i64(5120, 5120),
                            FixVec::new_i64(0, 0)],
                        indices: vec![3, 1, 2, 1, 4, 0, 3, 4, 1, 5, 6, 8, 5, 8, 9, 6, 7, 8],
                        polygons: vec![
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(-5120, -10240),
                                    FixVec::new_i64(-5120, -5120),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(5120, -5120),
                                    FixVec::new_i64(5120, -10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            },
                            ConvexPath {
                                path: vec![
                                    FixVec::new_i64(5120, 10240),
                                    FixVec::new_i64(5120, 5120),
                                    FixVec::new_i64(0, 0),
                                    FixVec::new_i64(-5120, 5120),
                                    FixVec::new_i64(-5120, 10240)],
                                side: vec![ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer, ConvexSide::Outer],
                            }],
                    }
                }
                _ => {
                    panic!("test not exist")
                }
            }
        }
    }
}
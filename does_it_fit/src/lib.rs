
pub use areas_volumes::GeometricalShapes;
pub use areas_volumes::GeometricalVolumes;
use areas_volumes::*;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes, 
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let area = x * y;
    let object_area = match objects {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };
    (object_area * times as f64) <= area as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes, // Now you can use the re-exported type directly
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let volume = x * y * z;
    let object_volume = match objects {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::Pyramid => triangular_pyramid_volume(a as f64, b),
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
    };
    (object_volume * times as f64) <= volume as f64
}
use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let container_area = areas_volumes::rectangle_area(x, y);
    
    let single_object_area = match objects {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
    };
    
    // Calculate the total area needed for all objects
    let total_area_needed = single_object_area * times as f64;
    
    // Check if the objects can fit
    total_area_needed <= container_area as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let container_volume = areas_volumes::parallelepiped_volume(x, y, z);
    
    let single_object_volume = match objects {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        GeometricalVolumes::Pyramid => {
            let base_area = a as f64; 
            areas_volumes::triangular_pyramid_volume(base_area, b)
        },
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
    };
    
    let total_volume_needed = single_object_volume * times as f64;
    
    total_volume_needed <= container_volume as f64
}
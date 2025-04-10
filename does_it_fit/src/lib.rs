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
        areas_volumes::GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        areas_volumes::GeometricalShapes::Circle => areas_volumes::circle_area(a),
        areas_volumes::GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        areas_volumes::GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
    };
    
    let total_area_needed = single_object_area * times as f64;
    
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
        areas_volumes::GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        areas_volumes::GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        areas_volumes::GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        areas_volumes::GeometricalVolumes::Pyramid => {
            let base_area = a as f64; 
            areas_volumes::triangular_pyramid_volume(base_area, b)
        },
        areas_volumes::GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
    };
    
    let total_volume_needed = single_object_volume * times as f64;
    
    total_volume_needed <= container_volume as f64
}